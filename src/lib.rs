//! # innertube-rs
//!
//! A fast, lightweight, and asynchronous pure Rust client for YouTube's internal API (InnerTube).
//!
//! `innertube-rs` provides direct access to YouTube metadata extraction, audio/video streaming URL
//! resolution (with automated signature & n-token deciphering via embedded QuickJS), search queries,
//! and channel/playlist scraping.
//!
//! ## Quick Start
//!
//! Add `innertube-rs` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! innertube-rs = { git = "https://github.com/caya8205-2/innertube-rs" }
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ### 1. Fetch Video Metadata & Streaming URLs
//!
//! ```no_run
//! use innertube_rs::{Innertube, FormatFilter, FormatType, QualityPreference};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize client (bootstraps session & decipher engine)
//!     let yt = Innertube::new().await?;
//!
//!     // Fetch metadata
//!     let video_info = yt.get_video_info("dQw4w9WgXcQ").await?;
//!     if let Some(details) = video_info.video_details {
//!         println!("Title: {}", details.title);
//!         println!("Duration: {}s", details.length_seconds);
//!     }
//!
//!     // Resolve highest quality audio-only stream URL (e.g. for music players)
//!     let audio_filter = FormatFilter {
//!         format_type: FormatType::AudioOnly,
//!         quality: QualityPreference::Highest,
//!         container: None,
//!     };
//!
//!     let audio_url = yt.get_stream_url("dQw4w9WgXcQ", &audio_filter).await?;
//!     println!("Playable Audio Stream URL: {}", audio_url);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 2. Search Videos, Channels, and Playlists
//!
//! ```no_run
//! use innertube_rs::{Innertube, SearchResultItem};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let yt = Innertube::new().await?;
//!     let results = yt.search("rick astley", None).await?;
//!
//!     for item in results.items {
//!         match item {
//!             SearchResultItem::Video(v) => println!("Video: {} ({})", v.title, v.video_id),
//!             SearchResultItem::Channel(c) => println!("Channel: {} ({})", c.title, c.channel_id),
//!             SearchResultItem::Playlist(p) => println!("Playlist: {} ({})", p.title, p.playlist_id),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 3. Browse Channels & Playlists
//!
//! ```no_run
//! use innertube_rs::Innertube;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let yt = Innertube::new().await?;
//!
//!     // Fetch channel details & top tracks
//!     let channel = yt.get_channel("@RickAstleyYT").await?;
//!     println!("Channel Name: {}", channel.name);
//!     println!("Top tracks: {}", channel.top_tracks.len());
//!
//!     // Fetch playlist tracklist
//!     let playlist = yt.get_playlist("PLlaN88a7y2_plecYoJxeQNnWiiN01LUcZ").await?;
//!     println!("Playlist Name: {}", playlist.name);
//!     println!("Tracks: {}", playlist.tracks.len());
//!
//!     Ok(())
//! }
//! ```

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

// Re-exports for convenient top-level access
pub use error::{InnertubeError, Result};
pub use models::format::{FormatFilter, FormatType, QualityPreference, StreamingFormat};
pub use models::video::{PlayerResponse, VideoDetails, StreamingData, PlayabilityStatus, Thumbnail};
pub use models::search::{SearchResults, SearchResultItem, SearchVideoItem, SearchChannelItem, SearchPlaylistItem};
pub use models::channel::{ChannelArtistView, ChannelTrack, ChannelPlaylist, YouTubePlaylistView};
pub use models::next::{AutoplayVideo, PlaylistPanelItem, RelatedVideo, WatchNextResults};
pub use models::transcript::{Transcript, TranscriptSegment, TranscriptTrack};
pub use models::comments::{Comment, CommentThread, CommentsResult};
pub use models::manifest::{ManifestStream, ParsedManifest};
pub use models::music::{
    MusicAlbumItem, MusicAlbumRef, MusicAlbumView, MusicArtistItem, MusicArtistRef,
    MusicExplore, MusicLyrics, MusicPlaylistItem, MusicSearchFilter, MusicSearchResults,
    MusicTrackItem,
};
pub use core::session::{Session, SessionOptions};
pub use core::player::Player;

use crate::endpoints::browse::{get_channel, get_playlist};
use crate::endpoints::comments::{get_comment_replies, get_comments};
use crate::endpoints::music::{get_music_album, get_music_explore, get_music_lyrics, search_music};
use crate::endpoints::next::get_watch_next;
use crate::endpoints::player::{fetch_player_response, resolve_stream_url, select_format};
use crate::endpoints::search::search;
use crate::endpoints::transcript::{get_transcript, get_transcript_tracks};

/// Main high-level Innertube client.
///
/// Holds the HTTP session state, API keys, client context, and the player decipher engine.
#[derive(Clone)]
pub struct Innertube {
    /// InnerTube session state and HTTP client.
    pub session: Arc<Session>,
    /// Player manager and signature/n-token decipher engine.
    pub player: Arc<Player>,
}

impl Innertube {
    /// Initialize a new `Innertube` client with default options.
    ///
    /// Bootstraps session data from YouTube (`sw.js_data`) and downloads the latest
    /// player script to initialize the QuickJS decipher engine.
    pub async fn new() -> Result<Self> {
        Self::with_options(SessionOptions::default()).await
    }

