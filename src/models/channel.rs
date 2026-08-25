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

impl ChannelAbout {
    /// Fetch the channel's recent uploads (Videos tab).
    pub async fn get_videos(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<ChannelVideosResponse> {
        crate::endpoints::channel::get_channel_videos(session, &self.channel_id, None).await
    }

    /// Fetch the channel's Shorts (Shorts tab).
    pub async fn get_shorts(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<ChannelShortsResponse> {
        crate::endpoints::channel::get_channel_shorts(session, &self.channel_id, None).await
    }

    /// Fetch the channel's Community posts.
    pub async fn get_community(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<crate::models::post::CommunityPostsResponse> {
        crate::endpoints::channel::get_channel_community(session, &self.channel_id, None).await
    }
}

/// Videos tab response with pagination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideosResponse {
    pub channel_id: String,
    pub videos: Vec<ChannelVideoItem>,
    pub continuation_token: Option<String>,
}

impl ChannelVideosResponse {
    /// Check if there are more videos to load.
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch the next page of videos for this channel.
    pub async fn get_continuation(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<ChannelVideosResponse> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            crate::error::InnertubeError::Other(
                "No continuation token available for channel videos".to_string(),
            )
        })?;
        crate::endpoints::channel::get_channel_videos(session, &self.channel_id, Some(token)).await
    }
}

/// Shorts tab response with pagination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelShortsResponse {
    pub channel_id: String,
    pub shorts: Vec<ChannelShortItem>,
    pub continuation_token: Option<String>,
}

impl ChannelShortsResponse {
    /// Check if there are more shorts to load.
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch the next page of shorts for this channel.
    pub async fn get_continuation(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<ChannelShortsResponse> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            crate::error::InnertubeError::Other(
                "No continuation token available for channel shorts".to_string(),
            )
        })?;
        crate::endpoints::channel::get_channel_shorts(session, &self.channel_id, Some(token)).await
    }
}
