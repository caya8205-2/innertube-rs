use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use serde_json::{json, Value};

/// YouTube Kids Manager (1:1 with Kids.ts).
pub struct KidsManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> KidsManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Fetch YouTube Kids Home Feed (`kidsHomeScreenRenderer`).
    pub async fn get_home(&self) -> Result<Value> {
        let resp = self
            .session
            .post_innertube_client("WEB_KIDS", "/browse", json!({ "browseId": "FEkids_home" }))
            .await?;
        resp.json().await.map_err(InnertubeError::Network)
    }

    /// Fetch YouTube Kids Home Feed (alias matching Kids.ts getHomeFeed).
    pub async fn get_home_feed(&self) -> Result<Value> {
        self.get_home().await
    }

    /// Search YouTube Kids.
    pub async fn search(&self, query: &str) -> Result<Value> {
        let resp = self
            .session
            .post_innertube_client("WEB_KIDS", "/search", json!({ "query": query }))
            .await?;
        resp.json().await.map_err(InnertubeError::Network)
    }
}
