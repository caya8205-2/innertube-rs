use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, COOKIE, ORIGIN, REFERER,
    USER_AGENT,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::{
    DEFAULT_CLIENT_VERSION, DEFAULT_INNERTUBE_KEY, DEFAULT_USER_AGENT, INNERTUBE_API_BASE_URL,
    YOUTUBE_BASE_URL,
};
use crate::error::{InnertubeError, Result};
use crate::models::context::{ClientContext, InnerTubeContext, RequestContext, UserContext};
use crate::utils::proto::{decode_visitor_data, encode_visitor_data, generate_random_string};

/// Configuration options for creating an InnerTube `Session`.
#[derive(Debug, Clone, Default)]
pub struct SessionOptions {
    pub lang: Option<String>,
    pub location: Option<String>,
    pub user_agent: Option<String>,
    pub account_index: Option<usize>,
    pub visitor_data: Option<String>,
    pub client_name: Option<String>,
    pub client_version: Option<String>,
    pub device_category: Option<String>,
    pub time_zone: Option<String>,
    pub enable_safety_mode: Option<bool>,
    pub generate_session_locally: Option<bool>,
    pub fail_fast: Option<bool>,
    pub cookie: Option<String>,
    pub po_token: Option<String>,
}

/// InnerTube Session holding context state, API key, and HTTP client.
#[derive(Debug, Clone)]
pub struct Session {
    pub context: InnerTubeContext,
    pub api_key: String,
    pub api_version: String,
    pub account_index: usize,
    pub config_data: Option<String>,
    pub cookie: Option<String>,
    pub po_token: Option<String>,
    pub http_client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SwSessionData {
    api_key: String,
    api_version: String,
    context: InnerTubeContext,
}

impl Session {
    /// Create a new `Session` instance.
    /// Fetches initial config from YouTube's `sw.js_data` or falls back to local generation.
    pub async fn create(options: SessionOptions) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .cookie_store(true)
            .gzip(true)
            .brotli(true)
            .build()
            .map_err(InnertubeError::Network)?;

        let generate_locally = options.generate_session_locally.unwrap_or(false);
        let fail_fast = options.fail_fast.unwrap_or(false);

        let sw_data = if !generate_locally {
            match Self::fetch_sw_session_data(&http_client, &options).await {
                Ok(data) => Some(data),
                Err(err) => {
                    if fail_fast {
                        return Err(err);
                    }
                    None
                }
            }
        } else {
            None
        };

        let (context, api_key, api_version) = if let Some(sw) = sw_data {
            (sw.context, sw.api_key, sw.api_version)
        } else {
            let default_ctx = Self::build_default_context(&options);
            (
                default_ctx,
                DEFAULT_INNERTUBE_KEY.to_string(),
                "v1".to_string(),
            )
        };

        Ok(Self {
            context,
            api_key,
            api_version,
            account_index: options.account_index.unwrap_or(0),
            config_data: None,
            cookie: options.cookie,
            po_token: options.po_token,
            http_client,
        })
    }

    /// Fetch session bootstrap data from `https://www.youtube.com/sw.js_data`.
    async fn fetch_sw_session_data(
        client: &reqwest::Client,
        options: &SessionOptions,
    ) -> Result<SwSessionData> {
        let lang = options.lang.as_deref().unwrap_or("en-US");
        let user_agent = options.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT);
        let time_zone = options.time_zone.as_deref().unwrap_or("UTC");

        let visitor_id = if let Some(ref vd) = options.visitor_data {
            decode_visitor_data(vd)
                .map(|v| v.id)
                .unwrap_or_else(|_| generate_random_string(11))
        } else {
            generate_random_string(11)
        };

        let escaped_tz = time_zone.replace('/', ".");
        let cookie_header = format!("PREF=tz={escaped_tz};VISITOR_INFO1_LIVE={visitor_id};");

