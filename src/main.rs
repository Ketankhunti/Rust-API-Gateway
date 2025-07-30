use std::sync::Arc;

use anyhow::Ok;
use rust_api_gateway::{app, config::GatewayConfig};
use tokio::net::{TcpListener};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

    eprintln!("Loading configuration from gateway_config.yaml...");
    let config = Arc::new(GatewayConfig::load("gateway_config.yaml")?);
    eprintln!("Configuration loaded successfully.");
    
    let app = app::create_app(config.clone()).await?;
    eprintln!("Application router created.");

    let listener = TcpListener::bind(&config.server.addr).await?;
    eprintln!("Server listeninng on {}",&config.server.addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())


}