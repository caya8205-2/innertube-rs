# Endpoint: Next (`src/endpoints/next.rs`)

Fetches next watch recommendations, autoplay queues, and related items from `/next`.

```rust
use innertube_rs::endpoints::next::{get_watch_next, get_related_videos};
```

---

## Functions

### `get_watch_next(session: &Session, video_id: &str) -> Result<WatchNextResults>`
Fetches current video details, autoplay next video, and related video list.

### `get_related_videos(session: &Session, video_id: &str) -> Result<Vec<RelatedVideo>>`
Convenience helper returning directly a list of related recommendations.
