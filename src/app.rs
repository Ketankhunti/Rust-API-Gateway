use std::sync::Arc;

use anyhow::Error;
use axum::{middleware::{self, from_fn_with_state}, routing::{any, get}, Router};
use http::StatusCode;

use crate::{middleware::auth, proxy::proxy_handler, state::AppState};


pub fn create_app(state: Arc<AppState>) -> Result<Router,Error> {
    let proxy_router = Router::new()
        .route("/{*path}", any(proxy_handler))
        .route_layer(from_fn_with_state(state.clone(),auth::layer));

    let router = Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .merge(proxy_router)
        .with_state(state);

    Ok(router)
}
