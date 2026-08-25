use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed SearchRefinementCard AST node (`searchRefinementCardRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRefinementCardNode {
    pub query: String,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<Value>,
}

impl SearchRefinementCardNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchRefinementCardRenderer").unwrap_or(val);

        let query = node
            .get("query")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("query").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(node));
        let endpoint = node.get("searchEndpoint").or_else(|| node.get("endpoint")).cloned();

        Some(Self {
            query,
            thumbnails,
            endpoint,
        })
    }
}

/// Strongly typed HorizontalCardList AST node (`horizontalCardListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalCardListNode {
    pub cards: Vec<Value>,
    pub header: Option<Value>,
}

impl HorizontalCardListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("horizontalCardListRenderer").unwrap_or(val);
        let cards = node.get("cards").and_then(Value::as_array).cloned().unwrap_or_default();
        let header = node.get("header").cloned();

        Some(Self { cards, header })
    }
}

/// Strongly typed ExpandableTab AST node (`expandableTabRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandableTabNode {
    pub title: String,
    pub selected: bool,
    pub endpoint: Option<Value>,
}

impl ExpandableTabNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("expandableTabRenderer").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let selected = node.get("selected").and_then(Value::as_bool).unwrap_or(false);
        let endpoint = node.get("endpoint").cloned();

        Some(Self {
            title,
            selected,
            endpoint,
        })
    }
}
