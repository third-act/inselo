pub mod credential_provider;
pub mod error;
pub mod models;

use std::sync::RwLock;

use chrono::{Duration, Utc};
use credential_provider::{CredentialProvider, Credentials, Token};
use error::{Error, Result};
use models::{
    auth::{AuthTokenRequest, AuthTokenResponse},
    orders::{CreateOrderRequest, CreateOrderResponse},
};
use reqwest::header::CONTENT_TYPE;

pub struct InseloClient<P: CredentialProvider> {
    base_url: String,
    client: reqwest::Client,
    credential_provider: P,
    token: RwLock<Option<Token>>,
}

// TODO: Builder to customize options/construct your own underlying client

impl<P: CredentialProvider> InseloClient<P> {
    pub fn new(base_url: String, credential_provider: P) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
            credential_provider,
            token: RwLock::new(None),
        }
    }

    pub async fn post_order(
        &self,
        payload: &CreateOrderRequest,
    ) -> Result<Option<CreateOrderResponse>> {
        let token = self.valid_auth_token().await?;
        let response = self
            .client
            .post(format!("{}/api/orders", &self.base_url))
            .bearer_auth(token.expose())
            .json(&payload)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(None),
            reqwest::StatusCode::CREATED => {
                let order_response = response.json().await?;
                Ok(Some(order_response))
            }
            _ => Err(Error::UnexpectedResponseCode(response.status())),
        }
    }

    async fn valid_auth_token_with_buffer(&self, buffer: Duration) -> Result<Token> {
        if let Some(token) = &*self.token.read().unwrap() {
            if token.expires_at - Utc::now() > buffer {
                return Ok(token.clone());
            }
        }

        let credentials = self.credential_provider.try_fetch_credentials()?;
        let token = self.get_auth_token(&credentials).await?;

        *self.token.write().unwrap() = Some(token.clone());

        Ok(token)
    }

    async fn valid_auth_token(&self) -> Result<Token> {
        self.valid_auth_token_with_buffer(Duration::seconds(30))
            .await
    }

    async fn get_auth_token(&self, credentials: &Credentials) -> Result<Token> {
        let response = self
            .client
            .post(format!("{}/auth0-login", &self.base_url))
            .json::<AuthTokenRequest>(&credentials.into())
            .send()
            .await?;

        let body = response.json::<AuthTokenResponse>().await?;

        Ok(body.into())
    }
}
