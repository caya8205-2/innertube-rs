use serde::{Deserialize, Serialize};

/// Sort order accepted by Community Post comment threads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PostCommentSort {
    #[default]
    TopComments,
    NewestFirst,
}

impl PostCommentSort {
    pub const fn proto_value(self) -> i32 {
        match self {
            Self::TopComments => 0,
            Self::NewestFirst => 1,
        }
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentsResult {
    pub total_comments_text: Option<String>,
    pub comments: Vec<CommentThread>,
    pub continuation_token: Option<String>,
}

impl CommentsResult {
    /// Check if there are further comment threads to load.
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch the next page of comment threads.
    pub async fn get_continuation(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<CommentsResult> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            crate::error::InnertubeError::Other(
                "No continuation token available for comments".to_string(),
            )
        })?;
        crate::endpoints::comments::get_comments(session, "", Some(token)).await
    }
}
