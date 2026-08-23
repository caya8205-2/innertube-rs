use regex::Regex;
use std::sync::Arc;
use crate::constants::YOUTUBE_BASE_URL;
use crate::error::{InnertubeError, Result};
use crate::utils::decipher::PlayerDecipherer;

/// YouTube Player manager handling player script downloading and decipher caching.
#[derive(Clone)]
pub struct Player {
    pub player_id: String,
    pub decipherer: Arc<PlayerDecipherer>,
}

impl Player {
    /// Create and initialize player by fetching the latest `base.js` from YouTube.
    pub async fn create(http_client: &reqwest::Client, player_id_override: Option<&str>) -> Result<Self> {
        let player_id = if let Some(id) = player_id_override {
            id.to_string()
        } else {
            Self::fetch_player_id(http_client).await?
        };

        let player_js = Self::fetch_player_js(http_client, &player_id).await?;
        let decipherer = Arc::new(PlayerDecipherer::new(&player_js)?);

        Ok(Self {
            player_id,
            decipherer,
        })
    }

    /// Extract current player ID from `https://www.youtube.com/iframe_api`.
    async fn fetch_player_id(client: &reqwest::Client) -> Result<String> {
        let url = format!("{YOUTUBE_BASE_URL}/iframe_api");
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        let text = resp.text().await.map_err(InnertubeError::Network)?;

        let re = Regex::new(r#"player\\/([a-zA-Z0-9_-]+)\\"#)
            .or_else(|_| Regex::new(r#"player/([a-zA-Z0-9_-]+)/"#))
            .map_err(|e| InnertubeError::Player(e.to_string()))?;

        if let Some(caps) = re.captures(&text) {
            if let Some(m) = caps.get(1) {
                return Ok(m.as_str().to_string());
            }
        }

        // Fallback search in sw.js or standard player regex
        let fallback_re = Regex::new(r#"/s/player/([a-zA-Z0-9_-]+)/"#)
            .map_err(|e| InnertubeError::Player(e.to_string()))?;

        if let Some(caps) = fallback_re.captures(&text) {
            if let Some(m) = caps.get(1) {
                return Ok(m.as_str().to_string());
            }
        }

        Err(InnertubeError::Player(
            "Failed to extract player ID from iframe_api".into(),
        ))
    }

    /// Fetch `base.js` for the given `player_id`.
    async fn fetch_player_js(client: &reqwest::Client, player_id: &str) -> Result<String> {
        let url = format!("{YOUTUBE_BASE_URL}/s/player/{player_id}/player_es6.vflset/en_US/base.js");
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(InnertubeError::Network)?;

        if !resp.status().is_success() {
            return Err(InnertubeError::Player(format!(
                "Failed to download player JS from {url}: HTTP {}",
                resp.status()
            )));
        }

        resp.text().await.map_err(InnertubeError::Network)
    }
}
