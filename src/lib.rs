pub mod config;
pub mod errors;
pub mod app;
pub mod state;
pub mod proxy;
pub mod middleware;
pub mod features;

use std::{net::{IpAddr, SocketAddr}, sync::Arc};

use anyhow::Result;
use axum::ServiceExt;
use axum_client_ip::ClientIp;
use reqwest::Client;
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;

use crate::{config::{ApiKeyStore, GatewayConfig, SecretsConfig}, features::rate_limiter::state::InMemoryRateLimitState, state::AppState};

pub async fn run(
    config: Arc<RwLock<GatewayConfig>>,
    secrets: Arc<SecretsConfig>,
    key_store: Arc<RwLock<ApiKeyStore>>,
) -> Result<()> {

    let app_state = Arc::new(AppState {
        config: config.clone(),
        secrets,
        key_store,
        rate_limit_store: Arc::new(InMemoryRateLimitState::new()),
        http_client: Client::new(),
    });

    let app = app::create_app(app_state)?;


    let config_guard = config.read().await;

    let addr  = config_guard.server.addr.clone();

    let listener = TcpListener::bind(&addr).await?;
    info!("Gateway listening on {}", &addr);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}