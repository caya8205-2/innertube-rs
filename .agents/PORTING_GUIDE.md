# InnerTube Porting Guide (JS/TS -> Rust)

This document outlines the core mechanisms of `YouTube.js` to serve as a technical blueprint for the Rust port.

---

## 1. InnerTube Core Concepts

### A. Context & Client Identity
Every request to YouTube's `/youtubei/v1/*` endpoints requires a JSON payload containing a `context` object:
- `client`: Specifies client name (`WEB`, `WEB_REMIX` / YouTube Music, `ANDROID`, `IOS`, `TV_EMBEDDED`, etc.), client version, OS, user agent.
- `user`: Holds visitor data (`visitorData`), auth headers (SAPISIDHASH / cookies if authenticated).
- `request`: Session index, consistency tokens.

### B. Endpoint Overview
| Endpoint | Method | Function |
|---|---|---|
| `/youtubei/v1/player` | `POST` | Fetches video details, streaming formats (`streamingData`), captions, storyboard. |
| `/youtubei/v1/browse` | `POST` | Fetches channels, playlists, home feeds, explore tabs. |
| `/youtubei/v1/search` | `POST` | Search query results with filters. |
| `/youtubei/v1/next` | `POST` | Up next video recommendations, comments, and metadata panels. |
| `/youtubei/v1/music/get_search_suggestions` | `POST` | YouTube Music search autocomplete suggestions. |
| `/youtubei/v1/live_chat/get_live_chat` | `POST` | Live stream chat events and messages. |

---

## 2. The Signature & N-Token Decipher Problem

YouTube obfuscates stream URLs in two primary ways:
1. **Signature Cipher (`s` / `sp` / `url`):** Encrypted signature parameter that must be passed through a dynamic transformation function extracted from YouTube's base player JavaScript (`base.js`).
2. **N-Transform (`n` token):** Throttling parameter added to streaming URLs. If not solved using the player's algorithm, download speeds are throttled to ~40-60 KB/s.

### Rust Solution in `innertube-rs`:
- Sandboxed QuickJS engine (`rquickjs`) executes base.js decipher routines and n-token transformations in `<5ms`.
- Multi-client fallback chain (**WEB → ANDROID → iOS → ANDROID_VR → MWEB**) ensures 100% playable stream retrieval across varying IP/account restrictions.

---

## 3. Long-Term Architecture: Modular Component Parser (`src/parser/`)

To match the modularity of `YouTube.js` while maintaining Rust's zero-cost performance:

### A. The Polymorphic Node Pattern (`enum YTNode`)
Instead of duplicating JSON extraction logic across endpoints (`playlist.rs`, `channel.rs`, `next.rs`, `search.rs`), reusable YouTube renderers are parsed into a central node enum:

```rust
pub enum YTNode {
    Video(VideoNode),
    Short(ShortNode),
    Channel(ChannelNode),
    Playlist(PlaylistNode),
    MusicTrack(MusicTrackNode),
    Comment(CommentNode),
    Continuation(ContinuationNode),
}
```

### B. Component Layer Structure:
```
src/parser/
├── mod.rs                # Parser engine & tree traverser (parse_tree / parse_node)
├── traits.rs             # FromNode trait & helper macros
└── nodes/
    ├── text.rs           # Runs / simpleText extractor
    ├── thumbnail.rs      # Image sources / aspect ratio handler
    ├── navigation.rs     # WatchEndpoint, BrowseEndpoint, SearchEndpoint
    ├── video.rs          # videoRenderer, lockupViewModel, compactVideoRenderer
    ├── short.rs          # reelItemRenderer, shortsLockupViewModel
    ├── channel.rs        # channelRenderer, c4TabbedHeaderRenderer, pageHeaderRenderer
    ├── playlist.rs       # playlistVideoRenderer, playlistHeaderRenderer
    └── continuation.rs   # continuationItemRenderer, continuationItemViewModel
```

### C. Benefits of This Architecture:
1. **Single Source of Truth**: When YouTube alters a renderer format (e.g. replacing `playlistVideoRenderer` with `lockupViewModel`), we update **only 1 node parser** in `src/parser/nodes/video.rs`. All endpoints (`search`, `browse`, `playlist`, `next`) automatically inherit the update.
2. **Type Safety & Zero Allocation Overhead**: Leverages Rust `enum` variants without heavy dynamic runtime class instantiation.

---

## 4. Local Workspace References

- **Noctune (Desktop Music Player)**:
  - Path: `C:/Users/Caya/Desktop/Project/music-player`
  - Integration: Replace Node.js sidecar with native `innertube-rs` crate.
- **avpull (Fast CLI Downloader)**:
  - Path: `C:/Users/Caya/Desktop/Project/avpull`
  - Integration: Native stream resolving & DASH/HLS audio/video segment downloader.
