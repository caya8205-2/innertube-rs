use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Strongly typed ShowEngagementPanelAction AST node (`showEngagementPanelEndpoint` / `showEngagementPanelAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowEngagementPanelActionNode {
    pub panel_identifier: Option<String>,
    pub content: Option<Value>,
}

impl ShowEngagementPanelActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("showEngagementPanelEndpoint")
            .or_else(|| val.get("showEngagementPanelAction"))
            .unwrap_or(val);

        let panel_identifier = node
            .get("panelIdentifier")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let content = node
            .get("engagementPanel")
            .or_else(|| node.get("content"))
            .cloned();

        Some(Self {
            panel_identifier,
            content,
        })
    }
}

/// Strongly typed UpdateEngagementPanelAction AST node (`updateEngagementPanelAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEngagementPanelActionNode {
    pub panel_identifier: Option<String>,
    pub content: Option<Value>,
}

impl UpdateEngagementPanelActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("updateEngagementPanelAction").unwrap_or(val);

        let panel_identifier = node
            .get("panelIdentifier")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let content = node.get("content").cloned();

        Some(Self {
            panel_identifier,
            content,
        })
    }
}

/// Strongly typed NavigateAction AST node (`navigateAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigateActionNode {
    pub endpoint: Option<Value>,
}

impl NavigateActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("navigateAction").unwrap_or(val);
        let endpoint = node.get("endpoint").cloned();

        Some(Self { endpoint })
    }
}

/// Strongly typed ShowLiveChatAction AST node (`showLiveChatAction` / `showLiveChatItemEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowLiveChatActionNode {
    pub client_id: Option<String>,
    pub chat_item: Option<Value>,
}

impl ShowLiveChatActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("showLiveChatAction")
            .or_else(|| val.get("showLiveChatItemEndpoint"))
            .unwrap_or(val);

        let client_id = node.get("clientId").and_then(Value::as_str).map(ToString::to_string);
        let chat_item = node.get("chatItem").or_else(|| node.get("item")).cloned();

        Some(Self {
            client_id,
            chat_item,
        })
    }
}
