# Model: Post (`src/models/post.rs`)

```rust
use innertube_rs::models::post::{CommunityPostsResponse, CommunityPost, CommunityPoll, PollChoice, PostImage};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPostsResponse {
    pub posts: Vec<CommunityPost>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPost {
    pub id: String,
    pub author_name: String,
    pub author_thumbnail: Option<String>,
    pub published_time: Option<String>,
    pub content: String,
    pub like_count: Option<String>,
    pub reply_count: Option<String>,
    pub images: Vec<PostImage>,
    pub poll: Option<CommunityPoll>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityPoll {
    pub choices: Vec<PollChoice>,
    pub total_votes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollChoice {
    pub text: String,
    pub vote_percentage: Option<String>,
    pub is_selected: bool,
}
```
