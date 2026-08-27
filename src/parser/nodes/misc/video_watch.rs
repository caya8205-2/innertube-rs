use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed TextFieldView AST node (`textFieldView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextFieldViewNode {
    pub display_properties: Option<Value>,
    pub content_properties: Option<Value>,
    pub initial_state: Option<Value>,
    pub form_field_metadata: Option<Value>,
}

impl TextFieldViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("textFieldView").unwrap_or(val);
        Some(Self {
            display_properties: node.get("displayProperties").cloned(),
            content_properties: node.get("contentProperties").cloned(),
            initial_state: node.get("initialState").cloned(),
            form_field_metadata: node.get("formFieldMetadata").cloned(),
        })
    }
}

/// Strongly typed ThirdPartyShareTargetSection AST node (`thirdPartyShareTargetSection`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyShareTargetSectionNode {
    pub share_targets: Vec<Value>,
}

impl ThirdPartyShareTargetSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thirdPartyShareTargetSection").unwrap_or(val);
        Some(Self {
            share_targets: node
                .get("shareTargets")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed ThumbnailBadgeView AST node (`thumbnailBadgeView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailBadgeViewNode {
    pub text: Option<String>,
    pub badge_style: Option<String>,
    pub background_color: Option<Value>,
    pub icon_name: Option<String>,
}

impl ThumbnailBadgeViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailBadgeView").unwrap_or(val);
        let icon_name = node.get("icon")
            .and_then(|i| i.get("sources"))
            .and_then(|s| s.as_array())
            .and_then(|a| a.first())
            .and_then(|s| s.get("clientResource"))
            .and_then(|c| c.get("imageName"))
            .and_then(|n| n.as_str())
            .map(String::from);

        Some(Self {
            text: node.get("text").and_then(|t| t.as_str()).map(String::from),
            badge_style: node.get("badgeStyle").and_then(|t| t.as_str()).map(String::from),
            background_color: node.get("backgroundColor").cloned(),
            icon_name,
        })
    }
}

/// Strongly typed ThumbnailBottomOverlayView AST node (`thumbnailBottomOverlayView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailBottomOverlayViewNode {
    pub progress_bar: Option<Value>,
    pub badges: Vec<Value>,
}

impl ThumbnailBottomOverlayViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailBottomOverlayView").unwrap_or(val);
        Some(Self {
            progress_bar: node.get("progressBar").cloned(),
            badges: node
                .get("badges")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed ThumbnailHoverOverlayToggleActionsView AST node (`thumbnailHoverOverlayToggleActionsView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailHoverOverlayToggleActionsViewNode {
    pub buttons: Vec<Value>,
}

impl ThumbnailHoverOverlayToggleActionsViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailHoverOverlayToggleActionsView").unwrap_or(val);
        Some(Self {
            buttons: node
                .get("buttons")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed ThumbnailHoverOverlayView AST node (`thumbnailHoverOverlayView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailHoverOverlayViewNode {
    pub icon_name: Option<String>,
    pub text: Option<TextNode>,
    pub style: Option<String>,
}

impl ThumbnailHoverOverlayViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailHoverOverlayView").unwrap_or(val);
        let icon_name = node.get("icon")
            .and_then(|i| i.get("sources"))
            .and_then(|s| s.as_array())
            .and_then(|a| a.first())
            .and_then(|s| s.get("clientResource"))
            .and_then(|c| c.get("imageName"))
            .and_then(|n| n.as_str())
            .map(String::from);

        Some(Self {
            icon_name,
            text: node.get("text").and_then(TextNode::from_value),
            style: node.get("style").and_then(|s| s.as_str()).map(String::from),
        })
    }
}

/// Strongly typed ThumbnailLandscapePortrait AST node (`thumbnailLandscapePortrait`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailLandscapePortraitNode {
    pub landscape: ThumbnailListNode,
    pub portrait: ThumbnailListNode,
}

impl ThumbnailLandscapePortraitNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailLandscapePortrait").unwrap_or(val);
        Some(Self {
            landscape: ThumbnailListNode::from_value(node.get("landscape").unwrap_or(&Value::Null)),
            portrait: ThumbnailListNode::from_value(node.get("portrait").unwrap_or(&Value::Null)),
        })
    }
}

/// Strongly typed ThumbnailOverlayAvatarStackView AST node (`thumbnailOverlayAvatarStackView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayAvatarStackViewNode {
    pub avatar_stack: Option<Value>,
}

impl ThumbnailOverlayAvatarStackViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayAvatarStackView").unwrap_or(val);
        Some(Self {
            avatar_stack: node.get("avatarStack").cloned(),
        })
    }
}

/// Strongly typed ThumbnailOverlayBadgeView AST node (`thumbnailOverlayBadgeView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayBadgeViewNode {
    pub badges: Vec<Value>,
    pub position: Option<String>,
}

impl ThumbnailOverlayBadgeViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayBadgeView").unwrap_or(val);
        Some(Self {
            badges: node
                .get("thumbnailBadges")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
            position: node.get("position").and_then(|s| s.as_str()).map(String::from),
        })
    }
}

/// Strongly typed ThumbnailOverlayPinking AST node (`thumbnailOverlayPinking`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayPinkingNode {
    pub hack: bool,
}

impl ThumbnailOverlayPinkingNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayPinking").unwrap_or(val);
        Some(Self {
            hack: node.get("hack").and_then(|h| h.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed ThumbnailOverlayPlaybackStatus AST node (`thumbnailOverlayPlaybackStatus`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayPlaybackStatusNode {
    pub texts: Vec<TextNode>,
}

impl ThumbnailOverlayPlaybackStatusNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayPlaybackStatus").unwrap_or(val);
        let texts = node
            .get("texts")
            .and_then(|t| t.as_array())
            .map(|arr| arr.iter().filter_map(TextNode::from_value).collect())
            .unwrap_or_default();

        Some(Self { texts })
    }
}

/// Strongly typed ThumbnailOverlayProgressBarView AST node (`thumbnailOverlayProgressBarView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayProgressBarViewNode {
    pub start_percent: Option<f64>,
}

impl ThumbnailOverlayProgressBarViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayProgressBarView").unwrap_or(val);
        Some(Self {
            start_percent: node.get("startPercent").and_then(|p| p.as_f64()),
        })
    }
}
