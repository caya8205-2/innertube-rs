use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::live_chat::{
    LiveChatMembership, LiveChatMessage, LiveChatSuperChat, LiveChatTextMessage,
};
use crate::parser::nodes::misc::author::AuthorNode;
use crate::parser::nodes::misc::text::TextNode;

/// AST Node for Live Chat Message items (`livechat/items/*`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatMessageNode {
    pub message: LiveChatMessage,
}

impl LiveChatMessageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        // 1. Standard Live Chat Text Message
        if let Some(msg) = val.get("liveChatTextMessageRenderer") {
            let id = msg.get("id").and_then(Value::as_str)?.to_string();
            let author = AuthorNode::from_value(msg);
            let message = TextNode::from_value(msg.get("message").unwrap_or(&Value::Null))
                .map(|t| t.text)
                .unwrap_or_default();
            let timestamp_usec = msg.get("timestampUsec")
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            let mut is_moderator = false;
            let mut is_owner = false;
            if let Some(badges) = msg.get("authorBadges").and_then(|b| b.as_array()) {
                for badge in badges {
                    let style = badge.pointer("/liveChatAuthorBadgeRenderer/customThumbnail/accessibility/accessibilityData/label")
                        .or_else(|| badge.pointer("/liveChatAuthorBadgeRenderer/tooltip"))
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    if style.to_lowercase().contains("moderator") {
                        is_moderator = true;
                    }
                    if style.to_lowercase().contains("owner") {
                        is_owner = true;
                    }
                }
            }

            return Some(LiveChatMessageNode {
                message: LiveChatMessage::Text(LiveChatTextMessage {
                    id,
                    author,
                    message,
                    timestamp_usec,
                    is_moderator,
                    is_owner,
                }),
            });
        }

        // 2. Super Chat / Paid Message
        if let Some(msg) = val.get("liveChatPaidMessageRenderer") {
            let id = msg.get("id").and_then(Value::as_str)?.to_string();
            let author = AuthorNode::from_value(msg);
            let message = TextNode::from_value(msg.get("message").unwrap_or(&Value::Null))
                .map(|t| t.text);
            let purchase_amount_text = TextNode::from_value(msg.get("purchaseAmountText").unwrap_or(&Value::Null))
                .map(|t| t.text)
                .unwrap_or_default();
            let timestamp_usec = msg.get("timestampUsec")
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let header_background_color = msg.get("headerBackgroundColor").and_then(Value::as_u64).map(|c| c as u32);

            return Some(LiveChatMessageNode {
                message: LiveChatMessage::SuperChat(LiveChatSuperChat {
                    id,
                    author,
                    message,
                    purchase_amount_text,
                    timestamp_usec,
                    header_background_color,
                }),
            });
        }

        // 3. Membership Item
        if let Some(msg) = val.get("liveChatMembershipItemRenderer") {
            let id = msg.get("id").and_then(Value::as_str)?.to_string();
            let author = AuthorNode::from_value(msg);
            let header_subtext = TextNode::from_value(msg.get("headerSubtext").unwrap_or(&Value::Null))
                .map(|t| t.text);
            let message = TextNode::from_value(msg.get("message").unwrap_or(&Value::Null))
                .map(|t| t.text);
            let timestamp_usec = msg.get("timestampUsec")
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            return Some(LiveChatMessageNode {
                message: LiveChatMessage::Membership(LiveChatMembership {
                    id,
                    author,
                    header_subtext,
                    message,
                    timestamp_usec,
                }),
            });
        }

        // 4. Viewer Engagement / System Message
        if let Some(msg) = val.get("liveChatViewerEngagementMessageRenderer") {
            let text = TextNode::from_value(msg.get("message").unwrap_or(&Value::Null))
                .map(|t| t.text)
                .unwrap_or_default();
            return Some(LiveChatMessageNode {
                message: LiveChatMessage::System(text),
            });
        }

        None
    }
}
