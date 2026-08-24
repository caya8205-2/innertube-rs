# Struct `Actions`

`innertube_rs::core::Actions` provides high-level authenticated mutations.

```rust
use innertube_rs::core::Actions;
```

---

## Supported Methods

### `like(session: &Session, video_id: &str)`
```rust
pub async fn like(session: &Session, video_id: &str) -> Result<ActionResult>
```

### `dislike(session: &Session, video_id: &str)`
```rust
pub async fn dislike(session: &Session, video_id: &str) -> Result<ActionResult>
```

### `remove_rating(session: &Session, video_id: &str)`
```rust
pub async fn remove_rating(session: &Session, video_id: &str) -> Result<ActionResult>
```

### `subscribe(session: &Session, channel_id: &str)`
```rust
pub async fn subscribe(session: &Session, channel_id: &str) -> Result<ActionResult>
```

### `unsubscribe(session: &Session, channel_id: &str)`
```rust
pub async fn unsubscribe(session: &Session, channel_id: &str) -> Result<ActionResult>
```

### `create_playlist(session: &Session, title: &str, video_ids: &[&str])`
```rust
pub async fn create_playlist(session: &Session, title: &str, video_ids: &[&str]) -> Result<String>
```

### `delete_playlist(session: &Session, playlist_id: &str)`
```rust
pub async fn delete_playlist(session: &Session, playlist_id: &str) -> Result<ActionResult>
```

### `add_to_playlist(session: &Session, playlist_id: &str, video_id: &str)`
```rust
pub async fn add_to_playlist(session: &Session, playlist_id: &str, video_id: &str) -> Result<ActionResult>
```

### `remove_from_playlist(session: &Session, playlist_id: &str, set_video_id: &str)`
```rust
pub async fn remove_from_playlist(session: &Session, playlist_id: &str, set_video_id: &str) -> Result<ActionResult>
```

### `create_comment(session: &Session, video_id: &str, text: &str)`
```rust
pub async fn create_comment(session: &Session, video_id: &str, text: &str) -> Result<ActionResult>
```
