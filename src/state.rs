use std::sync::Arc;

use reqwest::Client;

use crate::config::GatewayConfig;

use tokio::sync::RwLock;


pub struct AppState {
    pub config: Arc<RwLock<GatewayConfig>>,
    pub http_client: Client,
}