# Endpoint: Search (`src/endpoints/search.rs`)

Executes search queries on YouTube InnerTube `/search` endpoint.

```rust
use innertube_rs::endpoints::search::search;
```

---

## Functions

### `search(session: &Session, query: &str, filter: Option<&str>) -> Result<SearchResults>`
Searches videos, channels, and playlists matching the query string.

```rust
let results = yt.search("Rust programming", None).await?;
for item in results.items {
    match item {
        SearchResultItem::Video(v) => println!("Video: {}", v.title),
        SearchResultItem::Channel(c) => println!("Channel: {}", c.title),
        SearchResultItem::Playlist(p) => println!("Playlist: {}", p.title),
    }
}
```
