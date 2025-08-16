use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    ConfigUpdate,
    CertificateUpload,
    CertificateDelete,
    ConfigValidation,
    ServerStart,
    ServerStop,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditEventType::ConfigUpdate => write!(f, "config_update"),
            AuditEventType::CertificateUpload => write!(f, "certificate_upload"),
            AuditEventType::CertificateDelete => write!(f, "certificate_delete"),
            AuditEventType::ConfigValidation => write!(f, "config_validation"),
            AuditEventType::ServerStart => write!(f, "server_start"),
            AuditEventType::ServerStop => write!(f, "server_stop"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub details: String,
    pub user: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Clone)]
pub struct AuditLogger {
    db_path: PathBuf,
}

impl AuditLogger {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let logger = Self { db_path };
        logger.init_database()?;
        Ok(logger)
    }

    fn init_database(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                event_type TEXT NOT NULL,
                details TEXT NOT NULL,
                user TEXT,
                ip_address TEXT
            )",
            [],
        )?;

        // Create index for faster queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_logs(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_event_type ON audit_logs(event_type)",
            [],
        )?;

        Ok(())
    }

    pub async fn log_event(
        &self,
        event_type: AuditEventType,
        details: String,
        user: Option<String>,
        ip_address: Option<String>,
    ) -> Result<()> {
        let timestamp = Utc::now();
        let event_type_str = event_type.to_string();
        let event_type_str_clone = event_type_str.clone();
        let details_clone = details.clone();

        // Use tokio::task::spawn_blocking for database operations
        let db_path = self.db_path.clone();
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(&db_path)?;
            conn.execute(
                "INSERT INTO audit_logs (timestamp, event_type, details, user, ip_address) 
                 VALUES (?, ?, ?, ?, ?)",
                params![
                    timestamp.to_rfc3339(),
                    event_type_str,
                    details,
                    user,
                    ip_address,
                ],
            )?;
            Ok::<(), anyhow::Error>(())
        }).await??;

        info!("Audit log: {} - {}", event_type_str_clone, details_clone);
        Ok(())
    }

    pub async fn get_audit_logs(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        event_type: Option<AuditEventType>,
    ) -> Result<Vec<AuditLog>> {
        let db_path = self.db_path.clone();
        let event_type_str = event_type.map(|et| et.to_string());
        
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(&db_path)?;
            
            let mut query = "SELECT id, timestamp, event_type, details, user, ip_address 
                           FROM audit_logs ORDER BY timestamp DESC".to_string();
            
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
            
            if let Some(et) = event_type_str {
                query.push_str(" WHERE event_type = ?");
                params.push(Box::new(et));
            }
            
            if let Some(lim) = limit {
                query.push_str(&format!(" LIMIT {}", lim));
            }
            
            if let Some(off) = offset {
                query.push_str(&format!(" OFFSET {}", off));
            }
            
            let mut stmt = conn.prepare(&query)?;
            let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
                let event_type_str: String = row.get(2)?;
                let event_type = match event_type_str.as_str() {
                    "config_update" => AuditEventType::ConfigUpdate,
                    "certificate_upload" => AuditEventType::CertificateUpload,
                    "certificate_delete" => AuditEventType::CertificateDelete,
                    "config_validation" => AuditEventType::ConfigValidation,
                    "server_start" => AuditEventType::ServerStart,
                    "server_stop" => AuditEventType::ServerStop,
                    _ => AuditEventType::ConfigUpdate, // Default fallback
                };
                
                Ok(AuditLog {
                    id: Some(row.get(0)?),
                    timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(1)?)
                        .unwrap_or_else(|_| Utc::now().into())
                        .with_timezone(&Utc),
                    event_type,
                    details: row.get(3)?,
                    user: row.get(4)?,
                    ip_address: row.get(5)?,
                })
            })?;
            
            let mut logs = Vec::new();
            for row in rows {
                logs.push(row?);
            }
            
            Ok(logs)
        }).await?
    }

    pub async fn get_audit_stats(&self) -> Result<AuditStats> {
        let db_path = self.db_path.clone();
        
        tokio::task::spawn_blocking(move || {
            let conn = Connection::open(&db_path)?;
            
            let total_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audit_logs",
                [],
                |row| row.get(0),
            )?;
            
            let today_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audit_logs WHERE date(timestamp) = date('now')",
                [],
                |row| row.get(0),
            )?;
            
            let config_updates: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audit_logs WHERE event_type = 'config_update'",
                [],
                |row| row.get(0),
            )?;
            
            let certificate_ops: i64 = conn.query_row(
                "SELECT COUNT(*) FROM audit_logs WHERE event_type IN ('certificate_upload', 'certificate_delete')",
                [],
                |row| row.get(0),
            )?;
            
            Ok(AuditStats {
                total_events: total_count,
                events_today: today_count,
                config_updates,
                certificate_operations: certificate_ops,
            })
        }).await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_events: i64,
    pub events_today: i64,
    pub config_updates: i64,
    pub certificate_operations: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_audit_logger() -> (AuditLogger, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_audit.db");
        let audit_logger = AuditLogger::new(db_path).unwrap();
        (audit_logger, temp_dir)
    }

    #[tokio::test]
    async fn test_audit_logger_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_audit.db");
        
        let audit_logger = AuditLogger::new(db_path.clone()).unwrap();
        assert_eq!(audit_logger.db_path, db_path);
        
        // Verify database was created
        assert!(db_path.exists());
    }

    #[tokio::test]
    async fn test_log_event() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        let result = audit_logger.log_event(
            AuditEventType::ConfigUpdate,
            "Configuration updated".to_string(),
            Some("test_user".to_string()),
            Some("127.0.0.1".to_string()),
        ).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_log_multiple_events() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        // Log multiple events
        audit_logger.log_event(
            AuditEventType::ConfigUpdate,
            "Configuration updated".to_string(),
            None,
            None,
        ).await.unwrap();
        
        audit_logger.log_event(
            AuditEventType::CertificateUpload,
            "Certificate uploaded".to_string(),
            None,
            None,
        ).await.unwrap();
        
        audit_logger.log_event(
            AuditEventType::CertificateDelete,
            "Certificate deleted".to_string(),
            None,
            None,
        ).await.unwrap();
        
        // Verify all events were logged
        let logs = audit_logger.get_audit_logs(None, None, None).await.unwrap();
        assert_eq!(logs.len(), 3);
        
        // Check that all expected event types are present (order may vary)
        let event_types: Vec<_> = logs.iter().map(|log| &log.event_type).collect();
        assert!(event_types.contains(&&AuditEventType::ConfigUpdate));
        assert!(event_types.contains(&&AuditEventType::CertificateUpload));
        assert!(event_types.contains(&&AuditEventType::CertificateDelete));
    }

    #[tokio::test]
    async fn test_get_audit_logs() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        // Log some events
        audit_logger.log_event(
            AuditEventType::ConfigUpdate,
            "Configuration updated".to_string(),
            None,
            None,
        ).await.unwrap();
        
        audit_logger.log_event(
            AuditEventType::CertificateUpload,
            "Certificate uploaded".to_string(),
            None,
            None,
        ).await.unwrap();
        
        // Get all logs
        let logs = audit_logger.get_audit_logs(None, None, None).await.unwrap();
        assert_eq!(logs.len(), 2);
        
        // Verify log structure
        let config_log = logs.iter().find(|log| log.event_type == AuditEventType::ConfigUpdate).unwrap();
        assert_eq!(config_log.details, "Configuration updated");
        assert!(config_log.user.is_none());
        assert!(config_log.ip_address.is_none());
        assert!(config_log.id.is_some());
    }

    #[tokio::test]
    async fn test_get_audit_logs_with_limit() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        // Log multiple events
        for i in 0..5 {
            audit_logger.log_event(
                AuditEventType::ConfigUpdate,
                format!("Configuration update {}", i),
                None,
                None,
            ).await.unwrap();
        }
        
        // Get logs with limit
        let logs = audit_logger.get_audit_logs(Some(3), None, None).await.unwrap();
        assert_eq!(logs.len(), 3);
    }

    #[tokio::test]
    async fn test_get_audit_logs_with_offset() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        // Log multiple events
        for i in 0..5 {
            audit_logger.log_event(
                AuditEventType::ConfigUpdate,
                format!("Configuration update {}", i),
                None,
                None,
            ).await.unwrap();
        }
        
        // Get logs with offset
        let logs = audit_logger.get_audit_logs(Some(3), Some(2), None).await.unwrap();
        assert_eq!(logs.len(), 3);
    }

    #[tokio::test]
    async fn test_get_audit_stats() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        // Log events of different types
        audit_logger.log_event(
            AuditEventType::ConfigUpdate,
            "Configuration updated".to_string(),
            None,
            None,
        ).await.unwrap();
        
        audit_logger.log_event(
            AuditEventType::CertificateUpload,
            "Certificate uploaded".to_string(),
            None,
            None,
        ).await.unwrap();
        
        audit_logger.log_event(
            AuditEventType::CertificateDelete,
            "Certificate deleted".to_string(),
            None,
            None,
        ).await.unwrap();
        
        // Get stats
        let stats = audit_logger.get_audit_stats().await.unwrap();
        
        assert_eq!(stats.total_events, 3);
        assert_eq!(stats.events_today, 3);
        assert_eq!(stats.config_updates, 1);
        assert_eq!(stats.certificate_operations, 2);
    }

    #[tokio::test]
    async fn test_audit_event_type_display() {
        assert_eq!(AuditEventType::ConfigUpdate.to_string(), "config_update");
        assert_eq!(AuditEventType::CertificateUpload.to_string(), "certificate_upload");
        assert_eq!(AuditEventType::CertificateDelete.to_string(), "certificate_delete");
        assert_eq!(AuditEventType::ConfigValidation.to_string(), "config_validation");
        assert_eq!(AuditEventType::ServerStart.to_string(), "server_start");
        assert_eq!(AuditEventType::ServerStop.to_string(), "server_stop");
    }

    #[tokio::test]
    async fn test_audit_log_serialization() {
        let audit_log = AuditLog {
            id: Some(1),
            timestamp: Utc::now(),
            event_type: AuditEventType::ConfigUpdate,
            details: "Configuration updated".to_string(),
            user: Some("test_user".to_string()),
            ip_address: Some("127.0.0.1".to_string()),
        };
        
        let json = serde_json::to_string(&audit_log).unwrap();
        let deserialized: AuditLog = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.id, audit_log.id);
        assert_eq!(deserialized.event_type, audit_log.event_type);
        assert_eq!(deserialized.details, audit_log.details);
        assert_eq!(deserialized.user, audit_log.user);
        assert_eq!(deserialized.ip_address, audit_log.ip_address);
    }

    #[tokio::test]
    async fn test_audit_stats_serialization() {
        let stats = AuditStats {
            total_events: 10,
            events_today: 5,
            config_updates: 3,
            certificate_operations: 7,
        };
        
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: AuditStats = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.total_events, stats.total_events);
        assert_eq!(deserialized.events_today, stats.events_today);
        assert_eq!(deserialized.config_updates, stats.config_updates);
        assert_eq!(deserialized.certificate_operations, stats.certificate_operations);
    }

    #[tokio::test]
    async fn test_audit_logger_with_user_and_ip() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        let result = audit_logger.log_event(
            AuditEventType::ConfigUpdate,
            "Configuration updated by user".to_string(),
            Some("admin".to_string()),
            Some("192.168.1.100".to_string()),
        ).await;
        
        assert!(result.is_ok());
        
        let logs = audit_logger.get_audit_logs(None, None, None).await.unwrap();
        assert_eq!(logs.len(), 1);
        
        let log = &logs[0];
        assert_eq!(log.user, Some("admin".to_string()));
        assert_eq!(log.ip_address, Some("192.168.1.100".to_string()));
    }

    #[tokio::test]
    async fn test_audit_logger_error_handling() {
        // Test with invalid database path
        let invalid_path = PathBuf::from("/invalid/path/test.db");
        let result = AuditLogger::new(invalid_path);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_logger_concurrent_access() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        // Spawn multiple tasks to log events concurrently
        let mut handles = vec![];
        
        for i in 0..10 {
            let audit_logger_clone = audit_logger.clone();
            let handle = tokio::spawn(async move {
                audit_logger_clone.log_event(
                    AuditEventType::ConfigUpdate,
                    format!("Concurrent update {}", i),
                    None,
                    None,
                ).await
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
        
        // Verify all events were logged
        let logs = audit_logger.get_audit_logs(None, None, None).await.unwrap();
        assert_eq!(logs.len(), 10);
    }

    #[tokio::test]
    async fn test_audit_logger_empty_stats() {
        let (audit_logger, _temp_dir) = create_test_audit_logger();
        
        let stats = audit_logger.get_audit_stats().await.unwrap();
        
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.events_today, 0);
        assert_eq!(stats.config_updates, 0);
        assert_eq!(stats.certificate_operations, 0);
    }
}
