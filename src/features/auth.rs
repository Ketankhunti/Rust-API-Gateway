use std::collections::HashSet;

use http::HeaderMap;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{config::{AuthConfig, AuthType}, errors::AppError};

static JWT_SECRET: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret("a-string-secret-at-least-256-bits-long".as_ref())
});

const API_KEY_ADMIN: &str = "admin-secret-key";
const API_KEY_USER: &str = "user-secret-key";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,  // Subject (Uer Id)
    pub roles: Vec<String>,
    pub exp: usize,  // Required for JWT validation
}

pub fn verify_token(headers: &HeaderMap, auth_config: &AuthConfig) -> Result<Claims, AppError> {
    
    let token = extract_bearer_token(headers)?;

    match auth_config.auth_type {
        AuthType::Jwt => verify_jwt(token),
        AuthType::ApiKey => verify_api_key(token),
    }

}

fn extract_bearer_token(headers: &HeaderMap) -> Result<&str, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::MissingAuthToken)?;

    auth_header.strip_prefix("Bearer ")
        .ok_or(AppError::InvalidAuthHeader)
}

pub fn check_roles(user_roles: &[String], required_roles: &[String]) -> Result<(), AppError> {
    let user_roles_set : HashSet<_> = user_roles.iter().collect();
    for required_role in required_roles {
        if !user_roles_set.contains(required_role) {
            return Err(AppError::InsufficientPermissions);
        }
    }
    Ok(())
}

// ------- Private Helper Functions  -----

fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    info!(token);
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let token_data = decode::<Claims>(token, &JWT_SECRET, &validation)
        .map_err(|err| 
            match err.kind()  {
                ErrorKind::ExpiredSignature => AppError::TokenExpired,
                _ => AppError::AuthFailed("Invalid JWT".to_string()),
            }
        )?;
    info!(token_data.claims.sub);
    info!("{:#?}", token_data.claims.roles);
    info!(token_data.claims.exp);
    Ok(token_data.claims)
}

fn verify_api_key(token: &str) -> Result<Claims,AppError>  {
    match token  {
        API_KEY_ADMIN => Ok(
            Claims { sub: "admin_user".to_string(), 
            roles: vec!["admin".to_string(), "user".to_string()] , 
            exp: 0, // Not used for API keys 
            }
        ),
        API_KEY_USER => Ok(Claims {
            sub: "normal_user".to_string(),
            roles: vec!["user".to_string()],
            exp: 0,
        }),
        _ => Err(AppError::AuthFailed("Invalid API Key".to_string()))
    }
}