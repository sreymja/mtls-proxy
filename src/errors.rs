use serde::{Deserialize, Serialize};
use std::fmt;
use warp::http::StatusCode;
use warp::reject::Reject;

/// Standard error codes for the mTLS proxy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorCode {
    // Configuration errors
    ConfigValidationFailed,
    ConfigUpdateFailed,
    ConfigLoadFailed,
    
    // Certificate errors
    CertificateUploadFailed,
    CertificateDeleteFailed,
    CertificateNotFound,
    CertificateInvalid,
    CertificateParseError,
    
    // File system errors
    FileNotFound,
    FilePermissionDenied,
    FileSystemError,
    FileTooLarge,
    
    // Network errors
    ConnectionFailed,
    Timeout,
    RateLimitExceeded,
    RequestTooLarge,
    
    // Database errors
    DatabaseError,
    AuditLogError,
    
    // Validation errors
    ValidationError,
    InvalidInput,
    MissingRequiredField,
    
    // Internal errors
    InternalError,
    SerializationError,
    DeserializationError,
    
    // Authentication/Authorization errors
    Unauthorized,
    Forbidden,
    
    // Not found errors
    NotFound,
    EndpointNotFound,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::ConfigValidationFailed => write!(f, "CONFIG_VALIDATION_FAILED"),
            ErrorCode::ConfigUpdateFailed => write!(f, "CONFIG_UPDATE_FAILED"),
            ErrorCode::ConfigLoadFailed => write!(f, "CONFIG_LOAD_FAILED"),
            ErrorCode::CertificateUploadFailed => write!(f, "CERTIFICATE_UPLOAD_FAILED"),
            ErrorCode::CertificateDeleteFailed => write!(f, "CERTIFICATE_DELETE_FAILED"),
            ErrorCode::CertificateNotFound => write!(f, "CERTIFICATE_NOT_FOUND"),
            ErrorCode::CertificateInvalid => write!(f, "CERTIFICATE_INVALID"),
            ErrorCode::CertificateParseError => write!(f, "CERTIFICATE_PARSE_ERROR"),
            ErrorCode::FileNotFound => write!(f, "FILE_NOT_FOUND"),
            ErrorCode::FilePermissionDenied => write!(f, "FILE_PERMISSION_DENIED"),
            ErrorCode::FileSystemError => write!(f, "FILE_SYSTEM_ERROR"),
            ErrorCode::FileTooLarge => write!(f, "FILE_TOO_LARGE"),
            ErrorCode::ConnectionFailed => write!(f, "CONNECTION_FAILED"),
            ErrorCode::Timeout => write!(f, "TIMEOUT"),
            ErrorCode::RateLimitExceeded => write!(f, "RATE_LIMIT_EXCEEDED"),
            ErrorCode::RequestTooLarge => write!(f, "REQUEST_TOO_LARGE"),
            ErrorCode::DatabaseError => write!(f, "DATABASE_ERROR"),
            ErrorCode::AuditLogError => write!(f, "AUDIT_LOG_ERROR"),
            ErrorCode::ValidationError => write!(f, "VALIDATION_ERROR"),
            ErrorCode::InvalidInput => write!(f, "INVALID_INPUT"),
            ErrorCode::MissingRequiredField => write!(f, "MISSING_REQUIRED_FIELD"),
            ErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            ErrorCode::SerializationError => write!(f, "SERIALIZATION_ERROR"),
            ErrorCode::DeserializationError => write!(f, "DESERIALIZATION_ERROR"),
            ErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ErrorCode::Forbidden => write!(f, "FORBIDDEN"),
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::EndpointNotFound => write!(f, "ENDPOINT_NOT_FOUND"),
        }
    }
}

/// Standard error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub timestamp: String,
    pub path: Option<String>,
    pub request_id: Option<String>,
}

impl ErrorResponse {
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            path: None,
            request_id: None,
        }
    }
    
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

/// Application error types
#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Certificate(CertificateError),
    FileSystem(FileSystemError),
    Network(NetworkError),
    Database(DatabaseError),
    Validation(ValidationError),
    Internal(InternalError),
}

#[derive(Debug)]
pub struct ConfigError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub struct CertificateError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub struct FileSystemError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub struct NetworkError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub struct DatabaseError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
    pub field_errors: Option<Vec<FieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct InternalError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

impl Reject for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Configuration error: {}", e.message),
            AppError::Certificate(e) => write!(f, "Certificate error: {}", e.message),
            AppError::FileSystem(e) => write!(f, "File system error: {}", e.message),
            AppError::Network(e) => write!(f, "Network error: {}", e.message),
            AppError::Database(e) => write!(f, "Database error: {}", e.message),
            AppError::Validation(e) => write!(f, "Validation error: {}", e.message),
            AppError::Internal(e) => write!(f, "Internal error: {}", e.message),
        }
    }
}

