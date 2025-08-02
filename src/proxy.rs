use axum::{body::Body, extract::{Path, State}, http::{HeaderMap, Request, Uri}, response::{IntoResponse, Response}};
use http::Method;
use tracing::info;
use http_body_util::BodyExt;
use bytes::Bytes;
use std::sync::Arc;

use crate::{errors::AppError, state::AppState};

#[axum::debug_handler]
pub async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> Result<Response, AppError> {

    let request_path = format!("/{}", path);
    info!("Received request for path: {}", request_path);

    let config_guard = state.config.read().await;
    let matched_route = config_guard
    .find_route_for_path(&request_path);

    let route = match matched_route {
        Some(route) => route,
        None => return Err(AppError::RouteNotFound),
    };

    // let destination_path = request_path.strip_prefix(&route.path).unwrap_or("");
    
    let destination_url = format!("{}{}", route.destination, request_path);

    info!("Destination Url: {}", destination_url);

    let body_bytes: Bytes = body.collect().await
        .map_err(|e| {
        tracing::error!("Failed to read request body: {}", e);
        AppError::InternalServerError
        })?
        .to_bytes();

    let request = state.http_client
        .request(method, &destination_url)
        .headers(headers)
        .body(body_bytes)
        .build()
        .map_err(|e|{
            tracing::error!("Failed to build reqwest request: {}", e);
            AppError::InvalidDestination(destination_url)
        })?;

        let response = state.http_client.execute(request).await?;

        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.bytes().await.map_err(AppError::from)?;
        let body = Body::from(bytes);

        let mut response_builder = Response::builder().status(status);
        for (name, value) in headers.iter() {
            response_builder = response_builder.header(name, value);
        }

        let response = response_builder.body(body).unwrap();

        Ok(response)    
        

}