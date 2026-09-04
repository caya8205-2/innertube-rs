use crate::core::actions::Actions;
use crate::core::session::Session;
use crate::endpoints::playlist::{get_playlist, get_playlist_continuation};
use crate::error::{InnertubeError, Result};
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

    /// Remove videos from a playlist by their set video IDs.
    pub async fn remove_videos(&self, playlist_id: &str, set_video_ids: &[&str]) -> Result<ActionResult> {
        Actions::remove_from_playlist(self.session, playlist_id, set_video_ids).await
    }

    /// Remove videos from a playlist by video IDs, resolving each
    /// `setVideoId` by paginating the playlist (legacy
    /// `PlaylistManager.removeVideos`). With `use_set_video_ids`, the given
    /// IDs are matched against set video IDs directly.
    pub async fn remove_videos_by_id(
        &self,
        playlist_id: &str,
        video_ids: &[&str],
        use_set_video_ids: bool,
    ) -> Result<ActionResult> {
        self.session.ensure_authenticated()?;

        let playlist = self.get(playlist_id).await?;
        if !playlist.is_editable {
            return Err(InnertubeError::Other(format!(
                "This playlist cannot be edited. ({playlist_id})"
            )));
        }

        let mut set_ids: Vec<String> = Vec::new();
        let mut page_videos = playlist.videos.clone();
        let mut continuation = playlist.continuation_token.clone();

        loop {
            for video in &page_videos {
                let key_matches = if use_set_video_ids {
                    video.set_video_id.as_deref()
                } else {
                    Some(video.id.as_str())
                };
                if key_matches.is_some_and(|k| video_ids.contains(&k)) {
                    if let Some(ref set_id) = video.set_video_id {
                        if !set_ids.contains(set_id) {
                            set_ids.push(set_id.clone());
                        }
                    }
                }
            }

            if set_ids.len() >= video_ids.len() {
                break;
            }

            match continuation.take() {
                Some(token) => {
                    let page = self.get_continuation(&token).await?;
                    page_videos = page.videos;
                    continuation = page.continuation_token;
                }
                None => break,
            }
        }

        if set_ids.is_empty() {
            return Err(InnertubeError::Other(format!(
                "Given video ids were not found in this playlist. ({video_ids:?})"
            )));
        }

        let set_id_refs: Vec<&str> = set_ids.iter().map(String::as_str).collect();
        Actions::remove_from_playlist(self.session, playlist_id, &set_id_refs).await
    }

    /// Move a video within a playlist by video IDs, resolving both
    /// `setVideoId`s by paginating the playlist (legacy
    /// `PlaylistManager.moveVideo`).
    pub async fn move_video_by_id(
        &self,
        playlist_id: &str,
        moved_video_id: &str,
        predecessor_video_id: &str,
    ) -> Result<ActionResult> {
        self.session.ensure_authenticated()?;

        let playlist = self.get(playlist_id).await?;
        if !playlist.is_editable {
            return Err(InnertubeError::Other(format!(
                "This playlist cannot be edited. ({playlist_id})"
            )));
        }

        let mut moved_set_id: Option<String> = None;
        let mut predecessor_set_id: Option<String> = None;
        let mut page_videos = playlist.videos.clone();
        let mut continuation = playlist.continuation_token.clone();

        loop {
            for video in &page_videos {
                if video.id == moved_video_id && moved_set_id.is_none() {
                    moved_set_id = video.set_video_id.clone();
                }
                if video.id == predecessor_video_id && predecessor_set_id.is_none() {
                    predecessor_set_id = video.set_video_id.clone();
                }
            }

            if moved_set_id.is_some() && predecessor_set_id.is_some() {
                break;
            }

            match continuation.take() {
                Some(token) => {
                    let page = self.get_continuation(&token).await?;
                    page_videos = page.videos;
                    continuation = page.continuation_token;
                }
                None => break,
            }
        }

        let moved = moved_set_id.ok_or_else(|| {
            InnertubeError::Other(format!(
                "Video {moved_video_id} was not found in this playlist."
            ))
        })?;
        let predecessor = predecessor_set_id.ok_or_else(|| {
            InnertubeError::Other(format!(
                "Video {predecessor_video_id} was not found in this playlist."
            ))
        })?;

        Actions::move_playlist_video(self.session, playlist_id, &moved, &predecessor).await
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
