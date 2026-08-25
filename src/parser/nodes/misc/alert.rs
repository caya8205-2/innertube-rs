use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed Alert AST node (`alertRenderer` / `alertWithActionsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertNode {
    pub alert_type: Option<String>,
    pub text: String,
    pub dismiss_button: Option<Value>,
}

impl AlertNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("alertRenderer")
            .or_else(|| val.get("alertWithActionsRenderer"))
            .unwrap_or(val);

        let text = node
            .get("text")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("text")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })?;

        let alert_type = node.get("type").and_then(Value::as_str).map(ToString::to_string);
        let dismiss_button = node.get("dismissButton").cloned();

        Some(Self {
            alert_type,
            text,
            dismiss_button,
        })
    }
}

/// Strongly typed Card AST node (`cardRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardNode {
    pub card_id: Option<String>,
    pub teaser: Option<String>,
    pub endpoint: Option<Value>,
}

impl CardNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("cardRenderer").unwrap_or(val);
        let card_id = node.get("cardId").and_then(Value::as_str).map(ToString::to_string);
        let teaser = node
            .pointer("/teaser/simpleCardTeaserRenderer/message")
            .and_then(TextNode::from_value)
            .map(|t| t.text);
        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();

        Some(Self {
            card_id,
            teaser,
            endpoint,
        })
    }
}

/// Strongly typed Clarification / EmergencyOnebox AST node (`clarificationRenderer` / `emergencyOneboxRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClarificationNode {
    pub title: String,
    pub text: Option<String>,
    pub source: Option<String>,
    pub source_url: Option<String>,
}

impl ClarificationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("clarificationRenderer")
            .or_else(|| val.get("emergencyOneboxRenderer"))
            .unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("title")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })?;

        let text = node
            .get("text")
            .or_else(|| node.get("content"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let source = node.pointer("/source/simpleText").and_then(Value::as_str).map(ToString::to_string);
        let source_url = node
            .pointer("/endpoint/urlEndpoint/url")
            .or_else(|| node.pointer("/navigationEndpoint/urlEndpoint/url"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            title,
            text,
            source,
            source_url,
        })
    }
}

/// Strongly typed Poll AST node (`pollRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollNode {
    pub question: String,
    pub choices: Vec<String>,
    pub total_votes: Option<String>,
}

impl PollNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pollRenderer").unwrap_or(val);
        let question = node
            .get("question")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("question")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            })
            .unwrap_or_default();

        let choices = node
            .get("choices")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|c| {
                        c.get("text")
                            .and_then(TextNode::from_value)
                            .map(|t| t.text)
                            .or_else(|| c.get("text").and_then(Value::as_str).map(ToString::to_string))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let total_votes = node
            .get("totalVotes")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        Some(Self {
            question,
            choices,
            total_votes,
        })
    }
}
