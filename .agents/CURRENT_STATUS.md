# innertube-rs — Current Status

> **Terakhir Diperbarui**: 24 Agustus 2026  
> **Status Repositori**: `v0.3.0` (Active Development — Full YouTube & YouTube Music Suite)  
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`)

---

## 1. Ringkasan Status Fitur (Feature Matrix)

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data`, multi-client `post_innertube_client` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID → iOS → ANDROID_VR → MWEB, dengan penerusan PO-token & cookie |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`/`720p`/`360p`) |
| **YouTube Music Search & Filters** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Filter khusus: *Songs, Albums, Artists, Playlists, Videos* via `WEB_REMIX` context |
| **YouTube Music Albums & Tracklist** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi album, header cover, artist, tahun rilis, dan seluruh tracklist video IDs |
| **YouTube Music Lyrics Engine** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Resolusi tab `MPLY...` dan ekstraksi lirik lagu (LyricFind/Musixmatch) |
| **YouTube Music Explore & Charts** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Top Trending Songs, Top Videos, New Album Releases, dan Moods/Genres |
| **Watch Next & Recommendations** (`src/endpoints/next.rs`) | 🟢 **Ready** | ✅ Passed | Rekomendasi video (`lockupViewModel` & `compactVideoRenderer`), autoplay queue, playlist panel |
| **Subtitles & Transcripts** (`src/endpoints/transcript.rs`) | 🟢 **Ready** | ✅ Passed | Timed transcript JSON3 & XML parser, export SRT & WebVTT, multi-language caption tracks |
| **Comments & Threads Engine** (`src/endpoints/comments.rs`) | 🟢 **Ready** | ✅ Passed | Top comments, pinned comments, author badges, likes, reply threads (`entityBatchUpdate` support) |
| **HLS & DASH Manifest Parser** (`src/utils/manifest.rs`) | 🟢 **Ready** | ✅ Passed | Native Master M3U8 & MPD representation parser (bandwidth, resolutions, codecs) |
| **CI/CD Auto-Build & Release** (`.github/workflows/release.yml`) | 🟢 **Ready** | ✅ Passed | Multi-platform binary auto-build (Windows, Linux, macOS) & GitHub Release saat bump versi |
| **Diagnostic Test Suite** (`examples/`) | 🟢 **Ready** | ✅ Passed | **22 script pengujian mandiri** di folder `examples/` |
| **Unit Test & Linter Standards** | 🟢 **Ready** | ✅ Passed | 11 unit tests passing 100%, 3 doc tests passing, 0 clippy warnings |

---

## 2. Struktur Codebase & Diagnostic Suite

```
innertube-rs/
├── .agents/
│   ├── AGENTS.md                         # Panduan context agent & path downstream
│   ├── CURRENT_STATUS.md                 # Status aktif project saat ini
│   ├── PORTING_GUIDE.md                  # Panduan teknis porting TypeScript -> Rust
│   ├── PORTING_STATUS.md                 # Paritas modul vs YouTube.js
│   └── archived/
│       └── PORTING_PLAN.md               # Arsip master plan porting (Phase 0–6)
├── Cargo.toml                            # Dependencies: tokio, reqwest, rquickjs, prost, serde
├── build.rs                              # Protobuf build automation (protoc-bin-vendored)
├── protos/                               # Protobuf schemas (params.proto, common.proto)
├── src/
│   ├── lib.rs                            # Top-level Innertube client & public re-exports
│   ├── constants.rs                      # URLs, API keys, client user agents
│   ├── error.rs                          # Typed InnertubeError enum
│   ├── bin/cli.rs                        # CLI binary (info, stream, download commands)
│   ├── core/                             # Session, Player, HttpClient
│   ├── endpoints/                        # Player, Search, Browse, Next, Transcript, Comments, Music
│   ├── models/                           # Video, Format, Search, Channel, Next, Transcript, Comments, Manifest, Music
│   └── utils/                            # QuickJS decipher engine, Protobuf helpers, Manifest parser
└── examples/
    ├── test_music_search.rs              # YouTube Music filtered search (Songs, Albums, Artists, Playlists)
    ├── get_music_album.rs                # YouTube Music album details & tracklist scraper
    ├── get_music_explore.rs              # YouTube Music explore & trending charts
    ├── get_music_lyrics.rs               # YouTube Music song lyrics tester
    ├── get_comments.rs                   # Comments & reply threads extraction tester
    ├── get_transcript.rs                 # Timed subtitle / transcript & SRT/VTT exporter
    ├── get_watch_next.rs                 # Watch Next (/next) recommendations & autoplay tester
    ├── test_manifest_parser.rs           # HLS (.m3u8) & DASH (.mpd) manifest parser tester
    ├── test_clients.rs                   # Multi-client diagnostic tester (iOS, ANDROID, VR, MWEB, WEB)
    ├── test_cdn_modes.rs                 # CDN Range header vs query params vs full GET tester
    ├── test_mweb_stream.rs               # MWEB deciphered HD stream downloader
    ├── test_ntoken_standalone.rs         # Standalone QuickJS decipher & n-token tester
    ├── test_native_botguard.rs           # QuickJS BotGuard challenge VM executor
    ├── test_native_botguard_full.rs      # Full 2-step BotGuard and Google GenerateIT tester
    ├── test_android_vr_stream.rs         # ANDROID_VR direct stream URL tester
    ├── test_android_testsuite.rs         # ANDROID_TESTSUITE endpoint tester
    ├── test_tv_client.rs                 # TVHTML5 embedded player endpoint tester
    ├── test_web_creator.rs               # WEB_CREATOR endpoint tester
    ├── test_web_embedded.rs              # WEB_EMBEDDED_PLAYER endpoint tester
    ├── test_cpn_streaming.rs             # Client Playback Nonce streaming tester
    ├── test_hls.rs                       # HLS & DASH manifest tester
    ├── test_http2_cdn.rs                 # HTTP/2 protocol CDN tester
    ├── download_audio.rs                 # Native audio stream range chunk download
    ├── get_video_info.rs                 # Video metadata and streamingData info
    └── search_and_browse.rs              # Search and channel scraping example
```

---

## 3. Status Integrasi ke Consumer Projects

### A. avpull (`C:\Users\Caya\Desktop\Project\avpull`)
* **Status**: 🟢 **Operational** (Engine native cepat + Auto-Detect browser cookies).

### B. Noctune (`C:\Users\Caya\Desktop\Project\music-player`)
* **Status**: 🟢 **Ready for Integration**
* Semua kebutuhan Noctune: Streaming Audio native, YouTube Music search filters, Tracklist Album, Rekomendasi Radio/Next, Transkrip/Lyrics SRT & WebVTT sudah lengkap 100%.
