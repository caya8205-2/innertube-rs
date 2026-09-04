use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a continuation token (`ContinuationItem.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContinuationNode {
    pub token: String,
    pub endpoint_type: Option<String>,
}

impl ContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("continuationItemRenderer")
            .or_else(|| val.get("continuationItemViewModel"))
            .unwrap_or(val);

        let token = target.pointer("/continuationEndpoint/continuationCommand/token")
            .or_else(|| target.pointer("/continuationEndpoint/command/token"))
            .or_else(|| target.pointer("/continuationEndpoint/browseContinuationEndpoint/continuation"))
            .or_else(|| target.pointer("/continuationEndpoint/nextContinuationData/continuation"))
            .or_else(|| target.pointer("/continuationEndpoint/searchContinuationEndpoint/continuation"))
            .and_then(|t| t.as_str())?
            .to_string();

        let endpoint_type = target.pointer("/continuationEndpoint/continuationCommand/request")
            .and_then(|r| r.as_str())
            .map(|s| s.to_string());

        Some(Self {
            token,
            endpoint_type,
        })
    }
}

/// Extract a continuation token from the first entry of a `continuations`
/// array. `nextRadioContinuationData` only applies to playlist panels and
/// `reloadContinuationData` only to section/music-shelf lists (legacy
/// per-class rules).
fn first_continuation_token(
    target: &Value,
    include_radio: bool,
    include_reload: bool,
) -> Option<String> {
    let first = target.get("continuations")?.as_array()?.first()?;
    let token = first
        .pointer("/nextContinuationData/continuation")
        .or_else(|| {
            include_radio
                .then(|| first.pointer("/nextRadioContinuationData/continuation"))
                .flatten()
        })
        .or_else(|| {
            include_reload
                .then(|| first.pointer("/reloadContinuationData/continuation"))
                .flatten()
        })
        .and_then(Value::as_str)?;
    Some(token.to_string())
}

/// `sectionListContinuation` wrapper (legacy `SectionListContinuation`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionListContinuationNode {
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
}

impl SectionListContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sectionListContinuation")?;
        Some(Self {
            contents: node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation: first_continuation_token(node, false, true),
        })
    }
}

/// `itemSectionContinuation` wrapper (legacy `ItemSectionContinuation`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionContinuationNode {
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
}

impl ItemSectionContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("itemSectionContinuation")?;
        Some(Self {
            contents: node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation: first_continuation_token(node, false, false),
        })
    }
}

/// `gridContinuation` wrapper (legacy `GridContinuation`; legacy `contents`
/// getter aliases `items`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridContinuationNode {
    pub items: Vec<Value>,
    pub continuation: Option<String>,
}

impl GridContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridContinuation")?;
        Some(Self {
            items: node.get("items").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation: first_continuation_token(node, false, false),
        })
    }
}

/// `musicShelfContinuation` wrapper (legacy `MusicShelfContinuation`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicShelfContinuationNode {
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
}

impl MusicShelfContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicShelfContinuation")?;
        Some(Self {
            contents: node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation: first_continuation_token(node, false, true),
        })
    }
}

/// `musicPlaylistShelfContinuation` wrapper (legacy
/// `MusicPlaylistShelfContinuation`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlaylistShelfContinuationNode {
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
}

impl MusicPlaylistShelfContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicPlaylistShelfContinuation")?;
        Some(Self {
            contents: node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation: first_continuation_token(node, false, false),
        })
    }
}

/// `playlistPanelContinuation` wrapper (legacy `PlaylistPanelContinuation`;
/// falls back to `nextRadioContinuationData`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelContinuationNode {
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
    /// True when the continuation came from `nextRadioContinuationData`.
    pub is_radio: bool,
}

impl PlaylistPanelContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistPanelContinuation")?;
        let continuation = first_continuation_token(node, true, false);
        let is_radio = continuation.is_some()
            && node
                .pointer("/continuations/0/nextContinuationData/continuation")
                .is_none();
        Some(Self {
            contents: node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation,
            is_radio,
        })
    }
}

/// `reloadContinuationItemsCommand` (legacy `ReloadContinuationItemsCommand`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReloadContinuationItemsCommandNode {
    pub target_id: Option<String>,
    pub slot: Option<String>,
    /// Legacy reads `continuationItems` (kept under the `contents` name for
    /// consistency with sibling wrappers).
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
}

impl ReloadContinuationItemsCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("reloadContinuationItemsCommand")?;
        Some(Self {
            target_id: node.get("targetId").and_then(Value::as_str).map(ToString::to_string),
            slot: node.get("slot").and_then(Value::as_str).map(ToString::to_string),
            contents: node
                .get("continuationItems")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            continuation: node
                .get("continuation")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}

/// `showMiniplayerCommand` (legacy `ShowMiniplayerCommand` in
/// parser/continuations.ts).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowMiniplayerCommandNode {
    pub show_miniplayer: Option<bool>,
}

impl ShowMiniplayerCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showMiniplayerCommand")?;
        Some(Self {
            show_miniplayer: node.get("showMiniplayer").and_then(Value::as_bool),
        })
    }
}

/// Typed live chat continuation token (legacy `Continuation` for live chat):
/// timed, invalidation, or replay variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatContinuationToken {
    pub token: String,
    /// `timed`, `invalidation`, or `replay`.
    pub continuation_type: String,
    pub timeout_ms: Option<u64>,
}

/// `liveChatContinuation` (legacy `LiveChatContinuation`): chat actions (with
/// `clickTrackingParams` stripped, per legacy) plus the next continuation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatContinuationNode {
    pub actions: Vec<Value>,
    pub continuation: Option<LiveChatContinuationToken>,
    pub viewer_name: Option<String>,
}

/// Legacy strips `clickTrackingParams` from live chat actions.
fn strip_click_tracking_params(actions: &[Value]) -> Vec<Value> {
    actions
        .iter()
        .map(|action| match action {
            Value::Object(map) => {
                let mut map = map.clone();
                map.remove("clickTrackingParams");
                Value::Object(map)
            }
            other => other.clone(),
        })
        .collect()
}

impl LiveChatContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatContinuation")?;
        let first = node
            .get("continuations")
            .and_then(Value::as_array)
            .and_then(|arr| arr.first());

        let continuation = first.and_then(|first| {
            for (key, kind) in [
                ("timedContinuationData", "timed"),
                ("invalidationContinuationData", "invalidation"),
                ("liveChatReplayContinuationData", "replay"),
            ] {
                if let Some(data) = first.get(key) {
                    let token = data.get("continuation").and_then(Value::as_str)?;
                    return Some(LiveChatContinuationToken {
                        token: token.to_string(),
                        continuation_type: kind.to_string(),
                        timeout_ms: data.get("timeoutMs").and_then(Value::as_u64),
                    });
                }
            }
            None
        });

        Some(Self {
            actions: node
                .get("actions")
                .and_then(Value::as_array)
                .map(|arr| strip_click_tracking_params(arr))
                .unwrap_or_default(),
            continuation,
            viewer_name: node
                .get("viewerName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}
