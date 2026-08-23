use serde::{Deserialize, Serialize};

/// An individual YouTube comment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub comment_id: String,
    pub author_name: String,
    pub author_thumbnail: Option<String>,
    pub author_channel_id: Option<String>,
    pub text: String,
    pub published_time: Option<String>,
    pub like_count_text: Option<String>,
    pub like_count: Option<u64>,
    pub reply_count: Option<u64>,
    pub is_pinned: bool,
    pub is_author_channel_owner: bool,
    pub reply_continuation_token: Option<String>,
}

/// A top-level comment thread including child replies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentThread {
    pub comment: Comment,
    #[serde(default)]
    pub replies: Vec<Comment>,
    pub replies_continuation_token: Option<String>,
}

/// Comments list result containing comment threads and pagination continuation token.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentsResult {
    pub total_comments_text: Option<String>,
    pub comments: Vec<CommentThread>,
    pub continuation_token: Option<String>,
}
