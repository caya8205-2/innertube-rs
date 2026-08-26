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

/// Strongly typed SearchFilterGroup AST node (`searchFilterGroupRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilterGroupNode {
    pub title: Option<String>,
    pub filters: Vec<Value>,
}

impl SearchFilterGroupNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchFilterGroupRenderer").unwrap_or(val);
        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);
        let filters = node
            .get("filters")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self { title, filters })
    }
}

/// Strongly typed SearchFilter AST node (`searchFilterRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilterNode {
    pub label: String,
    pub endpoint: Option<Value>,
    pub selected: bool,
    pub tooltip: Option<String>,
}

impl SearchFilterNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchFilterRenderer").unwrap_or(val);

        let label = node
            .get("label")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("label").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();
        let selected = node.get("status").and_then(Value::as_str).map(|s| s == "FILTER_STATUS_SELECTED").unwrap_or(false);
        let tooltip = node.get("tooltip").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            label,
            endpoint,
            selected,
            tooltip,
        })
    }
}

