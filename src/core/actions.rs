use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::actions::{ActionResult, CreateCommentResult, CreatePlaylistResult};
use crate::utils::proto::encode_create_comment_params;

/// Account Mutations and User Interactions Manager (`Actions.ts` & `InteractionManager.ts`).
pub struct Actions;

impl Actions {
    /// Like a YouTube video (`POST /like/like`, TV client per legacy).
    pub async fn like(session: &Session, video_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = rating_payload(video_id);

        let resp = session.post_innertube_client("TV", "/like/like", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("LIKE".to_string()),
            action_id: None,
        })
    }

    /// Dislike a YouTube video (`POST /like/dislike`, TV client per legacy).
    pub async fn dislike(session: &Session, video_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = rating_payload(video_id);

        let resp = session.post_innertube_client("TV", "/like/dislike", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("DISLIKE".to_string()),
            action_id: None,
        })
    }

    /// Remove like or dislike rating from a video (`POST /like/removelike`, TV client per legacy).
    pub async fn remove_rating(session: &Session, video_id: &str) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = rating_payload(video_id);

        let resp = session.post_innertube_client("TV", "/like/removelike", payload).await?;
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
        let payload = subscription_payload(channel_ids, true);

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
        let payload = subscription_payload(channel_ids, false);

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
        // Legacy deletePlaylistServiceEndpoint uses `sourcePlaylistId`.
        let payload = delete_playlist_payload(playlist_id);

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
        let actions = remove_playlist_actions(set_video_ids);

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

    /// Set the title / name of a playlist (`POST /browse/edit_playlist`).
    pub async fn set_playlist_name(
        session: &Session,
        playlist_id: &str,
        name: &str,
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = set_playlist_name_payload(playlist_id, name);

        let resp = session
            .post_innertube("/browse/edit_playlist", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("NAME_UPDATED".to_string()),
            action_id: None,
        })
    }

    /// Set the description of a playlist (`POST /browse/edit_playlist`).
    pub async fn set_playlist_description(
        session: &Session,
        playlist_id: &str,
        description: &str,
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "playlistId": playlist_id,
            "actions": [{
                "action": "ACTION_SET_PLAYLIST_DESCRIPTION",
                "playlistDescription": description
            }]
        });

        let resp = session
            .post_innertube("/browse/edit_playlist", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("DESCRIPTION_UPDATED".to_string()),
            action_id: None,
        })
    }

    /// Move a video to after another video in a playlist (`POST /browse/edit_playlist`).
    pub async fn move_playlist_video(
        session: &Session,
        playlist_id: &str,
        set_video_id: &str,
        predecessor_set_video_id: &str,
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = json!({
            "playlistId": playlist_id,
            "actions": [{
                "action": "ACTION_MOVE_VIDEO_AFTER",
                "setVideoId": set_video_id,
                "movedSetVideoIdPredecessor": predecessor_set_video_id
            }]
        });

        let resp = session
            .post_innertube("/browse/edit_playlist", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("MOVED".to_string()),
            action_id: None,
        })
    }

    /// Add a playlist to the user's library (`POST /like/like`).
    pub async fn add_playlist_to_library(
        session: &Session,
        playlist_id: &str,
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = rating_payload(playlist_id);

        let resp = session.post_innertube("/like/like", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("ADDED_TO_LIBRARY".to_string()),
            action_id: None,
        })
    }

    /// Remove a playlist from the user's library (`POST /like/removelike`).
    pub async fn remove_playlist_from_library(
        session: &Session,
        playlist_id: &str,
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let payload = rating_payload(playlist_id);

        let resp = session.post_innertube("/like/removelike", payload).await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("REMOVED_FROM_LIBRARY".to_string()),
            action_id: None,
        })
    }

    /// Translate text using YouTube's comment translation feature
    /// (`POST /comment/perform_comment_action`, action type 22).
    pub async fn translate(
        session: &Session,
        text: &str,
        target_language: &str,
        video_id: Option<&str>,
        comment_id: Option<&str>,
    ) -> Result<crate::models::actions::TranslateResult> {
        let action = crate::utils::proto::encode_comment_action_params(
            22,
            &crate::utils::proto::CommentActionParamsArgs {
                comment_id: comment_id.map(ToString::to_string),
                video_id: video_id.map(ToString::to_string),
                text: Some(text.to_string()),
                target_language: Some(target_language.to_string()),
            },
        )?;

        // Route through execute: legacy munges `action` into `actions: [...]`.
        let response = Self::execute(
            session,
            "/comment/perform_comment_action",
            json!({ "action": action }),
        )
        .await?;
        let status_code = response.status_code;
        let data = response.data;

        let translated_content = data
            .pointer("/frameworkUpdates/entityBatchUpdate/mutations/0/payload/commentEntityPayload/translatedContent/content")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Ok(crate::models::actions::TranslateResult {
            success: resp_success(&data, status_code),
            status_code,
            translated_content,
            data,
        })
    }

    /// Modify notification preferences for a channel (`POST /notification/modify_channel_preference`).
    pub async fn set_notification_preferences(
        session: &Session,
        channel_id: &str,
        pref_type: crate::models::actions::NotificationPreferenceType,
    ) -> Result<ActionResult> {
        session.ensure_authenticated()?;
        let params = crate::utils::proto::encode_notification_preferences(
            channel_id,
            pref_type.index(),
        )?;
        let payload = json!({
            "params": params
        });

        let resp = session
            .post_innertube("/notification/modify_channel_preference", payload)
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let success = raw.get("error").is_none();
        Ok(ActionResult {
            success,
            status: Some("PREFERENCE_UPDATED".to_string()),
            action_id: None,
        })
    }

    /// Generic action dispatcher matching `Actions.execute` (1:1 with YouTube.js).
    ///
    /// Supports legacy control keys in `args`: `parse`, `override_endpoint`,
    /// `skip_auth_check`, `request`, `clientActions`, `settingItemIdForClient`
    /// (stripped before sending), `action`, `boolValue`, `token` (munged),
    /// `client` (routes context adjustment), and `protobuf` +
    /// `serialized_data` (raw protobuf body).
    pub async fn execute(
        session: &Session,
        endpoint: &str,
        args: Value,
    ) -> Result<ApiResponse> {
        let prepared = prepare_execute(&args, session.is_authenticated())?;
        let target = prepared.endpoint_override.as_deref().unwrap_or(endpoint);

        let resp = match &prepared.body {
            ExecuteBody::Protobuf(bytes) => {
                session.post_innertube_protobuf(target, bytes.clone()).await?
            }
            ExecuteBody::Json(body) => match &prepared.client {
                Some(client) => {
                    session
                        .post_innertube_client(client, target, body.clone())
                        .await?
                }
                None => session.post_innertube(target, body.clone()).await?,
            },
        };

        let status_code = resp.status().as_u16();
        let success = resp.status().is_success();
        let mut data: Value = resp.json().await.map_err(InnertubeError::Network)?;

        let parsed = if prepared.parse {
            data = follow_navigate_redirect(session, data).await?;
            Some(crate::parser::Parser::parse_response(&data))
        } else {
            None
        };

        Ok(ApiResponse {
            success,
            status_code,
            data,
            parsed,
        })
    }
}

