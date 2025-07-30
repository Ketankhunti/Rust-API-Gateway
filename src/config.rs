use std::{fs, path::Path};

use anyhow::Ok;
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub name: String,
    pub path: String,
    pub destination: String,
}

impl GatewayConfig {
    pub fn load<P: AsRef<Path>> (path: P) -> Result<Self,anyhow::Error> {
        let content = fs::read_to_string(path)?;
        let config: GatewayConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}