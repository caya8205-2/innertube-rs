# Model: Music (`src/models/music.rs`)

```rust
use innertube_rs::models::music::{
    MusicArtistPage,
    MusicAlbumView,
    MusicLyrics,
    MusicExplore,
    MusicHomeFeed,
    MusicSearchResults,
    MusicSearchFilter,
};
```

---

## Structs & Enums

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicArtistPage {
    pub name: String,
    pub description: Option<String>,
    pub subscribers: Option<String>,
    pub thumbnails: Vec<ThumbnailNode>,
    pub top_songs: Vec<MusicTrackItem>,
    pub albums: Vec<MusicAlbumItem>,
    pub singles: Vec<MusicAlbumItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicAlbumView {
    pub title: String,
    pub artist: Option<String>,
    pub year: Option<String>,
    pub thumbnails: Vec<ThumbnailNode>,
    pub tracks: Vec<MusicTrackItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicLyrics {
    pub lyrics: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicSearchFilter {
    Songs,
    Videos,
    Albums,
    Artists,
    Playlists,
}
```
