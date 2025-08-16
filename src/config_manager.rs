use crate::config::Config;
use crate::errors::{AppError, ErrorCode, config_error, certificate_error, filesystem_error, validation_error};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdateRequest {
    pub target_url: String,
    pub timeout_secs: u64,
    pub max_connections: usize,
 // Optional for updates
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateUpload {
    pub cert_type: CertificateType,
    pub filename: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateType {
    Client,
    Key,
    CA,
}

impl std::fmt::Display for CertificateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CertificateType::Client => write!(f, "client"),
            CertificateType::Key => write!(f, "key"),
            CertificateType::CA => write!(f, "ca"),
        }
    }
}

#[derive(Clone)]
pub struct ConfigManager {
    config_path: PathBuf,
    certs_dir: PathBuf,
    config: Arc<RwLock<Config>>,
}

impl ConfigManager {
    pub fn new(config: Config) -> Self {
        // Use development paths if running in development environment
        let config_path = if std::env::var("RUST_ENV").unwrap_or_default() == "development" {
            PathBuf::from("./config/config.toml")
        } else {
            PathBuf::from("/etc/mtls-proxy/config.toml")
        };
        
        let certs_dir = if std::env::var("RUST_ENV").unwrap_or_default() == "development" {
            PathBuf::from("./certs")
        } else {
            PathBuf::from("/etc/mtls-proxy/certs")
        };
        
        Self {
            config_path,
            certs_dir,
            config: Arc::new(RwLock::new(config)),
        }
    }
    
    pub async fn get_current_config(&self) -> Result<Config> {
        Ok(self.config.read().await.clone())
    }
    
    pub async fn update_config(&self, update: ConfigUpdateRequest) -> Result<(), AppError> {
        let mut config = self.config.write().await;
        
        // Update configuration fields
        config.target.base_url = update.target_url;
        config.target.timeout_secs = update.timeout_secs;
        config.server.max_connections = update.max_connections;

        
        // Validate the updated configuration
        if let Err(e) = config.validate() {
            return Err(config_error(
                ErrorCode::ConfigValidationFailed,
                "Configuration validation failed",
                Some(&e.to_string()),
            ));
        }
        
        // Save to disk
        if let Err(e) = self.save_config_to_disk(&config).await {
            return Err(filesystem_error(
                ErrorCode::FileSystemError,
                "Failed to save configuration to disk",
                Some(&e.to_string()),
            ));
        }
        
        info!("Configuration updated successfully");
        Ok(())
    }
    
    pub async fn upload_certificate(&self, upload: CertificateUpload) -> Result<(), AppError> {
        // Validate certificate content
        if let Err(e) = self.validate_certificate_content(&upload) {
            return Err(certificate_error(
                ErrorCode::CertificateInvalid,
                "Invalid certificate content",
                Some(&e.to_string()),
            ));
        }
        
        // Determine file path based on certificate type
        let filename = match upload.cert_type {
            CertificateType::Client => "client.crt",
            CertificateType::Key => "client.key",
            CertificateType::CA => "ca.crt",
        };
        
        let file_path = self.certs_dir.join(filename);
        
        // Ensure certificates directory exists
        if let Err(e) = fs::create_dir_all(&self.certs_dir) {
            return Err(filesystem_error(
                ErrorCode::FileSystemError,
                "Failed to create certificates directory",
                Some(&e.to_string()),
            ));
        }
        
        // Write certificate file
        if let Err(e) = fs::write(&file_path, &upload.content) {
            return Err(filesystem_error(
                ErrorCode::FileSystemError,
                "Failed to write certificate file",
                Some(&e.to_string()),
            ));
        }
        
        // Set proper permissions
        if let Err(e) = self.set_certificate_permissions(&file_path, &upload.cert_type) {
            return Err(filesystem_error(
                ErrorCode::FilePermissionDenied,
                "Failed to set certificate permissions",
                Some(&e.to_string()),
            ));
        }
        
        // Update configuration to reflect new certificate paths
        if let Err(e) = self.update_config_certificate_paths().await {
            return Err(config_error(
                ErrorCode::ConfigUpdateFailed,
                "Failed to update certificate paths in configuration",
                Some(&e.to_string()),
            ));
        }
        
        info!("Certificate {} uploaded successfully", upload.cert_type);
        Ok(())
    }
    
