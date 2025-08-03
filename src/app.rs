use std::{net::IpAddr, sync::Arc};

use anyhow::Error;
use axum::{middleware::from_fn_with_state, routing::{any, get}, Router};
use http::StatusCode;
use axum_client_ip::{ClientIpSource};

use crate::{middleware::{auth::auth::layer as auth_layer, rate_limiter::rate_limit::layer as ratelimiter_layer}, proxy::proxy_handler, state::AppState};


pub fn create_app(state: Arc<AppState>) -> Result<Router,Error> {
    let proxy_router = Router::new()
        .route("/{*path}", any(proxy_handler))
        .route_layer(
            from_fn_with_state(state.clone(), ratelimiter_layer)
        )
        .route_layer(from_fn_with_state(state.clone(),auth_layer));

    let router = Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .merge(proxy_router)
        .with_state(state)
        .layer(ClientIpSource::ConnectInfo.into_extension());
        
    Ok(router)
}
