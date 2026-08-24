# innertube-rs — Current Status

> **Terakhir Diperbarui**: 24 Agustus 2026  
> **Status Repositori**: `v0.4.0` (Full Read-Operations Parity & Modular AST Parser Complete)  
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`)

---

## 1. Ringkasan Status Fitur (Feature Matrix)

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data`, multi-client `post_innertube_client` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Modular AST Parser** (`src/parser/nodes/`) | 🟢 **Ready** | ✅ Passed | 1:1 parity dengan upstream AST nodes (Text, Thumbnail, Author, Navigation, Video, Short, Playlist, Channel, Music, Comments, Continuation) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID → iOS → ANDROID_VR → MWEB, dengan penerusan PO-token & cookie |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`/`720p`/`360p`) |
| **Search Autocomplete & Suggestions** (`src/endpoints/suggestions.rs`) | 🟢 **Ready** | ✅ Passed | Suggestion keyword instan untuk YouTube & YouTube Music |
| **Full YouTube Playlist Scraper** (`src/endpoints/playlist.rs`) | 🟢 **Ready** | ✅ Passed | Metadata header, total videos count, `lockupViewModel` & `playlistVideoRenderer`, continuations |
| **Channel Extended Tabs** (`src/endpoints/channel.rs`) | 🟢 **Ready** | ✅ Passed | Tab *Videos* (recent uploads), *Shorts* (`shortsLockupViewModel`), and Channel *About* metadata |
| **YouTube Main Feeds** (`src/endpoints/feed.rs`) | 🟢 **Ready** | ✅ Passed | Home Feed (`FEwhat_to_watch`), Trending (`FEtrending`), Hashtag Feed (`FEhashtag`) |
| **Guide Navigation Menu** (`src/endpoints/guide.rs`) | 🟢 **Ready** | ✅ Passed | Endpoint `/guide`, menu utama, library, Explore categories |
| **YouTube Music Search & Filters** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Filter khusus: *Songs, Albums, Artists, Playlists, Videos* via `WEB_REMIX` context |
| **YouTube Music Albums & Tracklist** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi album, header cover, artist, tahun rilis, dan seluruh tracklist video IDs |
| **YouTube Music Dedicated Artist Page** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Top Songs, Albums, Singles & EPs, Videos, Similar Artists, dan bio Wikipedia |
| **YouTube Music Home Feed** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Dynamic shelves (*Quick picks*, *Recommended albums*, *Mixed for you*) |
| **YouTube Music Lyrics Engine** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Resolusi tab `MPLY...` dan ekstraksi lirik lagu (LyricFind/Musixmatch) |
| **YouTube Music Explore & Charts** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Top Trending Songs, Top Videos, New Album Releases, dan Moods/Genres |
| **Watch Next & Recommendations** (`src/endpoints/next.rs`) | 🟢 **Ready** | ✅ Passed | Rekomendasi video (`lockupViewModel` & `compactVideoRenderer`), autoplay queue, playlist panel |
| **Subtitles & Transcripts** (`src/endpoints/transcript.rs`) | 🟢 **Ready** | ✅ Passed | Timed transcript JSON3 & XML parser, export SRT & WebVTT, multi-language caption tracks |
| **Comments & Threads Engine** (`src/endpoints/comments.rs`) | 🟢 **Ready** | ✅ Passed | Top comments, pinned comments, author badges, likes, reply threads (`entityBatchUpdate` support) |
| **HLS & DASH Manifest Parser** (`src/utils/manifest.rs`) | 🟢 **Ready** | ✅ Passed | Native Master M3U8 & MPD representation parser (bandwidth, resolutions, codecs) |
| **CI/CD Auto-Build & Release** (`.github/workflows/release.yml`) | 🟢 **Ready** | ✅ Passed | Multi-platform binary auto-build (Windows, Linux, macOS universal) & GitHub Release saat bump versi |
| **Diagnostic Test Suite** (`examples/`) | 🟢 **Ready** | ✅ Passed | **29 script pengujian mandiri** di folder `examples/` |
| **Unit Test & Linter Standards** | 🟢 **Ready** | ✅ Passed | 18 unit tests passing 100%, 0 clippy warnings |

---

## 2. Struktur Codebase & Diagnostic Suite

```
innertube-rs/
├── .agents/
│   ├── AGENTS.md                         # Panduan context agent & path downstream
│   ├── CURRENT_STATUS.md                 # Status aktif project saat ini
│   ├── PORTING_GUIDE.md                  # Panduan teknis arsitektur modular component AST parser
│   ├── PORTING_STATUS.md                 # Paritas modul vs YouTube.js
│   └── archived/
│       └── PORTING_PLAN.md               # Arsip master plan porting
├── Cargo.toml                            # Dependencies: tokio, reqwest, rquickjs, prost, serde
├── build.rs                              # Protobuf build automation (protoc-bin-vendored)
├── protos/                               # Protobuf schemas (params.proto, common.proto)
├── src/
│   ├── lib.rs                            # Top-level Innertube client & public re-exports
│   ├── constants.rs                      # URLs, API keys, client user agents
│   ├── error.rs                          # Typed InnertubeError enum
│   ├── bin/cli.rs                        # CLI binary (info, stream, download commands)
│   ├── core/                             # Session, Player, HttpClient
│   ├── parser/                           # Central AST Parser & Modular Component Nodes (src/parser/nodes/)
│   ├── endpoints/                        # Player, Search, Browse, Next, Transcript, Comments, Music, Suggestions, Playlist, Channel, Feed, Guide
│   ├── models/                           # Video, Format, Search, Channel, Next, Transcript, Comments, Manifest, Music, Suggestions, Playlist, Feed, Guide
│   └── utils/                            # QuickJS decipher engine, Protobuf helpers, Manifest parser
└── examples/                             # 29 runnable diagnostic & verification examples
```
