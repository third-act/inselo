use chrono::{Duration, Utc};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::credential_provider::{Credentials, Token};

use super::ClientId;

// NOTE: This should *not* be camel case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenRequest {
    pub client_id: ClientId,
    pub client_secret: String,
}

impl From<&Credentials> for AuthTokenRequest {
    fn from(value: &Credentials) -> Self {
        Self {
            client_id: value.client_id.clone(),
            client_secret: value.client_secret.expose_secret().to_string(),
        }
    }
}

// NOTE: Is *not* camelCase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenResponse {
    #[serde(rename = "access_token")]
    pub auth_token: String,
    pub scope: String,
    #[serde(rename = "expires_in")]
    pub expires_in_seconds: u32,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    Bearer,
}

impl From<AuthTokenResponse> for Token {
    fn from(value: AuthTokenResponse) -> Self {
        Self {
            token: SecretString::from(value.auth_token),
            expires_at: Utc::now() + Duration::seconds(value.expires_in_seconds as i64),
        }
    }
}
