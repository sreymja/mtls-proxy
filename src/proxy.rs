use anyhow::Result;
use chrono::Utc;
use hyper::body::Body;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tokio::time::Duration;
use uuid::Uuid;
use warp::{
    http::{Request, Response, StatusCode},
    Filter,
};

use crate::audit::AuditLogger;
use crate::config::Config;
use crate::config_manager::{CertificateUpload, ConfigManager, ConfigUpdateRequest};
use crate::error_handler::{
    create_simple_success_response, handle_rejection, log_anyhow_error, log_app_error, log_error,
};
use crate::errors::{filesystem_error, internal_error, validation_error, ErrorCode};
use crate::logging::LogManager;
use crate::metrics::Metrics;
use crate::rate_limit::{RateLimiter, RateLimiterConfig};
use crate::tls::TlsClient;

pub struct ProxyServer {
    config: Config,
    tls_client: TlsClient,
    log_manager: LogManager,
    metrics: Metrics,
    rate_limiter: RateLimiter,
    config_manager: ConfigManager,
    audit_logger: AuditLogger,
}

#[derive(Clone)]
struct AppState {
    log_manager: Arc<LogManager>,
    tls_client: Arc<TlsClient>,
    metrics: Arc<Metrics>,
    rate_limiter: Arc<RateLimiter>,
    audit_logger: Arc<AuditLogger>,
    config_manager: Arc<ConfigManager>,
    target_url: String,
    timeout_duration: Duration,
    max_request_size_mb: u64,
    #[allow(dead_code)]
    max_concurrent_requests: usize,
}

impl ProxyServer {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize TLS client (temporarily disabled for debugging)
        let tls_client = match TlsClient::new(
            &config.tls.client_cert_path,
            &config.tls.client_key_path,
            config.tls.ca_cert_path.as_deref(),
            config.tls.verify_hostname,
        ) {
            Ok(client) => client,
            Err(e) => {
                tracing::error!("Failed to initialize TLS client: {}", e);
                return Err(e);
            }
        };

        // Initialize log manager
        let log_manager = LogManager::new(
            &config.logging.sqlite_db_path,
            &config.logging.log_dir.to_string_lossy(),
            config.logging.max_log_size_mb,
            config.logging.retention_days,
            config.logging.compression_enabled,
        )?;

        // Initialize metrics
        let metrics = Metrics::new().await?;

        // Initialize rate limiter
        let rate_limiter_config = RateLimiterConfig {
            requests_per_second: config.server.rate_limit_requests_per_second,
            burst_size: config.server.rate_limit_burst_size,
        };
        let rate_limiter = RateLimiter::new(rate_limiter_config);

        // Initialize config manager
        let config_manager = ConfigManager::new(config.clone());

        // Initialize audit logger
        let audit_db_path = if std::env::var("RUST_ENV").unwrap_or_default() == "development" {
            std::path::PathBuf::from("./logs/audit.db")
        } else {
            std::path::PathBuf::from("/var/log/mtls-proxy/audit.db")
        };
        let audit_logger = AuditLogger::new(audit_db_path)?;

