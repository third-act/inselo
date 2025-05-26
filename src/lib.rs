pub mod credential_provider;
pub mod error;
pub mod models;

use std::sync::RwLock;

use chrono::{Duration, Utc};
use credential_provider::{CredentialProvider, Credentials, Token};
use error::{Error, Result, ValidationError};
use models::{
    auth::{AuthTokenRequest, AuthTokenResponse},
    orders::{CreateOrderRequest, CreateOrderResponse},
};

pub struct InseloClient {
    base_url: String,
    client: reqwest::Client,
    credential_provider: Box<dyn CredentialProvider>,
    token: RwLock<Option<Token>>,
}

// TODO: Builder to customize options/construct your own underlying client

impl InseloClient {
    pub fn new<P: CredentialProvider + 'static>(base_url: String, credential_provider: P) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
            credential_provider: Box::new(credential_provider),
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
            .send()
            .await?;

        let status = response.status();

        match status {
            reqwest::StatusCode::OK => Ok(None),
            reqwest::StatusCode::CREATED => {
                let text = response.text().await.map_err(|e| Error::ParseError {
                    message: format!("Failed to read response text: {}", e),
                    status_code: status,
                    body: None,
                })?;

                serde_json::from_str::<CreateOrderResponse>(&text)
                    .map_err(|err| Error::ParseError {
                        message: err.to_string(),
                        status_code: status,
                        body: Some(text),
                    })
                    .map(Some)
            }
            reqwest::StatusCode::BAD_REQUEST | reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                let text = response.text().await.map_err(|e| Error::ParseError {
                    message: format!("Failed to read error response: {}", e),
                    status_code: status,
                    body: None,
                })?;

                if let Ok(validation_error) = serde_json::from_str::<ValidationError>(&text) {
                    Err(Error::Validation(validation_error))
                } else {
                    Err(Error::RequestError(text))
                }
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(Error::Unauthenticated),
            reqwest::StatusCode::FORBIDDEN => Err(Error::Forbidden),
            reqwest::StatusCode::NOT_FOUND => Err(Error::NotFound),
            status if status.is_server_error() => Err(Error::ServerError {
                status_code: status.as_u16(),
            }),
            _ => Err(Error::UnexpectedResponseCode(status)),
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
            .post(format!("{}/api/auth0-login", &self.base_url))
            .json::<AuthTokenRequest>(&credentials.into())
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let body_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response body".to_string());
            return Err(Error::ParseError {
                message: format!("Auth token request failed with status {}", status),
                status_code: status,
                body: Some(body_text),
            });
        }

        let text = response.text().await.map_err(|e| Error::ParseError {
            message: format!("Failed to read auth token response text: {}", e),
            status_code: status,
            body: None,
        })?;

        let parsed_response =
            serde_json::from_str::<AuthTokenResponse>(&text).map_err(|err| Error::ParseError {
                message: format!("Failed to parse AuthTokenResponse: {}", err),
                status_code: status,
                body: Some(text),
            })?;

        Ok(parsed_response.into())
    }
}
