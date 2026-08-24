use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::text::TextNode;

/// Represents a single comment (`Comment.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommentNode {
    pub comment_id: String,
    pub author_name: String,
    pub author_id: Option<String>,
    pub author_thumbnail: Option<String>,
    pub text: String,
    pub published_time: Option<String>,
    pub like_count: Option<String>,
    pub reply_count: Option<u32>,
    pub is_pinned: bool,
    pub is_author_channel_owner: bool,
    pub reply_continuation_token: Option<String>,
}

/// Represents a comment thread container (`CommentThread.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CommentThreadNode {
    pub comment: CommentNode,
    pub replies: Vec<CommentNode>,
    pub continuation_token: Option<String>,
}

impl CommentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("commentRenderer")
            .or_else(|| val.pointer("/comment/commentRenderer"))
            .unwrap_or(val);

        let comment_id = target.get("commentId")
            .and_then(|c| c.as_str())?
            .to_string();

        let author_name = target.pointer("/authorText/simpleText")
            .or_else(|| target.pointer("/authorText/runs/0/text"))
            .and_then(|a| a.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let author_id = target.pointer("/authorEndpoint/browseEndpoint/browseId")
            .and_then(|id| id.as_str())
            .map(|s| s.to_string());

        let author_thumbnail = target.pointer("/authorThumbnail/thumbnails/0/url")
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let text = target.get("contentText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let published_time = target.get("publishedTimeText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let like_count = target.pointer("/voteCount/simpleText")
            .or_else(|| target.pointer("/voteCount/runs/0/text"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let is_pinned = target.pointer("/pinnedCommentBadge").is_some();
        let is_author_channel_owner = target.get("authorIsChannelOwner")
            .and_then(|o| o.as_bool())
            .unwrap_or(false);

        Some(Self {
            comment_id,
            author_name,
            author_id,
            author_thumbnail,
            text,
            published_time,
            like_count,
            reply_count: None,
            is_pinned,
            is_author_channel_owner,
            reply_continuation_token: None,
        })
    }
}

impl CommentThreadNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("commentThreadRenderer").unwrap_or(val);

        let mut comment = target.pointer("/comment/commentRenderer")
            .and_then(CommentNode::from_value)?;

        if let Some(num_str) = target.pointer("/replies/commentRepliesRenderer/viewReplies/buttonRenderer/text")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
        {
            let digits: String = num_str.chars().filter(|c| c.is_ascii_digit()).collect();
            comment.reply_count = digits.parse().ok();
        }

        let reply_token = target.pointer("/replies/commentRepliesRenderer/contents/0/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        comment.reply_continuation_token = reply_token.clone();

        Some(Self {
            comment,
            replies: Vec::new(),
            continuation_token: reply_token,
        })
    }
}