        Ok(Self {
            config,
            tls_client,
            log_manager,
            metrics,
            rate_limiter,
            config_manager,
            audit_logger,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let addr = SocketAddr::new(self.config.server.host.parse()?, self.config.server.port);

        let state = AppState {
            log_manager: Arc::new(self.log_manager.clone()),
            tls_client: Arc::new(self.tls_client.clone()),
            metrics: Arc::new(self.metrics.clone()),
            rate_limiter: Arc::new(self.rate_limiter.clone()),
            audit_logger: Arc::new(self.audit_logger.clone()),
            config_manager: Arc::new(self.config_manager.clone()),
            target_url: self.config.target.base_url.clone(),
            timeout_duration: Duration::from_secs(self.config.target.timeout_secs),
            max_request_size_mb: self.config.server.max_request_size_mb,
            max_concurrent_requests: self.config.server.max_concurrent_requests,
        };

        // Create the routes
        let routes = create_routes(state);

        tracing::info!("Starting proxy server on {}", addr);

        // Start the server with graceful shutdown
        let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(addr, async {
            signal::ctrl_c()
                .await
                .expect("Failed to listen for ctrl+c signal");
            tracing::info!("Received shutdown signal, starting graceful shutdown...");
        });

        server.await;
        tracing::info!("Server shutdown complete");

        Ok(())
    }
}

fn create_routes(state: AppState) -> impl Filter<Extract = impl warp::Reply> + Clone {
    let state_filter = warp::any().map(move || state.clone());

    // UI Routes (no authentication needed for development)
    let dashboard_route = warp::path!("ui")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(dashboard_handler);

    let dashboard_alt_route = warp::path!("ui" / "dashboard")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(dashboard_handler);

    let config_route = warp::path!("ui" / "config")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(config_handler);

    let logs_route = warp::path!("ui" / "logs")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(logs_handler);

    let audit_route = warp::path!("ui" / "audit")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(audit_handler);

    let health_route = warp::path!("ui" / "health")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(health_handler);

    // API Routes (no authentication needed for development)
    let api_logs_route = warp::path!("ui" / "api" / "logs")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(api_logs_handler);

    let api_stats_route = warp::path!("ui" / "api" / "stats")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(api_stats_handler);

    // Configuration API Routes (no authentication needed for development)
    let api_config_current_route = warp::path!("ui" / "api" / "config" / "current")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(api_config_current_handler);

    let api_config_update_route = warp::path!("ui" / "api" / "config" / "update")
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|config_update: ConfigUpdateRequest, state: AppState| {
            api_config_update_handler(state, config_update)
        });

    let api_config_validate_route = warp::path!("ui" / "api" / "config" / "validate")
        .and(warp::post())
        .and(state_filter.clone())
        .and_then(api_config_validate_handler);

    let api_certificates_upload_route = warp::path!("ui" / "api" / "certificates" / "upload")
        .and(warp::post())
        .and(warp::body::content_length_limit(10 * 1024 * 1024)) // 10MB limit
        .and(warp::multipart::form())
        .and(state_filter.clone())
        .and_then(
            |form: warp::multipart::FormData, state: AppState| async move {
                api_certificates_upload_multipart_handler(state, form).await
            },
        );

    let api_certificates_list_route = warp::path!("ui" / "api" / "certificates" / "list")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(api_certificates_list_handler);

    let api_certificates_delete_route =
        warp::path!("ui" / "api" / "certificates" / "delete" / String)
            .and(warp::delete())
            .and(state_filter.clone())
            .and_then(|filename: String, state: AppState| {
                api_certificates_delete_handler(state, filename)
            });

    // Audit API routes
    let api_audit_logs_route = warp::path!("ui" / "api" / "audit" / "logs")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(api_audit_logs_handler);

    let api_audit_stats_route = warp::path!("ui" / "api" / "audit" / "stats")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(api_audit_stats_handler);

    // Metrics endpoint
    let metrics_route = warp::path!("metrics")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(metrics_handler);

    // Legacy health route
    let legacy_health_route = warp::path!("health").and(warp::get()).map(|| {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"status":"healthy","service":"mtls-proxy"}"#))
            .unwrap()
    });

    // Proxy route (catch-all) - This is the main proxy functionality
    let proxy_route = warp::any()
        .and(warp::body::bytes())
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::query::raw())
        .and(warp::header::headers_cloned())
        .and(state_filter)
        .and_then(proxy_handler);

    // Combine all routes - Certificate upload must come before proxy route
    dashboard_route
        .or(dashboard_alt_route)
        .or(config_route)
        .or(logs_route)
        .or(audit_route)
        .or(health_route)
        .or(api_logs_route)
        .or(api_stats_route)
        .or(api_config_current_route)
        .or(api_config_update_route)
        .or(api_config_validate_route)
        .or(api_certificates_upload_route) // Must come before proxy_route
        .or(api_certificates_list_route)
        .or(api_certificates_delete_route)
        .or(api_audit_logs_route)
        .or(api_audit_stats_route)
        .or(metrics_route)
        .or(legacy_health_route)
        .or(proxy_route) // Catch-all route must be last
        .recover(handle_rejection)
}

