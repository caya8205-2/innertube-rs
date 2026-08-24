# Endpoint: Player (`src/endpoints/player.rs`)

Fetches metadata, stream formats, and deciphered streaming URLs from InnerTube's `/player` endpoint.

```rust
use innertube_rs::endpoints::player::{get_video_info, get_stream_url, select_format};
```

---

## Functions

### `get_video_info(session: &Session, video_id: &str) -> Result<VideoInfo>`
Sends a request to `/player` with client metadata and parses the response into a `VideoInfo` struct containing details and all available stream formats.

### `get_stream_url(session: &Session, player: &Player, video_id: &str, filter: &FormatFilter) -> Result<String>`
Resolves the best matching format for the filter, deciphers signatures and throttling parameters via `Player`, and returns a playable stream URL.

### `select_format<'a>(info: &'a VideoInfo, filter: &FormatFilter) -> Option<&'a Format>`
Filters available formats according to type (`AudioOnly`, `VideoOnly`, `Both`), quality (`Highest`, `Lowest`, `Exact`), and container (`mp4`, `webm`).
