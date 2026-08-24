# Endpoint: Playlist (`src/endpoints/playlist.rs`)

Fetches playlist details and tracks with pagination support.

```rust
use innertube_rs::endpoints::playlist::{get_playlist, get_playlist_continuation};
```

---

## Functions

### `get_playlist(session: &Session, playlist_id: &str) -> Result<PlaylistView>`
Fetches playlist header information, author, thumbnails, and initial tracklist.

### `get_playlist_continuation(session: &Session, token: &str) -> Result<PlaylistContinuation>`
Fetches subsequent pages of videos for large playlists using continuation tokens.