        let url = format!("{YOUTUBE_BASE_URL}/sw.js_data");
        let response = client
            .get(&url)
            .header(ACCEPT, "*/*")
            .header(ACCEPT_LANGUAGE, lang)
            .header(USER_AGENT, user_agent)
            .header(REFERER, format!("{YOUTUBE_BASE_URL}/sw.js"))
            .header(COOKIE, cookie_header)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        if !response.status().is_success() {
            return Err(InnertubeError::Api {
                status: response.status().to_string(),
                message: format!("Failed to retrieve sw.js_data: HTTP {}", response.status()),
            });
        }

        let text = response.text().await.map_err(InnertubeError::Network)?;
        let clean_json_str = text.trim_start_matches(")]}'").trim();

        let data: Value = serde_json::from_str(clean_json_str)?;

        // JSPB array: data[0][2] -> [[device_info], api_key]
        let ytcfg = data
            .get(0)
            .and_then(|v| v.get(2))
            .ok_or_else(|| InnertubeError::Other("Invalid sw.js_data structure".into()))?;

        let device_info = ytcfg
            .get(0)
            .and_then(|v| v.get(0))
            .ok_or_else(|| InnertubeError::Other("device_info not found in sw.js_data".into()))?;

        let api_key = ytcfg
            .get(1)
            .and_then(Value::as_str)
            .unwrap_or(DEFAULT_INNERTUBE_KEY)
            .to_string();

        let hl = options.lang.clone().unwrap_or_else(|| {
            device_info
                .get(0)
                .and_then(Value::as_str)
                .unwrap_or("en")
                .to_string()
        });
        let gl = options.location.clone().unwrap_or_else(|| {
            device_info
                .get(1)
                .and_then(Value::as_str)
                .unwrap_or("US")
                .to_string()
        });
        let remote_host = device_info
            .get(3)
            .and_then(Value::as_str)
            .map(|s| s.to_string());
        let visitor_data = options.visitor_data.clone().or_else(|| {
            device_info
                .get(13)
                .and_then(Value::as_str)
                .map(|s| s.to_string())
        });
        let client_version = options.client_version.clone().unwrap_or_else(|| {
            device_info
                .get(16)
                .and_then(Value::as_str)
                .unwrap_or(DEFAULT_CLIENT_VERSION)
                .to_string()
        });
        let os_name = device_info
            .get(17)
            .and_then(Value::as_str)
            .unwrap_or("Windows")
            .to_string();
        let os_version = device_info
            .get(18)
            .and_then(Value::as_str)
            .unwrap_or("10.0")
            .to_string();
        let client_tz = device_info
            .get(79)
            .and_then(Value::as_str)
            .map(|s| s.to_string())
            .or_else(|| options.time_zone.clone());

        let context = InnerTubeContext {
            client: ClientContext {
                hl,
                gl,
                remote_host,
                visitor_data,
                client_name: options
                    .client_name
                    .clone()
                    .unwrap_or_else(|| "WEB".to_string()),
                client_version,
                os_name,
                os_version,
                platform: options
                    .device_category
                    .clone()
                    .unwrap_or_else(|| "DESKTOP".to_string())
                    .to_uppercase(),
                client_form_factor: "UNKNOWN_FORM_FACTOR".to_string(),
                user_agent: user_agent.to_string(),
                android_sdk_version: None,
                device_make: None,
                device_model: None,
                time_zone: client_tz,
                utc_offset_minutes: Some(0),
            },
            user: Some(UserContext {
                enable_safety_mode: options.enable_safety_mode.unwrap_or(false),
                locked_safety_mode: false,
            }),
            request: Some(RequestContext { use_ssl: true }),
        };

