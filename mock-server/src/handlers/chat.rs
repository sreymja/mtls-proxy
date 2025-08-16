use crate::config::Config;
use crate::responses::chat::{ChatCompletionRequest, ChatResponseGenerator};

use hyper::{Request, Response, StatusCode};
use rand::Rng;
use serde_json;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use http_body_util::BodyExt;

pub async fn chat_completions_handler(
    req: Request<http_body_util::Full<hyper::body::Bytes>>,
    config: Arc<Config>,
) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    // Check for random errors based on configuration
    if should_return_error(&config) {
        return Ok(create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
            "internal_error",
        ));
    }

    // Parse request body
    let body_bytes = match req.into_body().collect().await {
        Ok(collected) => collected.to_bytes().to_vec(),
        Err(_) => {
            return Ok(create_error_response(
                StatusCode::BAD_REQUEST,
                "Invalid request body",
                "invalid_request_error",
            ));
        }
    };

    let chat_request: ChatCompletionRequest = match serde_json::from_slice(&body_bytes) {
        Ok(request) => request,
        Err(_) => {
            return Ok(create_error_response(
                StatusCode::BAD_REQUEST,
                "Invalid JSON in request body",
                "invalid_request_error",
            ));
        }
    };

    // Validate model
    if !config.models.available.contains(&chat_request.model) {
        return Ok(create_error_response(
            StatusCode::BAD_REQUEST,
            "Model not found",
            "model_not_found",
        ));
    }

    // Add configurable delay
    let delay_ms = config.responses.default_delay_ms;
    sleep(Duration::from_millis(delay_ms)).await;

    // Generate response
    let generator = ChatResponseGenerator::new();
    
    if chat_request.stream.unwrap_or(false) {
        // Streaming response
        match generator.generate_streaming_response(&chat_request) {
            Ok(chunks) => {
                // Create all the chunks upfront
                let mut full_content = String::new();

                // Add all the data chunks
                for chunk in chunks {
                    let json = serde_json::to_string(&chunk).unwrap_or_default();
                    full_content.push_str(&format!("data: {}\n\n", json));
                }

                // Add the final DONE marker
                full_content.push_str("data: [DONE]\n\n");

                // Convert to bytes and create a Full response
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/plain; charset=utf-8")
                    .header("Cache-Control", "no-cache")
                    .header("Connection", "keep-alive")
                    .body(http_body_util::Full::from(hyper::body::Bytes::from(full_content)))
                    .unwrap())
            }
            Err(_) => Ok(create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate streaming response",
                "internal_error",
            )),
        }
    } else {
        // Standard response
        match generator.generate_response(&chat_request) {
            Ok(response) => {
                let json = serde_json::to_string(&response).unwrap_or_else(|_| {
                    serde_json::to_string(&serde_json::json!({
                        "error": {
                            "message": "Failed to serialize response",
                            "type": "internal_error",
                            "code": "serialization_error"
                        }
                    })).unwrap()
                });

                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(http_body_util::Full::new(hyper::body::Bytes::from(json)))
                    .unwrap())
            }
            Err(_) => Ok(create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate response",
                "internal_error",
            )),
        }
    }
}

fn should_return_error(config: &Config) -> bool {
    let mut rng = rand::thread_rng();
    let error_rate = config.responses.error_rate_percent as f32 / 100.0;
    rng.gen::<f32>() < error_rate
}

fn create_error_response(
    status: StatusCode,
    message: &str,
    error_type: &str,
) -> Response<http_body_util::Full<hyper::body::Bytes>> {
    let error_json = serde_json::json!({
        "error": {
            "message": message,
            "type": error_type,
            "code": error_type
        }
    });

    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(http_body_util::Full::from(hyper::body::Bytes::from(
            serde_json::to_string(&error_json).unwrap()
        )))
        .unwrap()
}
