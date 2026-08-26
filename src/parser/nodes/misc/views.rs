use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed BadgeView AST node (`badgeView`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeViewNode {
    pub badge_text: String,
    pub badge_style: Option<String>,
    pub accessibility_label: Option<String>,
}

impl BadgeViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("badgeView").unwrap_or(val);

        let badge_text = node
            .get("badgeText")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        let badge_style = node.get("badgeStyle").and_then(Value::as_str).map(ToString::to_string);
        let accessibility_label = node
            .get("accessibilityText")
            .or_else(|| node.get("accessibilityLabel"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            badge_text,
            badge_style,
            accessibility_label,
        })
    }
}

/// Strongly typed CallToActionButton AST node (`callToActionButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallToActionButtonNode {
    pub label: String,
    pub icon_type: Option<String>,
    pub style: Option<String>,
    pub endpoint: Option<Value>,
}

impl CallToActionButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("callToActionButtonRenderer").unwrap_or(val);

        let label = node
            .get("label")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("label").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);
        let style = node.get("style").and_then(Value::as_str).map(ToString::to_string);
        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();

        Some(Self {
            label,
            icon_type,
            style,
            endpoint,
        })
    }
}

/// Strongly typed ButtonCardView AST node (`buttonCardView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonCardViewNode {
    pub title: String,
    pub icon_name: Option<String>,
    pub endpoint: Option<Value>,
}

impl ButtonCardViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("buttonCardView").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let icon_name = node.get("iconName").and_then(Value::as_str).map(ToString::to_string);
        let endpoint = node.get("onTap").or_else(|| node.get("endpoint")).cloned();

        Some(Self {
            title,
            icon_name,
            endpoint,
        })
    }
}

/// Strongly typed AvatarView AST node (`avatarView`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarViewNode {
    pub image: ThumbnailListNode,
    pub avatar_image_size: Option<String>,
}

impl AvatarViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("avatarView").unwrap_or(val);
        let image = ThumbnailListNode::from_value(node.get("image").unwrap_or(node));
        let avatar_image_size = node
            .get("avatarImageSize")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            image,
            avatar_image_size,
        })
    }
}

/// Strongly typed CompactLink AST node (`compactLinkRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactLinkNode {
    pub title: String,
    pub subtitle: Option<String>,
    pub icon_type: Option<String>,
    pub endpoint: Option<Value>,
    pub style: Option<String>,
}

impl CompactLinkNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactLinkRenderer").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let subtitle = node
            .get("subtitle")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);
        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("serviceEndpoint")).cloned();
        let style = node.get("style").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            title,
            subtitle,
            icon_type,
            endpoint,
            style,
        })
    }
}
