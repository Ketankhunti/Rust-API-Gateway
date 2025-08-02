use std::sync::Arc;

use anyhow::Error;
use axum::{routing::{any, get}, Router};
use http::StatusCode;

use crate::{proxy::proxy_handler, state::AppState};


pub fn create_app(state: Arc<AppState>) -> Result<Router,Error> {
    let app = Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .route("/{*path}", any(proxy_handler))
        .with_state(state);
    Ok(app)
}
