use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mtls-proxy")]
#[command(about = "mTLS Proxy Server for secure API proxying")]
#[command(version)]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Server host address
    #[arg(long, value_name = "HOST")]
    pub host: Option<String>,

    /// Server port
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,

    /// Target server URL
    #[arg(long, value_name = "URL")]
    pub target_url: Option<String>,

    /// Client certificate path
    #[arg(long, value_name = "FILE")]
    pub client_cert: Option<PathBuf>,

    /// Client private key path
    #[arg(long, value_name = "FILE")]
    pub client_key: Option<PathBuf>,

    /// CA certificate path
    #[arg(long, value_name = "FILE")]
    pub ca_cert: Option<PathBuf>,

    /// Disable hostname verification
    #[arg(long)]
    pub no_verify_hostname: bool,

    /// Request timeout in seconds
    #[arg(long, value_name = "SECONDS")]
    pub timeout: Option<u64>,

    /// Log level
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    pub log_level: String,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Show configuration and exit
    #[arg(long)]
    pub show_config: bool,
}

impl Cli {
    pub fn get_log_level(&self) -> &str {
        if self.verbose {
            "debug"
        } else {
            &self.log_level
        }
    }
}
