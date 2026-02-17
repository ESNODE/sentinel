// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    crate::http::HttpState: axum::extract::FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let http_state = crate::http::HttpState::from_ref(state);
        
        // If auth is disabled in config, allow all
        if !http_state.security.enable_auth {
            return Ok(AuthenticatedUser {
                user_id: "admin-anonymous".to_string(),
                role: "admin".to_string(),
            });
        }

        // Basic JWT extraction from Authorization header
        let auth_header = parts.headers.get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        if let Some(_token) = auth_header {
            // In a real implementation, we would verify the JWT here using jsonwebtoken crate
            // For now, we mock a successful verification if token is "esnode-dev-token"
            if _token == "esnode-dev-token" {
                return Ok(AuthenticatedUser {
                    user_id: "dev-user".to_string(),
                    role: "admin".to_string(),
                });
            }
        }

        Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response())
    }
}

pub async fn oidc_callback() -> impl IntoResponse {
    // Placeholder for OIDC callback logic
    StatusCode::NOT_IMPLEMENTED
}
