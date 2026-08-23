use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTrack {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub artist_id: String,
    pub album: String,
    pub duration: u32,
    pub thumbnail: String,
    pub youtube_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelPlaylist {
    pub id: String,
    pub name: String,
    pub total_tracks: u32,
    pub image: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelArtistView {
    pub id: String,
    pub name: String,
    pub genres: Vec<String>,
    pub popularity: Option<u32>,
    pub followers: Option<String>,
    pub image: Option<String>,
    pub spotify_url: Option<String>,
    pub top_tracks: Vec<ChannelTrack>,
    pub albums: Vec<Value>,
    pub channel_playlists: Vec<ChannelPlaylist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YouTubePlaylistView {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub tracks: Vec<ChannelTrack>,
}

/// Channel video item from Videos tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideoItem {
    pub video_id: String,
    pub title: String,
    pub published_time: Option<String>,
    pub duration: Option<String>,
    pub views: Option<String>,
    pub thumbnail: Option<String>,
}

/// Channel Short item from Shorts tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelShortItem {
    pub video_id: String,
    pub title: String,
    pub views: Option<String>,
    pub thumbnail: Option<String>,
}

/// Channel About / Profile information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAbout {
    pub channel_id: String,
    pub title: String,
    pub description: Option<String>,
    pub subscriber_count: Option<String>,
    pub video_count: Option<String>,
    pub view_count: Option<String>,
    pub joined_date: Option<String>,
    pub country: Option<String>,
    pub custom_url: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
}

/// Videos tab response with pagination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideosResponse {
    pub channel_id: String,
    pub videos: Vec<ChannelVideoItem>,
    pub continuation_token: Option<String>,
}

/// Shorts tab response with pagination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelShortsResponse {
    pub channel_id: String,
    pub shorts: Vec<ChannelShortItem>,
    pub continuation_token: Option<String>,
}
