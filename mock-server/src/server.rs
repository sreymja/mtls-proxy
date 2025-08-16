use anyhow::{Result, anyhow};
use hyper::{body::Incoming, http::{Request, Response, StatusCode}, service::service_fn};
use hyper::body::Bytes;
use http_body_util::Full;
use std::net::SocketAddr;
use std::sync::Arc;
use http_body_util::BodyExt;
use hyper_util::rt::TokioIo;

use crate::config::Config;
use crate::handlers::{chat_completions_handler, health_handler, models_handler};
use crate::tls::TlsServer;
use crate::ui::handlers::*;

// Helper function to convert Request<Incoming> to Request<Full<Bytes>>
async fn convert_incoming_to_full(mut req: Request<Incoming>) -> Result<Request<Full<Bytes>>, anyhow::Error> {
    // Collect the body chunks
    let body_bytes = match req.body_mut().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => return Err(anyhow!("Failed to collect body: {}", err)),
    };

    // Create a new request with the same parts but Full<Bytes> body
    let (parts, _) = req.into_parts();
    Ok(Request::from_parts(parts, Full::new(body_bytes)))
}

pub struct MockServer {
    config: Config,
    tls_server: TlsServer,
}

impl MockServer {
    pub fn new(config: Config) -> Result<Self> {
        // Initialize TLS server
        let tls_server = TlsServer::new(
            &config.tls.cert_path,
            &config.tls.key_path,
            config.tls.ca_cert_path.as_deref(),
            config.tls.require_client_cert,
        )?;

        Ok(Self {
            config,
            tls_server,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let addr = SocketAddr::new(
            self.config.server.host.parse()?,
            self.config.server.port,
        );

        let config = Arc::new(self.config.clone());
        let available_models = self.config.models.available.clone();

        tracing::info!("Starting mock GPT server on {}", addr);

        // Start the server with TLS
        let listener = tokio::net::TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            let acceptor = self.tls_server.acceptor().clone();

            // Clone the data for each connection
            let config_clone = config.clone();
            let models_clone = available_models.clone();

            tokio::spawn(async move {
                match acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        // Create a new service function for each connection
                        let service = service_fn(move |req: Request<Incoming>| {
                            let config = config_clone.clone();
                            let available_models = models_clone.clone();

                            async move {
                                Self::handle_request(req, config, available_models).await
                            }
                        });

                        // Use hyper_util for TLS stream compatibility
                        if let Err(e) = hyper_util::server::conn::auto::Builder::new(
                            hyper_util::rt::TokioExecutor::new()
                        )
                            .serve_connection(TokioIo::new(tls_stream), service)
                            .await
                        {
                            tracing::error!("Connection error: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("TLS accept error: {}", e);
                    }
                }
            });
        }
    }

    async fn handle_request(
        req: Request<Incoming>,
        config: Arc<Config>,
        available_models: Vec<String>,
    ) -> Result<Response<Full<Bytes>>, anyhow::Error> {
        let path = req.uri().path();
        let method = req.method().as_str();

        // Handle UI routes first
        match (method, path) {
            // UI Routes
            ("GET", "/ui") | ("GET", "/ui/") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match dashboard_handler(full_req, config.clone()).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("Dashboard handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            ("GET", "/ui/dashboard") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match dashboard_handler(full_req, config.clone()).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("Dashboard handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            ("GET", "/ui/requests") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match requests_handler(full_req).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("Requests handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            ("GET", path) if path.starts_with("/ui/request/") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match request_detail_handler(full_req).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("Request detail handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            ("GET", "/ui/health") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match health_handler(full_req).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("Health handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            
            // API Routes
            ("GET", "/ui/api/requests") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match api_requests_handler(full_req).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("API requests handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            ("GET", "/ui/api/stats") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match api_stats_handler(full_req).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("API stats handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            
            // Static Files
            ("GET", path) if path.starts_with("/ui/static/") => {
                let full_req = match convert_incoming_to_full(req).await {
                    Ok(req) => req,
                    Err(e) => {
                        tracing::error!("Failed to convert request: {}", e);
                        return Err(e);
                    }
                };

                match static_file_handler(full_req).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        tracing::error!("Static file handler error: {}", e);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Full::new(Bytes::from("Internal server error")))
                            .unwrap())
                    }
                }
            }
            
            // API Routes - handle with logging
            ("GET", "/health") => {
                Self::handle_api_request_with_logging(req, config.clone(), available_models, |req, _config| {
                    health_handler(req)
                }).await
            }
            ("GET", "/v1/models") => {
                Self::handle_api_request_with_logging(req, config.clone(), available_models, |req, _config| {
                    models_handler(req)
                }).await
            }
            ("POST", "/v1/chat/completions") => {
                Self::handle_api_request_with_logging(req, config.clone(), available_models, chat_completions_handler).await
            }
            _ => {
                // Return 404 for unknown endpoints
                let response = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(
                        serde_json::json!({
                            "error": {
                                "message": "Endpoint not found",
                                "type": "invalid_request_error",
                                "code": "endpoint_not_found"
                            }
                        })
                        .to_string(),
                    )))
                    .unwrap();

                Ok(response)
            }
        }
    }

    async fn handle_api_request_with_logging<F, Fut>(
        req: Request<Incoming>,
        config: Arc<Config>,
        _available_models: Vec<String>,
        handler: F,
    ) -> Result<Response<Full<Bytes>>, anyhow::Error>
    where
        F: FnOnce(Request<Full<Bytes>>, Arc<Config>) -> Fut,
        Fut: std::future::Future<Output=Result<Response<Full<Bytes>>, std::convert::Infallible>>,
    {
        let start_time = std::time::Instant::now();
        let request_id = uuid::Uuid::new_v4().to_string();
        
        // Extract request details
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let client_ip = req
            .extensions()
            .get::<std::net::SocketAddr>()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        // Extract headers
        let mut headers = std::collections::HashMap::new();
        for (name, value) in req.headers() {
            headers.insert(name.to_string(), value.to_str().unwrap_or("").to_string());
        }
        
        // Extract body
        let body_bytes = match req.into_body().collect().await {
            Ok(collected) => collected.to_bytes().to_vec(),
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Invalid request body")))
                    .unwrap());
            }
        };
        
        let body = if !body_bytes.is_empty() {
            Some(String::from_utf8_lossy(&body_bytes).to_string())
        } else {
            None
        };
        
        // Rebuild request for handler
        let req_with_extensions = Request::builder()
            .method(method.parse::<hyper::Method>().unwrap())
            .uri(path.clone())
            .body(Full::new(Bytes::from(body_bytes)))
            .unwrap();
        
        // Call the handler
        let response: Result<Response<Full<Bytes>>, std::convert::Infallible> = handler(req_with_extensions, config).await;
        
        // Extract response details
        let (response_status, response_body) = match &response {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body = resp.body().clone().collect().await;
                let body_str = match body {
                    Ok(collected) => String::from_utf8_lossy(&collected.to_bytes()).to_string(),
                    Err(_) => "Error reading response body".to_string(),
                };
                (status, Some(body_str))
            }
            Err(_) => (500, Some("Internal server error".to_string())),
        };
        
        let response_time_ms = start_time.elapsed().as_millis() as u64;
        
        // Log the request
        let log_entry = RequestLogEntry {
            id: request_id,
            timestamp: chrono::Utc::now(),
            method,
            path,
            headers,
            body,
            response_status,
            response_body,
            response_time_ms,
            client_ip,
        };
        
        log_request(log_entry);

        response.map_err(|_| anyhow!("Handler error"))
    }
}