/// Browse IDs that require an authenticated session, mirroring
/// `Actions.#needsLogin` in YouTube.js.
pub const LOGIN_REQUIRED_BROWSE_IDS: [&str; 11] = [
    "FElibrary",
    "FEhistory",
    "FEsubscriptions",
    "FEchannels",
    "FEplaylist_aggregation",
    "FEmusic_listening_review",
    "FEmusic_library_landing",
    "SPaccount_overview",
    "SPaccount_notifications",
    "SPaccount_privacy",
    "SPtime_watched",
];

impl Actions {
    /// Playback tracking stats call (legacy `Actions.stats`): GET with
    /// `ver=2`, `c`, `cbrver`, `cver` plus caller params.
    pub async fn stats(
        session: &Session,
        url: &str,
        client_name: &str,
        client_version: &str,
        params: &[(&str, String)],
    ) -> Result<reqwest::Response> {
        let mut parsed = url::Url::parse(url)
            .map_err(|e| InnertubeError::Format(format!("Invalid stats URL: {e}")))?;
        {
            let mut qp = parsed.query_pairs_mut();
            qp.append_pair("ver", "2");
            qp.append_pair("c", &client_name.to_lowercase());
            qp.append_pair("cbrver", client_version);
            qp.append_pair("cver", client_version);
            for (k, v) in params {
                qp.append_pair(k, v);
            }
        }

        let resp = session
            .http_client
            .get(parsed.as_str())
            .send()
            .await
            .map_err(InnertubeError::Network)?;
        Ok(resp)
    }
}

/// Prepared request body for `Actions::execute`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteBody {
    Json(Value),
    Protobuf(Vec<u8>),
}

/// Result of applying legacy `Actions.execute` argument munging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedExecute {
    pub body: ExecuteBody,
    pub endpoint_override: Option<String>,
    pub parse: bool,
    pub client: Option<String>,
}

