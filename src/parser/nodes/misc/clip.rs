use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed ClipCreation AST node (`clipCreationRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipCreationNode {
    pub video_id: Option<String>,
    pub title: Option<String>,
    pub duration_text: Option<String>,
    pub scrubber: Option<Value>,
}

impl ClipCreationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("clipCreationRenderer").unwrap_or(val);

        let video_id = node
            .get("videoId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let duration_text = node
            .get("durationText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let scrubber = node.get("scrubber").cloned();

        Some(Self {
            video_id,
            title,
            duration_text,
            scrubber,
        })
    }
}

/// Strongly typed ClipCreationScrubber AST node (`clipCreationScrubberRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipCreationScrubberNode {
    pub min_length_ms: Option<u64>,
    pub max_length_ms: Option<u64>,
    pub default_length_ms: Option<u64>,
    pub window_size_ms: Option<u64>,
    pub start_label: Option<String>,
}

impl ClipCreationScrubberNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("clipCreationScrubberRenderer").unwrap_or(val);

        let min_length_ms = node.get("minLengthMs").and_then(Value::as_u64);
        let max_length_ms = node.get("maxLengthMs").and_then(Value::as_u64);
        let default_length_ms = node.get("defaultLengthMs").and_then(Value::as_u64);
        let window_size_ms = node.get("windowSizeMs").and_then(Value::as_u64);
        let start_label = node
            .get("startLabel")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            min_length_ms,
            max_length_ms,
            default_length_ms,
            window_size_ms,
            start_label,
        })
    }
}