    pub async fn list_certificates(&self) -> Result<Vec<String>> {
        let mut certificates = Vec::new();
        
        if self.certs_dir.exists() {
            for entry in fs::read_dir(&self.certs_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if let Some(filename) = path.file_name() {
                    if let Some(name) = filename.to_str() {
                        certificates.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(certificates)
    }
    
    pub async fn delete_certificate(&self, filename: &str) -> Result<()> {
        let file_path = self.certs_dir.join(filename);
        
        if file_path.exists() {
            fs::remove_file(&file_path)?;
            info!("Certificate {} deleted successfully", filename);
        } else {
            warn!("Certificate file {} not found", filename);
        }
        
        Ok(())
    }
    
    pub async fn validate_config(&self) -> Result<()> {
        let config = self.config.read().await;
        config.validate()?;
        Ok(())
    }
    
    async fn save_config_to_disk(&self, config: &Config) -> Result<()> {
        // Create config directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Convert config to TOML format
        let toml_string = toml::to_string_pretty(config)?;
        
        // Write to disk
        fs::write(&self.config_path, toml_string)?;
        
        info!("Configuration saved to {}", self.config_path.display());
        Ok(())
    }
    
    fn validate_certificate_content(&self, upload: &CertificateUpload) -> Result<()> {
        match upload.cert_type {
            CertificateType::Client | CertificateType::CA => {
                // Validate certificate format
                let content_str = String::from_utf8_lossy(&upload.content);
                if !content_str.contains("-----BEGIN CERTIFICATE-----") {
                    anyhow::bail!("Invalid certificate format");
                }
            }
            CertificateType::Key => {
                // Validate private key format
                let content_str = String::from_utf8_lossy(&upload.content);
                if !content_str.contains("-----BEGIN PRIVATE KEY-----") && 
                   !content_str.contains("-----BEGIN RSA PRIVATE KEY-----") {
                    anyhow::bail!("Invalid private key format");
                }
            }
        }
        
        Ok(())
    }
    
    fn set_certificate_permissions(&self, file_path: &PathBuf, cert_type: &CertificateType) -> Result<()> {
        match cert_type {
            CertificateType::Key => {
                // Private key should have restrictive permissions
                fs::set_permissions(file_path, fs::Permissions::from_mode(0o600))?;
            }
            CertificateType::Client | CertificateType::CA => {
                // Certificates can have read permissions
                fs::set_permissions(file_path, fs::Permissions::from_mode(0o644))?;
            }
        }
        
        // Set ownership to mtls-proxy user (if running as root)
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::chown;
            use std::os::unix::fs::PermissionsExt;
            
            // Try to set ownership to mtls-proxy user (UID 1000 is typical for service users)
            if let Ok(uid) = std::env::var("SUDO_UID").or_else(|_| std::env::var("UID")) {
                if let Ok(uid) = uid.parse::<u32>() {
                    let _ = chown(file_path, Some(uid), Some(uid));
                }
            }
        }
        
        Ok(())
    }
    
    async fn update_config_certificate_paths(&self) -> Result<()> {
        let mut config = self.config.write().await;
        
        // Update certificate paths to point to the uploaded files
        config.tls.client_cert_path = self.certs_dir.join("client.crt");
        config.tls.client_key_path = self.certs_dir.join("client.key");
        
        // Set CA certificate path if it exists
        let ca_path = self.certs_dir.join("ca.crt");
        if ca_path.exists() {
            config.tls.ca_cert_path = Some(ca_path);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ServerConfig, TlsConfig, LoggingConfig, TargetConfig, UiConfig};
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config() -> Config {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8443,
                max_connections: 100,
                connection_timeout_secs: 30,
                connection_pool_size: 10,
                max_request_size_mb: 10,
                max_concurrent_requests: 50,
                rate_limit_requests_per_second: 100,
                rate_limit_burst_size: 200,
            },
            tls: TlsConfig {
                client_cert_path: PathBuf::from("certs/client.crt"),
                client_key_path: PathBuf::from("certs/client.key"),
                ca_cert_path: None,
                verify_hostname: true,
            },
            logging: LoggingConfig {
                log_dir: PathBuf::from("logs"),
                max_log_size_mb: 100,
                retention_days: 30,
                compression_enabled: true,
                sqlite_db_path: PathBuf::from("logs/proxy_logs.db"),
            },
            target: TargetConfig {
                base_url: "https://example.com".to_string(),
                timeout_secs: 60,
            },
            ui: Some(UiConfig {
                enabled: true,
                port: None,
                host: None,
            }),
        }
    }

    #[tokio::test]
    async fn test_config_manager_creation() {
        let config = create_test_config();
        let config_manager = ConfigManager::new(config);
        
        assert_eq!(config_manager.config_path, PathBuf::from("./config/config.toml"));
        assert_eq!(config_manager.certs_dir, PathBuf::from("./certs"));
    }

    #[tokio::test]
    async fn test_get_current_config() {
        let config = create_test_config();
        let config_manager = ConfigManager::new(config.clone());
        
        let current_config = config_manager.get_current_config().await.unwrap();
        assert_eq!(current_config.target.base_url, config.target.base_url);
        assert_eq!(current_config.target.timeout_secs, config.target.timeout_secs);
    }

    #[tokio::test]
    async fn test_update_config_success() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let certs_dir = temp_dir.path().join("certs");
        
        let mut config = create_test_config();
        config.target.base_url = "https://test.example.com".to_string();
        
        let config_manager = ConfigManager::new(config);
        
        let update = ConfigUpdateRequest {
            target_url: "https://new.example.com".to_string(),
            timeout_secs: 120,
            max_connections: 200,
        };
        
        let result = config_manager.update_config(update).await;
        assert!(result.is_ok());
        
        let updated_config = config_manager.get_current_config().await.unwrap();
        assert_eq!(updated_config.target.base_url, "https://new.example.com");
        assert_eq!(updated_config.target.timeout_secs, 120);
        assert_eq!(updated_config.server.max_connections, 200);
    }

    #[tokio::test]
    async fn test_update_config_validation_error() {
        let config = create_test_config();
        let config_manager = ConfigManager::new(config);
        
        let update = ConfigUpdateRequest {
            target_url: "http://invalid-url.com".to_string(), // Should fail validation
            timeout_secs: 60,
            max_connections: 100,
        };
        
        let result = config_manager.update_config(update).await;
        assert!(result.is_err());
        
        if let Err(AppError::Config(e)) = result {
            assert_eq!(e.code, ErrorCode::ConfigValidationFailed);
        } else {
            panic!("Expected Config error");
        }
    }

    #[tokio::test]
    async fn test_upload_certificate_success() {
        let temp_dir = TempDir::new().unwrap();
        let certs_dir = temp_dir.path().join("certs");
        fs::create_dir_all(&certs_dir).unwrap();
        
        let config = create_test_config();
        let mut config_manager = ConfigManager::new(config);
        // Override the certs directory for testing
        config_manager.certs_dir = certs_dir.clone();
        
        let cert_content = b"-----BEGIN CERTIFICATE-----\nMIIDiDCCAnCgAwIBAgIUZtVzwAULNmpRMhGZoCZ93kGnvewwDQYJKoZIhvcNAQEL\nBQAwXDELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRYwFAYDVQQHDA1TYW4gRnJh\n-----END CERTIFICATE-----";
        
        let upload = CertificateUpload {
            cert_type: CertificateType::Client,
            filename: "test_client.crt".to_string(),
            content: cert_content.to_vec(),
        };
        
        let result = config_manager.upload_certificate(upload).await;
        assert!(result.is_ok());
        
        // Verify file was created
        let expected_path = certs_dir.join("client.crt");
        assert!(expected_path.exists());
    }

    #[tokio::test]
    async fn test_upload_certificate_invalid_content() {
        let config = create_test_config();
        let config_manager = ConfigManager::new(config);
        
        let invalid_content = b"Invalid certificate content";
        
        let upload = CertificateUpload {
            cert_type: CertificateType::Client,
            filename: "invalid.crt".to_string(),
            content: invalid_content.to_vec(),
        };
        
        let result = config_manager.upload_certificate(upload).await;
        assert!(result.is_err());
        
        if let Err(AppError::Certificate(e)) = result {
            assert_eq!(e.code, ErrorCode::CertificateInvalid);
        } else {
            panic!("Expected Certificate error");
        }
    }

    #[tokio::test]
    async fn test_list_certificates() {
        let temp_dir = TempDir::new().unwrap();
        let certs_dir = temp_dir.path().join("certs");
        fs::create_dir_all(&certs_dir).unwrap();
        
        // Create some test certificate files
        fs::write(certs_dir.join("client.crt"), "test cert").unwrap();
        fs::write(certs_dir.join("server.crt"), "test cert").unwrap();
        fs::write(certs_dir.join("ca.crt"), "test cert").unwrap();
        
        let config = create_test_config();
        let mut config_manager = ConfigManager::new(config);
        // Override the certs directory for testing
        config_manager.certs_dir = certs_dir.clone();
        
        let certificates = config_manager.list_certificates().await.unwrap();
        assert!(certificates.contains(&"client.crt".to_string()));
        assert!(certificates.contains(&"server.crt".to_string()));
        assert!(certificates.contains(&"ca.crt".to_string()));
    }

    #[tokio::test]
    async fn test_delete_certificate() {
        let temp_dir = TempDir::new().unwrap();
        let certs_dir = temp_dir.path().join("certs");
        fs::create_dir_all(&certs_dir).unwrap();
        
        let cert_path = certs_dir.join("test.crt");
        fs::write(&cert_path, "test cert").unwrap();
        assert!(cert_path.exists());
        
        let config = create_test_config();
        let mut config_manager = ConfigManager::new(config);
        // Override the certs directory for testing
        config_manager.certs_dir = certs_dir.clone();
        
        let result = config_manager.delete_certificate("test.crt").await;
        assert!(result.is_ok());
        assert!(!cert_path.exists());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_certificate() {
        let config = create_test_config();
        let config_manager = ConfigManager::new(config);
        
        let result = config_manager.delete_certificate("nonexistent.crt").await;
        assert!(result.is_ok()); // Should not error, just log warning
    }

    #[tokio::test]
    async fn test_validate_config() {
        let config = create_test_config();
        let config_manager = ConfigManager::new(config);
        
        let result = config_manager.validate_config().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_certificate_type_display() {
        assert_eq!(CertificateType::Client.to_string(), "client");
        assert_eq!(CertificateType::Key.to_string(), "key");
        assert_eq!(CertificateType::CA.to_string(), "ca");
    }

    #[test]
    fn test_config_update_request_serialization() {
        let request = ConfigUpdateRequest {
            target_url: "https://example.com".to_string(),
            timeout_secs: 60,
            max_connections: 100,
        };
        
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ConfigUpdateRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.target_url, request.target_url);
        assert_eq!(deserialized.timeout_secs, request.timeout_secs);
        assert_eq!(deserialized.max_connections, request.max_connections);
    }

    #[test]
    fn test_certificate_upload_serialization() {
        let upload = CertificateUpload {
            cert_type: CertificateType::Client,
            filename: "test.crt".to_string(),
            content: b"test content".to_vec(),
        };
        
        let json = serde_json::to_string(&upload).unwrap();
        let deserialized: CertificateUpload = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.cert_type.to_string(), upload.cert_type.to_string());
        assert_eq!(deserialized.filename, upload.filename);
        assert_eq!(deserialized.content, upload.content);
    }
}