async fn dashboard_handler(_state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let dashboard_html = include_str!("ui/templates/dashboard.html");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(dashboard_html))
        .unwrap())
}

async fn config_handler(_state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let config_html = include_str!("ui/templates/config.html");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(config_html))
        .unwrap())
}

async fn logs_handler(_state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let logs_html = include_str!("ui/templates/logs.html");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(logs_html))
        .unwrap())
}

async fn audit_handler(_state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let audit_html = include_str!("ui/templates/audit.html");

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(audit_html))
        .unwrap())
}

async fn health_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let req = Request::builder()
        .method("GET")
        .uri("/ui/health")
        .body(Body::empty())
        .unwrap();

    let config = Arc::new(Config::default());
    match crate::ui::handlers::health_handler(req, state.log_manager, config).await {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn api_logs_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let req = Request::builder()
        .method("GET")
        .uri("/ui/api/logs")
        .body(Body::empty())
        .unwrap();

    match crate::ui::handlers::api_logs_handler(req, state.log_manager).await {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn api_stats_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    let req = Request::builder()
        .method("GET")
        .uri("/ui/api/stats")
        .body(Body::empty())
        .unwrap();

    match crate::ui::handlers::api_stats_handler(req, state.log_manager).await {
        Ok(response) => Ok(response),
        Err(_) => Err(warp::reject::not_found()),
    }
}

// Configuration API handlers
async fn api_config_current_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    match state.config_manager.get_current_config().await {
        Ok(config) => {
            let response = serde_json::to_string(&config).map_err(|_| warp::reject::not_found())?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response))
                .unwrap())
        }
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn api_config_update_handler(
    state: AppState,
    config_update: ConfigUpdateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Validate input
    if config_update.target_url.is_empty() {
        return Err(warp::reject::custom(validation_error(
            "Target URL is required",
            Some(vec![crate::errors::FieldError {
                field: "target_url".to_string(),
                message: "Target URL cannot be empty".to_string(),
                value: Some(config_update.target_url.clone()),
            }]),
        )));
    }

    if config_update.timeout_secs == 0 {
        return Err(warp::reject::custom(validation_error(
            "Timeout must be greater than 0",
            Some(vec![crate::errors::FieldError {
                field: "timeout_secs".to_string(),
                message: "Timeout must be greater than 0".to_string(),
                value: Some(config_update.timeout_secs.to_string()),
            }]),
        )));
    }

    match state
        .config_manager
        .update_config(config_update.clone())
        .await
    {
        Ok(_) => {
            // Log audit event
            if let Err(e) = state
                .audit_logger
                .log_event(
                    crate::audit::AuditEventType::ConfigUpdate,
                    format!(
                        "Configuration updated: target_url={}, timeout_secs={}, max_connections={}",
                        config_update.target_url,
                        config_update.timeout_secs,
                        config_update.max_connections
                    ),
                    None,
                    None,
                )
                .await
            {
                log_anyhow_error(&e, "audit_logging", &request_id);
            }

            Ok(create_simple_success_response("Configuration updated successfully").unwrap())
        }
        Err(e) => {
            log_app_error(&e, "config_update", &request_id);
            Err(warp::reject::custom(e))
        }
    }
}

async fn api_config_validate_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    match state.config_manager.validate_config().await {
        Ok(_) => {
            let response = serde_json::json!({
                "status": "success",
                "message": "Configuration is valid"
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response.to_string()))
                .unwrap())
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Configuration validation failed: {}", e)
            });

            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(Body::from(error_response.to_string()))
                .unwrap())
        }
    }
}