    /// Initialize a new `Innertube` client with custom session options.
    pub async fn with_options(options: SessionOptions) -> Result<Self> {
        let session = Session::create(options).await?;
        let player = Player::create(&session.http_client, None).await?;

        Ok(Self {
            session: Arc::new(session),
            player: Arc::new(player),
        })
    }

    /// Fetch metadata and available streaming formats for a YouTube video ID.
    pub async fn get_video_info(&self, video_id: &str) -> Result<PlayerResponse> {
        fetch_player_response(
            &self.session,
            video_id,
            Some(self.player.decipherer.signature_timestamp),
        )
        .await
    }

    /// Retrieve a decrypted, playable streaming URL matching the specified filter.
    ///
    /// Automatically applies signature deciphering and n-token transformations to ensure
    /// the returned URL is not throttled or forbidden (403).
    pub async fn get_stream_url(&self, video_id: &str, filter: &FormatFilter) -> Result<String> {
        let player_res = self.get_video_info(video_id).await?;
        let format = select_format(&player_res, filter)?;
        resolve_stream_url(format, &self.player.decipherer)
    }

    /// Execute a search query for videos, channels, and playlists.
    pub async fn search(&self, query: &str, continuation_token: Option<&str>) -> Result<SearchResults> {
        search(&self.session, query, continuation_token).await
    }

    /// Fetch channel profile, videos, and playlists by channel ID or handle (e.g. `@RickAstleyYT` or `UC...`).
    pub async fn get_channel(&self, channel_id_or_handle: &str) -> Result<ChannelArtistView> {
        get_channel(&self.session, channel_id_or_handle).await
    }

    /// Fetch playlist tracklist by playlist ID.
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<YouTubePlaylistView> {
        get_playlist(&self.session, playlist_id).await
    }

    /// Fetch watch next details including recommended/related videos, autoplay, and playlist queue.
    pub async fn get_watch_next(&self, video_id: &str) -> Result<WatchNextResults> {
        get_watch_next(&self.session, video_id, None, None, None).await
    }

    /// Fetch recommended related videos for a given video ID.
    pub async fn get_related_videos(&self, video_id: &str) -> Result<Vec<RelatedVideo>> {
        let next_res = self.get_watch_next(video_id).await?;
        Ok(next_res.related_videos)
    }

    /// Fetch watch next details when playing within a playlist.
    pub async fn get_playlist_watch_next(
        &self,
        video_id: &str,
        playlist_id: &str,
        playlist_index: Option<usize>,
    ) -> Result<WatchNextResults> {
        get_watch_next(&self.session, video_id, Some(playlist_id), playlist_index, None).await
    }

    /// Fetch continuation results for watch next recommendations using a continuation token.
    pub async fn get_watch_next_continuation(&self, continuation_token: &str) -> Result<WatchNextResults> {
        get_watch_next(&self.session, "", None, None, Some(continuation_token)).await
    }

    /// Fetch available caption tracks for a video.
    pub async fn get_transcript_tracks(&self, video_id: &str) -> Result<Vec<TranscriptTrack>> {
        get_transcript_tracks(&self.session, video_id).await
    }

    /// Fetch timed transcript/subtitles for a video in the specified language (or first available).
    pub async fn get_transcript(&self, video_id: &str, lang: Option<&str>) -> Result<Transcript> {
        get_transcript(&self.session, video_id, lang).await
    }

    /// Fetch top comment threads for a video ID or continuation token.
    pub async fn get_comments(&self, video_id: &str) -> Result<CommentsResult> {
        get_comments(&self.session, video_id, None).await
    }

    /// Fetch next page of comments using a continuation token.
    pub async fn get_comments_continuation(&self, continuation_token: &str) -> Result<CommentsResult> {
        get_comments(&self.session, "", Some(continuation_token)).await
    }

    /// Fetch child replies for a specific comment thread.
    pub async fn get_comment_replies(&self, continuation_token: &str) -> Result<Vec<Comment>> {
        get_comment_replies(&self.session, continuation_token).await
    }

    /// Perform a filtered search on YouTube Music (`WEB_REMIX`).
    pub async fn search_music(
        &self,
        query: &str,
        filter: Option<MusicSearchFilter>,
    ) -> Result<MusicSearchResults> {
        search_music(&self.session, query, filter).await
    }

    /// Fetch song lyrics from YouTube Music for a given video ID.
    pub async fn get_music_lyrics(&self, video_id: &str) -> Result<MusicLyrics> {
        get_music_lyrics(&self.session, video_id).await
    }

    /// Fetch YouTube Music album details and tracklist by browse ID (e.g. `MPREb_...`).
    pub async fn get_music_album(&self, browse_id: &str) -> Result<MusicAlbumView> {
        get_music_album(&self.session, browse_id).await
    }

    /// Fetch YouTube Music explore and trending page data.
    pub async fn get_music_explore(&self) -> Result<MusicExplore> {
        get_music_explore(&self.session).await
    }
}
