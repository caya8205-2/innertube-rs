use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Strongly typed AppendContinuationItemsAction AST node (`AppendContinuationItemsAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendContinuationItemsActionNode {
    pub contents: Option<Vec<Value>>,
    pub target: Option<String>,
}

impl AppendContinuationItemsActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("appendContinuationItemsAction").unwrap_or(val);
        Some(Self {
            contents: node.get("continuationItems").and_then(|v| v.as_array()).cloned(),
            target: node.get("target").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

/// Strongly typed GetMultiPageMenuAction AST node (`GetMultiPageMenuAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiPageMenuActionNode {
    pub menu: Option<Value>,
}

impl GetMultiPageMenuActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("getMultiPageMenuAction").unwrap_or(val);
        Some(Self {
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed OpenPopupAction AST node (`OpenPopupAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPopupActionNode {
    pub popup: Option<Value>,
    pub popup_type: Option<String>,
}

impl OpenPopupActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("openPopupAction").unwrap_or(val);
        Some(Self {
            popup: node.get("popup").cloned(),
            popup_type: node.get("popupType").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

/// Strongly typed SendFeedbackAction AST node (`SendFeedbackAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendFeedbackActionNode {
    pub bucket: Option<String>,
}

impl SendFeedbackActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sendFeedbackAction").unwrap_or(val);
        Some(Self {
            bucket: node.get("bucket").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

/// Strongly typed SignalAction AST node (`SignalAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalActionNode {
    pub signal: Option<String>,
}

impl SignalActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("signalAction").unwrap_or(val);
        Some(Self {
            signal: node.get("signal").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

/// Strongly typed UpdateChannelSwitcherPageAction AST node (`UpdateChannelSwitcherPageAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChannelSwitcherPageActionNode {
    pub header: Option<Value>,
    pub contents: Option<Vec<Value>>,
}

impl UpdateChannelSwitcherPageActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateChannelSwitcherPageAction").unwrap_or(val);
        let page = node.get("page");
        Some(Self {
            header: page.and_then(|p| p.get("header")).cloned(),
            contents: page.and_then(|p| p.get("contents")).and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed UpdateSubscribeButtonAction AST node (`UpdateSubscribeButtonAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubscribeButtonActionNode {
    pub channel_id: Option<String>,
    pub subscribed: bool,
}

impl UpdateSubscribeButtonActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateSubscribeButtonAction").unwrap_or(val);
        Some(Self {
            channel_id: node.get("channelId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            subscribed: node.get("subscribed").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed AddToPlaylistCommand AST node (`AddToPlaylistCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylistCommandNode {
    pub open_miniplayer: bool,
    pub video_id: Option<String>,
    pub list_type: Option<String>,
    pub endpoint: Option<Value>,
    pub video_ids: Option<Vec<String>>,
}

impl AddToPlaylistCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("addToPlaylistCommand").unwrap_or(val);
        Some(Self {
            open_miniplayer: node.get("openMiniplayer").and_then(|v| v.as_bool()).unwrap_or(false),
            video_id: node.get("videoId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            list_type: node.get("listType").and_then(|v| v.as_str()).map(|s| s.to_string()),
            endpoint: node.get("onCreateListCommand").cloned(),
            video_ids: node.get("videoIds")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|i| i.as_str().map(|s| s.to_string())).collect()),
        })
    }
}

/// Strongly typed ContinuationCommand AST node (`ContinuationCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationCommandNode {
    pub request: Option<String>,
    pub token: Option<String>,
    pub form_data: Option<Value>,
}

impl ContinuationCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("continuationCommand").unwrap_or(val);
        Some(Self {
            request: node.get("request").and_then(|v| v.as_str()).map(|s| s.to_string()),
            token: node.get("token").and_then(|v| v.as_str()).map(|s| s.to_string()),
            form_data: node.get("formData").cloned(),
        })
    }
}

/// Strongly typed ShowSheetCommand AST node (`ShowSheetCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowSheetCommandNode {
    pub inline_content: Option<Value>,
    pub remove_default_padding: bool,
}

impl ShowSheetCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showSheetCommand").unwrap_or(val);
        Some(Self {
            inline_content: node.get("panelLoadingStrategy").and_then(|p| p.get("inlineContent")).cloned(),
            remove_default_padding: node.get("removeDefaultPadding").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub surface: Option<String>,
    pub tag: Option<String>,
}

impl Identifier {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            surface: val.get("surface").and_then(|v| v.as_str()).map(|s| s.to_string()),
            tag: val.get("tag").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

/// Strongly typed UpdateEngagementPanelContentCommand AST node (`UpdateEngagementPanelContentCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEngagementPanelContentCommandNode {
    pub content_source_panel_identifier: Option<Identifier>,
    pub target_panel_identifier: Option<Identifier>,
}

impl UpdateEngagementPanelContentCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateEngagementPanelContentCommand").unwrap_or(val);
        Some(Self {
            content_source_panel_identifier: node.get("contentSourcePanelIdentifier").and_then(Identifier::from_value),
            target_panel_identifier: node.get("targetPanelIdentifier").and_then(Identifier::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttIds {
    pub encrypted_video_id: Option<String>,
    pub external_channel_id: Option<String>,
    pub comment_id: Option<String>,
    pub external_owner_id: Option<String>,
    pub artist_id: Option<String>,
    pub playlist_id: Option<String>,
    pub external_post_id: Option<String>,
    pub share_id: Option<String>,
}

impl AttIds {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            encrypted_video_id: val.get("encryptedVideoId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            external_channel_id: val.get("externalChannelId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            comment_id: val.get("commentId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            external_owner_id: val.get("externalOwnerId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            artist_id: val.get("artistId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            playlist_id: val.get("playlistId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            external_post_id: val.get("externalPostId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            share_id: val.get("shareId").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

/// Strongly typed RunAttestationCommand AST node (`RunAttestationCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAttestationCommandNode {
    pub engagement_type: Option<String>,
    pub ids: Option<Vec<AttIds>>,
}

impl RunAttestationCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("runAttestationCommand").unwrap_or(val);
        Some(Self {
            engagement_type: node.get("engagementType").and_then(|v| v.as_str()).map(|s| s.to_string()),
            ids: node.get("ids").and_then(|v| v.as_array()).map(|arr| {
                arr.iter().filter_map(AttIds::from_value).collect()
            }),
        })
    }
}
