use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::misc::text::TextNode;

/// Strongly typed LiveChatItemList AST node (`liveChatItemListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatItemListNode {
    pub max_items_to_display: Option<String>,
    pub more_comments_below_button: Option<Value>,
}

impl LiveChatItemListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatItemListRenderer").unwrap_or(val);
        Some(Self {
            max_items_to_display: node
                .get("maxItemsToDisplay")
                .and_then(|v| v.as_str())
                .map(String::from),
            more_comments_below_button: node.get("moreCommentsBelowButton").cloned(),
        })
    }
}

/// Strongly typed LiveChatParticipantsList AST node (`liveChatParticipantsListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatParticipantsListNode {
    pub title: Option<TextNode>,
    pub participants: Vec<Value>,
}

impl LiveChatParticipantsListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatParticipantsListRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            participants: node
                .get("participants")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed LiveChatActionPanel AST node (`liveChatActionPanelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatActionPanelNode {
    pub id: Option<String>,
    pub contents: Option<Value>,
    pub target_id: Option<String>,
}

impl LiveChatActionPanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatActionPanelRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("id").and_then(|v| v.as_str()).map(String::from),
            contents: node.get("contents").cloned(),
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed AddBannerToLiveChatCommand AST node (`addBannerToLiveChatCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBannerToLiveChatCommandNode {
    pub banner: Option<Value>,
}

impl AddBannerToLiveChatCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("addBannerToLiveChatCommand").unwrap_or(val);
        Some(Self {
            banner: node.get("bannerRenderer").cloned(),
        })
    }
}

/// Strongly typed RemoveBannerForLiveChatCommand AST node (`removeBannerForLiveChatCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBannerForLiveChatCommandNode {
    pub target_action_id: Option<String>,
}

impl RemoveBannerForLiveChatCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("removeBannerForLiveChatCommand").unwrap_or(val);
        Some(Self {
            target_action_id: node
                .get("targetActionId")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed AddLiveChatTickerItemAction AST node (`addLiveChatTickerItemAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiveChatTickerItemActionNode {
    pub item: Option<Value>,
    pub duration_sec: Option<String>,
}

impl AddLiveChatTickerItemActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("addLiveChatTickerItemAction").unwrap_or(val);
        Some(Self {
            item: node.get("item").cloned(),
            duration_sec: node
                .get("durationSec")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed DimChatItemAction AST node (`dimChatItemAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimChatItemActionNode {
    pub client_assigned_id: Option<String>,
}

impl DimChatItemActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dimChatItemAction").unwrap_or(val);
        Some(Self {
            client_assigned_id: node
                .get("clientAssignedId")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed RemoveChatItemAction AST node (`removeChatItemAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveChatItemActionNode {
    pub target_item_id: Option<String>,
}

impl RemoveChatItemActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("removeChatItemAction").unwrap_or(val);
        Some(Self {
            target_item_id: node
                .get("targetItemId")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed RemoveChatItemByAuthorAction AST node (`removeChatItemByAuthorAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveChatItemByAuthorActionNode {
    pub external_channel_id: Option<String>,
}

impl RemoveChatItemByAuthorActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("removeChatItemByAuthorAction").unwrap_or(val);
        Some(Self {
            external_channel_id: node
                .get("externalChannelId")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed ReplaceChatItemAction AST node (`replaceChatItemAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceChatItemActionNode {
    pub target_item_id: Option<String>,
    pub replacement_item: Option<Value>,
}

impl ReplaceChatItemActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("replaceChatItemAction").unwrap_or(val);
        Some(Self {
            target_item_id: node
                .get("targetItemId")
                .and_then(|v| v.as_str())
                .map(String::from),
            replacement_item: node.get("replacementItem").cloned(),
        })
    }
}

/// Strongly typed ReplayChatItemAction AST node (`replayChatItemAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayChatItemActionNode {
    pub actions: Vec<Value>,
    pub video_offset_time_msec: Option<String>,
}

impl ReplayChatItemActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("replayChatItemAction").unwrap_or(val);
        Some(Self {
            actions: node
                .get("actions")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
            video_offset_time_msec: node
                .get("videoOffsetTimeMsec")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed UpdateLiveChatPollAction AST node (`updateLiveChatPollAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLiveChatPollActionNode {
    pub poll_to_update: Option<Value>,
}

impl UpdateLiveChatPollActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateLiveChatPollAction").unwrap_or(val);
        Some(Self {
            poll_to_update: node.get("pollToUpdate").cloned(),
        })
    }
}