async fn api_certificates_upload_multipart_handler(
    state: AppState,
    mut form: warp::multipart::FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
    use futures_util::StreamExt;
    use warp::Buf;

    let request_id = uuid::Uuid::new_v4().to_string();
    let mut cert_type = None;
    let mut filename = None;
    let mut content = Vec::new();

    while let Some(part) = form.next().await {
        let mut part = match part {
            Ok(part) => part,
            Err(e) => {
                log_error(&e, "form_parsing", &request_id);
                return Err(warp::reject::custom(internal_error(
                    ErrorCode::DeserializationError,
                    "Failed to parse form data",
                    Some(&e.to_string()),
                )));
            }
        };

        let name = part.name();
        match name {
            "cert_type" => {
                if let Some(Ok(data)) = part.data().await {
                    cert_type = Some(String::from_utf8_lossy(data.chunk()).to_string());
                } else {
                    return Err(warp::reject::custom(validation_error(
                        "Missing or invalid cert_type",
                        Some(vec![crate::errors::FieldError {
                            field: "cert_type".to_string(),
                            message: "Certificate type is required".to_string(),
                            value: None,
                        }]),
                    )));
                }
            }
            "file" => {
                filename = part.filename().map(|s| s.to_string());
                let mut file_content = Vec::new();
                let mut stream = part.stream();
                while let Some(chunk) = stream.next().await {
                    let chunk = match chunk {
                        Ok(chunk) => chunk,
                        Err(e) => {
                            log_error(&e, "file_reading", &request_id);
                            return Err(warp::reject::custom(filesystem_error(
                                ErrorCode::FileSystemError,
                                "Failed to read file data",
                                Some(&e.to_string()),
                            )));
                        }
                    };
                    file_content.extend_from_slice(chunk.chunk());
                }
                content = file_content;
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    // Validate required fields
    let cert_type = match cert_type {
        Some(ct) => ct,
        None => {
            return Err(warp::reject::custom(validation_error(
                "Missing cert_type field",
                Some(vec![crate::errors::FieldError {
                    field: "cert_type".to_string(),
                    message: "Certificate type is required".to_string(),
                    value: None,
                }]),
            )));
        }
    };

    let filename = match filename {
        Some(f) => f,
        None => {
            return Err(warp::reject::custom(validation_error(
                "Missing filename",
                Some(vec![crate::errors::FieldError {
                    field: "file".to_string(),
                    message: "File is required".to_string(),
                    value: None,
                }]),
            )));
        }
    };

    if content.is_empty() {
        return Err(warp::reject::custom(validation_error(
            "No file content provided",
            Some(vec![crate::errors::FieldError {
                field: "file".to_string(),
                message: "File content cannot be empty".to_string(),
                value: None,
            }]),
        )));
    }

    // Convert cert_type string to CertificateType enum
    let cert_type_enum = match cert_type.as_str() {
        "client" => crate::config_manager::CertificateType::Client,
        "key" => crate::config_manager::CertificateType::Key,
        "ca" => crate::config_manager::CertificateType::CA,
        _ => {
            return Err(warp::reject::custom(validation_error(
                &format!(
                    "Invalid cert_type: {}. Must be 'client', 'key', or 'ca'",
                    cert_type
                ),
                Some(vec![crate::errors::FieldError {
                    field: "cert_type".to_string(),
                    message: "Invalid certificate type. Must be 'client', 'key', or 'ca'"
                        .to_string(),
                    value: Some(cert_type.clone()),
                }]),
            )));
        }
    };

    // Create CertificateUpload struct
    let upload = CertificateUpload {
        cert_type: cert_type_enum,
        filename,
        content,
    };

    // Use existing upload logic
    match state
        .config_manager
        .upload_certificate(upload.clone())
        .await
    {
        Ok(_) => {
            // Log audit event
            if let Err(e) = state
                .audit_logger
                .log_event(
                    crate::audit::AuditEventType::CertificateUpload,
                    format!(
                        "Certificate uploaded: type={}, filename={}",
                        upload.cert_type, upload.filename
                    ),
                    None,
                    None,
                )
                .await
            {
                log_anyhow_error(&e, "audit_logging", &request_id);
            }

            Ok(create_simple_success_response("Certificate uploaded successfully").unwrap())
        }
        Err(e) => {
            log_app_error(&e, "certificate_upload", &request_id);
            Err(warp::reject::custom(e))
        }
    }
}

async fn api_certificates_list_handler(
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    match state.config_manager.list_certificates().await {
        Ok(certificates) => {
            let response = serde_json::json!({
                "status": "success",
                "certificates": certificates
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response.to_string()))
                .unwrap())
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to list certificates: {}", e)
            });

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(error_response.to_string()))
                .unwrap())
        }
    }
}

