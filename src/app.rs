use std::sync::Arc;

use anyhow::Error;
use axum::{extract::State, routing::get, Router};

use crate::config::GatewayConfig;


pub async fn create_app(config: Arc<GatewayConfig>) -> Result<Router,Error> {
    let app = Router::new()
        .route("/", get(root_handler))
        .with_state(config);
    Ok(app)
}

async fn root_handler(
    State(config): State<Arc<GatewayConfig>>,
) -> String {
    format!(
        "Gateway is running! Configured to listen on {}. Number of routes: {}",
        config.server.addr,
        config.routes.len()
    )
}