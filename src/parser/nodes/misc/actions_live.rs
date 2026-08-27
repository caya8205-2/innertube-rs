use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed ReplaceLiveChatAction AST node (`replaceLiveChatAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceLiveChatActionNode {
    pub to_replace: Option<String>,
    pub replacement: Option<Value>,
}

impl ReplaceLiveChatActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("replaceLiveChatAction").unwrap_or(val);
        Some(Self {
            to_replace: node
                .get("toReplace")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            replacement: node.get("replacement").cloned(),
        })
    }
}

/// Strongly typed UpdateDateTextAction AST node (`updateDateTextAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDateTextActionNode {
    pub date_text: Option<String>,
}

impl UpdateDateTextActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateDateTextAction").unwrap_or(val);
        Some(Self {
            date_text: node
                .get("dateText")
                .and_then(TextNode::from_value)
                .map(|t| t.text),
        })
    }
}

/// Strongly typed UpdateDescriptionAction AST node (`updateDescriptionAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDescriptionActionNode {
    pub description: Option<TextNode>,
}

impl UpdateDescriptionActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateDescriptionAction").unwrap_or(val);
        Some(Self {
            description: node.get("description").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed UpdateTitleAction AST node (`updateTitleAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTitleActionNode {
    pub title: Option<TextNode>,
}

impl UpdateTitleActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateTitleAction").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed UpdateToggleButtonTextAction AST node (`updateToggleButtonTextAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateToggleButtonTextActionNode {
    pub default_text: Option<String>,
    pub toggled_text: Option<String>,
    pub button_id: Option<String>,
}

impl UpdateToggleButtonTextActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateToggleButtonTextAction").unwrap_or(val);
        Some(Self {
            default_text: node
                .get("defaultText")
                .and_then(TextNode::from_value)
                .map(|t| t.text),
            toggled_text: node
                .get("toggledText")
                .and_then(TextNode::from_value)
                .map(|t| t.text),
            button_id: node
                .get("buttonId")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
    }
}

/// Strongly typed UpdateViewershipAction AST node (`updateViewershipAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateViewershipActionNode {
    pub view_count_node: Option<Value>,
}

impl UpdateViewershipActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateViewershipAction").unwrap_or(val);
        Some(Self {
            view_count_node: node.get("viewCount").cloned(),
        })
    }
}

/// Strongly typed BumperUserEduContentView AST node (`bumperUserEduContentView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BumperUserEduContentViewNode {
    pub text: Option<TextNode>,
    pub image_name: Option<String>,
    pub image_color: Option<u64>,
}

impl BumperUserEduContentViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("bumperUserEduContentView").unwrap_or(val);
        
        let mut image_name = None;
        let mut image_color = None;
        
        if let Some(sources) = node.get("image").and_then(|i| i.get("sources")).and_then(|s| s.as_array()) {
            if let Some(first_source) = sources.first() {
                if let Some(client_resource) = first_source.get("clientResource") {
                    image_name = client_resource
                        .get("imageName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    image_color = client_resource
                        .get("imageColor")
                        .and_then(|v| v.as_u64());
                }
            }
        }
        
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            image_name,
            image_color,
        })
    }
}

/// Strongly typed PdgReplyButtonView AST node (`pdgReplyButtonView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdgReplyButtonViewNode {
    pub reply_button: Option<Value>,
    pub reply_count_entity_key: Option<String>,
    pub reply_count_placeholder: Option<TextNode>,
}

impl PdgReplyButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pdgReplyButtonView").unwrap_or(val);
        Some(Self {
            reply_button: node.get("replyButton").cloned(),
            reply_count_entity_key: node
                .get("replyCountEntityKey")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            reply_count_placeholder: node
                .get("replyCountPlaceholder")
                .and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollaborationFormDataNode {
    pub collaborator_channel_ids: Option<Vec<String>>,
    pub is_allow_new_collaborators_enabled: Option<bool>,
    pub is_collaboration_enabled: Option<bool>,
    pub is_invite_collaborators_button_enabled: Option<bool>,
}

/// Strongly typed PlaylistCollaborationFormSchema AST node (`playlistCollaborationFormSchema`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollaborationFormSchemaNode {
    pub id: Option<String>,
    pub initial_values: Option<PlaylistCollaborationFormDataNode>,
}

impl PlaylistCollaborationFormSchemaNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistCollaborationFormSchema").unwrap_or(val);
        
        let initial_values = node.get("initialValues").map(|iv| {
            PlaylistCollaborationFormDataNode {
                collaborator_channel_ids: iv
                    .get("collaboratorChannelIds")
                    .and_then(|c| c.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    }),
                is_allow_new_collaborators_enabled: iv
                    .get("isAllowNewCollaboratorsEnabled")
                    .and_then(|v| v.as_bool()),
                is_collaboration_enabled: iv
                    .get("isCollaborationEnabled")
                    .and_then(|v| v.as_bool()),
                is_invite_collaborators_button_enabled: iv
                    .get("isInviteCollaboratorsButtonEnabled")
                    .and_then(|v| v.as_bool()),
            }
        });
        
        Some(Self {
            id: node.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            initial_values,
        })
    }
}

/// Strongly typed PlaylistCollaborationViewModelPlaylistCollaboratorData AST node (`playlistCollaborationViewModelPlaylistCollaboratorData`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollaborationViewModelPlaylistCollaboratorDataNode {
    pub remove_collaborator_confirmation_dialog: Option<Value>,
    pub external_channel_id: Option<String>,
    pub collaborator_content_list_item: Option<Value>,
}

impl PlaylistCollaborationViewModelPlaylistCollaboratorDataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playlistCollaborationViewModelPlaylistCollaboratorData")
            .unwrap_or(val);
        Some(Self {
            remove_collaborator_confirmation_dialog: node
                .get("removeCollaboratorConfirmationDialog")
                .cloned(),
            external_channel_id: node
                .get("externalChannelId")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            collaborator_content_list_item: node.get("collaboratorContentListItem").cloned(),
        })
    }
}

/// Strongly typed SubscriptionButton AST node (`subscriptionButton`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionButtonNode {
    pub text: Option<TextNode>,
    pub subscribed: Option<bool>,
    pub subscription_type: Option<String>,
}

impl SubscriptionButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("subscriptionButton").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            subscribed: node.get("subscribed").and_then(|v| v.as_bool()),
            subscription_type: node
                .get("type")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        })
    }
}

/// Strongly typed CommandContext AST node (`commandContext`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandContextNode {
    pub on_focus: Option<Value>,
    pub on_hidden: Option<Value>,
    pub on_touch_end: Option<Value>,
    pub on_touch_move: Option<Value>,
    pub on_long_press: Option<Value>,
    pub on_tap: Option<Value>,
    pub on_touch_start: Option<Value>,
    pub on_visible: Option<Value>,
    pub on_first_visible: Option<Value>,
    pub on_hover: Option<Value>,
}

impl CommandContextNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commandContext").unwrap_or(val);
        Some(Self {
            on_focus: node.get("onFocus").cloned(),
            on_hidden: node.get("onHidden").cloned(),
            on_touch_end: node.get("onTouchEnd").cloned(),
            on_touch_move: node.get("onTouchMove").cloned(),
            on_long_press: node.get("onLongPress").cloned(),
            on_tap: node.get("onTap").cloned(),
            on_touch_start: node.get("onTouchStart").cloned(),
            on_visible: node.get("onVisible").cloned(),
            on_first_visible: node.get("onFirstVisible").cloned(),
            on_hover: node.get("onHover").cloned(),
        })
    }
}
