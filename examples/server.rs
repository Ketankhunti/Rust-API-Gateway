use std::sync::Arc;
use rust_api_gateway::{app, config::{ApiKeyStore, GatewayConfig, SecretsConfig}};
use tracing::{info, Level};
use tokio::sync::RwLock;
use rust_api_gateway::run;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    dotenv().ok();

    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();

    info!("Loading secrets...");
    let secrets = Arc::new(SecretsConfig::from_env()?);

    info!("Loading gateway configuration...");
    let config_path = "gateway_config.yaml";
    let config = Arc::new(RwLock::new(GatewayConfig::load(
        config_path,
    )?));
    info!("Configuration loaded successfully.");

    let key_store_path   = config.read().await.identity.api_key_store_path.clone(); 
    
    info!(path = ?key_store_path, "Loading API key store...");

    let key_store = Arc::new(RwLock::new(ApiKeyStore::load(&key_store_path)?));

    // Start the server with all configurations
    run(config, secrets, key_store).await
}