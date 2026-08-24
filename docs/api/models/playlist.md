# Model: Playlist (`src/models/playlist.rs`)

```rust
use innertube_rs::models::playlist::{PlaylistView, PlaylistVideoItem, PlaylistContinuation};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistView {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub author_id: Option<String>,
    pub description: Option<String>,
    pub video_count: Option<u32>,
    pub view_count: Option<String>,
    pub last_updated: Option<String>,
    pub thumbnail: Option<String>,
    pub videos: Vec<PlaylistVideoItem>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistVideoItem {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub author_id: Option<String>,
    pub duration: Option<String>,
    pub duration_ms: Option<u64>,
    pub thumbnail: Option<String>,
    pub is_playable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistContinuation {
    pub videos: Vec<PlaylistVideoItem>,
    pub continuation_token: Option<String>,
}
```
