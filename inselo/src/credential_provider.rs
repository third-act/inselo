use chrono::{DateTime, Utc};
use secrecy::ExposeSecret;
use secrecy::SecretString;

use crate::{error::Result, models::ClientId};

/// Implement this to source your credentials when they're needed. By default the token has a TTL
/// of 1 hour.
pub trait CredentialProvider {
    fn try_fetch_credentials(&self) -> Result<Credentials>;
}

impl<F> CredentialProvider for F
where
    F: Fn() -> Result<Credentials> + Send + Sync + 'static,
{
    fn try_fetch_credentials(&self) -> Result<Credentials> {
        self()
    }
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub client_id: ClientId,
    pub client_secret: SecretString,
}

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
