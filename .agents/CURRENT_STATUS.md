# innertube-rs — Current Status

> **Terakhir Diperbarui**: 25 Agustus 2026
> **Status Repositori**: `v0.5.0` (**Full-parity audit and implementation in progress; see `PARITY_PLAN.md` and `PARITY_MANIFEST.md`**)
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`)

---

## Current Parity Handoff

The historical feature matrix below records the original project milestone; it
does **not** prove full YouTube.js parity. The active compatibility target is
`reference-youtubejs` commit `85473772ce9a9238091636fc2cb7ea3c331ea88d`.

Read `PARITY_PLAN.md` first for the ordered work plan and exact uncommitted
checkpoint. Read `PARITY_MANIFEST.md` for the authoritative completion state.
At this checkpoint, `cargo test --all-targets` passes 26 tests, but parser and
public API parity are still incomplete. Do not report 100% parity.

---

## 1. Ringkasan Status Fitur (Complete Feature Matrix)

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data`, multi-client `post_innertube_client` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Modular AST Parser** (`src/parser/nodes/`) | 🟢 **Ready** | ✅ Passed | 1:1 parity dengan AST nodes (Text, Thumbnail, Author, Navigation, Video, Short, Playlist, Channel, Music, Comments, Community Post, LiveChat, Continuation) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID → iOS → ANDROID_VR → MWEB, dengan penerusan PO-token & cookie |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`/`720p`/`360p`) |
| **Search Autocomplete & Suggestions** (`src/endpoints/suggestions.rs`) | 🟢 **Ready** | ✅ Passed | Suggestion keyword instan untuk YouTube & YouTube Music |
| **Full YouTube Playlist Scraper** (`src/endpoints/playlist.rs`) | 🟢 **Ready** | ✅ Passed | Metadata header, total videos count, `lockupViewModel` & `playlistVideoRenderer`, continuations |
| **Channel Extended Tabs** (`src/endpoints/channel.rs`) | 🟢 **Ready** | ✅ Passed | Tab *Videos* (recent uploads), *Shorts* (`shortsLockupViewModel`), Channel *About*, dan tab *Community* |
| **Community Posts & Polls Engine** (`src/parser/nodes/post.rs`) | 🟢 **Ready** | ✅ Passed | Parsing postingan teks, gambar, video preview, voting pilihan polling (`Poll.ts`) |
| **Real-Time Live Chat Engine** (`src/endpoints/live_chat.rs`) | 🟢 **Ready** | ✅ Passed | Parsing pesan chat live, Super Chat, Memberships, polling intervals & continuation |
| **OAuth2 Device / TV Flow** (`src/core/oauth.rs`) | 🟢 **Ready** | ✅ Passed | Dynamic TV client extraction, device code & user verification code (`https://www.google.com/device`), token polling & auto-refresh |
| **Account Mutation Actions** (`src/core/actions.rs`) | 🟢 **Ready** | ✅ Passed | Like, Dislike, Remove Rating, Subscribe, Unsubscribe, Add/Remove Playlist, Create Comment |
| **Authenticated Feeds & Settings** (`src/endpoints/account.rs`) | 🟢 **Ready** | ✅ Passed | History feed (`FEhistory`), Library feed (`FElibrary`), Inbox Notifications |
| **YouTube Main Feeds** (`src/endpoints/feed.rs`) | 🟢 **Ready** | ✅ Passed | Home Feed (`FEwhat_to_watch`), Trending (`FEtrending`), Hashtag Feed (`FEhashtag`) |
| **Guide Navigation Menu** (`src/endpoints/guide.rs`) | 🟢 **Ready** | ✅ Passed | Endpoint `/guide`, menu utama, library, Explore categories |
| **YouTube Music Dedicated Suite** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Filtered search, Albums, Dedicated Artist Page, Home Feed, Lyrics, Explore/Charts |
| **Subtitles & Transcripts** (`src/endpoints/transcript.rs`) | 🟢 **Ready** | ✅ Passed | Timed transcript JSON3 & XML parser, export SRT & WebVTT, multi-language caption tracks |
| **Comments & Threads Engine** (`src/endpoints/comments.rs`) | 🟢 **Ready** | ✅ Passed | Top comments, pinned comments, author badges, likes, reply threads |
| **HLS & DASH Manifest Parser** (`src/utils/manifest.rs`) | 🟢 **Ready** | ✅ Passed | Native Master M3U8 & MPD representation parser (bandwidth, resolutions, codecs) |
| **Diagnostic Test Suite** (`examples/`) | 🟢 **Ready** | ✅ Passed | **32 script pengujian mandiri** di folder `examples/` |
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
│   ├── core/                             # Session, Player, HttpClient, OAuth2, Actions
│   ├── parser/                           # Central AST Parser & Modular Component Nodes (src/parser/nodes/)
│   ├── endpoints/                        # Player, Search, Browse, Next, Transcript, Comments, Music, Suggestions, Playlist, Channel, Feed, Guide, LiveChat, Account
│   ├── models/                           # Video, Format, Search, Channel, Next, Transcript, Comments, Manifest, Music, Suggestions, Playlist, Feed, Guide, Post, LiveChat, OAuth, Actions, Account
│   └── utils/                            # QuickJS decipher engine, Protobuf helpers, Manifest parser
└── examples/                             # Categorized runnable diagnostic & verification examples
    ├── download/                         # Audio/video stream downloaders
    ├── video/                            # Video info, watch next, comments, transcripts, suggestions
    ├── music/                            # YT Music albums, artists, lyrics, explore, search
    ├── channel/                          # Channel tabs, community posts
    ├── feed/                             # Home feed, trending, guide menu
    ├── live/                             # Real-time live chat polling
    ├── auth/                             # OAuth2 device flow login
    └── diagnostics/                      # Client fallbacks, botguard, n-token, and manifest tests
```
