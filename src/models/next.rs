use serde::{Deserialize, Serialize};
use super::video::Thumbnail;

/// A video recommendation or related video item from the `/next` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedVideo {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub author_id: Option<String>,
    pub duration_text: Option<String>,
    pub duration_seconds: Option<u64>,
    pub thumbnails: Vec<Thumbnail>,
    pub view_count_text: Option<String>,
    pub published_time_text: Option<String>,
    pub is_live: bool,
    pub is_upcoming: bool,
}

/// An upcoming autoplay video recommendation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AutoplayVideo {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub thumbnails: Vec<Thumbnail>,
}

/// An item in the playlist panel when watching a video in a playlist.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelItem {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub index: Option<usize>,
    pub is_selected: bool,
    pub thumbnails: Vec<Thumbnail>,
}

/// Consolidated Watch Next results from the `/next` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextResults {
    pub current_video_id: String,
    pub current_title: Option<String>,
    pub current_author: Option<String>,
    pub autoplay: Option<AutoplayVideo>,
    pub related_videos: Vec<RelatedVideo>,
    pub playlist_items: Vec<PlaylistPanelItem>,
    pub continuation_token: Option<String>,
}
