use anyhow::Result;
use clap::Parser;
use mtls_proxy::cli::Cli;
use mtls_proxy::config::Config;
use mtls_proxy::proxy::ProxyServer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(match cli.get_log_level() {
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        })
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    info!("Starting mTLS Proxy Server");

    // Load configuration
    let mut config = Config::load()?;

    // Override configuration with CLI arguments
    if let Some(host) = cli.host {
        config.server.host = host;
    }
    if let Some(port) = cli.port {
        config.server.port = port;
    }
    if let Some(target_url) = cli.target_url {
        config.target.base_url = target_url;
    }
    if let Some(client_cert) = cli.client_cert {
        config.tls.client_cert_path = client_cert;
    }
    if let Some(client_key) = cli.client_key {
        config.tls.client_key_path = client_key;
    }
    if let Some(ca_cert) = cli.ca_cert {
        config.tls.ca_cert_path = Some(ca_cert);
    }
    if cli.no_verify_hostname {
        config.tls.verify_hostname = false;
    }
    if let Some(timeout) = cli.timeout {
        config.target.timeout_secs = timeout;
    }

    info!("Configuration loaded successfully");

    // Show configuration and exit if requested
    if cli.show_config {
        println!("Configuration:");
        println!("  Server: {}:{}", config.server.host, config.server.port);
        println!("  Max Connections: {}", config.server.max_connections);
        println!(
            "  Max Request Size: {}MB",
            config.server.max_request_size_mb
        );
        println!(
            "  Max Concurrent Requests: {}",
            config.server.max_concurrent_requests
        );
        println!(
            "  Connection Pool Size: {}",
            config.server.connection_pool_size
        );
        println!(
            "  Rate Limit: {}/s (burst: {})",
            config.server.rate_limit_requests_per_second, config.server.rate_limit_burst_size
        );

        println!("  Target: {}", config.target.base_url);
        println!("  Client Cert: {}", config.tls.client_cert_path.display());
        println!("  Client Key: {}", config.tls.client_key_path.display());
        println!(
            "  CA Cert: {:?}",
            config.tls.ca_cert_path.as_ref().map(|p| p.display())
        );
        println!("  Verify Hostname: {}", config.tls.verify_hostname);
        println!("  Timeout: {}s", config.target.timeout_secs);
        return Ok(());
    }

    // Create and start proxy server
    let proxy = ProxyServer::new(config).await?;
    proxy.start().await?;

    Ok(())
}
