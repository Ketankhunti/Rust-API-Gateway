use std::sync::Arc;
use rust_api_gateway::{app, config::GatewayConfig};
use tracing::{info, Level};
use tokio::sync::RwLock;
use rust_api_gateway::run;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();

    info!("Loading configuration...");
    let config = Arc::new(RwLock::new(GatewayConfig::load(
        "gateway_config.yaml",
    )?));
    info!("Configuration loaded successfully.");

    run(config).await
}