use std::{ops::Deref, sync::Arc};

use axum::{extract::{Request, State}, middleware::Next, response::Response};
use http::Uri;

use crate::{config::RouteConfig, errors::AppError, features::{auth::verify_token}, state::AppState};

// axum middleware layer for authentication
pub async fn layer (
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next
) -> Result<Response, AppError> 
{
    let route = find_route_for_uri(&req.uri(), state.clone()).await?;

    let auth_required = route.auth.as_ref().map_or(false, |auth| auth.required);

    if auth_required {
        verify_token(req.headers())?;
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