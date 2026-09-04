use serde::{Deserialize, Serialize};

/// OAuth2 Client credentials extracted from YouTube TV base.js.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2ClientID {
    pub client_id: String,
    pub client_secret: String,
}

/// OAuth2 Tokens and credentials (`OAuth2Tokens.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Tokens {
    pub access_token: String,
    pub refresh_token: String,
    /// Unix epoch seconds (legacy stores an ISO 8601 string; see
    /// `core::oauth::tokens_expiry_epoch`).
    pub expiry_date: String,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}

/// Device and user verification code response (`DeviceAndUserCode.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAndUserCode {
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u64,
    pub interval: u64,
}
