use crate::core::actions::Actions;
use crate::core::session::Session;
use crate::endpoints::playlist::{get_playlist, get_playlist_continuation};
use crate::error::Result;
use crate::models::actions::{ActionResult, CreatePlaylistResult};
use crate::models::playlist::{PlaylistContinuation, PlaylistView};

/// Playlist Manager (1:1 with PlaylistManager.ts).
pub struct PlaylistManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> PlaylistManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Fetch a full YouTube playlist view.
    pub async fn get(&self, playlist_id: &str) -> Result<PlaylistView> {
        get_playlist(self.session, playlist_id).await
    }

    /// Fetch next page of playlist videos using continuation token.
    pub async fn get_continuation(&self, continuation_token: &str) -> Result<PlaylistContinuation> {
        get_playlist_continuation(self.session, continuation_token).await
    }

    /// Create a new YouTube playlist.
    pub async fn create(&self, title: &str, video_ids: Option<&[&str]>) -> Result<CreatePlaylistResult> {
        Actions::create_playlist(self.session, title, video_ids).await
    }

    /// Delete a YouTube playlist.
    pub async fn delete(&self, playlist_id: &str) -> Result<ActionResult> {
        Actions::delete_playlist(self.session, playlist_id).await
    }

    /// Add videos to a playlist.
    pub async fn add_videos(&self, playlist_id: &str, video_ids: &[&str]) -> Result<ActionResult> {
        Actions::add_to_playlist(self.session, playlist_id, video_ids).await
    }

    /// Remove videos from a playlist.
    pub async fn remove_videos(&self, playlist_id: &str, set_video_ids: &[&str]) -> Result<ActionResult> {
        Actions::remove_from_playlist(self.session, playlist_id, set_video_ids).await
    }

    /// Move a video within a playlist.
    pub async fn move_video(
        &self,
        playlist_id: &str,
        set_video_id: &str,
        predecessor_set_video_id: &str,
    ) -> Result<ActionResult> {
        Actions::move_playlist_video(
            self.session,
            playlist_id,
            set_video_id,
            predecessor_set_video_id,
        )
        .await
    }

    /// Set playlist title / name.
    pub async fn set_name(&self, playlist_id: &str, name: &str) -> Result<ActionResult> {
        Actions::set_playlist_name(self.session, playlist_id, name).await
    }

    /// Set playlist description.
    pub async fn set_description(&self, playlist_id: &str, description: &str) -> Result<ActionResult> {
        Actions::set_playlist_description(self.session, playlist_id, description).await
    }

    /// Add playlist to user's library.
    pub async fn add_to_library(&self, playlist_id: &str) -> Result<ActionResult> {
        Actions::add_playlist_to_library(self.session, playlist_id).await
    }

    /// Remove playlist from user's library.
    pub async fn remove_from_library(&self, playlist_id: &str) -> Result<ActionResult> {
        Actions::remove_playlist_from_library(self.session, playlist_id).await
    }
}
