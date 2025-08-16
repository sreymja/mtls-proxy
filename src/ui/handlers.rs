use crate::config::Config;
use crate::logging::LogManager;
use crate::ui::static_files;
use crate::ui::templates;
use chrono::{Duration, Utc};
use hyper::{Body, Request, Response, StatusCode};
use serde_json;
use std::convert::Infallible;
use std::sync::Arc;

pub async fn dashboard_handler(
    _req: Request<Body>,
    log_manager: Arc<LogManager>,
) -> Result<Response<Body>, Infallible> {
    let stats = get_dashboard_stats(log_manager).await;

    let html = templates::dashboard_template(&stats);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(html))
        .unwrap())
}

pub async fn logs_handler(
    req: Request<Body>,
    log_manager: Arc<LogManager>,
) -> Result<Response<Body>, Infallible> {
    let query = req.uri().query().unwrap_or("");
    let params = parse_query_params(query);

    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);
    let offset = params
        .get("offset")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let method = params.get("method").cloned();
    let status_code = params
        .get("status_code")
        .and_then(|s| s.parse::<u16>().ok());

    let logs = log_manager
        .search_logs(
            None, // start_time
            None, // end_time
            method.as_deref(),
            status_code,
            Some(limit + offset),
        )
        .await
        .unwrap_or_default();

    let logs = logs
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();

    let html = templates::logs_template(&logs, &params);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(html))
        .unwrap())
}

pub async fn health_handler(
    _req: Request<Body>,
    log_manager: Arc<LogManager>,
    config: Arc<Config>,
) -> Result<Response<Body>, Infallible> {
    let health_status = get_health_status(log_manager, config).await;

    let html = templates::health_template(&health_status);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(html))
        .unwrap())
}

pub async fn api_logs_handler(
    req: Request<Body>,
    log_manager: Arc<LogManager>,
) -> Result<Response<Body>, Infallible> {
    let query = req.uri().query().unwrap_or("");
    let params = parse_query_params(query);

    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);
    let offset = params
        .get("offset")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let method = params.get("method").cloned();
    let status_code = params
        .get("status_code")
        .and_then(|s| s.parse::<u16>().ok());

    let logs = log_manager
        .search_logs(
            None,
            None,
            method.as_deref(),
            status_code,
            Some(limit + offset),
        )
        .await
        .unwrap_or_default();

    let logs = logs
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&logs).unwrap_or_else(|_| "[]".to_string());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from(json))
        .unwrap())
}

pub async fn api_stats_handler(
    _req: Request<Body>,
    log_manager: Arc<LogManager>,
) -> Result<Response<Body>, Infallible> {
    let stats = get_dashboard_stats(log_manager).await;

    let json = serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from(json))
        .unwrap())
}

pub async fn static_file_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    let (content, content_type) = match path {
        "/ui/static/style.css" => (static_files::CSS, "text/css"),
        "/ui/static/script.js" => (static_files::JS, "application/javascript"),
        "/ui/static/favicon.ico" => (static_files::FAVICON, "image/x-icon"),
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap());
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .body(Body::from(content))
        .unwrap())
}

// Helper functions

async fn get_dashboard_stats(log_manager: Arc<LogManager>) -> serde_json::Value {
    let now = Utc::now();
    let one_hour_ago = now - Duration::hours(1);
    let one_day_ago = now - Duration::days(1);

    let recent_logs = log_manager
        .search_logs(Some(one_hour_ago), Some(now), None, None, Some(1000))
        .await
        .unwrap_or_default();

    let daily_logs = log_manager
        .search_logs(Some(one_day_ago), Some(now), None, None, Some(10000))
        .await
        .unwrap_or_default();

    let total_requests = recent_logs.len();
    let successful_requests = recent_logs
        .iter()
        .filter(|(_, resp)| resp.as_ref().map(|r| r.status_code < 400).unwrap_or(false))
        .count();
    let error_requests = total_requests - successful_requests;

    let avg_response_time = if !recent_logs.is_empty() {
        let total_time: u64 = recent_logs
            .iter()
            .filter_map(|(_, resp)| resp.as_ref().map(|r| r.duration_ms))
            .sum();
        total_time as f64 / recent_logs.len() as f64
    } else {
        0.0
    };

    let requests_per_hour = daily_logs.len() as f64 / 24.0;

    serde_json::json!({
        "total_requests": total_requests,
        "successful_requests": successful_requests,
        "error_requests": error_requests,
        "success_rate": if total_requests > 0 { (successful_requests as f64 / total_requests as f64) * 100.0 } else { 0.0 },
        "avg_response_time": avg_response_time,
        "requests_per_hour": requests_per_hour,
        "last_updated": now.to_rfc3339()
    })
}

async fn get_health_status(log_manager: Arc<LogManager>, config: Arc<Config>) -> serde_json::Value {
    let now = Utc::now();
    let five_minutes_ago = now - Duration::minutes(5);

    let recent_logs = log_manager
        .search_logs(Some(five_minutes_ago), Some(now), None, None, Some(100))
        .await
        .unwrap_or_default();

    let is_healthy = !recent_logs.is_empty();
    let last_request = recent_logs
        .first()
        .map(|(req, _)| req.timestamp.to_rfc3339())
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
            "target_url": config.target.base_url
        }
    })
}

fn parse_query_params(query: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();

    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }

    params
}
