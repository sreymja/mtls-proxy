use crate::config::Config;
use hyper::{Request, Response, StatusCode};
use http_body_util;
use serde_json;
use std::convert::Infallible;
use std::sync::Arc;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::Serialize;

// In-memory storage for request/response logs (for demo purposes)
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::ui::{templates, static_files};

static REQUEST_LOGS: Lazy<Mutex<Vec<RequestLogEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug, Clone, Serialize)]
pub struct RequestLogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub response_status: u16,
    pub response_body: Option<String>,
    pub response_time_ms: u64,
    pub client_ip: String,
}

pub async fn dashboard_handler(
    _req: Request<http_body_util::Full<hyper::body::Bytes>>,
    config: Arc<Config>
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let stats = get_dashboard_stats().await;
    
    let html = templates::dashboard_template(&stats, &config);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(hyper::body::Bytes::from(html).into())
        .unwrap())
}

pub async fn requests_handler(
    req: Request<http_body_util::Full<hyper::body::Bytes>>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let query = req.uri().query().unwrap_or("");
    let params = parse_query_params(query);
    
    let limit = params.get("limit").and_then(|s| s.parse::<usize>().ok()).unwrap_or(50);
    let offset = params.get("offset").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let method = params.get("method").cloned();
    let status_code = params.get("status_code").and_then(|s| s.parse::<u16>().ok());
    
    let logs = get_filtered_logs(method.as_deref(), status_code, limit, offset).await;
    
    let html = templates::requests_template(&logs, &params);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(hyper::body::Bytes::from(html).into())
        .unwrap())
}

pub async fn request_detail_handler(
    req: Request<http_body_util::Full<hyper::body::Bytes>>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let path = req.uri().path();
    let request_id = path.split('/').last().unwrap_or("");

    let log_entry = get_request_by_id(request_id).await;

    let html = templates::request_detail_template(&log_entry);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(hyper::body::Bytes::from(html).into())
        .unwrap())
}

pub async fn health_handler(
    _req: Request<http_body_util::Full<hyper::body::Bytes>>,
    config: Arc<Config>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let health_status = get_health_status(config).await;
    
    let html = templates::health_template(&health_status);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(hyper::body::Bytes::from(html).into())
        .unwrap())
}

pub async fn api_requests_handler(
    req: Request<http_body_util::Full<hyper::body::Bytes>>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let query = req.uri().query().unwrap_or("");
    let params = parse_query_params(query);
    
    let limit = params.get("limit").and_then(|s| s.parse::<usize>().ok()).unwrap_or(50);
    let offset = params.get("offset").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let method = params.get("method").cloned();
    let status_code = params.get("status_code").and_then(|s| s.parse::<u16>().ok());
    
    let logs = get_filtered_logs(method.as_deref(), status_code, limit, offset).await;
    
    let json = serde_json::to_string(&logs).unwrap_or_else(|_| "[]".to_string());
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(hyper::body::Bytes::from(json).into())
        .unwrap())
}

pub async fn api_stats_handler(
    _req: Request<http_body_util::Full<hyper::body::Bytes>>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let stats = get_dashboard_stats().await;
    
    let json = serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string());
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(hyper::body::Bytes::from(json).into())
        .unwrap())
}

pub async fn static_file_handler(
    req: Request<http_body_util::Full<hyper::body::Bytes>>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    let path = req.uri().path();
    
    let (content, content_type) = match path {
        "/ui/static/style.css" => (static_files::CSS, "text/css"),
        "/ui/static/script.js" => (static_files::JS, "application/javascript"),
        "/ui/static/favicon.ico" => (std::str::from_utf8(static_files::FAVICON).unwrap_or(""), "image/x-icon"),
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(hyper::body::Bytes::from("404 Not Found").into())
                .unwrap());
        }
    };
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .body(hyper::body::Bytes::from(content).into())
        .unwrap())
}

// Public function to log requests (called from main request handlers)
pub fn log_request(entry: RequestLogEntry) {
    if let Ok(mut logs) = REQUEST_LOGS.lock() {
        logs.push(entry);
        // Keep only last 1000 requests to prevent memory issues
        if logs.len() > 1000 {
            logs.remove(0);
        }
    }
}

// Helper functions

