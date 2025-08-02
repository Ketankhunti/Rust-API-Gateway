// use std::sync::Arc;

// use anyhow::Ok;
// use reqwest::Client;
// use rust_api_gateway::{app, config::GatewayConfig, state::AppState};
// use tokio::net::{TcpListener};
// use tracing::{info, Level};
// use tokio::sync::RwLock;

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     tracing_subscriber::fmt()
//     .with_max_level(Level::INFO)
//     .init();

//     info!("Loading configuration from gateway_config.yaml...");
//     let config = Arc::new(RwLock::new(GatewayConfig::load("gateway_config.yaml")?));
//     info!("Configuration loaded successfully.");

//     let app_state = Arc::new(AppState {
//         config: config.clone(),
//         http_client: Client::new(),
//     });

//     let app = app::create_app(app_state.clone())?;
//     info!("Application router created.");

//     let listener = TcpListener::bind(&config.read().await.server.addr).await?;
//     info!("Server listening on {}", &config.read().await.server.addr);
//     axum::serve(listener, app.into_make_service()).await?;

//     Ok(())
// }

fn main() {
    
}