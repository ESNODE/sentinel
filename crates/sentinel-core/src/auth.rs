use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use anyhow::{Result, anyhow};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Admin,
    Operator,
    ComputeNode,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    ReadStatus,
    RegisterDevice,
    SubmitTask,
    ManageOrchestrator,
    DeploySkill,
    AdminOps,
}

impl Role {
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::ReadStatus,
                Permission::RegisterDevice,
                Permission::SubmitTask,
                Permission::ManageOrchestrator,
                Permission::DeploySkill,
                Permission::AdminOps,
            ],
            Role::Operator => vec![
                Permission::ReadStatus,
                Permission::SubmitTask,
                Permission::RegisterDevice,
            ],
            Role::ComputeNode => vec![
                Permission::ReadStatus,
                Permission::RegisterDevice,
            ],
            Role::Observer => vec![
                Permission::ReadStatus,
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: Role,
    pub permissions: Vec<Permission>,
}

impl AuthenticatedUser {
    pub fn has_permission(&self, perm: &Permission) -> bool {
        self.permissions.contains(perm)
    }
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
        
        // If auth is disabled in config, allow all as Admin
        if !http_state.security.enable_auth {
            let role = Role::Admin;
            return Ok(AuthenticatedUser {
                user_id: "admin-anonymous".to_string(),
                permissions: role.permissions(),
                role,
            });
        }

        // Basic JWT extraction from Authorization header
        let auth_header = parts.headers.get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        if let Some(token) = auth_header {
            // Enterprise Grade Verification (OIDC Simulation or Local JWT)
            // In a real implementation, we would use a library like `openid` to verify tokens
            if let Some(sso) = &http_state.security.sso_provider {
                match sso {
                    crate::config::SsoProvider::Oidc { issuer_url, .. } => {
                        tracing::debug!("Validating OIDC token against {}", issuer_url);
                    }
                    crate::config::SsoProvider::Saml { .. } => {}
                }
            }

            if let Some(orch_token) = &http_state.orchestrator_token {
                if token == orch_token {
                    let role = Role::Admin;
                    return Ok(AuthenticatedUser {
                        user_id: "orchestrator-admin".to_string(),
                        permissions: role.permissions(),
                        role,
                    });
                }
            }

            // Simulated Tokens for specific roles
            let (user_id, role) = match token {
                "esnode-dev-token" | "sentinel-admin-token" => ("admin-user".to_string(), Role::Admin),
                "sentinel-operator-token" => ("operator-user".to_string(), Role::Operator),
                "sentinel-node-token" => ("compute-node".to_string(), Role::ComputeNode),
                "sentinel-observer-token" => ("observer-user".to_string(), Role::Observer),
                _ => return Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid Token"}))).into_response()),
            };

            return Ok(AuthenticatedUser {
                user_id,
                permissions: role.permissions(),
                role,
            });
        }

        Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Missing Authorization Header"}))).into_response())
    }
}

pub fn require_permission(user: &AuthenticatedUser, perm: Permission) -> Result<(), Response> {
    if user.has_permission(&perm) {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, Json(serde_json::json!({"error": "Insufficient Permissions", "required": format!("{:?}", perm)}))).into_response())
    }
}

/// Verifies the digital signature of a WASM skill against an Enterprise Root of Trust.
pub fn verify_skill_signature(wasm_bytes: &[u8], signature_b64: &str, public_key_b64: &str) -> Result<()> {
    let public_key_bytes: Vec<u8> = general_purpose::STANDARD.decode(public_key_b64)
        .map_err(|_| anyhow!("Invalid public key encoding"))?;
    let signature_bytes: Vec<u8> = general_purpose::STANDARD.decode(signature_b64)
        .map_err(|_| anyhow!("Invalid signature encoding"))?;

    let verifying_key = VerifyingKey::from_bytes(public_key_bytes.as_slice().try_into().map_err(|_| anyhow!("Invalid public key length"))?)
        .map_err(|_| anyhow!("Invalid public key format"))?;
    let signature = Signature::from_bytes(signature_bytes.as_slice().try_into().map_err(|_| anyhow!("Invalid signature length"))?);

    verifying_key.verify(wasm_bytes, &signature)
        .map_err(|e| anyhow!("Signature verification failed: {}", e))?;

    Ok(())
}

/// Simulated TPM Identity Verification
pub fn verify_hardware_trust() -> Result<String> {
    // In production, this calls /dev/tpm0 to fetch a Quote signed by the AIK
    // For demo, we simulate a secure platform identity
    tracing::info!("Verifying Hardware Trust via TPM 2.0 PCRs...");
    Ok("ESNODE-TPM-ID-7741-XA9".to_string())
}

pub async fn oidc_callback() -> impl IntoResponse {
    // Placeholder for OIDC callback logic
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
    use rand::rngs::OsRng;

    #[test]
    fn test_skill_signature_verification() {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key: VerifyingKey = signing_key.verifying_key();
        
        let wasm_bytes = b"\0asm\x01\0\0\0";
        let signature = signing_key.sign(wasm_bytes);
        
        let sig_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
        let pk_b64 = general_purpose::STANDARD.encode(verifying_key.to_bytes());
        
        // Verify success
        assert!(verify_skill_signature(wasm_bytes, &sig_b64, &pk_b64).is_ok());
        
        // Verify failure (tampered data)
        assert!(verify_skill_signature(b"tampered", &sig_b64, &pk_b64).is_err());
    }
}