async fn api_certificates_delete_handler(
    state: AppState,
    filename: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match state.config_manager.delete_certificate(&filename).await {
        Ok(_) => {
            // Log audit event
            let _ = state
                .audit_logger
                .log_event(
                    crate::audit::AuditEventType::CertificateDelete,
                    format!("Certificate deleted: filename={}", filename),
                    None,
                    None,
                )
                .await;

            let response = serde_json::json!({
                "status": "success",
                "message": format!("Certificate {} deleted successfully", filename)
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response.to_string()))
                .unwrap())
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete certificate {}: {}", filename, e)
            });

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(error_response.to_string()))
                .unwrap())
        }
    }
}

async fn metrics_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    match state.metrics.get_metrics().await {
        Ok(metrics) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain; version=0.0.4; charset=utf-8")
            .body(Body::from(metrics))
            .unwrap()),
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn api_audit_logs_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    match state.audit_logger.get_audit_logs(None, None, None).await {
        Ok(logs) => {
            let response = serde_json::json!({
                "status": "success",
                "logs": logs
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response.to_string()))
                .unwrap())
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to get audit logs: {}", e)
            });

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(error_response.to_string()))
                .unwrap())
        }
    }
}

async fn api_audit_stats_handler(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    match state.audit_logger.get_audit_stats().await {
        Ok(stats) => {
            let response = serde_json::json!({
                "status": "success",
                "stats": stats
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::from(response.to_string()))
                .unwrap())
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to get audit stats: {}", e)
            });

            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(Body::from(error_response.to_string()))
                .unwrap())
        }
    }
}

async fn proxy_handler(
    body: hyper::body::Bytes,
    method: warp::http::Method,
    path: warp::path::FullPath,
    query: String,
    headers: warp::http::HeaderMap,
    state: AppState,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Record metrics
    state.metrics.record_request_start().await;
    state.metrics.record_connection_start().await;

    // Check rate limit
    if state.rate_limiter.check_async().await.is_err() {
        tracing::warn!("Rate limit exceeded");
        state.metrics.record_error("request").await;
        state.metrics.record_connection_end().await;
        return Err(warp::reject::custom(ProxyError::RateLimitExceeded));
    }

    // Check request size limit
    let max_size = (state.max_request_size_mb * 1024 * 1024) as usize; // Convert MB to bytes
    if body.len() > max_size {
        tracing::warn!(
            "Request body too large: {} bytes (limit: {} bytes)",
            body.len(),
            max_size
        );
        state.metrics.record_error("request").await;
        state.metrics.record_connection_end().await;
        return Err(warp::reject::custom(ProxyError::RequestTooLarge));
    }
    let start_time = std::time::Instant::now();
    let request_id = Uuid::new_v4().to_string();

    // Build target URL
    let target_url = if !query.is_empty() {
        format!("{}{}?{}", state.target_url, path.as_str(), query)
    } else {
        format!("{}{}", state.target_url, path.as_str())
    };

    tracing::info!(
        "Proxying request {} {} -> {}",
        method,
        path.as_str(),
        target_url
    );

    // Create request to target
    let mut target_req = hyper::Request::builder().method(method).uri(target_url);

    // Copy headers (excluding hop-by-hop headers)
    for (name, value) in headers {
        if let Some(name) = name {
            if !is_hop_by_hop_header(name.as_str()) {
                target_req = target_req.header(name, value);
            }
        }
    }

    // Add proxy headers
    target_req = target_req
        .header("X-Forwarded-For", "127.0.0.1")
        .header("X-Forwarded-Proto", "http")
        .header("X-Request-ID", &request_id);

    // Build the target request
    let target_req = target_req.body(Body::from(body.clone())).unwrap();

    // Log the incoming request
    let request_log = crate::logging::RequestLog {
        id: request_id.clone(),
        timestamp: Utc::now(),
        method: target_req.method().to_string(),
        uri: target_req.uri().to_string(),
        headers: format!("{:?}", target_req.headers()),
        body_size: body.len(),
        client_ip: "127.0.0.1".to_string(),
    };

    if let Err(e) = state.log_manager.log_request(request_log).await {
        tracing::error!("Failed to log request: {}", e);
    }

    // Forward the request to target using mTLS
    let response =
        forward_request_with_mtls(target_req, &state.tls_client, state.timeout_duration).await;

    // Log the response
    let duration_ms = start_time.elapsed().as_millis() as u64;
    let response_log = match &response {
        Ok(resp) => {
            // Try to get content length from headers
            let body_size = resp
                .headers()
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);

            crate::logging::ResponseLog {
                request_id: request_id.clone(),
                timestamp: Utc::now(),
                status_code: resp.status().as_u16(),
                headers: format!("{:?}", resp.headers()),
                body_size,
                duration_ms,
            }
        }
        Err(_) => crate::logging::ResponseLog {
            request_id: request_id.clone(),
            timestamp: Utc::now(),
            status_code: 500,
            headers: "{}".to_string(),
            body_size: 0,
            duration_ms,
        },
    };

    if let Err(e) = state.log_manager.log_response(response_log).await {
        tracing::error!("Failed to log response: {}", e);
    }

    // Record metrics
    let duration = start_time.elapsed().as_secs_f64();
    state.metrics.record_request_end(duration).await;
    state.metrics.record_connection_end().await;

    // Return the response or error
    match response {
        Ok(resp) => {
            state.metrics.record_response(resp.status().as_u16()).await;
            Ok(resp)
        }
        Err(e) => {
            tracing::error!("Proxy request failed: {}", e);
            state.metrics.record_error("request").await;
            Err(warp::reject::custom(ProxyError::ForwardError))
        }
    }
}

