use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed EndscreenElement AST node (`endscreenElementRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndscreenElementNode {
    pub style: String,
    pub title: Option<String>,
    pub endpoint: Option<Value>,
    pub image: Option<Value>,
    pub aspect_ratio: Option<f64>,
}

impl EndscreenElementNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("endscreenElementRenderer").unwrap_or(val);
        let style = node
            .get("style")
            .and_then(Value::as_str)
            .unwrap_or("VIDEO")
            .to_string();

        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);
        let endpoint = node.get("endpoint").or_else(|| node.get("navigationEndpoint")).cloned();
        let image = node.get("image").cloned();
        let aspect_ratio = node.get("aspectRatio").and_then(Value::as_f64);

        Some(Self {
            style,
            title,
            endpoint,
            image,
            aspect_ratio,
        })
    }
}

/// Strongly typed Endscreen AST node (`endscreenRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndscreenNode {
    pub elements: Vec<EndscreenElementNode>,
    pub start_ms: Option<u64>,
}

impl EndscreenNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("endscreenRenderer").unwrap_or(val);
        let elements = node
            .get("elements")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(EndscreenElementNode::from_value).collect())
            .unwrap_or_default();

        let start_ms = node
            .get("startMs")
            .and_then(Value::as_str)
            .and_then(|s| s.parse().ok())
            .or_else(|| node.get("startMs").and_then(Value::as_u64));

        Some(Self { elements, start_ms })
    }
}
