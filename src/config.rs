use anyhow::Result;
use config::{Config as ConfigFile, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub logging: LoggingConfig,
    pub target: TargetConfig,
    pub ui: Option<UiConfig>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout_secs: u64,
    pub connection_pool_size: usize,
    pub max_request_size_mb: u64,
    pub max_concurrent_requests: usize,
    pub rate_limit_requests_per_second: u32,
    pub rate_limit_burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub client_cert_path: PathBuf,
    pub client_key_path: PathBuf,
    pub ca_cert_path: Option<PathBuf>,
    pub verify_hostname: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub log_dir: PathBuf,
    pub max_log_size_mb: u64,
    pub retention_days: u32,
    pub compression_enabled: bool,
    pub sqlite_db_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    pub base_url: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub enabled: bool,
    pub port: Option<u16>,
    pub host: Option<String>,
}



impl Config {
    pub fn load() -> Result<Self> {
        let config = ConfigFile::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("MTLS_PROXY"))
            .build()?;

        let config: Config = config.try_deserialize()?;
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        // Validate server configuration
        if self.server.port == 0 {
            anyhow::bail!("Server port cannot be 0");
        }
        if self.server.max_connections == 0 {
            anyhow::bail!("Max connections cannot be 0");
        }
        if self.server.connection_timeout_secs == 0 {
            anyhow::bail!("Connection timeout cannot be 0");
        }
        if self.server.max_request_size_mb == 0 {
            anyhow::bail!("Max request size cannot be 0");
        }
        if self.server.max_concurrent_requests == 0 {
            anyhow::bail!("Max concurrent requests cannot be 0");
        }

        // Validate TLS configuration
        if !self.tls.client_cert_path.exists() {
            anyhow::bail!("Client certificate file does not exist: {}", self.tls.client_cert_path.display());
        }
        if !self.tls.client_key_path.exists() {
            anyhow::bail!("Client key file does not exist: {}", self.tls.client_key_path.display());
        }
        if let Some(ref ca_path) = self.tls.ca_cert_path {
            if !ca_path.exists() {
                anyhow::bail!("CA certificate file does not exist: {}", ca_path.display());
            }
        }

        // Validate target configuration
        if self.target.base_url.is_empty() {
            anyhow::bail!("Target base URL cannot be empty");
        }
        if !self.target.base_url.starts_with("https://") {
            anyhow::bail!("Target base URL must start with 'https://'");
        }
        if self.target.timeout_secs == 0 {
            anyhow::bail!("Target timeout cannot be 0");
        }

        // Validate logging configuration
        if self.logging.max_log_size_mb == 0 {
            anyhow::bail!("Max log size cannot be 0");
        }
        if self.logging.retention_days == 0 {
            anyhow::bail!("Retention days cannot be 0");
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8443,
                max_connections: 1000,
                connection_timeout_secs: 30,
                connection_pool_size: 10,
                max_request_size_mb: 10,
                max_concurrent_requests: 100,
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
                base_url: "https://gpt-4o-mini.internal:443".to_string(),
                timeout_secs: 60,
            },
            ui: Some(UiConfig {
                enabled: true,
                port: None,
                host: None,
            }),

        }
    }
}
