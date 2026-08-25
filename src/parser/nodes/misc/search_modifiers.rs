use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed DidYouMean AST node (`didYouMeanRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidYouMeanNode {
    pub corrected_query: String,
    pub endpoint: Option<Value>,
}

impl DidYouMeanNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("didYouMeanRenderer").unwrap_or(val);
        let corrected_query = node
            .get("correctedQuery")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("correctedQuery")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })?;

        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();

        Some(Self {
            corrected_query,
            endpoint,
        })
    }
}

/// Strongly typed ShowingResultsFor AST node (`showingResultsForRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowingResultsForNode {
    pub corrected_query: String,
    pub original_query_endpoint: Option<Value>,
}

impl ShowingResultsForNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showingResultsForRenderer").unwrap_or(val);
        let corrected_query = node
            .get("correctedQuery")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("correctedQuery")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })?;

        let original_query_endpoint = node
            .get("originalQueryEndpoint")
            .or_else(|| node.get("navigationEndpoint"))
            .cloned();

        Some(Self {
            corrected_query,
            original_query_endpoint,
        })
    }
}

/// Strongly typed SearchSubMenu AST node (`searchSubMenuRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSubMenuNode {
    pub title: Option<String>,
    pub groups: Vec<Value>,
    pub button: Option<Value>,
}

impl SearchSubMenuNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchSubMenuRenderer").unwrap_or(val);
        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);
        let groups = node
            .get("groups")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let button = node.get("button").cloned();

        Some(Self {
            title,
            groups,
            button,
        })
    }
}