async fn forward_request_with_mtls(
    req: hyper::Request<Body>,
    tls_client: &TlsClient,
    timeout_duration: Duration,
) -> Result<hyper::Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    // Parse the target URL
    let target_uri = req.uri().to_string();
    let url = url::Url::parse(&target_uri)?;
    let host = url.host_str().ok_or("No host in URL")?;
    let port = url.port().unwrap_or(443);

    // Create TCP connection
    let addr = format!("{}:{}", host, port);
    let tcp_stream = tokio::net::TcpStream::connect(&addr).await?;

    // Establish TLS connection
    let tls_stream = tls_client
        .connector()
        .connect(host.try_into()?, tcp_stream)
        .await?;

    // Create HTTP client with the TLS connection
    let (mut sender, conn) = hyper::client::conn::Builder::new()
        .handshake(tls_stream)
        .await?;

    // Spawn the connection
    tokio::task::spawn(async move {
        if let Err(e) = conn.await {
            tracing::error!("Connection error: {}", e);
        }
    });

    // Send the request with timeout
    let response = tokio::time::timeout(timeout_duration, sender.send_request(req)).await;

    match response {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => {
            tracing::error!("Request failed: {}", e);
            Err(Box::new(e))
        }
        Err(_) => {
            tracing::error!("Request timeout");
            Ok(hyper::Response::builder()
                .status(StatusCode::GATEWAY_TIMEOUT)
                .body(Body::from("Gateway Timeout"))
                .unwrap())
        }
    }
}

pub fn is_hop_by_hop_header(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailers"
            | "transfer-encoding"
            | "upgrade"
    )
}

#[derive(Debug)]
pub enum ProxyError {
    BodyReadError,
    ForwardError,
    RequestTooLarge,
    RateLimitExceeded,
}

impl warp::reject::Reject for ProxyError {}

impl Clone for LogManager {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
            log_dir: self.log_dir.clone(),
            max_log_size_mb: self.max_log_size_mb,
            retention_days: self.retention_days,
            compression_enabled: self.compression_enabled,
        }
    }
}

impl Clone for TlsClient {
    fn clone(&self) -> Self {
        Self {
            connector: self.connector.clone(),
        }
    }
}
