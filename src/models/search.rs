use serde::{Deserialize, Serialize};
use crate::models::video::Thumbnail;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchVideoItem {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub channel_id: String,
    pub duration: Option<String>,
    pub view_count: Option<String>,
    pub published_time: Option<String>,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchChannelItem {
    pub channel_id: String,
    pub title: String,
    pub subscriber_count: Option<String>,
    pub video_count: Option<String>,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPlaylistItem {
    pub playlist_id: String,
    pub title: String,
    pub author: String,
    pub video_count: Option<String>,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SearchResultItem {
    Video(SearchVideoItem),
    Channel(SearchChannelItem),
    Playlist(SearchPlaylistItem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchPrioritize {
    Relevance,
    Popularity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadDateFilter {
    All,
    Today,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchTypeFilter {
    All,
    Video,
    Shorts,
    Channel,
    Playlist,
    Movie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationFilter {
    All,
    OverTwentyMins,
    UnderThreeMins,
    ThreeToTwentyMins,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFilter {
    Hd,
    Subtitles,
    CreativeCommons,
    Feature3d,
    Live,
    Purchased,
    Feature4k,
    Feature360,
    Location,
    Hdr,
    Vr180,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchFilters {
    pub prioritize: Option<SearchPrioritize>,
    pub upload_date: Option<UploadDateFilter>,
    pub search_type: Option<SearchTypeFilter>,
    pub duration: Option<DurationFilter>,
    #[serde(default)]
    pub features: Vec<FeatureFilter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub query: String,
    pub items: Vec<SearchResultItem>,
    pub continuation_token: Option<String>,
}
