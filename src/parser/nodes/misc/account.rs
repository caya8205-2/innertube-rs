use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed Notification AST node (`notificationRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationNode {
    pub notification_id: Option<String>,
    pub primary_text: String,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<Value>,
    pub unread: bool,
}

impl NotificationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("notificationRenderer").unwrap_or(val);

        let notification_id = node
            .get("notificationId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let primary_text = node
            .get("shortMessage")
            .or_else(|| node.get("primaryText"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(node));
        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();
        let unread = node.get("unread").and_then(Value::as_bool).unwrap_or(false);

        Some(Self {
            notification_id,
            primary_text,
            thumbnails,
            endpoint,
            unread,
        })
    }
}

/// Strongly typed HistorySuggestion AST node (`historySuggestionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistorySuggestionNode {
    pub suggestion: String,
    pub endpoint: Option<Value>,
}

impl HistorySuggestionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("historySuggestionRenderer").unwrap_or(val);

        let suggestion = node
            .get("suggestion")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("suggestion").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();

        Some(Self { suggestion, endpoint })
    }
}

/// Strongly typed AccountSectionList AST node (`accountSectionListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSectionListNode {
    pub contents: Vec<Value>,
    pub header: Option<Value>,
}

impl AccountSectionListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accountSectionListRenderer").unwrap_or(val);
        let contents = node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default();
        let header = node.get("header").cloned();

        Some(Self { contents, header })
    }
}

/// Strongly typed AccountItem AST node (`accountItemRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountItemNode {
    pub account_name: String,
    pub account_photo: ThumbnailListNode,
    pub is_selected: bool,
}

impl AccountItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accountItemRenderer").unwrap_or(val);

        let account_name = node
            .get("accountName")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("accountName").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let account_photo = ThumbnailListNode::from_value(node.get("accountPhoto").unwrap_or(node));
        let is_selected = node.get("isSelected").and_then(Value::as_bool).unwrap_or(false);

        Some(Self {
            account_name,
            account_photo,
            is_selected,
        })
    }
}

/// Strongly typed AccountItemSection AST node (`accountItemSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountItemSectionNode {
    pub contents: Vec<Value>,
    pub header: Option<Value>,
}

impl AccountItemSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accountItemSectionRenderer").unwrap_or(val);
        let contents = node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default();
        let header = node.get("header").cloned();

        Some(Self { contents, header })
    }
}

/// Strongly typed AccountItemSectionHeader AST node (`accountItemSectionHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountItemSectionHeaderNode {
    pub title: String,
}

impl AccountItemSectionHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accountItemSectionHeaderRenderer").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        Some(Self { title })
    }
}

