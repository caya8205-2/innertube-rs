# Struct: `Actions`

`innertube_rs::core::Actions` provides high-level authenticated mutation endpoints for YouTube accounts.

```rust
use innertube_rs::core::Actions;
```

---

## 1. Supported Operations

### Video Ratings
```rust
pub async fn like(session: &Session, video_id: &str) -> Result<ActionResult>
pub async fn dislike(session: &Session, video_id: &str) -> Result<ActionResult>
pub async fn remove_rating(session: &Session, video_id: &str) -> Result<ActionResult>
```

### Channel Subscriptions
```rust
pub async fn subscribe(session: &Session, channel_id: &str) -> Result<ActionResult>
pub async fn unsubscribe(session: &Session, channel_id: &str) -> Result<ActionResult>
```

### Playlist Mutations
```rust
pub async fn create_playlist(session: &Session, title: &str, video_ids: &[&str]) -> Result<String>
pub async fn delete_playlist(session: &Session, playlist_id: &str) -> Result<ActionResult>
pub async fn add_to_playlist(session: &Session, playlist_id: &str, video_id: &str) -> Result<ActionResult>
pub async fn remove_from_playlist(session: &Session, playlist_id: &str, set_video_id: &str) -> Result<ActionResult>
```

### Comments
```rust
pub async fn create_comment(session: &Session, video_id: &str, text: &str) -> Result<ActionResult>
```
