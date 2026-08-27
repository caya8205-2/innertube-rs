use serde::{Deserialize, Serialize};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::parser::nodes::video::VideoNode;
use crate::parser::nodes::{ChannelNode, PlaylistNode};

/// Generic parsed feed matching YouTube.js `Feed<T>` mixin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Feed<T = serde_json::Value> {
    pub items: Vec<T>,
    pub videos: Vec<VideoNode>,
    pub channels: Vec<ChannelNode>,
    pub playlists: Vec<PlaylistNode>,
    pub continuation_token: Option<String>,
}

impl<T> Feed<T> {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }
}

/// Generic browse feed used by legacy account and discovery destinations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowseFeed {
    pub browse_id: String,
    pub videos: Vec<VideoNode>,
    pub channels: Vec<ChannelNode>,
    pub playlists: Vec<PlaylistNode>,
    pub continuation_token: Option<String>,
}

impl BrowseFeed {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch next batch of browse feed contents.
    pub async fn get_continuation(&self, session: &Session) -> Result<BrowseFeed> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            InnertubeError::NotFound("There are no continuations available for this BrowseFeed".into())
        })?;
        crate::endpoints::feed::get_browse_continuation(session, token).await
    }
}

/// A category filter chip in Home Feed (`ChipCloudChip.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterChip {
    pub text: String,
    pub params: Option<String>,
    pub is_selected: bool,
}

/// YouTube Home Feed (`HomeFeed.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HomeFeed {
    pub filter_chips: Vec<FilterChip>,
    pub videos: Vec<VideoNode>,
    pub continuation_token: Option<String>,
}

impl HomeFeed {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch next batch of home feed videos.
    pub async fn get_continuation(&self, session: &Session) -> Result<HomeFeed> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            InnertubeError::NotFound("There are no continuations available for this HomeFeed".into())
        })?;
        crate::endpoints::feed::get_home_feed_continuation(session, token).await
    }
}

/// A tab in YouTube Trending page (Now, Music, Gaming, Movies).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrendingTab {
    pub title: String,
    pub params: Option<String>,
    pub is_selected: bool,
}

/// YouTube Trending page (`Explore.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrendingFeed {
    pub current_tab: String,
    pub tabs: Vec<TrendingTab>,
    pub videos: Vec<VideoNode>,
}

/// YouTube Hashtag page (`HashtagFeed.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HashtagFeed {
    pub hashtag: String,
    pub header_title: Option<String>,
    pub video_count_text: Option<String>,
    pub channel_count_text: Option<String>,
    pub videos: Vec<VideoNode>,
    pub continuation_token: Option<String>,
}

impl HashtagFeed {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch next batch of hashtag feed videos.
    pub async fn get_continuation(&self, session: &Session) -> Result<HashtagFeed> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            InnertubeError::NotFound("There are no continuations available for this HashtagFeed".into())
        })?;
        let payload = serde_json::json!({
            "continuation": token,
        });
        let resp = session.post_innertube("/browse", payload).await?;
        let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;
        crate::endpoints::feed::parse_hashtag_response(&self.hashtag, &raw)
    }
}
