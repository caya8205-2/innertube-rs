use serde::{Deserialize, Serialize};

/// Search filter types specific to YouTube Music (`WEB_REMIX`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MusicSearchFilter {
    Songs,
    Videos,
    Albums,
    Artists,
    Playlists,
    FeaturedPlaylists,
    CommunityPlaylists,
}

impl MusicSearchFilter {
    /// Return the Protobuf search parameter string used by YouTube Music.
    pub fn to_param_str(self) -> &'static str {
        match self {
            Self::Songs => "EgWKAQIIAWoQEAMQBBAJEA4QChAFEBEQEBA%3D",
            Self::Videos => "EgWKAQIQAWoQEAMQBBAJEA4QChAFEBEQEBA%3D",
            Self::Albums => "EgWKAQIYAWoQEAMQBBAJEA4QChAFEBEQEBA%3D",
            Self::Artists => "EgWKAQIgAWoQEAMQBBAJEA4QChAFEBEQEBA%3D",
            Self::Playlists => "EgWKAQIwAWoQEAMQBBAJEA4QChAFEBEQEBA%3D",
            Self::FeaturedPlaylists => "EgeKAQQoADgBagwQDhAKEAMQBBAJEAU%3D",
            Self::CommunityPlaylists => "EgeKAQQoAEABagwQDhAKEAMQBBAJEAU%3D",
        }
    }
}

/// Artist reference containing name and optional browse ID.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicArtistRef {
    pub name: String,
    pub browse_id: Option<String>,
}

/// Album reference containing title and optional browse ID.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicAlbumRef {
    pub title: String,
    pub browse_id: Option<String>,
}

/// A track item in YouTube Music (song or music video).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicTrackItem {
    pub video_id: String,
    pub title: String,
    pub artists: Vec<MusicArtistRef>,
    pub album: Option<MusicAlbumRef>,
    pub duration: Option<String>,
    pub duration_ms: Option<u64>,
    pub thumbnail: Option<String>,
    pub is_explicit: bool,
}

/// An album card item in YouTube Music search / explore.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicAlbumItem {
    pub browse_id: String,
    pub title: String,
    pub artist: Option<String>,
    pub year: Option<String>,
    pub thumbnail: Option<String>,
    pub track_count: Option<u32>,
}

/// An artist card item in YouTube Music search / explore.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicArtistItem {
    pub browse_id: String,
    pub name: String,
    pub subscribers: Option<String>,
    pub thumbnail: Option<String>,
}

/// A playlist item in YouTube Music.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlaylistItem {
    pub browse_id: String,
    pub title: String,
    pub author: Option<String>,
    pub track_count: Option<u32>,
    pub thumbnail: Option<String>,
}

/// Consolidated YouTube Music search results.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicSearchResults {
    pub query: String,
    pub filter: Option<MusicSearchFilter>,
    pub songs: Vec<MusicTrackItem>,
    pub videos: Vec<MusicTrackItem>,
    pub albums: Vec<MusicAlbumItem>,
    pub artists: Vec<MusicArtistItem>,
    pub playlists: Vec<MusicPlaylistItem>,
    pub continuation_token: Option<String>,
}

/// Track lyrics extracted from YouTube Music.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicLyrics {
    pub lyrics_text: String,
    pub footer: Option<String>,
    pub title: Option<String>,
    pub is_synced: bool,
}

/// Full details of an album including its tracklist.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicAlbumView {
    pub browse_id: String,
    pub title: String,
    pub artist: Option<String>,
    pub year: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub tracks: Vec<MusicTrackItem>,
}

/// YouTube Music explore and trending page data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MusicExplore {
    pub top_songs: Vec<MusicTrackItem>,
    pub top_videos: Vec<MusicTrackItem>,
    pub top_artists: Vec<MusicArtistItem>,
    pub new_releases: Vec<MusicAlbumItem>,
    pub moods_and_genres: Vec<String>,
}
