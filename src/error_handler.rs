use crate::errors::{AppError, ErrorCode, ErrorResponse};
use crate::proxy::ProxyError;
use serde_json;
use std::convert::Infallible;
use uuid::Uuid;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::reject::{MethodNotAllowed, PayloadTooLarge};
use warp::{Rejection, Reply};

/// Custom error handler that provides consistent error responses
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let request_id = Uuid::new_v4().to_string();
    let path = err.find::<warp::path::FullPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();

    let (status, error_response) = if err.is_not_found() {
        (
            StatusCode::NOT_FOUND,
            ErrorResponse::new(
                ErrorCode::NotFound,
                "The requested resource was not found".to_string(),
            )
            .with_path(path)
            .with_request_id(request_id.clone()),
        )
    } else if err.find::<MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            ErrorResponse::new(
                ErrorCode::InvalidInput,
                "Method not allowed for this endpoint".to_string(),
            )
            .with_path(path)
            .with_request_id(request_id.clone()),
        )
    } else if err.find::<PayloadTooLarge>().is_some() {
        (
            StatusCode::PAYLOAD_TOO_LARGE,
            ErrorResponse::new(
                ErrorCode::RequestTooLarge,
                "Request payload is too large".to_string(),
            )
            .with_path(path)
            .with_request_id(request_id.clone()),
        )
    } else if let Some(app_error) = err.find::<AppError>() {
        let status = app_error.status_code();
        let error_response = app_error.to_error_response(Some(path), Some(request_id.clone()));
        (status, error_response)
    } else if let Some(proxy_error) = err.find::<ProxyError>() {
        match proxy_error {
            ProxyError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                ErrorResponse::new(
                    ErrorCode::RateLimitExceeded,
                    "Rate limit exceeded. Please try again later.".to_string(),
                )
                .with_path(path)
                .with_request_id(request_id.clone()),
            ),
            ProxyError::RequestTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                ErrorResponse::new(
                    ErrorCode::RequestTooLarge,
                    "Request payload is too large".to_string(),
                )
                .with_path(path)
                .with_request_id(request_id.clone()),
            ),
            ProxyError::ForwardError => (
                StatusCode::BAD_GATEWAY,
                ErrorResponse::new(
                    ErrorCode::ConnectionFailed,
                    "Failed to forward request to target server".to_string(),
                )
                .with_path(path)
                .with_request_id(request_id.clone()),
            ),
            ProxyError::BodyReadError => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::new(
                    ErrorCode::InvalidInput,
                    "Failed to read request body".to_string(),
                )
                .with_path(path)
                .with_request_id(request_id.clone()),
            ),
        }
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (
            StatusCode::PAYLOAD_TOO_LARGE,
            ErrorResponse::new(
                ErrorCode::RequestTooLarge,
                "Request payload is too large".to_string(),
            )
            .with_path(path)
            .with_request_id(request_id.clone()),
        )
    } else {
        // Log unexpected errors
        tracing::error!("Unhandled rejection: {:?}", err);
        
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::new(
                ErrorCode::InternalError,
                "An unexpected error occurred".to_string(),
            )
            .with_path(path)
            .with_request_id(request_id.clone()),
        )
    };

    // Log the error with request ID for debugging
    tracing::info!(
        "Error response [{}]: {} - {}",
        request_id,
        status.as_u16(),
        error_response.message
    );

    let json = serde_json::to_string(&error_response)
        .unwrap_or_else(|_| r#"{"code":"SERIALIZATION_ERROR","message":"Failed to serialize error response"}"#.to_string());

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("X-Request-ID", request_id)
        .body(Body::from(json))
        .unwrap())
}

/// Helper function to create a user-friendly error response
pub fn create_error_response(
    code: ErrorCode,
    message: &str,
    details: Option<&str>,
    path: Option<&str>,
) -> ErrorResponse {
    let mut error_response = ErrorResponse::new(code, message.to_string());
    
    if let Some(details) = details {
        error_response = error_response.with_details(details.to_string());
    }
    
    if let Some(path) = path {
        error_response = error_response.with_path(path.to_string());
    }
    
    error_response = error_response.with_request_id(Uuid::new_v4().to_string());
    
    error_response
}

/// Helper function to create a validation error response
pub fn create_validation_error_response(
    message: &str,
    field_errors: Vec<crate::errors::FieldError>,
    path: Option<&str>,
) -> ErrorResponse {
    let mut error_response = ErrorResponse::new(
        ErrorCode::ValidationError,
        message.to_string(),
    );
    
    let details = format!("Validation failed for {} field(s)", field_errors.len());
    error_response = error_response.with_details(details);
    
    if let Some(path) = path {
        error_response = error_response.with_path(path.to_string());
    }
    
    error_response = error_response.with_request_id(Uuid::new_v4().to_string());
    
    error_response
}

/// Helper function to create a success response with consistent structure
pub fn create_success_response<T: serde::Serialize>(
    data: T,
    message: Option<&str>,
) -> Result<impl Reply, Infallible> {
    let response = serde_json::json!({
        "status": "success",
        "message": message.unwrap_or("Operation completed successfully"),
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "request_id": Uuid::new_v4().to_string(),
    });

    let json = serde_json::to_string(&response)
        .unwrap_or_else(|_| r#"{"status":"error","message":"Failed to serialize response"}"#.to_string());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

/// Helper function to create a simple success response
pub fn create_simple_success_response(message: &str) -> Result<impl Reply, Infallible> {
    let response = serde_json::json!({
        "status": "success",
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "request_id": Uuid::new_v4().to_string(),
    });

    let json = serde_json::to_string(&response)
        .unwrap_or_else(|_| r#"{"status":"error","message":"Failed to serialize response"}"#.to_string());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(json))
        .unwrap())
}

/// Middleware to add request ID to all requests
pub fn with_request_id() -> impl warp::Filter<Extract = (String,), Error = Infallible> + Clone {
    use warp::Filter;
    warp::any().map(|| Uuid::new_v4().to_string())
}

/// Helper function to log errors with context
pub fn log_error(error: &dyn std::error::Error, context: &str, request_id: &str) {
    tracing::error!(
        "Error in {} [{}]: {}",
        context,
        request_id,
        error
    );
}

/// Helper function to log anyhow errors with context
pub fn log_anyhow_error(error: &anyhow::Error, context: &str, request_id: &str) {
    tracing::error!(
        "Error in {} [{}]: {}",
        context,
        request_id,
        error
    );
}

/// Helper function to log AppError with context
pub fn log_app_error(error: &crate::errors::AppError, context: &str, request_id: &str) {
    tracing::error!(
        "Error in {} [{}]: {}",
        context,
        request_id,
        error
    );
}

/// Helper function to log warnings with context
pub fn log_warning(message: &str, context: &str, request_id: &str) {
    tracing::warn!(
        "Warning in {} [{}]: {}",
        context,
        request_id,
        message
    );
}
