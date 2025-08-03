use std::sync::Arc;

use reqwest::Client;

use crate::{config::{ApiKeyStore, GatewayConfig, SecretsConfig}, features::rate_limiter::state::RateLimitState};

use tokio::sync::RwLock;


pub struct AppState {
    pub config: Arc<RwLock<GatewayConfig>>,
    pub secrets: Arc<SecretsConfig>,
    pub key_store: Arc<RwLock<ApiKeyStore>>,
    pub rate_limit_store: Arc<dyn RateLimitState>,
    pub http_client: Client,
}