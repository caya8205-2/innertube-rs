use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed ProductListHeader AST node (`productListHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductListHeaderNode {
    pub title: Option<TextNode>,
    pub suppress_padding_disclaimer: bool,
}

impl ProductListHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("productListHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            suppress_padding_disclaimer: node
                .get("suppressPaddingDisclaimer")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        })
    }
}

/// Strongly typed ProductListItem AST node (`productListItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductListItemNode {
    pub title: Option<TextNode>,
    pub accessibility_title: Option<String>,
    pub thumbnail: ThumbnailListNode,
    pub price: Option<String>,
    pub endpoint: Option<Value>,
    pub merchant_name: Option<String>,
    pub stay_in_app: bool,
    pub view_button: Option<Value>,
}

impl ProductListItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("productListItemRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            accessibility_title: node
                .get("accessibilityTitle")
                .and_then(|v| v.as_str())
                .map(String::from),
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            price: node.get("price").and_then(|v| v.as_str()).map(String::from),
            endpoint: node.get("onClickCommand").cloned(),
            merchant_name: node
                .get("merchantName")
                .and_then(|v| v.as_str())
                .map(String::from),
            stay_in_app: node.get("stayInApp").and_then(|v| v.as_bool()).unwrap_or(false),
            view_button: node.get("viewButton").cloned(),
        })
    }
}

/// Strongly typed ProfileColumnStats AST node (`profileColumnStatsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileColumnStatsNode {
    pub items: Vec<Value>,
}

impl ProfileColumnStatsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("profileColumnStatsRenderer").unwrap_or(val);
        Some(Self {
            items: node
                .get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed ProfileColumnStatsEntry AST node (`profileColumnStatsEntryRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileColumnStatsEntryNode {
    pub label: Option<TextNode>,
    pub value: Option<TextNode>,
}

impl ProfileColumnStatsEntryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("profileColumnStatsEntryRenderer").unwrap_or(val);
        Some(Self {
            label: node.get("label").and_then(TextNode::from_value),
            value: node.get("value").and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizChoice {
    pub text: Option<TextNode>,
    pub is_correct: bool,
}

/// Strongly typed Quiz AST node (`quizRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizNode {
    pub choices: Vec<QuizChoice>,
    pub total_votes: Option<TextNode>,
}

impl QuizNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("quizRenderer").unwrap_or(val);
        
        let choices = node
            .get("choices")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .map(|choice| QuizChoice {
                        text: choice.get("text").and_then(TextNode::from_value),
                        is_correct: choice
                            .get("isCorrect")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    })
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            choices,
            total_votes: node.get("totalVotes").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed RecognitionShelf AST node (`recognitionShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognitionShelfNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub avatars: Vec<ThumbnailListNode>,
    pub button: Option<Value>,
    pub surface: Option<String>,
}

impl RecognitionShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("recognitionShelfRenderer").unwrap_or(val);
        
        let avatars = node
            .get("avatars")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .map(ThumbnailListNode::from_value)
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            avatars,
            button: node.get("button").cloned(),
            surface: node.get("surface").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed RelatedChipCloud AST node (`relatedChipCloudRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedChipCloudNode {
    pub content: Option<Value>,
    pub show_prominent_chips: bool,
}

impl RelatedChipCloudNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("relatedChipCloudRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").cloned(),
            show_prominent_chips: node
                .get("showProminentChips")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        })
    }
}

/// Strongly typed RichListHeader AST node (`richListHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichListHeaderNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub title_style: Option<String>,
    pub icon_type: Option<String>,
}

impl RichListHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("richListHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            title_style: node
                .get("titleStyle")
                .and_then(|v| v.get("style"))
                .and_then(|v| v.as_str())
                .map(String::from),
            icon_type: node
                .get("icon")
                .and_then(|v| v.get("iconType"))
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed RichMetadata AST node (`richMetadataRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMetadataNode {
    pub thumbnail: ThumbnailListNode,
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub call_to_action: Option<TextNode>,
    pub icon_type: Option<String>,
    pub endpoint: Option<Value>,
}

impl RichMetadataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("richMetadataRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            call_to_action: node.get("callToAction").and_then(TextNode::from_value),
            icon_type: node
                .get("callToActionIcon")
                .and_then(|v| v.get("iconType"))
                .and_then(|v| v.as_str())
                .map(String::from),
            endpoint: node.get("endpoint").cloned(),
        })
    }
}

/// Strongly typed RichMetadataRow AST node (`richMetadataRowRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMetadataRowNode {
    pub contents: Vec<Value>,
}

impl RichMetadataRowNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("richMetadataRowRenderer").unwrap_or(val);
        Some(Self {
            contents: node
                .get("contents")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed SearchBox AST node (`searchBoxRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchBoxNode {
    pub endpoint: Option<Value>,
    pub search_button: Option<Value>,
    pub clear_button: Option<Value>,
    pub placeholder_text: Option<TextNode>,
}

impl SearchBoxNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchBoxRenderer").unwrap_or(val);
        Some(Self {
            endpoint: node.get("endpoint").cloned(),
            search_button: node.get("searchButton").cloned(),
            clear_button: node.get("clearButton").cloned(),
            placeholder_text: node.get("placeholderText").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed SearchFilterOptionsDialog AST node (`searchFilterOptionsDialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilterOptionsDialogNode {
    pub title: Option<TextNode>,
    pub groups: Vec<Value>,
}

impl SearchFilterOptionsDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchFilterOptionsDialogRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            groups: node
                .get("groups")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}
