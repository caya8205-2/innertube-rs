use serde::{Deserialize, Serialize};

/// An individual video item in a YouTube playlist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoItem {
    /// Video ID (e.g. `dQw4w9WgXcQ`).
    pub id: String,
    /// Video Title.
    pub title: String,
    /// Channel/Author name.
    pub author: String,
    /// Channel ID (e.g. `UC...`).
    pub author_id: Option<String>,
    /// Duration text (e.g. "3:33").
    pub duration: Option<String>,
    /// Duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// Thumbnail URL.
    pub thumbnail: Option<String>,
    /// Index/position in the playlist (1-indexed).
    pub index: Option<u32>,
    /// Whether the video is playable (not private or deleted).
    pub is_playable: bool,
}

/// Full YouTube playlist details view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistView {
    /// Playlist ID (e.g. `PL...` or `VLPL...`).
    pub id: String,
    /// Playlist Title.
    pub title: String,
    /// Playlist Owner/Author.
    pub author: Option<String>,
    /// Playlist Owner Channel ID.
    pub author_id: Option<String>,
    /// Playlist Description.
    pub description: Option<String>,
    /// Total number of videos in the playlist.
    pub video_count: Option<u32>,
    /// Total view count text (e.g. "1,234,567 views").
    pub view_count: Option<String>,
    /// Last updated date text.
    pub last_updated: Option<String>,
    /// Thumbnail URL.
    pub thumbnail: Option<String>,
    /// Videos in the current page.
    pub videos: Vec<PlaylistVideoItem>,
    /// Pagination token for fetching next page of videos.
    pub continuation_token: Option<String>,
}

/// Continuation response for additional playlist items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistContinuation {
    /// Videos in this page.
    pub videos: Vec<PlaylistVideoItem>,
    /// Continuation token for the next page.
    pub continuation_token: Option<String>,
}
