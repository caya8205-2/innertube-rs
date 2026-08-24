# Struct: `Innertube`

`innertube_rs::Innertube` is the primary high-level client for `innertube-rs`.

```rust
use innertube_rs::Innertube;
```

---

## 1. Initialization

### `Innertube::new()`
```rust
pub async fn new() -> Result<Self>
```
Creates a new `Innertube` instance with default options, bootstrapping visitor data and initializing the embedded QuickJS player decipher engine.

### `Innertube::with_options(options: SessionOptions)`
```rust
pub async fn with_options(options: SessionOptions) -> Result<Self>
```
Creates a new instance with customized `SessionOptions` (e.g. custom cookies, PO-tokens, or HTTP proxy).

---

## 2. Video & Stream Resolution Methods

### `get_video_info(video_id: &str)`
```rust
pub async fn get_video_info(&self, video_id: &str) -> Result<VideoInfo>
```
Fetches metadata, title, author, description, and available media formats for a video.

### `get_stream_url(video_id: &str, filter: &FormatFilter)`
```rust
pub async fn get_stream_url(&self, video_id: &str, filter: &FormatFilter) -> Result<String>
```
Resolves and deciphers a direct playable HTTPS media stream URL matching the specified filter (e.g. highest quality audio-only or 1080p video).

---

## 3. Search & Channel Methods

### `search(query: &str, filter: Option<&str>)`
```rust
pub async fn search(&self, query: &str, filter: Option<&str>) -> Result<SearchResults>
```

### `get_channel(channel_id_or_handle: &str)`
```rust
pub async fn get_channel(&self, id: &str) -> Result<YouTubeChannelProfile>
```

### `get_channel_videos(channel_id: &str, continuation: Option<&str>)`
```rust
pub async fn get_channel_videos(&self, channel_id: &str, continuation: Option<&str>) -> Result<ChannelVideosResponse>
```

### `get_channel_community(channel_id: &str, continuation: Option<&str>)`
```rust
pub async fn get_channel_community(&self, channel_id: &str, continuation: Option<&str>) -> Result<CommunityPostsResponse>
```

---

## 4. YouTube Music Suite

### `search_music(query: &str, filter: Option<MusicSearchFilter>)`
```rust
pub async fn search_music(&self, query: &str, filter: Option<MusicSearchFilter>) -> Result<MusicSearchResults>
```

### `get_music_artist(artist_id: &str)`
```rust
pub async fn get_music_artist(&self, artist_id: &str) -> Result<MusicArtistPage>
```

### `get_music_album(album_id: &str)`
```rust
pub async fn get_music_album(&self, album_id: &str) -> Result<MusicAlbumView>
```

### `get_music_lyrics(video_id: &str)`
```rust
pub async fn get_music_lyrics(&self, video_id: &str) -> Result<MusicLyrics>
```

---

## 5. Live Chat, Feeds & Account

### `get_live_chat(continuation_token: &str)`
```rust
pub async fn get_live_chat(&self, continuation_token: &str) -> Result<LiveChatResponse>
```

### `get_home_feed(params: Option<&str>)`
```rust
pub async fn get_home_feed(&self, params: Option<&str>) -> Result<HomeFeed>
```

### `get_trending(tab_params: Option<&str>)`
```rust
pub async fn get_trending(&self, tab_params: Option<&str>) -> Result<TrendingFeed>
```

### `like(video_id: &str)` / `dislike(video_id: &str)`
```rust
pub async fn like(&self, video_id: &str) -> Result<ActionResult>
```
