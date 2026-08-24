# Struct `Innertube`

The main entry point for interacting with YouTube and YouTube Music.

```rust
use innertube_rs::Innertube;
```

---

## Constructor

### `Innertube::new()`
```rust
pub async fn new() -> Result<Self>
```
Creates an `Innertube` instance with default options, bootstrapping visitor tokens and initializing the QuickJS decipher engine.

### `Innertube::with_options(options: SessionOptions)`
```rust
pub async fn with_options(options: SessionOptions) -> Result<Self>
```
Creates an instance with custom configuration (cookies, PO-tokens, HTTP proxy).

---

## Video & Playback Methods

### `get_video_info(video_id: &str)`
```rust
pub async fn get_video_info(&self, video_id: &str) -> Result<VideoInfo>
```
Fetches video metadata, title, author, views, descriptions, and formats.

### `get_stream_url(video_id: &str, filter: &FormatFilter)`
```rust
pub async fn get_stream_url(&self, video_id: &str, filter: &FormatFilter) -> Result<String>
```
Resolves and deciphers a direct playable HTTPS media URL.

### `get_watch_next(video_id: &str)`
```rust
pub async fn get_watch_next(&self, video_id: &str) -> Result<WatchNextResults>
```
Fetches autoplay, playlist queue, and related video recommendations.

### `get_comments(video_id: &str, continuation: Option<&str>)`
```rust
pub async fn get_comments(&self, video_id: &str, continuation: Option<&str>) -> Result<CommentsResult>
```
Fetches top and pinned comments, likes, and reply threads.

### `get_transcript(video_id: &str, lang: Option<&str>)`
```rust
pub async fn get_transcript(&self, video_id: &str, lang: Option<&str>) -> Result<Transcript>
```
Fetches timed transcripts and subtitles, supporting SRT and WebVTT export.

---

## Search & Discovery Methods

### `search(query: &str, filter: Option<&str>)`
```rust
pub async fn search(&self, query: &str, filter: Option<&str>) -> Result<SearchResults>
```

### `get_search_suggestions(query: &str)`
```rust
pub async fn get_search_suggestions(&self, query: &str) -> Result<Vec<String>>
```

### `get_home_feed(params: Option<&str>)`
```rust
pub async fn get_home_feed(&self, params: Option<&str>) -> Result<HomeFeed>
```

### `get_trending(params: Option<&str>)`
```rust
pub async fn get_trending(&self, params: Option<&str>) -> Result<TrendingFeed>
```

### `get_guide()`
```rust
pub async fn get_guide(&self) -> Result<GuideResponse>
```

---

## Channels & Playlists

### `get_channel(id: &str)`
```rust
pub async fn get_channel(&self, id: &str) -> Result<YouTubeChannelProfile>
```

### `get_channel_about(id: &str)`
```rust
pub async fn get_channel_about(&self, id: &str) -> Result<ChannelAbout>
```

### `get_channel_videos(id: &str, continuation: Option<&str>)`
```rust
pub async fn get_channel_videos(&self, id: &str, continuation: Option<&str>) -> Result<ChannelVideosResponse>
```

### `get_channel_shorts(id: &str, continuation: Option<&str>)`
```rust
pub async fn get_channel_shorts(&self, id: &str, continuation: Option<&str>) -> Result<ChannelShortsResponse>
```

### `get_channel_community(id: &str, continuation: Option<&str>)`
```rust
pub async fn get_channel_community(&self, id: &str, continuation: Option<&str>) -> Result<CommunityPostsResponse>
```

### `get_playlist(id: &str)`
```rust
pub async fn get_playlist(&self, id: &str) -> Result<PlaylistView>
```

---

## YouTube Music Suite

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

## Live Chat & Account

### `get_live_chat(continuation_token: &str)`
```rust
pub async fn get_live_chat(&self, continuation_token: &str) -> Result<LiveChatResponse>
```

### `like(video_id: &str)` / `dislike(video_id: &str)`
```rust
pub async fn like(&self, video_id: &str) -> Result<ActionResult>
pub async fn dislike(&self, video_id: &str) -> Result<ActionResult>
```

### `subscribe(channel_id: &str)` / `unsubscribe(channel_id: &str)`
```rust
pub async fn subscribe(&self, channel_id: &str) -> Result<ActionResult>
pub async fn unsubscribe(&self, channel_id: &str) -> Result<ActionResult>
```
