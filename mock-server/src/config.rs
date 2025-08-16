use anyhow::Result;
use config::{Config as ConfigFile, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub responses: ResponseConfig,
    pub models: ModelsConfig,
    pub scenarios: Option<ScenariosConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub ca_cert_path: Option<PathBuf>,
    pub require_client_cert: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseConfig {
    pub default_delay_ms: u64,
    pub error_rate_percent: u8,
    pub streaming_enabled: bool,
    pub max_tokens: usize,
    pub temperature: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelsConfig {
    pub available: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScenariosConfig {
    pub fast: Option<ScenarioConfig>,
    pub slow: Option<ScenarioConfig>,
    pub errors: Option<ScenarioConfig>,
    pub timeout: Option<ScenarioConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScenarioConfig {
    pub name: String,
    pub default_delay_ms: u64,
    pub error_rate_percent: u8,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = ConfigFile::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/test_scenarios").required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("MOCK_GPT"))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn get_scenario_config(&self, scenario_name: &str) -> Option<&ScenarioConfig> {
        self.scenarios.as_ref().and_then(|scenarios| match scenario_name {
            "fast" => scenarios.fast.as_ref(),
            "slow" => scenarios.slow.as_ref(),
            "errors" => scenarios.errors.as_ref(),
            "timeout" => scenarios.timeout.as_ref(),
            _ => None,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8443,
                max_connections: 1000,
            },
            tls: TlsConfig {
                cert_path: PathBuf::from("certs/server.crt"),
                key_path: PathBuf::from("certs/server.key"),
                ca_cert_path: Some(PathBuf::from("certs/ca.crt")),
                require_client_cert: true,
            },
            responses: ResponseConfig {
                default_delay_ms: 100,
                error_rate_percent: 0,
                streaming_enabled: true,
                max_tokens: 1000,
                temperature: 0.7,
            },
            models: ModelsConfig {
                available: vec![
                    "gpt-4o-mini".to_string(),
                    "gpt-4o".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ],
            },
            scenarios: None,
        }
    }
}
