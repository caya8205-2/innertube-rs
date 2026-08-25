use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed ProfileColumn AST node (`profileColumnRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileColumnNode {
    pub items: Vec<Value>,
    pub header: Option<Value>,
}

impl ProfileColumnNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("profileColumnRenderer").unwrap_or(val);
        let items = node.get("items").and_then(Value::as_array).cloned().unwrap_or_default();
        let header = node.get("header").cloned();

        Some(Self { items, header })
    }
}

/// Strongly typed ProfileColumnUserInfo AST node (`profileColumnUserInfoRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileColumnUserInfoNode {
    pub title: String,
    pub thumbnails: ThumbnailListNode,
    pub description: Option<String>,
}

impl ProfileColumnUserInfoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("profileColumnUserInfoRenderer").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(node));

        let description = node
            .get("description")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        Some(Self {
            title,
            thumbnails,
            description,
        })
    }
}

/// Strongly typed VerticalList AST node (`verticalListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalListNode {
    pub items: Vec<Value>,
    pub collapsed_item_count: Option<u64>,
}

impl VerticalListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("verticalListRenderer").unwrap_or(val);
        let items = node.get("items").and_then(Value::as_array).cloned().unwrap_or_default();
        let collapsed_item_count = node.get("collapsedItemCount").and_then(Value::as_u64);

        Some(Self {
            items,
            collapsed_item_count,
        })
    }
}
