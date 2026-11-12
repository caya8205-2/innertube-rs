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
    /// Watch progress reported by YouTube as a percentage (`0`–`100`).
    #[serde(default)]
    pub progress_percent: Option<u32>,
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

/// A chapter / key moment reported by YouTube for the current video.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VideoChapter {
    pub title: String,
    /// Chapter start time in milliseconds.
    pub start_ms: u64,
    /// Best available chapter thumbnail URL.
    pub thumbnail: Option<String>,
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
    /// Chapters / key moments of the current video, sorted by start time.
    #[serde(default)]
    pub chapters: Vec<VideoChapter>,
    pub continuation_token: Option<String>,
    /// Continuation token of the searchable-transcript engagement panel
    /// (legacy `MediaInfo.getTranscript` path).
    pub transcript_continuation_token: Option<String>,
}