impl AppError {
    pub fn to_error_response(&self, path: Option<String>, request_id: Option<String>) -> ErrorResponse {
        match self {
            AppError::Config(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
            AppError::Certificate(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
            AppError::FileSystem(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
            AppError::Network(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
            AppError::Database(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
            AppError::Validation(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
            AppError::Internal(e) => ErrorResponse::new(e.code.clone(), e.message.clone())
                .with_details(e.details.clone().unwrap_or_default())
                .with_path(path.unwrap_or_default())
                .with_request_id(request_id.unwrap_or_default()),
        }
    }
    
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Config(_) => StatusCode::BAD_REQUEST,
            AppError::Certificate(_) => StatusCode::BAD_REQUEST,
            AppError::FileSystem(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Network(_) => StatusCode::BAD_GATEWAY,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Error creation helpers
pub fn config_error(code: ErrorCode, message: &str, details: Option<&str>) -> AppError {
    AppError::Config(ConfigError {
        code,
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    })
}

pub fn certificate_error(code: ErrorCode, message: &str, details: Option<&str>) -> AppError {
    AppError::Certificate(CertificateError {
        code,
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    })
}

pub fn filesystem_error(code: ErrorCode, message: &str, details: Option<&str>) -> AppError {
    AppError::FileSystem(FileSystemError {
        code,
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    })
}

pub fn network_error(code: ErrorCode, message: &str, details: Option<&str>) -> AppError {
    AppError::Network(NetworkError {
        code,
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    })
}

pub fn database_error(code: ErrorCode, message: &str, details: Option<&str>) -> AppError {
    AppError::Database(DatabaseError {
        code,
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    })
}

pub fn validation_error(message: &str, field_errors: Option<Vec<FieldError>>) -> AppError {
    AppError::Validation(ValidationError {
        code: ErrorCode::ValidationError,
        message: message.to_string(),
        details: None,
        field_errors,
    })
}

pub fn internal_error(code: ErrorCode, message: &str, details: Option<&str>) -> AppError {
    AppError::Internal(InternalError {
        code,
        message: message.to_string(),
        details: details.map(|s| s.to_string()),
    })
}

/// User-friendly error messages
pub fn get_user_friendly_message(code: &ErrorCode) -> &'static str {
    match code {
        ErrorCode::ConfigValidationFailed => "The configuration is invalid. Please check your settings and try again.",
        ErrorCode::ConfigUpdateFailed => "Failed to update configuration. Please try again or contact support.",
        ErrorCode::ConfigLoadFailed => "Failed to load configuration. Please check your configuration files.",
        ErrorCode::CertificateUploadFailed => "Failed to upload certificate. Please check the file format and try again.",
        ErrorCode::CertificateDeleteFailed => "Failed to delete certificate. Please try again.",
        ErrorCode::CertificateNotFound => "Certificate not found. Please check the certificate name.",
        ErrorCode::CertificateInvalid => "The certificate is invalid. Please check the certificate format.",
        ErrorCode::CertificateParseError => "Failed to parse certificate. Please check the certificate format.",
        ErrorCode::FileNotFound => "File not found. Please check the file path.",
        ErrorCode::FilePermissionDenied => "Permission denied. Please check file permissions.",
        ErrorCode::FileSystemError => "File system error occurred. Please try again.",
        ErrorCode::FileTooLarge => "File is too large. Please use a smaller file.",
        ErrorCode::ConnectionFailed => "Connection failed. Please check your network connection.",
        ErrorCode::Timeout => "Request timed out. Please try again.",
        ErrorCode::RateLimitExceeded => "Rate limit exceeded. Please wait and try again.",
        ErrorCode::RequestTooLarge => "Request is too large. Please reduce the request size.",
        ErrorCode::DatabaseError => "Database error occurred. Please try again.",
        ErrorCode::AuditLogError => "Failed to log audit event. Please try again.",
        ErrorCode::ValidationError => "Validation failed. Please check your input.",
        ErrorCode::InvalidInput => "Invalid input provided. Please check your data.",
        ErrorCode::MissingRequiredField => "Required field is missing. Please provide all required fields.",
        ErrorCode::InternalError => "An internal error occurred. Please try again or contact support.",
        ErrorCode::SerializationError => "Failed to serialize data. Please try again.",
        ErrorCode::DeserializationError => "Failed to parse data. Please check the format.",
        ErrorCode::Unauthorized => "Unauthorized access. Please check your credentials.",
        ErrorCode::Forbidden => "Access forbidden. You don't have permission to perform this action.",
        ErrorCode::NotFound => "Resource not found. Please check the URL.",
        ErrorCode::EndpointNotFound => "API endpoint not found. Please check the URL.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_display() {
        assert_eq!(ErrorCode::ConfigValidationFailed.to_string(), "CONFIG_VALIDATION_FAILED");
        assert_eq!(ErrorCode::CertificateUploadFailed.to_string(), "CERTIFICATE_UPLOAD_FAILED");
        assert_eq!(ErrorCode::ValidationError.to_string(), "VALIDATION_ERROR");
        assert_eq!(ErrorCode::InternalError.to_string(), "INTERNAL_ERROR");
    }

    #[test]
    fn test_error_response_creation() {
        let error_response = ErrorResponse::new(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed".to_string(),
        );

        assert_eq!(error_response.code, "CONFIG_VALIDATION_FAILED");
        assert_eq!(error_response.message, "Configuration validation failed");
        assert!(error_response.details.is_none());
        assert!(error_response.path.is_none());
        assert!(error_response.request_id.is_none());
    }

    #[test]
    fn test_error_response_with_details() {
        let error_response = ErrorResponse::new(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed".to_string(),
        )
        .with_details("Target URL must start with https://".to_string());

        assert_eq!(error_response.code, "CONFIG_VALIDATION_FAILED");
        assert_eq!(error_response.message, "Configuration validation failed");
        assert_eq!(error_response.details, Some("Target URL must start with https://".to_string()));
    }

    #[test]
    fn test_error_response_with_path() {
        let error_response = ErrorResponse::new(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed".to_string(),
        )
        .with_path("/api/config/update".to_string());

        assert_eq!(error_response.path, Some("/api/config/update".to_string()));
    }

    #[test]
    fn test_error_response_with_request_id() {
        let error_response = ErrorResponse::new(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed".to_string(),
        )
        .with_request_id("test-request-id".to_string());

        assert_eq!(error_response.request_id, Some("test-request-id".to_string()));
    }

    #[test]
    fn test_app_error_status_code() {
        let config_error = config_error(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed",
            None,
        );
        assert_eq!(config_error.status_code(), StatusCode::BAD_REQUEST);

        let certificate_error = certificate_error(
            ErrorCode::CertificateUploadFailed,
            "Certificate upload failed",
            None,
        );
        assert_eq!(certificate_error.status_code(), StatusCode::BAD_REQUEST);

        let filesystem_error = filesystem_error(
            ErrorCode::FileSystemError,
            "File system error",
            None,
        );
        assert_eq!(filesystem_error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

        let network_error = network_error(
            ErrorCode::ConnectionFailed,
            "Connection failed",
            None,
        );
        assert_eq!(network_error.status_code(), StatusCode::BAD_GATEWAY);

        let validation_error = validation_error(
            "Validation failed",
            None,
        );
        assert_eq!(validation_error.status_code(), StatusCode::BAD_REQUEST);

        let internal_error = internal_error(
            ErrorCode::InternalError,
            "Internal error",
            None,
        );
        assert_eq!(internal_error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_to_error_response() {
        let config_error = config_error(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed",
            Some("Target URL must start with https://"),
        );

        let error_response = config_error.to_error_response(
            Some("/api/config/update".to_string()),
            Some("test-request-id".to_string()),
        );

        assert_eq!(error_response.code, "CONFIG_VALIDATION_FAILED");
        assert_eq!(error_response.message, "Configuration validation failed");
        assert_eq!(error_response.details, Some("Target URL must start with https://".to_string()));
        assert_eq!(error_response.path, Some("/api/config/update".to_string()));
        assert_eq!(error_response.request_id, Some("test-request-id".to_string()));
    }

    #[test]
    fn test_validation_error_with_field_errors() {
        let field_errors = vec![
            FieldError {
                field: "target_url".to_string(),
                message: "Target URL is required".to_string(),
                value: Some("".to_string()),
            },
            FieldError {
                field: "timeout_secs".to_string(),
                message: "Timeout must be greater than 0".to_string(),
                value: Some("0".to_string()),
            },
        ];

        let validation_error = validation_error("Validation failed", Some(field_errors.clone()));

        if let AppError::Validation(e) = validation_error {
            assert_eq!(e.code, ErrorCode::ValidationError);
            assert_eq!(e.message, "Validation failed");
            assert_eq!(e.field_errors, Some(field_errors));
        } else {
            panic!("Expected Validation error");
        }
    }

    #[test]
    fn test_user_friendly_messages() {
        assert_eq!(
            get_user_friendly_message(&ErrorCode::ConfigValidationFailed),
            "The configuration is invalid. Please check your settings and try again."
        );
        assert_eq!(
            get_user_friendly_message(&ErrorCode::CertificateUploadFailed),
            "Failed to upload certificate. Please check the file format and try again."
        );
        assert_eq!(
            get_user_friendly_message(&ErrorCode::ValidationError),
            "Validation failed. Please check your input."
        );
        assert_eq!(
            get_user_friendly_message(&ErrorCode::InternalError),
            "An internal error occurred. Please try again or contact support."
        );
    }

    #[test]
    fn test_error_creation_helpers() {
        let config_error = config_error(
            ErrorCode::ConfigUpdateFailed,
            "Failed to update configuration",
            Some("Permission denied"),
        );
        if let AppError::Config(e) = config_error {
            assert_eq!(e.code, ErrorCode::ConfigUpdateFailed);
            assert_eq!(e.message, "Failed to update configuration");
            assert_eq!(e.details, Some("Permission denied".to_string()));
        } else {
            panic!("Expected Config error");
        }

        let certificate_error = certificate_error(
            ErrorCode::CertificateInvalid,
            "Invalid certificate format",
            Some("Missing BEGIN CERTIFICATE"),
        );
        if let AppError::Certificate(e) = certificate_error {
            assert_eq!(e.code, ErrorCode::CertificateInvalid);
            assert_eq!(e.message, "Invalid certificate format");
            assert_eq!(e.details, Some("Missing BEGIN CERTIFICATE".to_string()));
        } else {
            panic!("Expected Certificate error");
        }

        let filesystem_error = filesystem_error(
            ErrorCode::FileNotFound,
            "Certificate file not found",
            Some("certs/client.crt"),
        );
        if let AppError::FileSystem(e) = filesystem_error {
            assert_eq!(e.code, ErrorCode::FileNotFound);
            assert_eq!(e.message, "Certificate file not found");
            assert_eq!(e.details, Some("certs/client.crt".to_string()));
        } else {
            panic!("Expected FileSystem error");
        }

        let network_error = network_error(
            ErrorCode::ConnectionFailed,
            "Failed to connect to target",
            Some("Connection timeout"),
        );
        if let AppError::Network(e) = network_error {
            assert_eq!(e.code, ErrorCode::ConnectionFailed);
            assert_eq!(e.message, "Failed to connect to target");
            assert_eq!(e.details, Some("Connection timeout".to_string()));
        } else {
            panic!("Expected Network error");
        }

        let database_error = database_error(
            ErrorCode::DatabaseError,
            "Database connection failed",
            Some("SQLite error"),
        );
        if let AppError::Database(e) = database_error {
            assert_eq!(e.code, ErrorCode::DatabaseError);
            assert_eq!(e.message, "Database connection failed");
            assert_eq!(e.details, Some("SQLite error".to_string()));
        } else {
            panic!("Expected Database error");
        }

        let internal_error = internal_error(
            ErrorCode::SerializationError,
            "Failed to serialize response",
            Some("JSON error"),
        );
        if let AppError::Internal(e) = internal_error {
            assert_eq!(e.code, ErrorCode::SerializationError);
            assert_eq!(e.message, "Failed to serialize response");
            assert_eq!(e.details, Some("JSON error".to_string()));
        } else {
            panic!("Expected Internal error");
        }
    }

    #[test]
    fn test_error_display() {
        let config_error = config_error(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed",
            None,
        );
        assert_eq!(
            config_error.to_string(),
            "Configuration error: Configuration validation failed"
        );

        let certificate_error = certificate_error(
            ErrorCode::CertificateUploadFailed,
            "Certificate upload failed",
            None,
        );
        assert_eq!(
            certificate_error.to_string(),
            "Certificate error: Certificate upload failed"
        );

        let validation_error = validation_error("Validation failed", None);
        assert_eq!(
            validation_error.to_string(),
            "Validation error: Validation failed"
        );
    }

    #[test]
    fn test_field_error_serialization() {
        let field_error = FieldError {
            field: "target_url".to_string(),
            message: "Target URL is required".to_string(),
            value: Some("".to_string()),
        };

        let json = serde_json::to_string(&field_error).unwrap();
        let deserialized: FieldError = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.field, field_error.field);
        assert_eq!(deserialized.message, field_error.message);
        assert_eq!(deserialized.value, field_error.value);
    }

    #[test]
    fn test_error_response_serialization() {
        let error_response = ErrorResponse::new(
            ErrorCode::ConfigValidationFailed,
            "Configuration validation failed".to_string(),
        )
        .with_details("Target URL must start with https://".to_string())
        .with_path("/api/config/update".to_string())
        .with_request_id("test-request-id".to_string());

        let json = serde_json::to_string(&error_response).unwrap();
        let deserialized: ErrorResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.code, error_response.code);
        assert_eq!(deserialized.message, error_response.message);
        assert_eq!(deserialized.details, error_response.details);
        assert_eq!(deserialized.path, error_response.path);
        assert_eq!(deserialized.request_id, error_response.request_id);
    }
}
