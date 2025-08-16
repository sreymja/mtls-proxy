use anyhow::Result;
use mock_gpt_server::config::Config;
use mock_gpt_server::server::MockServer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    info!("Starting Mock GPT-4o-mini API Server");

    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded successfully");

    // Create and start mock server
    let server = MockServer::new(config)?;
    server.start().await?;

    Ok(())
}
