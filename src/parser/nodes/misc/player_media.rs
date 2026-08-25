use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed PlayerCaptionsTracklist AST node (`playerCaptionsTracklistRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCaptionsTracklistNode {
    pub caption_tracks: Vec<Value>,
    pub audio_tracks: Vec<Value>,
    pub translation_languages: Vec<Value>,
}

impl PlayerCaptionsTracklistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playerCaptionsTracklistRenderer").unwrap_or(val);

        let caption_tracks = node
            .get("captionTracks")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let audio_tracks = node
            .get("audioTracks")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let translation_languages = node
            .get("translationLanguages")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self {
            caption_tracks,
            audio_tracks,
            translation_languages,
        })
    }
}

/// Strongly typed PlayerErrorMessage AST node (`playerErrorMessageRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerErrorMessageNode {
    pub reason: String,
    pub subreason: Option<String>,
    pub icon_type: Option<String>,
}

impl PlayerErrorMessageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playerErrorMessageRenderer").unwrap_or(val);

        let reason = node
            .get("reason")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("reason").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let subreason = node
            .get("subreason")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("subreason").and_then(Value::as_str).map(ToString::to_string));

        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            reason,
            subreason,
            icon_type,
        })
    }
}

/// Strongly typed PlayerLegacyDesktopYpcTrailer AST node (`playerLegacyDesktopYpcTrailerRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLegacyDesktopYpcTrailerNode {
    pub video_id: Option<String>,
    pub ypc_message: Option<String>,
}

impl PlayerLegacyDesktopYpcTrailerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playerLegacyDesktopYpcTrailerRenderer").unwrap_or(val);

        let video_id = node.get("videoRenderer").and_then(|v| v.get("videoId")).and_then(Value::as_str).map(ToString::to_string)
            .or_else(|| node.get("videoId").and_then(Value::as_str).map(ToString::to_string));

        let ypc_message = node
            .get("ypcMessage")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("ypcMessage").and_then(Value::as_str).map(ToString::to_string));

        Some(Self {
            video_id,
            ypc_message,
        })
    }
}
