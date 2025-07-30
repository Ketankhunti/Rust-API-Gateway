use axum::{http::StatusCode, response::{IntoResponse, Response}};


#[derive(Debug)]
pub enum AppError {
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        let (status, error_message) = match self {
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred")
            }
        };
        (status, error_message).into_response()
    }
}