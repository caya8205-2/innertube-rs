use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_LANGUAGE, CONTENT_TYPE, ORIGIN, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::constants::{DEFAULT_USER_AGENT, INNERTUBE_API_BASE_URL, YOUTUBE_BASE_URL};
use crate::error::{InnertubeError, Result};

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            USER_AGENT,
            HeaderValue::from_static(DEFAULT_USER_AGENT),
        );
        default_headers.insert(
            ORIGIN,
            HeaderValue::from_static(YOUTUBE_BASE_URL),
        );
        default_headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US,en;q=0.9"),
        );
        default_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .cookie_store(true)
            .build()
            .map_err(InnertubeError::Network)?;

        Ok(Self { client })
    }

    pub async fn post_innertube<B, R>(
        &self,
        endpoint: &str,
        body: &B,
        client_name_header: Option<&str>,
        client_version_header: Option<&str>,
        visitor_data: Option<&str>,
    ) -> Result<R>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{}", INNERTUBE_API_BASE_URL, endpoint.trim_start_matches('/'));

        let mut req = self.client.post(&url);

        if let Some(name) = client_name_header {
            req = req.header("X-Youtube-Client-Name", name);
        }
        if let Some(ver) = client_version_header {
            req = req.header("X-Youtube-Client-Version", ver);
        }
        if let Some(v_data) = visitor_data {
            req = req.header("X-Goog-Visitor-Id", v_data);
        }

        let resp = req
            .json(body)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        if !resp.status().is_success() {
            let status = resp.status().to_string();
            let text = resp.text().await.unwrap_or_default();
            return Err(InnertubeError::Api {
                status,
                message: text,
            });
        }

        resp.json::<R>().await.map_err(InnertubeError::Network)
    }

    pub async fn get_text(&self, url: &str) -> Result<String> {
        let resp = self.client
            .get(url)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        resp.text().await.map_err(InnertubeError::Network)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HttpClient")
    }
}
