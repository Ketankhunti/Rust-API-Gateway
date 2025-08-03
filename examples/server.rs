use std::sync::Arc;
use api_gateway::{config::{ApiKeyStore, GatewayConfig, SecretsConfig}, utils::config_path::Cli};
use tracing::{info, Level};
use tokio::sync::RwLock;
use api_gateway::run;
use dotenvy::dotenv;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let cli = Cli::parse();

    run(cli.config).await

}