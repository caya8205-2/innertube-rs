use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed PlayerOverlay AST node (`playerOverlayRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOverlayNode {
    pub actions: Vec<Value>,
    pub autonav_toggle: Option<Value>,
    pub decorated_player_bar: Option<Value>,
}

impl PlayerOverlayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playerOverlayRenderer").unwrap_or(val);
        let actions = node.get("actions").and_then(Value::as_array).cloned().unwrap_or_default();
        let autonav_toggle = node.get("autonavToggle").cloned();
        let decorated_player_bar = node.get("decoratedPlayerBarRenderer").cloned();

        Some(Self {
            actions,
            autonav_toggle,
            decorated_player_bar,
        })
    }
}

/// Strongly typed PlayerStoryboardSpec AST node (`playerStoryboardSpecRenderer` / `playerLiveStoryboardSpecRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStoryboardSpecNode {
    pub spec: Option<String>,
}

impl PlayerStoryboardSpecNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerStoryboardSpecRenderer")
            .or_else(|| val.get("playerLiveStoryboardSpecRenderer"))
            .unwrap_or(val);

        let spec = node
            .get("spec")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self { spec })
    }
}

/// Strongly typed TimedMarkerDecoration AST node (`timedMarkerDecorationRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimedMarkerDecorationNode {
    pub visible_time_range_start_millis: Option<u64>,
    pub visible_time_range_end_millis: Option<u64>,
    pub decoration_time_millis: Option<u64>,
    pub label: Option<String>,
    pub icon_type: Option<String>,
}

impl TimedMarkerDecorationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("timedMarkerDecorationRenderer").unwrap_or(val);

        let visible_time_range_start_millis = node
            .get("visibleTimeRangeStartMillis")
            .and_then(Value::as_u64)
            .or_else(|| node.get("visibleTimeRangeStartMillis").and_then(Value::as_str).and_then(|s| s.parse().ok()));

        let visible_time_range_end_millis = node
            .get("visibleTimeRangeEndMillis")
            .and_then(Value::as_u64)
            .or_else(|| node.get("visibleTimeRangeEndMillis").and_then(Value::as_str).and_then(|s| s.parse().ok()));

        let decoration_time_millis = node
            .get("decorationTimeMillis")
            .and_then(Value::as_u64)
            .or_else(|| node.get("decorationTimeMillis").and_then(Value::as_str).and_then(|s| s.parse().ok()));

        let label = node.get("label").and_then(TextNode::from_value).map(|t| t.text);
        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            visible_time_range_start_millis,
            visible_time_range_end_millis,
            decoration_time_millis,
            label,
            icon_type,
        })
    }
}
