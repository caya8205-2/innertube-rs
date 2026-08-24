# Model: Video (`src/models/video.rs`)

```rust
use innertube_rs::models::video::{VideoInfo, VideoDetails, RelatedVideo};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub video_details: Option<VideoDetails>,
    pub streaming_data: Option<StreamingData>,
    pub playability_status: Option<PlayabilityStatus>,
    pub formats: Vec<Format>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub length_seconds: String,
    pub channel_id: String,
    pub is_owner_viewing: Option<bool>,
    pub short_description: String,
    pub is_crawlable: Option<bool>,
    pub author: String,
    pub view_count: Option<String>,
    pub is_live_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedVideo {
    pub video_id: String,
    pub title: String,
    pub author: String,
    pub author_id: Option<String>,
    pub view_count: Option<String>,
    pub published_time: Option<String>,
    pub duration: Option<String>,
    pub duration_seconds: Option<u64>,
    pub thumbnail: Option<String>,
}
```
