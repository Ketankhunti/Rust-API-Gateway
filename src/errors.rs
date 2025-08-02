use axum::{http::StatusCode, response::{IntoResponse, Response}};


#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    RouteNotFound,
    ProxyError(reqwest::Error),
    InvalidDestination(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        let (status, error_message) = match self {
            AppError::RouteNotFound => (StatusCode::NOT_FOUND, "Route not found"),
            AppError::ProxyError(e) => {
                tracing::error!("Proxy error: {}", e);
                (StatusCode::BAD_GATEWAY, "Error proxying request")
            },
            AppError::InvalidDestination(url) => {
                tracing::error!("Invalid destination URL configured: {}", url);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid gateway configuration",
                )
            },
            AppError::InternalServerError => { 
                
                tracing::error!("Internal server error");
                (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred",
            )
        },
        };
        

        (status, error_message).into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::ProxyError(error)
    }
}