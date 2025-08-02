use http::HeaderMap;

use crate::errors::AppError;

pub fn verify_token(headers: &HeaderMap) -> Result<(), AppError> {
    const SECRET_API_KEY: &str = "super-secret-key";
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::MissingAuthToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::InvalidAuthHeader)?;

    if token == SECRET_API_KEY {
        Ok(())
    }
    else{
        Err(AppError::AuthFailed)
    }



}