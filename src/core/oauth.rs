use std::time::{Duration, SystemTime, UNIX_EPOCH};
use regex::Regex;
use serde_json::{json, Value};

use crate::constants::YOUTUBE_BASE_URL;
use crate::error::{InnertubeError, Result};
use crate::models::oauth::{DeviceAndUserCode, OAuth2ClientID, OAuth2Tokens};

pub const DEFAULT_TV_CLIENT_ID: &str = "861556708454-d6dlm3lh05dd871b57egmtvdjc4hg2mv.apps.googleusercontent.com";
pub const DEFAULT_TV_CLIENT_SECRET: &str = "GOCSPX-q_4m-2Vn6eB6sH-j42U7s8Y";

/// OAuth2 Manager for YouTube TV / Device flow.
pub struct OAuth2;

impl OAuth2 {
    /// Extract Google OAuth2 client credentials from YouTube TV application script.
    pub async fn get_client_id(http_client: &reqwest::Client) -> Result<OAuth2ClientID> {
        let resp = http_client.get(format!("{}/tv", YOUTUBE_BASE_URL))
            .header("User-Agent", "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version")
            .header("Referer", "https://www.youtube.com/tv")
            .send().await
            .map_err(InnertubeError::Network)?;

        let html = resp.text().await.map_err(InnertubeError::Network)?;

        // Regex for base-js script on YouTube TV
        let script_re = Regex::new(r#"<script\s+id="base-js"\s+src="([^"]+)"[^>]*></script>"#)
            .map_err(|e| InnertubeError::Other(e.to_string()))?;

        if let Some(caps) = script_re.captures(&html) {
            let script_url = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let full_url = if script_url.starts_with("http") {
                script_url.to_string()
            } else {
                format!("{}{}", YOUTUBE_BASE_URL, script_url)
            };

            let script_resp = http_client.get(&full_url).send().await.map_err(InnertubeError::Network)?;
            let script_content = script_resp.text().await.map_err(InnertubeError::Network)?;

            let client_re = Regex::new(r#"clientId:"(?P<client_id>[^"]+)",[^"]*?:"(?P<client_secret>[^"]+)""#)
                .map_err(|e| InnertubeError::Other(e.to_string()))?;

            if let Some(caps) = client_re.captures(&script_content) {
                let client_id = caps.name("client_id").map(|m| m.as_str()).unwrap_or("").to_string();
                let client_secret = caps.name("client_secret").map(|m| m.as_str()).unwrap_or("").to_string();

                if !client_id.is_empty() && !client_secret.is_empty() {
                    return Ok(OAuth2ClientID {
                        client_id,
                        client_secret,
                    });
                }
            }
        }

        // Fallback to known TV client credentials
        Ok(OAuth2ClientID {
            client_id: DEFAULT_TV_CLIENT_ID.to_string(),
            client_secret: DEFAULT_TV_CLIENT_SECRET.to_string(),
        })
    }

    /// Request a new device code and user verification code from Google OAuth2.
    pub async fn get_device_and_user_code(
        http_client: &reqwest::Client,
        client_id: &str,
    ) -> Result<DeviceAndUserCode> {
        let device_id = generate_uuid_v4();
        let payload = json!({
            "client_id": client_id,
            "scope": "http://gdata.youtube.com https://www.googleapis.com/auth/youtube-paid-content",
            "device_id": device_id,
            "device_model": "ytlr::"
        });

        let resp = http_client.post("https://oauth2.googleapis.com/device/code")
            .json(&payload)
            .send().await
            .map_err(InnertubeError::Network)?;

        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        if let Some(err) = raw.get("error").and_then(Value::as_str) {
            return Err(InnertubeError::Other(format!("OAuth2 device code error: {}", err)));
        }

        let device_code = raw.get("device_code").and_then(Value::as_str)
            .ok_or_else(|| InnertubeError::Other("device_code missing".to_string()))?
            .to_string();

        let user_code = raw.get("user_code").and_then(Value::as_str)
            .ok_or_else(|| InnertubeError::Other("user_code missing".to_string()))?
            .to_string();

        let verification_url = raw.get("verification_url").and_then(Value::as_str)
            .unwrap_or("https://www.google.com/device")
            .to_string();

        let expires_in = raw.get("expires_in").and_then(Value::as_u64).unwrap_or(1800);
        let interval = raw.get("interval").and_then(Value::as_u64).unwrap_or(5);

        Ok(DeviceAndUserCode {
            device_code,
            user_code,
            verification_url,
            expires_in,
            interval,
        })
    }

    /// Poll Google OAuth2 token endpoint until the user approves or the
    /// device code expires, mirroring legacy `pollForAccessToken` semantics:
    /// `authorization_pending`/`slow_down` keep polling at the given
    /// interval; `access_denied`, `expired_token`, and unknown errors are
    /// fatal.
    pub async fn poll_for_access_token(
        http_client: &reqwest::Client,
        client: &OAuth2ClientID,
        device_code: &str,
        interval_secs: u64,
    ) -> Result<OAuth2Tokens> {
        let poll_interval = Duration::from_secs(interval_secs);

        loop {
            tokio::time::sleep(poll_interval).await;

            let payload = json!({
                "client_id": client.client_id,
                "client_secret": client.client_secret,
                "code": device_code,
                "grant_type": "http://oauth.net/grant_type/device/1.0"
            });

            let resp = http_client.post("https://oauth2.googleapis.com/token")
                .json(&payload)
                .send().await
                .map_err(InnertubeError::Network)?;

            let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

            if let Some(err) = raw.get("error").and_then(Value::as_str) {
                if let Some(fatal) = map_poll_error(err) {
                    return Err(fatal);
                }
                continue;
            }

            let access_token = raw.get("access_token").and_then(Value::as_str)
                .ok_or_else(|| InnertubeError::OAuth2("access_token missing in response".to_string()))?
                .to_string();

            let refresh_token = raw.get("refresh_token").and_then(Value::as_str)
                .unwrap_or("")
                .to_string();

            let expires_in = raw.get("expires_in").and_then(Value::as_u64).unwrap_or(3600);
            let token_type = raw.get("token_type").and_then(Value::as_str).map(|s| s.to_string());

            let expiry_date = format_expiry_date(expires_in);

            return Ok(OAuth2Tokens {
                access_token,
                refresh_token,
                expiry_date,
                token_type,
                scope: raw.get("scope").and_then(Value::as_str).map(|s| s.to_string()),
            });
        }
    }

    /// Refresh an existing access token using a refresh token.
    pub async fn refresh_access_token(
        http_client: &reqwest::Client,
        client: &OAuth2ClientID,
        refresh_token: &str,
    ) -> Result<OAuth2Tokens> {
        let payload = json!({
            "client_id": client.client_id,
            "client_secret": client.client_secret,
            "refresh_token": refresh_token,
            "grant_type": "refresh_token"
        });

        let resp = http_client.post("https://oauth2.googleapis.com/token")
            .json(&payload)
            .send().await
            .map_err(InnertubeError::Network)?;

        if !resp.status().is_success() {
            return Err(InnertubeError::OAuth2(format!(
                "Failed to refresh access token: {}",
                resp.status()
            )));
        }

        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        if let Some(err) = raw.get("error").or_else(|| raw.get("error_code")) {
            return Err(InnertubeError::OAuth2(format!(
                "Authorization server returned an error: {err}"
            )));
        }

        let access_token = raw.get("access_token").and_then(Value::as_str)
            .ok_or_else(|| InnertubeError::OAuth2("access_token missing".to_string()))?
            .to_string();

        let expires_in = raw.get("expires_in").and_then(Value::as_u64).unwrap_or(3600);
        let token_type = raw.get("token_type").and_then(Value::as_str).map(|s| s.to_string());
        let expiry_date = format_expiry_date(expires_in);

        Ok(OAuth2Tokens {
            access_token,
            refresh_token: refresh_token.to_string(),
            expiry_date,
            token_type,
            scope: raw.get("scope").and_then(Value::as_str).map(|s| s.to_string()),
        })
    }

    /// Revoke the access token (`POST /o/oauth2/revoke?token=...`).
    pub async fn revoke_credentials(
        http_client: &reqwest::Client,
        tokens: &OAuth2Tokens,
    ) -> Result<()> {
        if tokens.access_token.is_empty() {
            return Err(InnertubeError::OAuth2("Access token not found".to_string()));
        }

        let url = format!(
            "{YOUTUBE_BASE_URL}/o/oauth2/revoke?token={}",
            tokens.access_token
        );
        http_client
            .post(&url)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        Ok(())
    }

    /// Legacy `validateTokens`: access token, refresh token, and expiry must
    /// all be present.
    pub fn validate_tokens(tokens: &OAuth2Tokens) -> bool {
        !tokens.access_token.is_empty()
            && !tokens.refresh_token.is_empty()
            && !tokens.expiry_date.is_empty()
    }

    /// Legacy `shouldRefreshToken`: true when the expiry instant has passed.
    pub fn should_refresh_token(tokens: &OAuth2Tokens) -> bool {
        tokens_expiry_epoch(tokens).is_some_and(|expiry| now_unix_seconds() > expiry)
    }
}

/// Map a device-flow polling error to a fatal error, or `None` to keep
/// polling (legacy switch in `pollForAccessToken`).
pub fn map_poll_error(error: &str) -> Option<InnertubeError> {
    match error {
        "authorization_pending" | "slow_down" => None,
        "access_denied" => Some(InnertubeError::OAuth2("Access was denied.".to_string())),
        "expired_token" => Some(InnertubeError::OAuth2(
            "The device code has expired.".to_string(),
        )),
        _ => Some(InnertubeError::OAuth2(
            "Server returned an unexpected error.".to_string(),
        )),
    }
}

/// Parse the expiry instant of stored tokens.
///
/// ponytail: legacy stores `expiry_date` as an ISO 8601 string; we store
/// unix epoch seconds (no date-formatting dependency). The comparison
/// semantics are identical. Switch to ISO if a date crate lands.
pub fn tokens_expiry_epoch(tokens: &OAuth2Tokens) -> Option<u64> {
    tokens.expiry_date.parse::<u64>().ok()
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn generate_uuid_v4() -> String {
    format!(
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        rand::random::<u32>(),
        rand::random::<u16>(),
        rand::random::<u16>() & 0x0fff,
        (rand::random::<u16>() & 0x3fff) | 0x8000,
        rand::random::<u64>() & 0xffffffffffff
    )
}

fn format_expiry_date(expires_in_secs: u64) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    (now + expires_in_secs).to_string()
}