/// Control keys stripped from the payload before sending (legacy `execute`).
const EXECUTE_CONTROL_KEYS: [&str; 8] = [
    "skip_auth_check",
    "override_endpoint",
    "parse",
    "request",
    "clientActions",
    "settingItemIdForClient",
    "protobuf",
    "serialized_data",
];

/// Apply the argument-munging rules of legacy `Actions.execute` to `args`.
/// Pure function; performs no I/O.
pub fn prepare_execute(args: &Value, logged_in: bool) -> Result<PreparedExecute> {
    let obj = args.as_object().cloned().unwrap_or_default();

    let endpoint_override = obj
        .get("override_endpoint")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let parse = obj
        .get("parse")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let is_protobuf = obj
        .get("protobuf")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    if is_protobuf {
        let bytes = decode_serialized_data(obj.get("serialized_data"))?;
        return Ok(PreparedExecute {
            body: ExecuteBody::Protobuf(bytes),
            endpoint_override,
            parse,
            client: None,
        });
    }

    let mut data = obj;

    if let Some(browse_id) = data.get("browseId").and_then(Value::as_str) {
        let skip_auth_check = data
            .get("skip_auth_check")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if !skip_auth_check && LOGIN_REQUIRED_BROWSE_IDS.contains(&browse_id) && !logged_in {
            return Err(InnertubeError::AuthenticationRequired(
                "You must be signed in to perform this operation.".to_string(),
            ));
        }
    }

    for key in EXECUTE_CONTROL_KEYS {
        data.remove(key);
    }

    if let Some(action) = data.remove("action") {
        data.insert("actions".to_string(), json!([action]));
    }

    if let Some(bool_value) = data.remove("boolValue") {
        data.insert("newValue".to_string(), json!({ "boolValue": bool_value }));
    }

    if let Some(token) = data.remove("token") {
        data.insert("continuation".to_string(), token);
    }

    let client = data
        .get("client")
        .and_then(Value::as_str)
        .map(ToString::to_string);

    if client.as_deref() == Some("YTMUSIC") {
        data.insert("isAudioOnly".to_string(), json!(true));
    }

    Ok(PreparedExecute {
        body: ExecuteBody::Json(Value::Object(data)),
        endpoint_override,
        parse,
        client,
    })
}

/// Decode `serialized_data` (byte array or base64 string) for protobuf calls.
fn decode_serialized_data(value: Option<&Value>) -> Result<Vec<u8>> {
    use base64::Engine;

    match value {
        Some(Value::Array(items)) => items
            .iter()
            .map(|v| {
                v.as_u64()
                    .and_then(|n| u8::try_from(n).ok())
                    .ok_or_else(|| {
                        InnertubeError::Format(
                            "serialized_data array must contain bytes (0-255)".to_string(),
                        )
                    })
            })
            .collect(),
        Some(Value::String(encoded)) => base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| InnertubeError::Format(format!("invalid serialized_data base64: {e}"))),
        _ => Err(InnertubeError::Format(
            "protobuf execute calls require serialized_data (byte array or base64)".to_string(),
        )),
    }
}

/// Follow `navigateAction` redirects in a parsed browse response, mirroring
/// the redirect handling in legacy `Actions.execute`.
///
/// ponytail: legacy recurses unbounded; we cap at 5 redirects to guarantee
/// termination. Raise the cap if YouTube ever chains deeper.
pub(crate) async fn follow_navigate_redirect(session: &Session, mut data: Value) -> Result<Value> {
    for _ in 0..5 {
        let endpoint_value = data
            .pointer("/on_response_received_actions/0/navigateAction/endpoint")
            .cloned();
        let Some(endpoint_value) = endpoint_value else {
            return Ok(data);
        };

        let node = crate::parser::nodes::NavigationEndpointNode::from_value(&endpoint_value)
            .ok_or_else(|| {
                InnertubeError::Format(
                    "navigateAction endpoint could not be parsed".to_string(),
                )
            })?;
        let path = node.api_path.clone().ok_or_else(|| {
            InnertubeError::NotFound(
                "navigateAction endpoint has no InnerTube API path".to_string(),
            )
        })?;

        let resp = session.post_innertube(&path, node.payload.clone()).await?;
        data = resp.json().await.map_err(InnertubeError::Network)?;
    }

    Err(InnertubeError::Other(
        "navigateAction redirect limit (5) exceeded".to_string(),
    ))
}

