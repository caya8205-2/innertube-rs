use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::actions::{ActionResult, CreateCommentResult, CreatePlaylistResult};
use crate::utils::proto::encode_create_comment_params;

/// Account Mutations and User Interactions Manager (`Actions.ts` & `InteractionManager.ts`).
pub struct Actions;

impl Actions {
    /// Like a YouTube video (`POST /like/like`).
    pub async fn like(session: &Session, video_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "target": {
                "videoId": video_id
            }
        });

        let resp = session.post_innertube("/like/like", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("LIKE".to_string()),
            action_id: None,
        })
    }

    /// Dislike a YouTube video (`POST /like/dislike`).
    pub async fn dislike(session: &Session, video_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "target": {
                "videoId": video_id
            }
        });

        let resp = session.post_innertube("/like/dislike", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("DISLIKE".to_string()),
            action_id: None,
        })
    }

    /// Remove like or dislike rating from a video (`POST /like/removelike`).
    pub async fn remove_rating(session: &Session, video_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "target": {
                "videoId": video_id
            }
        });

        let resp = session.post_innertube("/like/removelike", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("INDIFFERENT".to_string()),
            action_id: None,
        })
    }

    /// Subscribe to one or more YouTube channels (`POST /subscription/subscribe`).
    pub async fn subscribe(session: &Session, channel_ids: &[&str]) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "channelIds": channel_ids
        });

        let resp = session
            .post_innertube("/subscription/subscribe", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("SUBSCRIBED".to_string()),
            action_id: None,
        })
    }

    /// Unsubscribe from one or more YouTube channels (`POST /subscription/unsubscribe`).
    pub async fn unsubscribe(session: &Session, channel_ids: &[&str]) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "channelIds": channel_ids
        });

        let resp = session
            .post_innertube("/subscription/unsubscribe", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("UNSUBSCRIBED".to_string()),
            action_id: None,
        })
    }

    /// Create a new YouTube playlist (`POST /playlist/create`).
    pub async fn create_playlist(
        session: &Session,
        title: &str,
        video_ids: Option<&[&str]>,
    ) -> Result<CreatePlaylistResult> {
        session.ensure_authenticated()?;
        let mut payload = json!({
            "title": title,
        });

        if let Some(vids) = video_ids {
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("videoIds".to_string(), json!(vids));
            }
        }

        let resp = session.post_innertube("/playlist/create", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let playlist_id = raw
            .get("playlistId")
            .and_then(Value::as_str)
            .map(|s| s.to_string());
        let success = playlist_id.is_some();

        Ok(CreatePlaylistResult {
            success,
            playlist_id,
        })
    }

    /// Delete a YouTube playlist (`POST /playlist/delete`).
    pub async fn delete_playlist(session: &Session, playlist_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "playlistId": playlist_id
        });

        let resp = session.post_innertube("/playlist/delete", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("DELETED".to_string()),
            action_id: None,
        })
    }

    /// Add videos to an existing playlist (`POST /browse/edit_playlist`).
    pub async fn add_to_playlist(
        session: &Session,
        playlist_id: &str,
        video_ids: &[&str],
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let actions: Vec<Value> = video_ids
            .iter()
            .map(|vid| {
                json!({
                    "action": "ACTION_ADD_VIDEO",
                    "addedVideoId": vid
                })
            })
            .collect();

        let payload = json!({
            "playlistId": playlist_id,
            "actions": actions
        });

        let resp = session
            .post_innertube("/browse/edit_playlist", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("ADDED".to_string()),
            action_id: None,
        })
    }

    /// Remove videos from an existing playlist (`POST /browse/edit_playlist`).
    pub async fn remove_from_playlist(
        session: &Session,
        playlist_id: &str,
        set_video_ids: &[&str],
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let actions: Vec<Value> = set_video_ids
            .iter()
            .map(|set_id| {
                json!({
                    "action": "ACTION_REMOVE_VIDEO_BY_SET_ITEM_ID",
                    "setVideoId": set_id
                })
            })
            .collect();

        let payload = json!({
            "playlistId": playlist_id,
            "actions": actions
        });

        let resp = session
            .post_innertube("/browse/edit_playlist", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("REMOVED".to_string()),
            action_id: None,
        })
    }

    /// Create a top-level comment on a video (`POST /comment/create_comment`).
    pub async fn create_comment(
        session: &Session,
        video_id: &str,
        comment_text: &str,
    ) -> Result<CreateCommentResult> {
        session.ensure_authenticated()?;
        let create_comment_params = encode_create_comment_params(video_id)?;
        let payload = json!({
            "commentText": comment_text,
            "createCommentParams": create_comment_params,
        });

        let resp = session
            .post_innertube("/comment/create_comment", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let comment_id = raw.pointer("/actions/0/createCommentAction/contents/commentThreadRenderer/comment/commentRenderer/commentId")
            .and_then(Value::as_str)
            .map(|s| s.to_string());

        let success = comment_id.is_some() || raw.get("error").is_none();
        Ok(CreateCommentResult {
            success,
            comment_id,
        })
    }
}
