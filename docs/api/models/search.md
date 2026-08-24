# Model: Search (`src/models/search.rs`)

```rust
use innertube_rs::models::search::{SearchResults, SearchResultItem, VideoSearchResult, ChannelSearchResult, PlaylistSearchResult};
```

---

## Structs & Enums

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub query: String,
    pub estimated_results: Option<String>,
    pub items: Vec<SearchResultItem>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SearchResultItem {
    Video(VideoSearchResult),
    Channel(ChannelSearchResult),
    Playlist(PlaylistSearchResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSearchResult {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub author_id: Option<String>,
    pub duration: Option<String>,
    pub view_count: Option<String>,
    pub published_time: Option<String>,
    pub thumbnail: Option<String>,
}
```