/// An InnerTube API response matching `ApiResponse` in Actions.ts. When the
/// call used `parse: true`, `parsed` carries the typed
/// `parser::ParsedResponse` assembly (legacy returns `ParsedResponse<T>`).
#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub success: bool,
    pub status_code: u16,
    pub data: Value,
    pub parsed: Option<crate::parser::ParsedResponse>,
}

fn resp_success(data: &Value, status_code: u16) -> bool {
    (200..300).contains(&status_code) && data.get("error").is_none()
}

fn remove_playlist_actions(set_video_ids: &[&str]) -> Vec<Value> {
    set_video_ids
        .iter()
        .map(|set_id| {
            json!({
                "action": "ACTION_REMOVE_VIDEO",
                "setVideoId": set_id
            })
        })
        .collect()
}

fn rating_payload(video_id: &str) -> Value {
    json!({ "target": video_id })
}

/// Legacy subscribe/unsubscribe params (`EgIIAhgA` / `CgIIAhgA`).
fn subscription_payload(channel_ids: &[&str], subscribe: bool) -> Value {
    json!({
        "channelIds": channel_ids,
        "params": if subscribe { "EgIIAhgA" } else { "CgIIAhgA" }
    })
}

/// Legacy `deletePlaylistServiceEndpoint` payload (`sourcePlaylistId`).
fn delete_playlist_payload(playlist_id: &str) -> Value {
    json!({ "sourcePlaylistId": playlist_id })
}

