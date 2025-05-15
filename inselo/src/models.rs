use chrono::{DateTime, Duration, Utc};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

pub mod orders;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub client_id: ClientId,
    pub client_secret: SecretString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientId(String);

#[derive(Debug, Clone)]
pub struct Token {
    pub token: SecretString,
    pub expires_at: DateTime<Utc>,
}

impl Token {
    pub fn expose(&self) -> &str {
        self.token.expose_secret()
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokenResponse {
    pub auth_token: String,
    pub scope: String,
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
