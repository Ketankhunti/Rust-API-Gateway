use std::{fs, path::Path, sync::Arc};

use anyhow::Ok;
use serde::{Deserialize};
use tokio::sync::RwLock;


#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub routes: Vec<Arc<RouteConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String
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
pub struct RouteConfig {
    pub name: String,
    pub path: String,
    pub destination: String,
    pub auth: Option<AuthConfig>,
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