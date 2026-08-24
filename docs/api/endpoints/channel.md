# Endpoint: Channel (`src/endpoints/channel.rs`)

Fetches channel profiles, extended tab listings (Videos, Shorts, Playlists, About), and community posts via `/browse`.

```rust
use innertube_rs::endpoints::channel::{
    get_channel_about,
    get_channel_videos,
    get_channel_shorts,
    get_channel_community,
};
```

---

## Functions

### `get_channel_about(session: &Session, channel_id: &str) -> Result<ChannelAbout>`
Fetches channel description, joined date, subscriber counts, total views, avatar, and banner.

### `get_channel_videos(session: &Session, channel_id: &str, continuation: Option<&str>) -> Result<ChannelVideosResponse>`
Fetches uploaded videos and pagination tokens.

### `get_channel_shorts(session: &Session, channel_id: &str, continuation: Option<&str>) -> Result<ChannelShortsResponse>`
Fetches channel YouTube Shorts items.

### `get_channel_community(session: &Session, channel_id: &str, continuation: Option<&str>) -> Result<CommunityPostsResponse>`
Fetches channel community posts, images, and interactive voting polls.
