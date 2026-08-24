# Endpoint: Feed (`src/endpoints/feed.rs`)

Fetches YouTube feeds (Home, Trending, and Hashtags).

```rust
use innertube_rs::endpoints::feed::{get_home_feed, get_trending, get_hashtag_feed};
```

---

## Functions

### `get_home_feed(session: &Session, params: Option<&str>) -> Result<HomeFeed>`
Fetches the main YouTube browse home feed (`FEwhat_to_watch`).

### `get_trending(session: &Session, params: Option<&str>) -> Result<TrendingFeed>`
Fetches the trending videos and charts feed (`FEtrending`).

### `get_hashtag_feed(session: &Session, tag: &str) -> Result<HashtagFeed>`
Fetches videos tagged with a specific hashtag (`FEhashtag`).
