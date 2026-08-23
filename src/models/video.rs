use serde::{Deserialize, Serialize};
use crate::models::format::StreamingFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub length_seconds: String,
    pub channel_id: String,
    pub is_owner_viewing: Option<bool>,
    pub short_description: Option<String>,
    pub is_crawlable: Option<bool>,
    pub thumbnail: Option<ThumbnailContainer>,
    pub allow_ratings: Option<bool>,
    pub view_count: Option<String>,
    pub author: String,
    pub is_private: Option<bool>,
    pub is_unplugged_corpus: Option<bool>,
    pub is_live_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailContainer {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    pub expires_in_seconds: Option<String>,
    #[serde(default)]
    pub formats: Vec<StreamingFormat>,
    #[serde(default)]
    pub adaptive_formats: Vec<StreamingFormat>,
    pub dash_manifest_url: Option<String>,
    pub hls_manifest_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: String,
    pub reason: Option<String>,
    pub playable_in_embed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playability_status: PlayabilityStatus,
    pub video_details: Option<VideoDetails>,
    pub streaming_data: Option<StreamingData>,
}
