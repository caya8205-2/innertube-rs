use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed DynamicTextView AST node (`dynamicTextView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicTextViewNode {
    pub text: Option<TextNode>,
    pub max_lines: Option<f64>,
}

impl DynamicTextViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dynamicTextView").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            max_lines: node.get("maxLines").and_then(|v| {
                v.as_f64()
                    .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            }),
        })
    }
}

/// Strongly typed Element AST node (`elementRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementNode {
    pub model: Option<Value>,
    pub child_elements: Option<Vec<Value>>,
}

impl ElementNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("elementRenderer").unwrap_or(val);

        let new_element = node.get("newElement");
        let model = new_element
            .and_then(|ne| ne.get("type"))
            .and_then(|t| t.get("componentType"))
            .and_then(|ct| ct.get("model"))
            .cloned();

        let child_elements = new_element
            .and_then(|ne| ne.get("childElements"))
            .and_then(|ce| ce.as_array())
            .cloned();

        Some(Self {
            model,
            child_elements,
        })
    }
}

/// Strongly typed EmojiPickerCategory AST node (`emojiPickerCategoryRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiPickerCategoryNode {
    pub category_id: Option<String>,
    pub title: Option<TextNode>,
    pub emoji_ids: Option<Vec<String>>,
    pub image_loading_lazy: bool,
    pub category_type: Option<String>,
}

impl EmojiPickerCategoryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("emojiPickerCategoryRenderer").unwrap_or(val);

        let emoji_ids = node
            .get("emojiIds")
            .and_then(|ids| ids.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            });

        Some(Self {
            category_id: node.get("categoryId").and_then(|v| v.as_str().map(String::from)),
            title: node.get("title").and_then(TextNode::from_value),
            emoji_ids,
            image_loading_lazy: node
                .get("imageLoadingLazy")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            category_type: node
                .get("categoryType")
                .and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed EmojiPickerCategoryButton AST node (`emojiPickerCategoryButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiPickerCategoryButtonNode {
    pub category_id: Option<String>,
    pub icon_type: Option<String>,
    pub tooltip: Option<String>,
}

impl EmojiPickerCategoryButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("emojiPickerCategoryButtonRenderer").unwrap_or(val);
        Some(Self {
            category_id: node.get("categoryId").and_then(|v| v.as_str().map(String::from)),
            icon_type: node
                .get("icon")
                .and_then(|i| i.get("iconType"))
                .and_then(|v| v.as_str().map(String::from)),
            tooltip: node.get("tooltip").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed EmojiPickerUpsellCategory AST node (`emojiPickerUpsellCategoryRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiPickerUpsellCategoryNode {
    pub category_id: Option<String>,
    pub title: Option<TextNode>,
    pub upsell: Option<TextNode>,
    pub emoji_tooltip: Option<String>,
    pub endpoint: Option<Value>,
    pub emoji_ids: Option<Vec<String>>,
}

impl EmojiPickerUpsellCategoryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("emojiPickerUpsellCategoryRenderer").unwrap_or(val);

        let emoji_ids = node
            .get("emojiIds")
            .and_then(|ids| ids.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            });

        Some(Self {
            category_id: node.get("categoryId").and_then(|v| v.as_str().map(String::from)),
            title: node.get("title").and_then(TextNode::from_value),
            upsell: node.get("upsell").and_then(TextNode::from_value),
            emoji_tooltip: node
                .get("emojiTooltip")
                .and_then(|v| v.as_str().map(String::from)),
            endpoint: node.get("command").cloned(),
            emoji_ids,
        })
    }
}

/// Strongly typed EndScreenPlaylist AST node (`endScreenPlaylistRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndScreenPlaylistNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub author: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub video_count: Option<TextNode>,
}

impl EndScreenPlaylistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("endScreenPlaylistRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str().map(String::from)),
            title: node.get("title").and_then(TextNode::from_value),
            author: node
                .get("longBylineText")
                .and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            video_count: node
                .get("videoCountText")
                .and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed EomSettingsDisclaimer AST node (`eomSettingsDisclaimerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EomSettingsDisclaimerNode {
    pub disclaimer: Option<TextNode>,
    pub info_icon_type: Option<String>,
    pub usage_scenario: Option<String>,
}

impl EomSettingsDisclaimerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("eomSettingsDisclaimerRenderer").unwrap_or(val);
        Some(Self {
            disclaimer: node.get("disclaimer").and_then(TextNode::from_value),
            info_icon_type: node
                .get("infoIcon")
                .and_then(|i| i.get("iconType"))
                .and_then(|v| v.as_str().map(String::from)),
            usage_scenario: node
                .get("usageScenario")
                .and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandableMetadataHeader {
    pub collapsed_title: Option<TextNode>,
    pub collapsed_thumbnail: Option<ThumbnailListNode>,
    pub collapsed_label: Option<TextNode>,
    pub expanded_title: Option<TextNode>,
}

/// Strongly typed ExpandableMetadata AST node (`expandableMetadataRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandableMetadataNode {
    pub header: Option<ExpandableMetadataHeader>,
    pub expanded_content: Option<Value>,
    pub expand_button: Option<Value>,
    pub collapse_button: Option<Value>,
}

impl ExpandableMetadataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("expandableMetadataRenderer").unwrap_or(val);

        let header = node.get("header").map(|h| ExpandableMetadataHeader {
            collapsed_title: h.get("collapsedTitle").and_then(TextNode::from_value),
            collapsed_thumbnail: h
                .get("collapsedThumbnail")
                .map(ThumbnailListNode::from_value),
            collapsed_label: h.get("collapsedLabel").and_then(TextNode::from_value),
            expanded_title: h.get("expandedTitle").and_then(TextNode::from_value),
        });

        Some(Self {
            header,
            expanded_content: node.get("expandedContent").cloned(),
            expand_button: node.get("expandButton").cloned(),
            collapse_button: node.get("collapseButton").cloned(),
        })
    }
}

/// Strongly typed ExpandedShelfContents AST node (`expandedShelfContentsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedShelfContentsNode {
    pub items: Option<Vec<Value>>,
}

impl ExpandedShelfContentsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("expandedShelfContentsRenderer").unwrap_or(val);
        Some(Self {
            items: node.get("items").and_then(|i| i.as_array()).cloned(),
        })
    }
}

/// Strongly typed Factoid AST node (`factoidRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactoidNode {
    pub label: Option<TextNode>,
    pub value: Option<TextNode>,
    pub accessibility_text: Option<String>,
}

impl FactoidNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("factoidRenderer").unwrap_or(val);
        Some(Self {
            label: node.get("label").and_then(TextNode::from_value),
            value: node.get("value").and_then(TextNode::from_value),
            accessibility_text: node
                .get("accessibilityText")
                .and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed FancyDismissibleDialog AST node (`fancyDismissibleDialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FancyDismissibleDialogNode {
    pub dialog_message: Option<TextNode>,
    pub confirm_label: Option<TextNode>,
}

impl FancyDismissibleDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("fancyDismissibleDialogRenderer").unwrap_or(val);
        Some(Self {
            dialog_message: node.get("dialogMessage").and_then(TextNode::from_value),
            confirm_label: node.get("confirmLabel").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed FeedTabbedHeader AST node (`feedTabbedHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedTabbedHeaderNode {
    pub title: Option<TextNode>,
}

impl FeedTabbedHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("feedTabbedHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}
