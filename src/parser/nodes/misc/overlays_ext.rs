use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed ThumbnailOverlayHoverText AST node (`thumbnailOverlayHoverTextRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayHoverTextNode {
    pub text: Option<TextNode>,
    pub icon_type: Option<String>,
}

impl ThumbnailOverlayHoverTextNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayHoverTextRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            icon_type: node
                .get("icon")
                .and_then(|i| i.get("iconType"))
                .and_then(|t| t.as_str().map(String::from)),
        })
    }
}

/// Strongly typed ThumbnailOverlayEndorsement AST node (`thumbnailOverlayEndorsementRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayEndorsementNode {
    pub text: Option<String>,
}

impl ThumbnailOverlayEndorsementNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayEndorsementRenderer").unwrap_or(val);
        let text = node.get("text").and_then(TextNode::from_value).map(|t| t.to_string());
        Some(Self { text })
    }
}

/// Strongly typed ThumbnailOverlayNowPlaying AST node (`thumbnailOverlayNowPlayingRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayNowPlayingNode {
    pub text: Option<String>,
}

impl ThumbnailOverlayNowPlayingNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayNowPlayingRenderer").unwrap_or(val);
        let text = node.get("text").and_then(TextNode::from_value).map(|t| t.to_string());
        Some(Self { text })
    }
}

/// Strongly typed ThumbnailOverlayLoadingPreview AST node (`thumbnailOverlayLoadingPreviewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayLoadingPreviewNode {
    pub text: Option<TextNode>,
}

impl ThumbnailOverlayLoadingPreviewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayLoadingPreviewRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed ThumbnailOverlayInlineUnplayable AST node (`thumbnailOverlayInlineUnplayableRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayInlineUnplayableNode {
    pub text: Option<String>,
    pub icon_type: Option<String>,
}

impl ThumbnailOverlayInlineUnplayableNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayInlineUnplayableRenderer").unwrap_or(val);
        let text = node.get("text").and_then(TextNode::from_value).map(|t| t.to_string());
        Some(Self {
            text,
            icon_type: node
                .get("icon")
                .and_then(|i| i.get("iconType"))
                .and_then(|t| t.as_str().map(String::from)),
        })
    }
}

/// Strongly typed ThumbnailOverlayBottomPanel AST node (`thumbnailOverlayBottomPanelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayBottomPanelNode {
    pub text: Option<TextNode>,
    pub icon_type: Option<String>,
}

impl ThumbnailOverlayBottomPanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayBottomPanelRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            icon_type: node
                .get("icon")
                .and_then(|i| i.get("iconType"))
                .and_then(|t| t.as_str().map(String::from)),
        })
    }
}

/// Strongly typed ThumbnailOverlaySidePanel AST node (`thumbnailOverlaySidePanelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlaySidePanelNode {
    pub text: Option<TextNode>,
    pub icon_type: Option<String>,
}

impl ThumbnailOverlaySidePanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlaySidePanelRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            icon_type: node
                .get("icon")
                .and_then(|i| i.get("iconType"))
                .and_then(|t| t.as_str().map(String::from)),
        })
    }
}

/// Strongly typed ThumbnailOverlayToggleButton AST node (`thumbnailOverlayToggleButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayToggleButtonNode {
    pub is_toggled: Option<bool>,
    pub icon_type_toggled: Option<String>,
    pub icon_type_untoggled: Option<String>,
    pub tooltip_toggled: Option<String>,
    pub tooltip_untoggled: Option<String>,
    pub toggled_endpoint: Option<Value>,
    pub untoggled_endpoint: Option<Value>,
}

impl ThumbnailOverlayToggleButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayToggleButtonRenderer").unwrap_or(val);
        Some(Self {
            is_toggled: node.get("isToggled").and_then(|v| v.as_bool()),
            icon_type_toggled: node
                .get("toggledIcon")
                .and_then(|i| i.get("iconType"))
                .and_then(|v| v.as_str().map(String::from)),
            icon_type_untoggled: node
                .get("untoggledIcon")
                .and_then(|i| i.get("iconType"))
                .and_then(|v| v.as_str().map(String::from)),
            tooltip_toggled: node.get("toggledTooltip").and_then(|v| v.as_str().map(String::from)),
            tooltip_untoggled: node.get("untoggledTooltip").and_then(|v| v.as_str().map(String::from)),
            toggled_endpoint: node.get("toggledServiceEndpoint").cloned(),
            untoggled_endpoint: node.get("untoggledServiceEndpoint").cloned(),
        })
    }
}

/// Strongly typed DecoratedPlayerBar AST node (`decoratedPlayerBarRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecoratedPlayerBarNode {
    pub player_bar: Option<Value>,
    pub player_bar_action_button: Option<Value>,
}

impl DecoratedPlayerBarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("decoratedPlayerBarRenderer").unwrap_or(val);
        Some(Self {
            player_bar: node.get("playerBar").cloned(),
            player_bar_action_button: node.get("playerBarActionButton").cloned(),
        })
    }
}

/// Strongly typed ConfirmDialog AST node (`confirmDialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmDialogNode {
    pub title: Option<TextNode>,
    pub confirm_button: Option<Value>,
    pub cancel_button: Option<Value>,
    pub dialog_messages: Vec<TextNode>,
}

impl ConfirmDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("confirmDialogRenderer").unwrap_or(val);
        
        let dialog_messages = node
            .get("dialogMessages")
            .and_then(|arr| arr.as_array())
            .map(|arr| arr.iter().filter_map(TextNode::from_value).collect())
            .unwrap_or_default();

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            confirm_button: node.get("confirmButton").cloned(),
            cancel_button: node.get("cancelButton").cloned(),
            dialog_messages,
        })
    }
}

/// Strongly typed Dialog AST node (`dialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogNode {
    pub title: Option<TextNode>,
    pub content: Option<Value>,
}

impl DialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dialogRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            content: node.get("content").cloned(),
        })
    }
}

/// Strongly typed ModalWithTitleAndButton AST node (`modalWithTitleAndButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModalWithTitleAndButtonNode {
    pub title: Option<TextNode>,
    pub content: Option<TextNode>,
    pub button: Option<Value>,
}

impl ModalWithTitleAndButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("modalWithTitleAndButtonRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            content: node.get("content").and_then(TextNode::from_value),
            button: node.get("button").cloned(),
        })
    }
}