/// Legacy quirk: setName uses the snake_case `playlist_id` key (unlike
/// setDescription which uses `playlistId`).
fn set_playlist_name_payload(playlist_id: &str, name: &str) -> Value {
    json!({
        "playlist_id": playlist_id,
        "actions": [{
            "action": "ACTION_SET_PLAYLIST_NAME",
            "playlistName": name
        }]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playlist_removal_actions_match_legacy_protocol() {
        assert_eq!(
            remove_playlist_actions(&["set-video-a", "set-video-b"]),
            vec![
                json!({ "action": "ACTION_REMOVE_VIDEO", "setVideoId": "set-video-a" }),
                json!({ "action": "ACTION_REMOVE_VIDEO", "setVideoId": "set-video-b" }),
            ]
        );
    }

    #[test]
    fn rating_target_matches_legacy_like_endpoint_request() {
        assert_eq!(rating_payload("dQw4w9WgXcQ"), json!({ "target": "dQw4w9WgXcQ" }));
    }

    #[test]
    fn subscription_payloads_carry_legacy_params() {
        let sub = subscription_payload(&["UC_test"], true);
        assert_eq!(sub["channelIds"], json!(["UC_test"]));
        assert_eq!(sub["params"], json!("EgIIAhgA"));

        let unsub = subscription_payload(&["UC_test"], false);
        assert_eq!(unsub["params"], json!("CgIIAhgA"));
    }

    #[test]
    fn delete_playlist_uses_source_playlist_id() {
        let payload = delete_playlist_payload("PL_test");
        assert_eq!(payload, json!({ "sourcePlaylistId": "PL_test" }));
        assert!(payload.get("playlistId").is_none());
    }

    #[test]
    fn set_playlist_name_uses_legacy_snake_case_quirk() {
        let payload = set_playlist_name_payload("PL_test", "New Title");
        assert_eq!(payload["playlist_id"], json!("PL_test"));
        assert!(payload.get("playlistId").is_none());
        assert_eq!(payload["actions"][0]["action"], json!("ACTION_SET_PLAYLIST_NAME"));
        assert_eq!(payload["actions"][0]["playlistName"], json!("New Title"));
    }

    #[test]
    fn execute_munges_action_bool_value_and_token() {
        let args = json!({
            "action": { "action": "ACTION_ADD_VIDEO", "addedVideoId": "vid" },
            "boolValue": true,
            "token": "cont-token"
        });
        let prepared = prepare_execute(&args, false).unwrap();
        let ExecuteBody::Json(body) = prepared.body else {
            panic!("expected JSON body");
        };
        assert_eq!(
            body["actions"],
            json!([{ "action": "ACTION_ADD_VIDEO", "addedVideoId": "vid" }])
        );
        assert_eq!(body["newValue"], json!({ "boolValue": true }));
        assert_eq!(body["continuation"], json!("cont-token"));
        assert!(body.get("action").is_none());
        assert!(body.get("boolValue").is_none());
        assert!(body.get("token").is_none());
    }

    #[test]
    fn execute_strips_control_keys_and_extracts_flags() {
        let args = json!({
            "browseId": "FEwhat_to_watch",
            "skip_auth_check": true,
            "override_endpoint": "/custom",
            "parse": true,
            "request": { "x": 1 },
            "clientActions": [],
            "settingItemIdForClient": "abc"
        });
        let prepared = prepare_execute(&args, false).unwrap();
        assert_eq!(prepared.endpoint_override.as_deref(), Some("/custom"));
        assert!(prepared.parse);
        let ExecuteBody::Json(body) = prepared.body else {
            panic!("expected JSON body");
        };
        for key in [
            "skip_auth_check",
            "override_endpoint",
            "parse",
            "request",
            "clientActions",
            "settingItemIdForClient",
        ] {
            assert!(body.get(key).is_none(), "{key} must be stripped");
        }
        assert_eq!(body["browseId"], json!("FEwhat_to_watch"));
    }

    #[test]
    fn execute_rejects_login_gated_browse_ids_anonymously() {
        for id in LOGIN_REQUIRED_BROWSE_IDS {
            let args = json!({ "browseId": id });
            assert!(
                matches!(
                    prepare_execute(&args, false),
                    Err(InnertubeError::AuthenticationRequired(_))
                ),
                "{id} must require login"
            );
        }
    }

    #[test]
    fn execute_allows_gated_ids_with_auth_or_skip_flag() {
        let args = json!({ "browseId": "FEhistory" });
        assert!(prepare_execute(&args, true).is_ok());

        let skipped = json!({ "browseId": "FEhistory", "skip_auth_check": true });
        assert!(prepare_execute(&skipped, false).is_ok());

        let public = json!({ "browseId": "FEwhat_to_watch" });
        assert!(prepare_execute(&public, false).is_ok());
    }

    #[test]
    fn execute_marks_ytmusic_payloads_audio_only() {
        let args = json!({ "client": "YTMUSIC", "browseId": "FEmusic_home" });
        let prepared = prepare_execute(&args, false).unwrap();
        assert_eq!(prepared.client.as_deref(), Some("YTMUSIC"));
        let ExecuteBody::Json(body) = prepared.body else {
            panic!("expected JSON body");
        };
        assert_eq!(body["isAudioOnly"], json!(true));
    }

    #[test]
    fn execute_protobuf_body_from_byte_array() {
        let args = json!({
            "protobuf": true,
            "serialized_data": [1, 2, 3, 255],
            "override_endpoint": "/video_manager/metadata_update"
        });
        let prepared = prepare_execute(&args, false).unwrap();
        assert_eq!(prepared.body, ExecuteBody::Protobuf(vec![1, 2, 3, 255]));
        assert_eq!(
            prepared.endpoint_override.as_deref(),
            Some("/video_manager/metadata_update")
        );
    }

    #[test]
    fn execute_protobuf_body_from_base64() {
        let args = json!({
            "protobuf": true,
            "serialized_data": "AQID/w=="
        });
        let prepared = prepare_execute(&args, false).unwrap();
        assert_eq!(prepared.body, ExecuteBody::Protobuf(vec![1, 2, 3, 255]));
    }

    #[test]
    fn execute_protobuf_requires_serialized_data() {
        let args = json!({ "protobuf": true });
        assert!(prepare_execute(&args, false).is_err());

        let bad = json!({ "protobuf": true, "serialized_data": [300] });
        assert!(prepare_execute(&bad, false).is_err());
    }

    #[test]
    fn playlist_mutation_payloads_match_legacy_protocol() {
        let set_name_payload = json!({
            "playlistId": "PL_test",
            "actions": [{
                "action": "ACTION_SET_PLAYLIST_NAME",
                "playlistName": "New Title"
            }]
        });
        assert_eq!(
            set_name_payload["actions"][0]["action"],
            "ACTION_SET_PLAYLIST_NAME"
        );
        assert_eq!(
            set_name_payload["actions"][0]["playlistName"],
            "New Title"
        );

        let move_payload = json!({
            "playlistId": "PL_test",
            "actions": [{
                "action": "ACTION_MOVE_VIDEO_AFTER",
                "setVideoId": "set_1",
                "movedSetVideoIdPredecessor": "set_0"
            }]
        });
        assert_eq!(
            move_payload["actions"][0]["action"],
            "ACTION_MOVE_VIDEO_AFTER"
        );
        assert_eq!(
            move_payload["actions"][0]["setVideoId"],
            "set_1"
        );
        assert_eq!(
            move_payload["actions"][0]["movedSetVideoIdPredecessor"],
            "set_0"
        );
    }
}
