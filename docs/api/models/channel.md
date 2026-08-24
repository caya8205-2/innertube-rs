# Model: Channel (`src/models/channel.rs`)

```rust
use innertube_rs::models::channel::{YouTubeChannelProfile, ChannelAbout, ChannelVideosResponse, ChannelShortsResponse};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeChannelProfile {
    pub id: String,
    pub name: String,
    pub followers: Option<String>,
    pub top_tracks: Vec<ChannelTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelAbout {
    pub description: Option<String>,
    pub subscriber_count: Option<String>,
    pub view_count: Option<String>,
    pub joined_date: Option<String>,
    pub country: Option<String>,
    pub custom_url: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelVideosResponse {
    pub videos: Vec<ChannelVideoItem>,
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelShortsResponse {
    pub shorts: Vec<ChannelShortItem>,
    pub continuation_token: Option<String>,
}
```