        Ok(SwSessionData {
            api_key,
            api_version: "v1".to_string(),
            context,
        })
    }

    /// Build a default `InnerTubeContext` locally without network calls.
    pub fn build_default_context(options: &SessionOptions) -> InnerTubeContext {
        let current_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let visitor_data = options
            .visitor_data
            .clone()
            .unwrap_or_else(|| encode_visitor_data(&generate_random_string(11), current_ts));

        InnerTubeContext {
            client: ClientContext {
                hl: options.lang.clone().unwrap_or_else(|| "en".to_string()),
                gl: options.location.clone().unwrap_or_else(|| "US".to_string()),
                remote_host: None,
                visitor_data: Some(visitor_data),
                client_name: options
                    .client_name
                    .clone()
                    .unwrap_or_else(|| "WEB".to_string()),
                client_version: options
                    .client_version
                    .clone()
                    .unwrap_or_else(|| DEFAULT_CLIENT_VERSION.to_string()),
                os_name: "Windows".to_string(),
                os_version: "10.0".to_string(),
                platform: options
                    .device_category
                    .clone()
                    .unwrap_or_else(|| "DESKTOP".to_string())
                    .to_uppercase(),
                client_form_factor: "UNKNOWN_FORM_FACTOR".to_string(),
                user_agent: options
                    .user_agent
                    .clone()
                    .unwrap_or_else(|| DEFAULT_USER_AGENT.to_string()),
                android_sdk_version: None,
                device_make: None,
                device_model: None,
                time_zone: options
                    .time_zone
                    .clone()
                    .or_else(|| Some("UTC".to_string())),
                utc_offset_minutes: Some(0),
            },
            user: Some(UserContext {
                enable_safety_mode: options.enable_safety_mode.unwrap_or(false),
                locked_safety_mode: false,
            }),
            request: Some(RequestContext { use_ssl: true }),
        }
    }

    /// Get numeric client name ID for `X-Youtube-Client-Name` header.
    pub fn client_name_id(client_name: &str) -> &'static str {
        match client_name.to_uppercase().as_str() {
            "WEB" => "1",
            "MWEB" => "2",
            "ANDROID" => "3",
            "IOS" => "5",
            "TVHTML5" | "TV" => "7",
            "ANDROID_CREATOR" => "14",
            "ANDROID_MUSIC" => "21",
            "ANDROID_VR" => "28",
            "WEB_EMBEDDED_PLAYER" | "WEB_EMBEDDED" => "56",
            "WEB_CREATOR" => "62",
            "WEB_REMIX" | "YTMUSIC" => "67",
            "TVHTML5_SIMPLY" | "TV_SIMPLY" => "74",
            "WEB_KIDS" | "YTKIDS" => "76",
            "TVHTML5_SIMPLY_EMBEDDED_PLAYER" | "TV_EMBEDDED" => "85",
            "VISIONOS" => "101",
            _ => "1",
        }
    }

    /// Build standard header map for InnerTube requests.
    pub fn build_innertube_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("*"));
        headers.insert(ORIGIN, HeaderValue::from_static(YOUTUBE_BASE_URL));

        if let Ok(val) = HeaderValue::from_str(&self.context.client.user_agent) {
            headers.insert(USER_AGENT, val);
        }

        if let Some(ref vd) = self.context.client.visitor_data {
            if let Ok(val) = HeaderValue::from_str(vd) {
                headers.insert("X-Goog-Visitor-Id", val);
            }
        }

        let client_id = Self::client_name_id(&self.context.client.client_name);
        headers.insert("X-Youtube-Client-Name", HeaderValue::from_static(client_id));

        if let Ok(val) = HeaderValue::from_str(&self.context.client.client_version) {
            headers.insert("X-Youtube-Client-Version", val);
        }

        if let Some(ref cookie) = self.cookie {
            if let Ok(val) = HeaderValue::from_str(cookie) {
                headers.insert(COOKIE, val);
            }
        }

        headers
    }

    /// Whether this session has caller-supplied account credentials.
    ///
    /// This only establishes that credentials were configured; YouTube remains
    /// authoritative about whether they are valid for a particular request.
    pub fn is_authenticated(&self) -> bool {
        self.cookie
            .as_deref()
            .is_some_and(|cookie| !cookie.trim().is_empty())
    }

    /// Reject account mutations before they can be sent anonymously.
    pub fn ensure_authenticated(&self) -> Result<()> {
        if self.is_authenticated() {
            Ok(())
        } else {
            Err(InnertubeError::AuthenticationRequired(
                "provide an authenticated YouTube cookie when creating the Session".to_string(),
            ))
        }
    }

    pub(crate) async fn ensure_success(
        endpoint: &str,
        response: reqwest::Response,
    ) -> Result<reqwest::Response> {
        if response.status().is_success() {
            return Ok(response);
        }

        let status = response.status().to_string();
        let body = response.text().await.unwrap_or_default();
        let message: String = body.chars().take(8_192).collect();
        Err(InnertubeError::Api {
            status,
            message: format!("{endpoint}: {message}"),
        })
    }

    /// POST request to an InnerTube endpoint (e.g. `/browse`, `/search`, `/player`).
    pub async fn post_innertube(
        &self,
        endpoint: &str,
        mut payload: Value,
    ) -> Result<reqwest::Response> {
        let clean_endpoint = endpoint.trim_start_matches('/');
        let url = format!(
            "{INNERTUBE_API_BASE_URL}/{clean_endpoint}?prettyPrint=false&alt=json&key={}",
            self.api_key
        );

        if let Some(obj) = payload.as_object_mut() {
            if !obj.contains_key("context") {
                obj.insert("context".to_string(), serde_json::to_value(&self.context)?);
            }
        }

        let headers = self.build_innertube_headers();

        let res = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        Self::ensure_success(clean_endpoint, res).await
    }

    /// POST request to an InnerTube endpoint using a specific client type (e.g. `WEB_REMIX`, `ANDROID`).
    pub async fn post_innertube_client(
        &self,
        client_name: &str,
        endpoint: &str,
        mut payload: Value,
    ) -> Result<reqwest::Response> {
        let clean_endpoint = endpoint.trim_start_matches('/');
        let url = format!(
            "{INNERTUBE_API_BASE_URL}/{clean_endpoint}?prettyPrint=false&alt=json&key={}",
            self.api_key
        );

        let mut custom_context = self.context.clone();
        custom_context.client.client_name = client_name.to_string();
        if client_name == "WEB_REMIX" || client_name == "YTMUSIC" {
            custom_context.client.client_version = "1.20240820.01.00".to_string();
        }

        if let Some(obj) = payload.as_object_mut() {
            if !obj.contains_key("context") {
                obj.insert(
                    "context".to_string(),
                    serde_json::to_value(&custom_context)?,
                );
            }
        }

        let mut headers = self.build_innertube_headers();
        let client_id = Self::client_name_id(client_name);
        headers.insert("X-Youtube-Client-Name", HeaderValue::from_static(client_id));
        if client_name == "WEB_REMIX" || client_name == "YTMUSIC" {
            headers.insert(
                "X-Youtube-Client-Version",
                HeaderValue::from_static("1.20240820.01.00"),
            );
            headers.insert(
                ORIGIN,
                HeaderValue::from_static("https://music.youtube.com"),
            );
            headers.insert(
                REFERER,
                HeaderValue::from_static("https://music.youtube.com/"),
            );
        }

        let res = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        Self::ensure_success(clean_endpoint, res).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session_with_cookie(cookie: Option<&str>) -> Session {
        let options = SessionOptions::default();
        Session {
            context: Session::build_default_context(&options),
            api_key: "test-key".to_string(),
            api_version: "v1".to_string(),
            account_index: 0,
            config_data: None,
            cookie: cookie.map(ToString::to_string),
            po_token: None,
            http_client: reqwest::Client::new(),
        }
    }

    #[test]
    fn anonymous_session_cannot_mutate_account_state() {
        let session = session_with_cookie(None);
        assert!(!session.is_authenticated());
        assert!(matches!(
            session.ensure_authenticated(),
            Err(InnertubeError::AuthenticationRequired(_))
        ));
    }

    #[test]
    fn configured_cookie_allows_authenticated_request_path() {
        let session = session_with_cookie(Some("SID=test"));
        assert!(session.is_authenticated());
        assert!(session.ensure_authenticated().is_ok());
    }
}
