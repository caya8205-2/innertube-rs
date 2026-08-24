# Model: Comments (`src/models/comments.rs`)

```rust
use innertube_rs::models::comments::{CommentsResult, CommentThread, Comment};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsResult {
    pub comments: Vec<CommentThread>,
    pub total_comments_text: Option<String>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentThread {
    pub comment: Comment,
    pub replies: Vec<Comment>,
    pub replies_continuation_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
```
