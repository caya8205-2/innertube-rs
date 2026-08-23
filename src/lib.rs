pub mod proto {
    pub mod misc {
        include!(concat!(env!("OUT_DIR"), "/misc.rs"));
    }
}

pub mod error;
pub mod constants;
pub mod models;
pub mod core;
pub mod utils;
pub mod endpoints;

use std::sync::Arc;
use crate::core::player::Player;
use crate::core::session::{Session, SessionOptions};
use crate::endpoints::browse::{get_channel, get_playlist};
use crate::endpoints::player::{fetch_player_response, resolve_stream_url, select_format};
use crate::endpoints::search::search;
use crate::error::Result;
use crate::models::channel::{ChannelArtistView, YouTubePlaylistView};
use crate::models::format::FormatFilter;
use crate::models::search::SearchResults;
use crate::models::video::PlayerResponse;

/// Main high-level Innertube client.
#[derive(Clone)]
pub struct Innertube {
    pub session: Arc<Session>,
    pub player: Arc<Player>,
}

impl Innertube {
    /// Initialize a new Innertube client with default options.
    pub async fn new() -> Result<Self> {
        Self::with_options(SessionOptions::default()).await
    }

    /// Initialize a new Innertube client with custom session options.
    pub async fn with_options(options: SessionOptions) -> Result<Self> {
        let session = Session::create(options).await?;
        let player = Player::create(&session.http_client, None).await?;

        Ok(Self {
            session: Arc::new(session),
            player: Arc::new(player),
        })
    }

    /// Fetch metadata and streaming formats for a video ID.
    pub async fn get_video_info(&self, video_id: &str) -> Result<PlayerResponse> {
        fetch_player_response(
            &self.session,
            video_id,
            Some(self.player.decipherer.signature_timestamp),
        )
        .await
    }

    /// Retrieve a decrypted, playable streaming URL matching the specified filter.
    pub async fn get_stream_url(&self, video_id: &str, filter: &FormatFilter) -> Result<String> {
        let player_res = self.get_video_info(video_id).await?;
        let format = select_format(&player_res, filter)?;
        resolve_stream_url(format, &self.player.decipherer)
    }

    /// Execute search query for videos, channels, and playlists.
    pub async fn search(&self, query: &str, continuation_token: Option<&str>) -> Result<SearchResults> {
        search(&self.session, query, continuation_token).await
    }

    /// Fetch channel profile, videos, and playlists.
    pub async fn get_channel(&self, channel_id_or_handle: &str) -> Result<ChannelArtistView> {
        get_channel(&self.session, channel_id_or_handle).await
    }

    /// Fetch playlist tracklist.
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<YouTubePlaylistView> {
        get_playlist(&self.session, playlist_id).await
    }
}
