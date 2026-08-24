use serde::{Deserialize, Serialize};
use crate::parser::nodes::misc::author::AuthorNode;

/// A standard text message in YouTube Live Chat (`LiveChatTextMessage.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTextMessage {
    pub id: String,
    pub author: Option<AuthorNode>,
    pub message: String,
    pub timestamp_usec: u64,
    pub is_moderator: bool,
    pub is_owner: bool,
}

/// A Super Chat / Paid Message in YouTube Live Chat (`LiveChatPaidMessage.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSuperChat {
    pub id: String,
    pub author: Option<AuthorNode>,
    pub message: Option<String>,
    pub purchase_amount_text: String,
    pub timestamp_usec: u64,
    pub header_background_color: Option<u32>,
}

/// A new membership / milestone item in YouTube Live Chat (`LiveChatMembershipItem.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatMembership {
    pub id: String,
    pub author: Option<AuthorNode>,
    pub header_subtext: Option<String>,
    pub message: Option<String>,
    pub timestamp_usec: u64,
}

/// Strongly typed Live Chat message item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LiveChatMessage {
    Text(LiveChatTextMessage),
    SuperChat(LiveChatSuperChat),
    Membership(LiveChatMembership),
    System(String),
}

/// A single live chat poll response with message items and next continuation token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatResponse {
    pub messages: Vec<LiveChatMessage>,
    pub continuation_token: Option<String>,
    pub poll_timeout_ms: u64,
}
