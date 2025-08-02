use std::sync::Arc;

use reqwest::Client;

use crate::config::{ApiKeyStore, GatewayConfig, SecretsConfig};

use tokio::sync::RwLock;


pub struct AppState {
    pub config: Arc<RwLock<GatewayConfig>>,
    pub secrets: Arc<SecretsConfig>,
    pub key_store: Arc<RwLock<ApiKeyStore>>,
    pub http_client: Client,
}