async fn get_dashboard_stats() -> serde_json::Value {
    let logs = if let Ok(logs) = REQUEST_LOGS.lock() {
        logs.clone()
    } else {
        Vec::new()
    };
    
    let now = Utc::now();
    let one_hour_ago = now - Duration::hours(1);
    let one_day_ago = now - Duration::days(1);
    
    let recent_logs: Vec<_> = logs.iter()
        .filter(|log| log.timestamp >= one_hour_ago)
        .collect();
    
    let daily_logs: Vec<_> = logs.iter()
        .filter(|log| log.timestamp >= one_day_ago)
        .collect();
    
    let total_requests = recent_logs.len();
    let successful_requests = recent_logs.iter()
        .filter(|log| log.response_status < 400)
        .count();
    let error_requests = total_requests - successful_requests;
    
    let avg_response_time = if !recent_logs.is_empty() {
        let total_time: u64 = recent_logs.iter()
            .map(|log| log.response_time_ms)
            .sum();
        total_time as f64 / recent_logs.len() as f64
    } else {
        0.0
    };
    
    let requests_per_hour = daily_logs.len() as f64 / 24.0;
    
    // Method distribution
    let mut method_counts = HashMap::new();
    for log in &logs {
        *method_counts.entry(log.method.clone()).or_insert(0) += 1;
    }
    
    // Status code distribution
    let mut status_counts = HashMap::new();
    for log in &logs {
        *status_counts.entry(log.response_status).or_insert(0) += 1;
    }
    
    serde_json::json!({
        "total_requests": total_requests,
        "successful_requests": successful_requests,
        "error_requests": error_requests,
        "success_rate": if total_requests > 0 { (successful_requests as f64 / total_requests as f64) * 100.0 } else { 0.0 },
        "avg_response_time": avg_response_time,
        "requests_per_hour": requests_per_hour,
        "method_distribution": method_counts,
        "status_distribution": status_counts,
        "last_updated": now.to_rfc3339()
    })
}

async fn get_health_status(config: Arc<Config>) -> serde_json::Value {
    let now = Utc::now();
    let five_minutes_ago = now - Duration::minutes(5);
    
    let logs = if let Ok(logs) = REQUEST_LOGS.lock() {
        logs.clone()
    } else {
        Vec::new()
    };
    
    let recent_logs: Vec<_> = logs.iter()
        .filter(|log| log.timestamp >= five_minutes_ago)
        .collect();
    
    let is_healthy = !recent_logs.is_empty();
    let last_request = recent_logs.first()
        .map(|log| log.timestamp.to_rfc3339())
        .unwrap_or_else(|| "Never".to_string());
    
    serde_json::json!({
        "status": if is_healthy { "healthy" } else { "unhealthy" },
        "last_request": last_request,
        "uptime": "Running", // TODO: Add actual uptime tracking
        "version": env!("CARGO_PKG_VERSION"),
        "config": {
            "server_host": config.server.host,
            "server_port": config.server.port,
            "max_connections": config.server.max_connections,
            "available_models": config.models.available,
            "default_delay_ms": config.responses.default_delay_ms,
            "error_rate_percent": config.responses.error_rate_percent
        }
    })
}

async fn get_filtered_logs(
    method: Option<&str>,
    status_code: Option<u16>,
    limit: usize,
    offset: usize,
) -> Vec<RequestLogEntry> {
    let logs = if let Ok(logs) = REQUEST_LOGS.lock() {
        logs.clone()
    } else {
        Vec::new()
    };
    
    let mut filtered_logs: Vec<_> = logs.iter()
        .filter(|log| {
            if let Some(m) = method {
                if log.method != m {
                    return false;
                }
            }
            if let Some(status) = status_code {
                if log.response_status != status {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();
    
    // Sort by timestamp (newest first)
    filtered_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Apply pagination
    filtered_logs.into_iter()
        .skip(offset)
        .take(limit)
        .collect()
}

async fn get_request_by_id(request_id: &str) -> Option<RequestLogEntry> {
    let logs = if let Ok(logs) = REQUEST_LOGS.lock() {
        logs.clone()
    } else {
        Vec::new()
    };
    
    logs.into_iter()
        .find(|log| log.id == request_id)
}

fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }
    
    params
}
