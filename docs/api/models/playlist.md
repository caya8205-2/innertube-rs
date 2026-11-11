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
    pub is_editable: bool,
    pub is_mix: bool,
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

> **Mix playlists:** `is_mix` is `true` for YouTube Mix / radio playlists
> (`RD...` IDs, including the `VLRD...` browse wrapper). Mixes are
> auto-generated, so clients should open them through the watch page
> (`/watch?v=...&list=RD...`) instead of `/playlist`, and `is_editable` is
> always `false` for them.
