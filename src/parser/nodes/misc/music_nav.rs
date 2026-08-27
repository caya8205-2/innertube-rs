use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed MusicResponsiveListItemFixedColumn AST node (`musicResponsiveListItemFixedColumnRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicResponsiveListItemFixedColumnNode {
    pub title: Option<TextNode>,
    pub display_priority: Option<String>,
}

impl MusicResponsiveListItemFixedColumnNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicResponsiveListItemFixedColumnRenderer")
            .or_else(|| val.get("musicResponsiveListItemFlexColumnRenderer"))
            .unwrap_or(val);
        Some(Self {
            title: node.get("text").and_then(TextNode::from_value),
            display_priority: node.get("displayPriority").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed MusicResponsiveListItemFlexColumn AST node (`musicResponsiveListItemFlexColumnRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicResponsiveListItemFlexColumnNode {
    pub title: Option<TextNode>,
    pub display_priority: Option<String>,
}

impl MusicResponsiveListItemFlexColumnNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicResponsiveListItemFlexColumnRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("text").and_then(TextNode::from_value),
            display_priority: node.get("displayPriority").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed MusicTastebuilderShelf AST node (`musicTastebuilderShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicTastebuilderShelfNode {
    pub thumbnail: Option<Value>,
    pub primary_text: Option<TextNode>,
    pub secondary_text: Option<TextNode>,
    pub action_button: Option<Value>,
    pub is_visible: bool,
}

impl MusicTastebuilderShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicTastebuilderShelfRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").cloned(),
            primary_text: node.get("primaryText").and_then(TextNode::from_value),
            secondary_text: node.get("secondaryText").and_then(TextNode::from_value),
            action_button: node.get("actionButton").cloned(),
            is_visible: node.get("isVisible").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed MusicTastebuilderShelfThumbnail AST node (`musicTastebuilderShelfThumbnailRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicTastebuilderShelfThumbnailNode {
    pub thumbnail: ThumbnailListNode,
}

impl MusicTastebuilderShelfThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicTastebuilderShelfThumbnailRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
        })
    }
}

/// Strongly typed NotificationAction AST node (`notificationActionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationActionNode {
    pub response_text: Option<TextNode>,
}

impl NotificationActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("notificationActionRenderer").unwrap_or(val);
        Some(Self {
            response_text: node.get("responseText").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed OpenOnePickAddVideoModalCommand AST node (`openOnePickAddVideoModalCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOnePickAddVideoModalCommandNode {
    pub list_id: Option<String>,
    pub modal_title: Option<String>,
    pub select_button_label: Option<String>,
}

impl OpenOnePickAddVideoModalCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("openOnePickAddVideoModalCommand").unwrap_or(val);
        Some(Self {
            list_id: node.get("listId").and_then(|v| v.as_str().map(String::from)),
            modal_title: node.get("modalTitle").and_then(|v| v.as_str().map(String::from)),
            select_button_label: node.get("selectButtonLabel").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed PageHeader AST node (`pageHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageHeaderNode {
    pub page_title: Option<String>,
    pub content: Option<Value>,
}

impl PageHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pageHeaderRenderer").unwrap_or(val);
        Some(Self {
            page_title: node.get("pageTitle").and_then(|v| v.as_str().map(String::from)),
            content: node.get("content").cloned(),
        })
    }
}

/// Strongly typed PageHeaderView AST node (`pageHeaderView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageHeaderViewNode {
    pub title: Option<Value>,
    pub image: Option<Value>,
    pub animated_image: Option<Value>,
    pub hero_image: Option<Value>,
    pub metadata: Option<Value>,
    pub actions: Option<Value>,
    pub description: Option<Value>,
    pub attributation: Option<Value>,
    pub banner: Option<Value>,
}

impl PageHeaderViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pageHeaderView")
            .or_else(|| val.get("pageHeaderViewModel"))
            .unwrap_or(val);
        Some(Self {
            title: node.get("title").cloned(),
            image: node.get("image").cloned(),
            animated_image: node.get("animatedImage").cloned(),
            hero_image: node.get("heroImage").cloned(),
            metadata: node.get("metadata").cloned(),
            actions: node.get("actions").cloned(),
            description: node.get("description").cloned(),
            attributation: node.get("attributation").cloned(),
            banner: node.get("banner").cloned(),
        })
    }
}

/// Strongly typed PageIndicatorView AST node (`pageIndicatorView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageIndicatorViewNode {
    pub indicator_count: Option<u64>,
    pub selected_index: Option<u64>,
}

impl PageIndicatorViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pageIndicatorView")
            .or_else(|| val.get("pageIndicatorViewModel"))
            .unwrap_or(val);
        Some(Self {
            indicator_count: node.get("indicatorCount").and_then(|v| v.as_u64()),
            selected_index: node.get("selectedIndex").and_then(|v| v.as_u64()),
        })
    }
}

/// Strongly typed PageIntroduction AST node (`pageIntroductionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageIntroductionNode {
    pub header_text: Option<String>,
    pub body_text: Option<String>,
    pub page_title: Option<String>,
    pub header_icon_type: Option<String>,
}

impl PageIntroductionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pageIntroductionRenderer").unwrap_or(val);
        Some(Self {
            header_text: node.get("headerText")
                .and_then(|v| v.get("simpleText").or_else(|| v.pointer("/runs/0/text")))
                .and_then(|v| v.as_str().map(String::from)),
            body_text: node.get("bodyText")
                .and_then(|v| v.get("simpleText").or_else(|| v.pointer("/runs/0/text")))
                .and_then(|v| v.as_str().map(String::from)),
            page_title: node.get("pageTitle")
                .and_then(|v| v.get("simpleText").or_else(|| v.pointer("/runs/0/text")))
                .and_then(|v| v.as_str().map(String::from)),
            header_icon_type: node.get("headerIcon")
                .and_then(|v| v.get("iconType"))
                .and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed PanelFooterView AST node (`panelFooterView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelFooterViewNode {
    pub primary_button: Option<Value>,
    pub secondary_button: Option<Value>,
    pub should_hide_divider: bool,
}

impl PanelFooterViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("panelFooterView")
            .or_else(|| val.get("panelFooterViewModel"))
            .unwrap_or(val);
        Some(Self {
            primary_button: node.get("primaryButton").cloned(),
            secondary_button: node.get("secondaryButton").cloned(),
            should_hide_divider: node.get("shouldHideDivider").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed PivotButton AST node (`pivotButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotButtonNode {
    pub thumbnail: ThumbnailListNode,
    pub endpoint: Option<Value>,
    pub content_description: Option<TextNode>,
    pub target_id: Option<String>,
    pub sound_attribution_title: Option<TextNode>,
    pub waveform_animation_style: Option<String>,
    pub background_animation_style: Option<String>,
}

impl PivotButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pivotButtonRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            endpoint: node.get("onClickCommand").cloned(),
            content_description: node.get("contentDescription").and_then(TextNode::from_value),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
            sound_attribution_title: node.get("soundAttributionTitle").and_then(TextNode::from_value),
            waveform_animation_style: node.get("waveformAnimationStyle").and_then(|v| v.as_str().map(String::from)),
            background_animation_style: node.get("backgroundAnimationStyle").and_then(|v| v.as_str().map(String::from)),
        })
    }
}
