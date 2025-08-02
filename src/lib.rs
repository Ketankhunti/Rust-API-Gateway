pub mod config;
pub mod errors;
pub mod app;
pub mod state;
pub mod proxy;
pub mod middleware;
pub mod features;

use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;

use crate::{config::GatewayConfig, state::AppState};

pub async fn run(config: Arc<RwLock<GatewayConfig>>) -> Result<()> {
    let app_state = Arc::new(AppState {
        config: config.clone(),
        http_client: Client::new(),
    });

    let app = app::create_app(app_state)?;

    let config_guard = config.read().await;

    let addr  = config_guard.server.addr.clone();

    let listener = TcpListener::bind(&addr).await?;
    info!("Gateway listening on {}", &addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}