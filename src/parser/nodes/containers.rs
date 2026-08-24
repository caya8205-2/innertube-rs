use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a tab in twoColumnBrowseResultsRenderer or browse results.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TabNode {
    pub title: String,
    pub selected: bool,
    pub endpoint_params: Option<String>,
    pub content: Option<Value>,
}

impl TabNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("tabRenderer").unwrap_or(val);

        let title = target.get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let selected = target.get("selected")
            .and_then(|s| s.as_bool())
            .unwrap_or(false);

        let endpoint_params = target.pointer("/endpoint/browseEndpoint/params")
            .and_then(|p| p.as_str())
            .map(|s| s.to_string());

        let content = target.get("content").cloned();

        Some(Self {
            title,
            selected,
            endpoint_params,
            content,
        })
    }
}
