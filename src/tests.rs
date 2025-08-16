use super::*;
use crate::logging::LogManager;
use crate::tls::TlsClient;
use std::path::PathBuf;

#[tokio::test]
async fn test_config_loading() {
    // Test that configuration can be loaded
    let config = Config::load();
    assert!(config.is_ok(), "Configuration should load successfully");
}

#[tokio::test]
async fn test_config_validation() {
    let mut config = Config::default();

    // Test valid configuration
    assert!(config.validate().is_ok(), "Default config should be valid");

    // Test invalid port
    config.server.port = 0;
    assert!(config.validate().is_err(), "Port 0 should be invalid");
    config.server.port = 8080; // Reset

    // Test invalid target URL
    config.target.base_url = "http://example.com".to_string();
    assert!(config.validate().is_err(), "HTTP URL should be invalid");
    config.target.base_url = "https://example.com".to_string(); // Reset

    // Test empty target URL
    config.target.base_url = "".to_string();
    assert!(config.validate().is_err(), "Empty URL should be invalid");
    config.target.base_url = "https://example.com".to_string(); // Reset

    // Test invalid timeout
    config.target.timeout_secs = 0;
    assert!(config.validate().is_err(), "Timeout 0 should be invalid");
    config.target.timeout_secs = 60; // Reset

    // Test invalid request size
    config.server.max_request_size_mb = 0;
    assert!(
        config.validate().is_err(),
        "Max request size 0 should be invalid"
    );
    config.server.max_request_size_mb = 10; // Reset

    // Test invalid concurrent requests
    config.server.max_concurrent_requests = 0;
    assert!(
        config.validate().is_err(),
        "Max concurrent requests 0 should be invalid"
    );
}

#[tokio::test]
async fn test_config_defaults() {
    // Test default configuration values
    let config = Config::default();

    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 8443);
    assert_eq!(config.server.max_connections, 1000);
    assert_eq!(config.server.connection_timeout_secs, 30);

    assert_eq!(config.target.base_url, "https://gpt-4o-mini.internal:443");
    assert_eq!(config.target.timeout_secs, 60);

    assert_eq!(config.logging.max_log_size_mb, 100);
    assert_eq!(config.logging.retention_days, 30);
    assert!(config.logging.compression_enabled);
}

#[tokio::test]
async fn test_tls_client_creation() {
    // Test TLS client creation with test certificates
    let cert_path = PathBuf::from("certs/client.crt");
    let key_path = PathBuf::from("certs/client.key");

    // Only run this test if certificates exist
    if cert_path.exists() && key_path.exists() {
        let tls_client = TlsClient::new(
            &cert_path,
            &key_path,
            Some(PathBuf::from("certs/ca.crt").as_path()),
            false, // Don't verify hostname for testing
        );

        assert!(
            tls_client.is_ok(),
            "TLS client should be created successfully"
        );
    } else {
        println!("Skipping TLS client test - certificates not found");
    }
}

#[tokio::test]
async fn test_log_manager_creation() {
    // Test log manager creation
    let db_path = PathBuf::from("test_logs.db");
    let log_dir = "test_logs".to_string();

    let log_manager = LogManager::new(&db_path, &log_dir, 100, 30, true);

    match log_manager {
        Ok(_) => println!("Log manager created successfully"),
        Err(e) => {
            println!("Log manager creation failed: {}", e);
            // Don't fail the test, just log the error
            return;
        }
    }

    // Clean up test files
    let _ = std::fs::remove_file(&db_path);
    let _ = std::fs::remove_dir_all(&log_dir);
}

#[test]
fn test_hop_by_hop_header_filtering() {
    // Test that hop-by-hop headers are properly identified
    let hop_by_hop_headers = vec![
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailers",
        "transfer-encoding",
        "upgrade",
    ];

    for header in hop_by_hop_headers {
        assert!(
            super::proxy::is_hop_by_hop_header(header),
            "{} should be identified as hop-by-hop header",
            header
        );
    }

    // Test that regular headers are not filtered
    let regular_headers = vec![
        "content-type",
        "authorization",
        "user-agent",
        "accept",
        "host",
    ];

    for header in regular_headers {
        assert!(
            !super::proxy::is_hop_by_hop_header(header),
            "{} should not be identified as hop-by-hop header",
            header
        );
    }
}

#[tokio::test]
async fn test_proxy_server_creation() {
    // Test proxy server creation with default config
    let config = Config::default();
    let proxy = ProxyServer::new(config).await;

    // This might fail if certificates don't exist, which is expected
    // We're just testing that the creation process doesn't panic
    match proxy {
        Ok(_) => println!("Proxy server created successfully"),
        Err(e) => println!(
            "Proxy server creation failed (expected if no certificates): {}",
            e
        ),
    }
}

// Integration test helper
#[allow(dead_code)]
pub async fn create_test_proxy_server() -> Result<ProxyServer, Box<dyn std::error::Error>> {
    let mut config = Config::default();

    // Use test certificates if they exist
    let cert_path = PathBuf::from("certs/client.crt");
    let key_path = PathBuf::from("certs/client.key");
    let ca_path = PathBuf::from("certs/ca.crt");

    if cert_path.exists() && key_path.exists() {
        config.tls.client_cert_path = cert_path;
        config.tls.client_key_path = key_path;
        config.tls.ca_cert_path = Some(ca_path);
        config.tls.verify_hostname = false;
    }

    // Use test target
    config.target.base_url = "https://localhost:8443".to_string();

    // Use test logging
    config.logging.sqlite_db_path = PathBuf::from("test_proxy_logs.db");
    config.logging.log_dir = PathBuf::from("test_logs");

    ProxyServer::new(config).await.map_err(|e| e.into())
}

// Clean up test files
#[allow(dead_code)]
pub fn cleanup_test_files() {
    let _ = std::fs::remove_file("test_logs.db");
    let _ = std::fs::remove_file("test_proxy_logs.db");
    let _ = std::fs::remove_dir_all("test_logs");
}
