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

---

## 2. The Signature & N-Token Decipher Problem

YouTube obfuscates stream URLs in two primary ways:
1. **Signature Cipher (`s` / `sp` / `url`):** Encrypted signature parameter that must be passed through a dynamic transformation function extracted from YouTube's base player JavaScript (`base.js`).
2. **N-Transform (`n` token):** Throttling parameter added to streaming URLs. If not solved using the player's algorithm, download speeds are throttled to ~40-60 KB/s.

### Rust Porting Solution:
- Extract and cache the player script URL from initial HTML.
- Either:
  1. Parse the AST / regex pattern of the decipher function into native Rust execution logic, OR
  2. Embed a lightweight, sandboxed JS engine (e.g. `boa_engine` or `quickjs-rs`) strictly for running the decipher routines (similar to how `youtubei.js` and `yt-dlp` isolate JS evaluation).

---

## 3. Recommended Phased Implementation in Rust

```
[Phase 1: Foundation]
  ├── src/core/session.rs        (HTTP client, context builder, visitor data)
  ├── src/core/constants.rs      (Client types, user agents, Innertube API keys)
  └── src/utils/                 (URL handling, header generators)

[Phase 2: Player & Streaming]
  ├── src/endpoints/player.rs    (Call /player endpoint)
  ├── src/core/player.rs         (Decipher & n-token resolution)
  └── src/models/format.rs       (Video/Audio streaming formats, itags, bitrates)

[Phase 3: Browse & Search]
  ├── src/endpoints/browse.rs    (Channel & Playlist fetching)
  └── src/endpoints/search.rs    (Search queries & filters)

[Phase 4: High-Level Client & Tools]
  ├── src/lib.rs                 (Innertube main client API)
  └── examples/                  (Downloader CLI, audio stream fetcher)
```

---

## 4. Local Workspace References

- **Noctune (Reference Rust Scraper)**:
  - Path: `C:/Users/Caya/Desktop/Project/music-player`
  - Proven Rust HTML/ytInitialData Scraper: `src-tauri/src/youtube_channel.rs` (5ms fast channel & playlist extractor)
- **avpull (Target CLI Consumer)**:
  - Path: `C:/Users/Caya/Desktop/Project/avpull`

