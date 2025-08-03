use std::{net::IpAddr, sync::Arc};

use anyhow::Error;
use axum::{extract::State, middleware::from_fn_with_state, response::{IntoResponse, Response}, routing::{any, get}, Router};
use http::StatusCode;
use axum_client_ip::{ClientIpSource};

use crate::{middleware::{auth::auth::layer as auth_layer, cache::cache::layer as cache_layer, rate_limiter::rate_limit::layer as ratelimiter_layer}, proxy::proxy_handler, state::AppState};

async fn metrics_handler(state: State<Arc<AppState>>) -> String {
    state.prometheus_handle.as_ref().unwrap().render()
}

pub fn create_app(state: Arc<AppState>) -> Result<Router,Error> {
    let proxy_router = Router::new()
        .route("/{*path}", any(proxy_handler))
        .route_layer(from_fn_with_state(state.clone(), cache_layer))
        .route_layer(
            from_fn_with_state(state.clone(), ratelimiter_layer)
        ) 
        .route_layer(from_fn_with_state(state.clone(),auth_layer));

    let prometheus_router = Router::new()
        .route("/metrics", get(metrics_handler));

    let router = Router::new()
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .merge(proxy_router)
        .merge(prometheus_router)
        .with_state(state)
        .layer(ClientIpSource::ConnectInfo.into_extension());
 

    Ok(router)
}
