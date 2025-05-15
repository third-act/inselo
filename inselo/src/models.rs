use std::fmt;

use chrono::{DateTime, Duration, Utc};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

pub mod orders;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientId(String);

impl From<String> for ClientId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsOwnerId(u32);

impl From<u32> for GoodsOwnerId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderNumber(String);

impl From<String> for OrderNumber {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceNumber(String);

impl From<String> for ReferenceNumber {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerNumber(String);

impl From<String> for CustomerNumber {
    fn from(value: String) -> Self {
        Self(value)
    }
}

// TODO: Must be > 0, implement validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCount(u32);

impl From<u32> for ItemCount {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderType {
    code: OrderTypeInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderTypeInner {
    Business,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub client_id: ClientId,
    pub client_secret: SecretString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CountryCode {
    SE,
    NO,
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CountryCode::SE => write!(f, "SE"),
            CountryCode::NO => write!(f, "NO"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consignee {
    pub customer_number: CustomerNumber,
    pub name: String,
    pub address1: String,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub post_code: String,
    pub city: String,
    pub country_code: CountryCode,
    pub remark: Option<String>,
    pub door_code: Option<String>,
    pub advanced: AdvancedCosigneeOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedCosigneeOptions {
    /// This object holds information about SMS notifications for deliveries. If the consignee
    /// should be notified by SMS, you should include that information here.
    sms_notification: Option<SmsNotificationOptions>,
    /// This object holds information about telephone notifications for deliveries. If the
    /// consignee should be notified by telephone, you should include that information here.
    telephone_notification: Option<TelephoneNotification>,
    /// This object holds information about email notifications for deliveries. If the consignee
    /// should be notified by email, you should include that information here.
    email_notification: Option<EmailNotification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsNotificationOptions {
    pub to_be_notified: bool,
    #[serde(rename = "value")]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneNotification {
    pub to_be_notified: bool,
    #[serde(rename = "value")]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotification {
    pub to_be_notified: bool,
    #[serde(rename = "value")]
    pub email_address: String,
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
