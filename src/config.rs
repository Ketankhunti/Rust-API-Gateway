use std::{collections::HashMap, fs, path::Path, sync::Arc};

use anyhow::{Error, Ok};
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub routes: Vec<Arc<RouteConfig>>,
    pub identity: IdentityConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct IdentityConfig {
    pub api_key_store_path: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum AuthType {
    Jwt,
    ApiKey,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    #[serde(rename="type")]
    pub auth_type: AuthType,
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig{
    pub requests: u64,
    pub period: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub name: String,
    pub path: String,
    pub destination: String,
    pub auth: Option<AuthConfig>,
    pub rate_limit: Option<RateLimitConfig>
}

impl GatewayConfig {
    pub fn load<P: AsRef<Path>> (path: P) -> Result<Self,anyhow::Error> {
        let content = fs::read_to_string(path)?;
        let config: GatewayConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn find_route_for_path(&self, request_path: &str) -> Option<Arc<RouteConfig>> {
        self.routes
            .iter()
            .filter(|r| request_path.starts_with(&r.path))
            .max_by_key(|r| r.path.len())
            .cloned()
           
    }
}

//       API key store condig    //


#[derive(Debug, Deserialize, Clone)]
pub struct ApiKeyStore {
    pub keys: HashMap<String, ApiKeyDetails>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiKeyDetails {
    pub user_id: String,
    pub roles: Vec<String>,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "active".to_string()
}

impl ApiKeyStore {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content).map_err(Into::into)
    }
}

//      Secrets Config
pub struct SecretsConfig  {
    pub jwt_secret: String
}

impl SecretsConfig {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set in .env file"))?,
        })
    }
}
