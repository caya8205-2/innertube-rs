use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed MetadataBadge AST node (`metadataBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataBadgeNode {
    pub style: Option<String>,
    pub label: String,
    pub tooltip: Option<String>,
    pub icon_type: Option<String>,
}

impl MetadataBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("metadataBadgeRenderer").unwrap_or(val);
        let label = node
            .get("label")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| node.get("label").and_then(TextNode::from_value).map(|t| t.text))
            .or_else(|| {
                node.get("tooltip")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })
            .unwrap_or_default();

        let style = node.get("style").and_then(Value::as_str).map(ToString::to_string);
        let tooltip = node.get("tooltip").and_then(Value::as_str).map(ToString::to_string);
        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            style,
            label,
            tooltip,
            icon_type,
        })
    }
}

/// Strongly typed ViewCount AST node (`viewCountRenderer` or `videoViewCountRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewCountNode {
    pub view_count: String,
    pub short_view_count: Option<String>,
}

impl ViewCountNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("viewCountRenderer")
            .or_else(|| val.get("videoViewCountRenderer"))
            .unwrap_or(val);

        let view_count = node
            .get("viewCount")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("viewCount")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })?;

        let short_view_count = node
            .get("shortViewCount")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        Some(Self {
            view_count,
            short_view_count,
        })
    }
}

/// Strongly typed VideoViewCount AST node (`videoViewCountRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoViewCountNode {
    pub view_count: String,
    pub short_view_count: Option<String>,
}

impl VideoViewCountNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoViewCountRenderer").unwrap_or(val);
        let view_count = node
            .get("viewCount")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("viewCount")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })?;

        let short_view_count = node
            .get("shortViewCount")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        Some(Self {
            view_count,
            short_view_count,
        })
    }
}

/// Strongly typed VideoOwner AST node (`videoOwnerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOwnerNode {
    pub title: Option<String>,
    pub thumbnail: Option<Value>,
    pub navigation_endpoint: Option<Value>,
    pub subscription_button: Option<Value>,
}

impl VideoOwnerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoOwnerRenderer").unwrap_or(val);
        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);
        let thumbnail = node.get("thumbnail").cloned();
        let navigation_endpoint = node.get("navigationEndpoint").cloned();
        let subscription_button = node.get("subscriptionButton").cloned();

        Some(Self {
            title,
            thumbnail,
            navigation_endpoint,
            subscription_button,
        })
    }
}

/// Strongly typed MicroformatData AST node (`microformatDataRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroformatDataNode {
    pub url_canonical: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<Value>,
    pub tags: Vec<String>,
}

impl MicroformatDataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("microformatDataRenderer").unwrap_or(val);
        let url_canonical = node
            .get("urlCanonical")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);
        let description = node
            .get("description")
            .and_then(TextNode::from_value)
            .map(|t| t.text);
        let thumbnail = node.get("thumbnail").cloned();
        let tags = node
            .get("tags")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(ToString::to_string))
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            url_canonical,
            title,
            description,
            thumbnail,
            tags,
        })
    }
}
