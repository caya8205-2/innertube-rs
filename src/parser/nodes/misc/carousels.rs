use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed CarouselHeader AST node (`carouselHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselHeaderNode {
    pub contents: Vec<Value>,
}

impl CarouselHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("carouselHeaderRenderer").unwrap_or(val);
        Some(Self {
            contents: node
                .get("contents")
                .and_then(|v| v.as_array())
                .map(|arr| arr.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed CarouselItem AST node (`carouselItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselItemNode {
    #[serde(rename = "carouselItems")]
    pub items: Vec<Value>,
    pub background_color: Option<String>,
    pub layout_style: Option<String>,
    pub pagination_thumbnails: Option<ThumbnailListNode>,
    pub paginator_alignment: Option<String>,
}

impl CarouselItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("carouselItemRenderer").unwrap_or(val);
        Some(Self {
            items: node
                .get("carouselItems")
                .and_then(|v| v.as_array())
                .map(|arr| arr.to_vec())
                .unwrap_or_default(),
            background_color: node
                .get("backgroundColor")
                .and_then(|v| v.as_str().map(String::from).or_else(|| v.as_u64().map(|num| num.to_string()))),
            layout_style: node.get("layoutStyle").and_then(|v| v.as_str()).map(String::from),
            pagination_thumbnails: node.get("paginationThumbnails").map(ThumbnailListNode::from_value),
            paginator_alignment: node.get("paginatorAlignment").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed CarouselItemView AST node (`carouselItemView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselItemViewNode {
    pub item_type: Option<String>,
    pub carousel_item: Option<Value>,
}

impl CarouselItemViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("carouselItemView").unwrap_or(val);
        Some(Self {
            item_type: node.get("itemType").and_then(|v| v.as_str()).map(String::from),
            carousel_item: node.get("carouselItem").cloned(),
        })
    }
}

/// Strongly typed CarouselLockup AST node (`carouselLockupRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselLockupNode {
    pub info_rows: Vec<Value>,
    pub video_lockup: Option<Value>,
}

impl CarouselLockupNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("carouselLockupRenderer").unwrap_or(val);
        Some(Self {
            info_rows: node
                .get("infoRows")
                .and_then(|v| v.as_array())
                .map(|arr| arr.to_vec())
                .unwrap_or_default(),
            video_lockup: node.get("videoLockup").cloned(),
        })
    }
}

/// Strongly typed CarouselTitleView AST node (`carouselTitleView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselTitleViewNode {
    pub title: Option<String>,
    pub previous_button: Option<Value>,
    pub next_button: Option<Value>,
}

impl CarouselTitleViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("carouselTitleView").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(|v| v.as_str()).map(String::from),
            previous_button: node.get("previousButton").cloned(),
            next_button: node.get("nextButton").cloned(),
        })
    }
}

/// Strongly typed ChipBarView AST node (`chipBarView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChipBarViewNode {
    pub chips: Vec<Value>,
    pub chip_bar_state_entity_key: Option<String>,
    pub renderer_context: Option<Value>,
}

impl ChipBarViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("chipBarView").unwrap_or(val);
        Some(Self {
            chips: node
                .get("chips")
                .and_then(|v| v.as_array())
                .map(|arr| arr.to_vec())
                .unwrap_or_default(),
            chip_bar_state_entity_key: node.get("chipBarStateEntityKey").and_then(|v| v.as_str()).map(String::from),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed ChipView AST node (`chipView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChipViewNode {
    pub accessibility_hint: Option<String>,
    pub accessibility_label: Option<String>,
    pub text: Option<String>,
    pub trailing_text: Option<String>,
    pub display_type: Option<String>,
    pub max_text_width: Option<f64>,
    pub secondary_accessibility_label: Option<String>,
    pub original_text: Option<String>,
    pub tap_command: Option<Value>,
    pub secondary_tap_command: Option<Value>,
    pub chip_entity_key: Option<String>,
    pub selected: bool,
}

impl ChipViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("chipView").unwrap_or(val);
        Some(Self {
            accessibility_hint: node.get("accessibilityHint").and_then(|v| v.as_str()).map(String::from),
            accessibility_label: node.get("accessibilityLabel").and_then(|v| v.as_str()).map(String::from),
            text: node.get("text").and_then(|v| v.as_str()).map(String::from),
            trailing_text: node.get("trailingText").and_then(|v| v.as_str()).map(String::from),
            display_type: node.get("displayType").and_then(|v| v.as_str()).map(String::from),
            max_text_width: node.get("maxTextWidth").and_then(|v| v.as_f64()),
            secondary_accessibility_label: node.get("secondaryAccessibilityLabel").and_then(|v| v.as_str()).map(String::from),
            original_text: node.get("originalText").and_then(|v| v.as_str()).map(String::from),
            tap_command: node.get("tapCommand").cloned(),
            secondary_tap_command: node.get("secondaryTapCommand").cloned(),
            chip_entity_key: node.get("chipEntityKey").and_then(|v| v.as_str()).map(String::from),
            selected: node.get("selected").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed ContentListItemView AST node (`contentListItemView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentListItemViewNode {
    pub title: Option<TextNode>,
    pub action_button: Option<Value>,
    pub avatar: Option<Value>,
    pub image: Option<ThumbnailListNode>,
    pub metadata: Option<Value>,
    pub renderer_context: Option<Value>,
}

impl ContentListItemViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("contentListItemView").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            action_button: node.get("actionButton").cloned(),
            avatar: node.get("avatar").cloned(),
            image: node.get("image").map(ThumbnailListNode::from_value),
            metadata: node.get("metadata").cloned(),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed BackgroundPromo AST node (`backgroundPromoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundPromoNode {
    pub body_text: Option<TextNode>,
    pub cta_button: Option<Value>,
    pub icon_type: Option<String>,
    pub title: Option<TextNode>,
}

impl BackgroundPromoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("backgroundPromoRenderer").unwrap_or(val);
        Some(Self {
            body_text: node.get("bodyText").and_then(TextNode::from_value),
            cta_button: node.get("ctaButton").cloned(),
            icon_type: node.get("icon").and_then(|v| v.get("iconType")).and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed AttributionView AST node (`attributionView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributionViewNode {
    pub text: Option<TextNode>,
    pub suffix: Option<TextNode>,
}

impl AttributionViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("attributionView").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            suffix: node.get("suffix").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed AvatarStackView AST node (`avatarStackView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarStackViewNode {
    pub avatars: Vec<Value>,
    pub text: Option<TextNode>,
    pub avatar_cluster_size: Option<String>,
    pub layout_type: Option<String>,
    pub renderer_context: Option<Value>,
}

impl AvatarStackViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("avatarStackView").unwrap_or(val);
        Some(Self {
            avatars: node
                .get("avatars")
                .and_then(|v| v.as_array())
                .map(|arr| arr.to_vec())
                .unwrap_or_default(),
            text: node.get("text").and_then(TextNode::from_value),
            avatar_cluster_size: node.get("avatarClusterSize").and_then(|v| v.as_str()).map(String::from),
            layout_type: node.get("layoutType").and_then(|v| v.as_str()).map(String::from),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed AnimatedThumbnailOverlayView AST node (`animatedThumbnailOverlayView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimatedThumbnailOverlayViewNode {
    pub thumbnail: Option<ThumbnailListNode>,
}

impl AnimatedThumbnailOverlayViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("animatedThumbnailOverlayView").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value),
        })
    }
}
