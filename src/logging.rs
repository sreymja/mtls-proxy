use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestLog {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub uri: String,
    pub headers: String,
    pub body_size: usize,
    pub client_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLog {
    pub request_id: String,
    pub timestamp: DateTime<Utc>,
    pub status_code: u16,
    pub headers: String,
    pub body_size: usize,
    pub duration_ms: u64,
}

pub struct LogManager {
    pub(crate) conn: Arc<Mutex<Connection>>,
    pub(crate) log_dir: String,
    pub(crate) max_log_size_mb: u64,
    pub(crate) retention_days: u32,
    pub(crate) compression_enabled: bool,
}

impl LogManager {
    pub fn new(db_path: &Path, log_dir: &str, max_log_size_mb: u64, retention_days: u32, compression_enabled: bool) -> Result<Self> {
        // Ensure log directory exists
        std::fs::create_dir_all(log_dir)?;
        
        // Open SQLite connection with WAL mode for better concurrency
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        
        // Enable WAL mode for better concurrency
        let _: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
        
        // Create tables if they don't exist
        Self::create_tables(&conn)?;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            log_dir: log_dir.to_string(),
            max_log_size_mb,
            retention_days,
            compression_enabled,
        })
    }

    fn create_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                method TEXT NOT NULL,
                uri TEXT NOT NULL,
                headers TEXT NOT NULL,
                body_size INTEGER NOT NULL,
                client_ip TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS responses (
                request_id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                status_code INTEGER NOT NULL,
                headers TEXT NOT NULL,
                body_size INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                FOREIGN KEY (request_id) REFERENCES requests (id)
            )",
            [],
        )?;

        // Create indexes for efficient querying
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_requests_timestamp ON requests (timestamp)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_responses_timestamp ON responses (timestamp)",
            [],
        )?;

        Ok(())
    }

    pub async fn log_request(&self, request: RequestLog) -> Result<()> {
        let conn = self.conn.lock().await;
        
        conn.execute(
            "INSERT INTO requests (id, timestamp, method, uri, headers, body_size, client_ip) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &request.id,
                &request.timestamp.to_rfc3339(),
                &request.method,
                &request.uri,
                &request.headers,
                request.body_size as i64,
                &request.client_ip,
            ),
        )?;

        Ok(())
    }

    pub async fn log_response(&self, response: ResponseLog) -> Result<()> {
        let conn = self.conn.lock().await;
        
        conn.execute(
            "INSERT INTO responses (request_id, timestamp, status_code, headers, body_size, duration_ms) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &response.request_id,
                &response.timestamp.to_rfc3339(),
                response.status_code as i64,
                &response.headers,
                response.body_size as i64,
                response.duration_ms as i64,
            ),
        )?;

        Ok(())
    }

    pub async fn cleanup_old_logs(&self) -> Result<()> {
        let cutoff_date = Utc::now() - chrono::Duration::days(self.retention_days as i64);
        let conn = self.conn.lock().await;
        
        // Delete old responses first (due to foreign key constraint)
        conn.execute(
            "DELETE FROM responses WHERE timestamp < ?",
            [&cutoff_date.to_rfc3339()],
        )?;
        
        // Delete old requests
        conn.execute(
            "DELETE FROM requests WHERE timestamp < ?",
            [&cutoff_date.to_rfc3339()],
        )?;

        // Vacuum to reclaim space
        conn.execute("VACUUM", [])?;

        Ok(())
    }

    pub async fn get_request_by_id(&self, request_id: &str) -> Result<Option<RequestLog>> {
        let conn = self.conn.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, method, uri, headers, body_size, client_ip 
             FROM requests WHERE id = ?"
        )?;
        
        let mut rows = stmt.query([request_id])?;
        
        if let Some(row) = rows.next()? {
            let timestamp: String = row.get(1)?;
            Ok(Some(RequestLog {
                id: row.get(0)?,
                timestamp: DateTime::parse_from_rfc3339(&timestamp)?.with_timezone(&Utc),
                method: row.get(2)?,
                uri: row.get(3)?,
                headers: row.get(4)?,
                body_size: row.get(5)?,
                client_ip: row.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_response_by_request_id(&self, request_id: &str) -> Result<Option<ResponseLog>> {
        let conn = self.conn.lock().await;
        
        let mut stmt = conn.prepare(
            "SELECT request_id, timestamp, status_code, headers, body_size, duration_ms 
             FROM responses WHERE request_id = ?"
        )?;
        
        let mut rows = stmt.query([request_id])?;
        
        if let Some(row) = rows.next()? {
            let timestamp: String = row.get(1)?;
            Ok(Some(ResponseLog {
                request_id: row.get(0)?,
                timestamp: DateTime::parse_from_rfc3339(&timestamp)?.with_timezone(&Utc),
                status_code: row.get(2)?,
                headers: row.get(3)?,
                body_size: row.get(4)?,
                duration_ms: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn search_logs(
        &self,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        method: Option<&str>,
        status_code: Option<u16>,
        limit: Option<usize>,
    ) -> Result<Vec<(RequestLog, Option<ResponseLog>)>> {
        let conn = self.conn.lock().await;
        
        let mut query = String::from(
            "SELECT r.id, r.timestamp, r.method, r.uri, r.headers, r.body_size, r.client_ip,
                    resp.timestamp, resp.status_code, resp.headers, resp.body_size, resp.duration_ms
             FROM requests r
             LEFT JOIN responses resp ON r.id = resp.request_id
             WHERE 1=1"
        );
        
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        let mut param_count = 0;
        
        if let Some(start) = start_time {
            param_count += 1;
            query.push_str(&format!(" AND r.timestamp >= ?{}", param_count));
            params.push(Box::new(start.to_rfc3339()));
        }
        
        if let Some(end) = end_time {
            param_count += 1;
            query.push_str(&format!(" AND r.timestamp <= ?{}", param_count));
            params.push(Box::new(end.to_rfc3339()));
        }
        
        if let Some(m) = method {
            param_count += 1;
            query.push_str(&format!(" AND r.method = ?{}", param_count));
            params.push(Box::new(m.to_string()));
        }
        
        if let Some(status) = status_code {
            param_count += 1;
            query.push_str(&format!(" AND resp.status_code = ?{}", param_count));
            params.push(Box::new(status as i64));
        }
        
        query.push_str(" ORDER BY r.timestamp DESC");
        
        if let Some(l) = limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ?{}", param_count));
            params.push(Box::new(l as i64));
        }
        
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query(rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())))?;
        
        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            let req_timestamp: String = row.get(1)?;
            let request = RequestLog {
                id: row.get(0)?,
                timestamp: DateTime::parse_from_rfc3339(&req_timestamp)?.with_timezone(&Utc),
                method: row.get(2)?,
                uri: row.get(3)?,
                headers: row.get(4)?,
                body_size: row.get(5)?,
                client_ip: row.get(6)?,
            };
            
            let response = if let Ok(resp_timestamp) = row.get::<_, String>(7) {
                Some(ResponseLog {
                    request_id: request.id.clone(),
                    timestamp: DateTime::parse_from_rfc3339(&resp_timestamp)?.with_timezone(&Utc),
                    status_code: row.get(8)?,
                    headers: row.get(9)?,
                    body_size: row.get(10)?,
                    duration_ms: row.get(11)?,
                })
            } else {
                None
            };
            
            results.push((request, response));
        }
        
        Ok(results)
    }
}
