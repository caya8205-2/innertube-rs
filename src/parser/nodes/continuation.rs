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
/// array, accepting `nextContinuationData` and (for radio panels)
/// `nextRadioContinuationData`.
fn first_continuation_token(target: &Value, include_radio: bool) -> Option<String> {
    let first = target.get("continuations")?.as_array()?.first()?;
    let token = first
        .pointer("/nextContinuationData/continuation")
        .or_else(|| {
            include_radio.then_some(())
                .and_then(|_| first.pointer("/nextRadioContinuationData/continuation"))
        })
        .or_else(|| first.pointer("/reloadContinuationData/continuation"))
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
            continuation: first_continuation_token(node, false),
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
            continuation: first_continuation_token(node, false),
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
            continuation: first_continuation_token(node, false),
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
            continuation: first_continuation_token(node, false),
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
            continuation: first_continuation_token(node, false),
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
        let continuation = first_continuation_token(node, true);
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
    pub contents: Vec<Value>,
    pub continuation: Option<String>,
}

impl ReloadContinuationItemsCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("reloadContinuationItemsCommand")?;
        Some(Self {
            target_id: node.get("targetId").and_then(Value::as_str).map(ToString::to_string),
            slot: node.get("slot").and_then(Value::as_str).map(ToString::to_string),
            contents: node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation: node
                .get("continuation")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}

/// `liveChatContinuation` (legacy `LiveChatContinuation`): chat actions plus
/// the next continuation token across timed/invalidation/replay variants.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatContinuationNode {
    pub actions: Vec<Value>,
    pub continuation: Option<String>,
    pub viewer_name: Option<String>,
}

impl LiveChatContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatContinuation")?;
        let continuation = node
            .get("continuations")
            .and_then(Value::as_array)
            .and_then(|arr| arr.first())
            .and_then(|first| {
                first
                    .pointer("/timedContinuationData/continuation")
                    .or_else(|| first.pointer("/invalidationContinuationData/continuation"))
                    .or_else(|| first.pointer("/liveChatReplayContinuationData/continuation"))
            })
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            actions: node.get("actions").and_then(Value::as_array).cloned().unwrap_or_default(),
            continuation,
            viewer_name: node
                .get("viewerName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}
