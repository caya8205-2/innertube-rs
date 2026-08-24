# Endpoint: Music (`src/endpoints/music.rs`)

Provides full access to the YouTube Music suite.

```rust
use innertube_rs::endpoints::music::{
    search_music,
    get_music_artist,
    get_music_album,
    get_music_lyrics,
    get_music_explore,
    get_music_home,
};
```

---

## Functions

### `search_music(session: &Session, query: &str, filter: Option<MusicSearchFilter>) -> Result<MusicSearchResults>`
Searches tracks, albums, artists, or playlists with music-specific filters.

### `get_music_artist(session: &Session, artist_id: &str) -> Result<MusicArtistPage>`
Fetches artist biography, top songs, albums, and singles.

### `get_music_album(session: &Session, album_id: &str) -> Result<MusicAlbumView>`
Fetches album metadata, release year, header banner, and tracklist.

### `get_music_lyrics(session: &Session, video_id: &str) -> Result<MusicLyrics>`
Fetches synchronized or static song lyrics.

### `get_music_explore(session: &Session) -> Result<MusicExplore>`
Fetches new releases, top charts, and mood categories.

### `get_music_home(session: &Session) -> Result<MusicHomeFeed>`
Fetches personalized or regional YouTube Music home carousels.
