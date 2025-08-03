pub mod config;
pub mod errors;
pub mod app;
pub mod state;
pub mod proxy;
pub mod middleware;
pub mod features;
pub mod utils;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use dotenvy::dotenv;
use reqwest::Client;
use tokio::{net::TcpListener, sync::RwLock};
use tracing::{info, Level};

use crate::{config::{ApiKeyStore, GatewayConfig, SecretsConfig}, features::rate_limiter::state::InMemoryRateLimitState, state::AppState, utils::{config_path::Cli, hot_reload}};

pub async fn run(
    config_path: PathBuf,
) -> Result<()> {


    dotenv().ok();

    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();

    info!("Loading secrets...");
    let secrets = Arc::new(SecretsConfig::from_env()?);

    info!("Loading gateway configuration...");
    let config = Arc::new(RwLock::new(GatewayConfig::load(
        config_path.clone(),
    )?));
    info!("Configuration loaded successfully.");

    let key_store_path   = config.read().await.identity.api_key_store_path.clone(); 
    
    info!(path = ?key_store_path, "Loading API key store...");

    let key_store = Arc::new(RwLock::new(ApiKeyStore::load(&key_store_path)?));


    let app_state = Arc::new(AppState {
        config: config.clone(),
        secrets,
        key_store: key_store.clone(),
        rate_limit_store: Arc::new(InMemoryRateLimitState::new()),
        http_client: Client::new(),
    });

    // start hot reloader
    tokio::spawn(hot_reload::watch_config_files(
        config_path,
        config.clone(),
        key_store.clone(), // Clone for the watcher task
    ));

    let app = app::create_app(app_state)?;

    let config_guard = config.read().await;

    let addr  = config_guard.server.addr.clone();

    let listener = TcpListener::bind(&addr).await?;
    info!("Gateway listening on {}", &addr);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}