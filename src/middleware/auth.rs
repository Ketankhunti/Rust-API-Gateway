use std::{ops::Deref, sync::Arc};

use axum::{extract::{Request, State}, middleware::Next, response::Response};
use http::Uri;

use crate::{config::RouteConfig, errors::AppError, features::auth::{check_roles, verify_token}, state::AppState};

// axum middleware layer for authentication
pub async fn layer (
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next
) -> Result<Response, AppError> 
{
    let route = find_route_for_uri(&req.uri(), state.clone()).await?;

    if let Some(auth_config) = &route.auth {
        // 1. verify token
        let claims = verify_token(req.headers(), auth_config)?;

        // 2. if route requires specific roles, check them
        if let Some(required_roles) = &auth_config.roles {
            check_roles(&claims.roles, required_roles)?;
        }

        // 3. add claims to request extension so that downstream handlers could potentially access user info
        req.extensions_mut().insert(claims);
    }
    // if auth succeeds then pass request to the next layer
    Ok(next.run(req).await)
}

async fn find_route_for_uri(uri: &Uri, state: Arc<AppState>) -> Result<Arc<RouteConfig>,AppError> {

    let config_guard = state.config.read().await;

    config_guard
        .find_route_for_path(uri.path())
        .ok_or(AppError::RouteNotFound)
    
}