use std::{sync::Arc, time::Instant};
use bytes::Bytes;
use http::{HeaderMap, StatusCode};
use moka::future::Cache;
use reqwest::Client;

use crate::{config::{ApiKeyStore, GatewayConfig, SecretsConfig}, features::rate_limiter::state::RateLimitState};

use tokio::sync::RwLock;

pub struct CachedResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
    pub inserted_at: Instant
}

pub struct AppState {
    pub config: Arc<RwLock<GatewayConfig>>,
    pub secrets: Arc<SecretsConfig>,
    pub key_store: Arc<RwLock<ApiKeyStore>>,
    pub rate_limit_store: Arc<dyn RateLimitState>,
    pub cache: Arc<Cache<String,Arc<CachedResponse>>>,
    pub http_client: Client,
}