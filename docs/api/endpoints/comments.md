# Endpoint: Comments (`src/endpoints/comments.rs`)

Fetches comment threads and replies.

```rust
use innertube_rs::endpoints::comments::{get_comments, get_comment_replies};
```

---

## Functions

### `get_comments(session: &Session, video_id: &str, continuation_token: Option<&str>) -> Result<CommentsResult>`
Fetches top and pinned comments, total comment count, and pagination tokens.

### `get_comment_replies(session: &Session, reply_token: &str) -> Result<Vec<Comment>>`
Fetches replies belonging to a specific comment thread.
