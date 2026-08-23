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
