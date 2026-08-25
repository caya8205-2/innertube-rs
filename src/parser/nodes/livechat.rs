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

/// Strongly typed LiveChatPaidSticker AST node (`liveChatPaidStickerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPaidStickerNode {
    pub id: String,
    pub author_name: Option<String>,
    pub purchase_amount_text: String,
    pub sticker: Option<Value>,
}

impl LiveChatPaidStickerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatPaidStickerRenderer").unwrap_or(val);
        let id = node.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
        let author_name = node.get("authorName").and_then(TextNode::from_value).map(|t| t.text);
        let purchase_amount_text = node
            .get("purchaseAmountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();
        let sticker = node.get("sticker").cloned();

        Some(Self {
            id,
            author_name,
            purchase_amount_text,
            sticker,
        })
    }
}

/// Strongly typed LiveChatMembershipItem AST node (`liveChatMembershipItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatMembershipItemNode {
    pub id: String,
    pub author_name: Option<String>,
    pub header_subtext: Option<String>,
    pub message: Option<String>,
}

impl LiveChatMembershipItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatMembershipItemRenderer").unwrap_or(val);
        let id = node.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
        let author_name = node.get("authorName").and_then(TextNode::from_value).map(|t| t.text);
        let header_subtext = node.get("headerSubtext").and_then(TextNode::from_value).map(|t| t.text);
        let message = node.get("message").and_then(TextNode::from_value).map(|t| t.text);

        Some(Self {
            id,
            author_name,
            header_subtext,
            message,
        })
    }
}

/// Strongly typed LiveChatViewerEngagementMessage AST node (`liveChatViewerEngagementMessageRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatViewerEngagementMessageNode {
    pub id: Option<String>,
    pub message: String,
    pub action_button: Option<Value>,
    pub icon_type: Option<String>,
}

impl LiveChatViewerEngagementMessageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatViewerEngagementMessageRenderer").unwrap_or(val);
        let id = node.get("id").and_then(Value::as_str).map(ToString::to_string);
        let message = node
            .get("message")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();
        let action_button = node.get("actionButton").cloned();
        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            id,
            message,
            action_button,
            icon_type,
        })
    }
}

/// Strongly typed LiveChatBanner AST node (`liveChatBannerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatBannerNode {
    pub header: Option<String>,
    pub contents: Option<Value>,
    pub action_button: Option<Value>,
}

impl LiveChatBannerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatBannerRenderer").unwrap_or(val);
        let header = node.pointer("/header/liveChatBannerHeaderRenderer/text").and_then(TextNode::from_value).map(|t| t.text);
        let contents = node.get("contents").cloned();
        let action_button = node.get("actionButton").cloned();

        Some(Self {
            header,
            contents,
            action_button,
        })
    }
}

