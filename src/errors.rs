use axum::{http::StatusCode, response::{IntoResponse, Response}};


#[derive(Debug)]
pub enum AppError {
    // Auth errors
    AuthFailed,
    MissingAuthToken,
    InvalidAuthHeader,

    // Proxy errors
    RouteNotFound,
    ProxyError(reqwest::Error),
    InvalidDestination(String),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthFailed => (StatusCode::UNAUTHORIZED, "Authentication failed".to_string()),
            AppError::MissingAuthToken => (StatusCode::UNAUTHORIZED, "Missing 'Authorization' header".to_string()),
            AppError::InvalidAuthHeader => (StatusCode::UNAUTHORIZED, "Invalid 'Authorization' header format. Expected 'Bearer <token>'.".to_string()),

            AppError::RouteNotFound => (StatusCode::NOT_FOUND, "Route not found".to_string()),
            AppError::ProxyError(e) => {
                tracing::error!("Proxy error: {}", e);
                (StatusCode::BAD_GATEWAY, "Error proxying request".to_string())
            }
            AppError::InvalidDestination(url) => {
                tracing::error!("Invalid destination URL configured: {}", url);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid gateway configuration".to_string(),
                )
            }
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".to_string(),
            ),
        };

        (status, error_message).into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::ProxyError(error)
    }
}