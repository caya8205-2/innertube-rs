use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseFeedActionsNode {
    pub contents: Option<Vec<Value>>,
}

impl BrowseFeedActionsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("browseFeedActionsRenderer").unwrap_or(val);
        Some(Self {
            contents: node.get("contents").and_then(|v| v.as_array().cloned()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonViewNode {
    pub icon_name: Option<String>,
    pub style: Option<String>,
    pub button_size: Option<String>,
    pub icon_image: Option<ThumbnailListNode>,
    pub custom_dark_theme_border_color: Option<f64>,
    pub title: Option<String>,
    pub target_id: Option<String>,
    pub enable_full_width_margins: Option<bool>,
    pub custom_font_color: Option<f64>,
    #[serde(rename = "type")]
    pub button_type: Option<String>,
    pub enabled: Option<bool>,
    pub accessibility_id: Option<String>,
    pub custom_background_color: Option<f64>,
    pub on_long_press: Option<Value>,
    pub title_formatted: Option<Value>,
    pub on_visible: Option<Value>,
    pub icon_trailing: Option<bool>,
    pub accessibility_text: Option<String>,
}

impl ButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("buttonView").unwrap_or(val);
        Some(Self {
            icon_name: node.get("iconName").and_then(|v| v.as_str().map(String::from)),
            style: node.get("style").and_then(|v| v.as_str().map(String::from)),
            button_size: node.get("buttonSize").and_then(|v| v.as_str().map(String::from)),
            icon_image: node.get("iconImage").map(ThumbnailListNode::from_value),
            custom_dark_theme_border_color: node.get("customDarkThemeBorderColor").and_then(|v| v.as_f64()),
            title: node.get("title").and_then(|v| v.as_str().map(String::from)),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
            enable_full_width_margins: node.get("enableFullWidthMargins").and_then(|v| v.as_bool()),
            custom_font_color: node.get("customFontColor").and_then(|v| v.as_f64()),
            button_type: node.get("type").and_then(|v| v.as_str().map(String::from)),
            enabled: node.get("enabled").and_then(|v| v.as_bool()),
            accessibility_id: node.get("accessibilityId").and_then(|v| v.as_str().map(String::from)),
            custom_background_color: node.get("customBackgroundColor").and_then(|v| v.as_f64()),
            on_long_press: node.get("onLongPress").cloned(),
            title_formatted: node.get("titleFormatted").cloned(),
            on_visible: node.get("onVisible").cloned(),
            icon_trailing: node.get("iconTrailing").and_then(|v| v.as_bool()),
            accessibility_text: node.get("accessibilityText").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipSectionNode {
    pub contents: Option<Vec<Value>>,
}

impl ClipSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("clipSectionRenderer").unwrap_or(val);
        Some(Self {
            contents: node.get("contents").and_then(|v| v.as_array().cloned()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataPart {
    pub text: Option<TextNode>,
    pub avatar_stack: Option<Value>,
    pub enable_truncation: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRow {
    pub metadata_parts: Option<Vec<MetadataPart>>,
    pub badges: Option<Vec<Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMetadataViewNode {
    pub metadata_rows: Option<Vec<MetadataRow>>,
    pub delimiter: Option<String>,
}

impl ContentMetadataViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("contentMetadataView").unwrap_or(val);
        
        let metadata_rows = node.get("metadataRows").and_then(|rows| rows.as_array()).map(|rows| {
            rows.iter().map(|row| {
                MetadataRow {
                    metadata_parts: row.get("metadataParts").and_then(|parts| parts.as_array()).map(|parts| {
                        parts.iter().map(|part| {
                            MetadataPart {
                                text: part.get("text").and_then(TextNode::from_value),
                                avatar_stack: part.get("avatarStack").cloned(),
                                enable_truncation: node.get("enableTruncation").and_then(|v| v.as_bool()),
                            }
                        }).collect()
                    }),
                    badges: row.get("badges").and_then(|v| v.as_array().cloned()),
                }
            }).collect()
        });

        Some(Self {
            metadata_rows,
            delimiter: node.get("delimiter").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentPreviewImageViewNode {
    pub image: Option<ThumbnailListNode>,
    pub style: Option<String>,
}

impl ContentPreviewImageViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("contentPreviewImageView").unwrap_or(val);
        Some(Self {
            image: node.get("image").map(ThumbnailListNode::from_value),
            style: node.get("style").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemNode {
    pub trigger: Option<String>,
    pub button: Option<Value>,
    pub endpoint: Option<Value>,
}

impl ContinuationItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("continuationItemRenderer").unwrap_or(val);
        Some(Self {
            trigger: node.get("trigger").and_then(|v| v.as_str().map(String::from)),
            button: node.get("button").cloned(),
            endpoint: node.get("continuationEndpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationItemViewNode {
    pub trigger: Option<String>,
    pub endpoint: Option<Value>,
}

impl ContinuationItemViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("continuationItemView").unwrap_or(val);
        Some(Self {
            trigger: node.get("trigger").and_then(|v| v.as_str().map(String::from)),
            endpoint: node.get("continuationCommand").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationBarNode {
    pub availability_message: Option<Value>,
}

impl ConversationBarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("conversationBarRenderer").unwrap_or(val);
        Some(Self {
            availability_message: node.get("availabilityMessage").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyLinkNode {
    pub copy_button: Option<Value>,
    pub short_url: Option<String>,
    pub style: Option<String>,
}

impl CopyLinkNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("copyLinkRenderer").unwrap_or(val);
        Some(Self {
            copy_button: node.get("copyButton").cloned(),
            short_url: node.get("shortUrl").and_then(|v| v.as_str().map(String::from)),
            style: node.get("style").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaylistDialogNode {
    #[serde(rename = "dialogTitle")]
    pub title: Option<TextNode>,
    pub title_placeholder: Option<String>,
    pub privacy_option: Option<Value>,
    pub cancel_button: Option<Value>,
    pub create_button: Option<Value>,
}

impl CreatePlaylistDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("createPlaylistDialogRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("dialogTitle").and_then(TextNode::from_value),
            title_placeholder: node.get("titlePlaceholder").and_then(|v| v.as_str().map(String::from)),
            privacy_option: node.get("privacyOption").cloned(),
            cancel_button: node.get("cancelButton").cloned(),
            create_button: node.get("createButton").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaylistDialogFormViewNode {
    pub playlist_title: Option<Value>,
    pub playlist_visibility: Option<Value>,
    pub disable_playlist_collaborate: bool,
    pub create_playlist_params_collaboration_enabled: Option<String>,
    pub create_playlist_params_collaboration_disabled: Option<String>,
    pub video_ids: Option<Vec<String>>,
}

impl CreatePlaylistDialogFormViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("createPlaylistDialogFormView").unwrap_or(val);
        Some(Self {
            playlist_title: node.get("playlistTitle").cloned(),
            playlist_visibility: node.get("playlistVisibility").cloned(),
            disable_playlist_collaborate: node.get("disablePlaylistCollaborate").and_then(|v| v.as_bool()).unwrap_or(false),
            create_playlist_params_collaboration_enabled: node.get("createPlaylistParamsCollaborationEnabled").and_then(|v| v.as_str().map(String::from)),
            create_playlist_params_collaboration_disabled: node.get("createPlaylistParamsCollaborationDisabled").and_then(|v| v.as_str().map(String::from)),
            video_ids: node.get("videoIds").and_then(|v| {
                v.as_array().map(|arr| {
                    arr.iter().filter_map(|i| i.as_str().map(String::from)).collect()
                })
            }),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecoratedAvatarViewNode {
    pub avatar: Option<Value>,
    pub a11y_label: Option<String>,
    pub renderer_context: Option<Value>,
}

impl DecoratedAvatarViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("decoratedAvatarView").unwrap_or(val);
        Some(Self {
            avatar: node.get("avatar").cloned(),
            a11y_label: node.get("a11yLabel").and_then(|v| v.as_str().map(String::from)),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}
