use std::fmt;

use serde::{Deserialize, Serialize};

use super::CustomerNumber;

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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct SmsNotificationOptions {
    pub to_be_notified: bool,
    #[serde(rename = "value")]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelephoneNotification {
    pub to_be_notified: bool,
    #[serde(rename = "value")]
    pub phone_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailNotification {
    pub to_be_notified: bool,
    #[serde(rename = "value")]
    pub email_address: String,
}
