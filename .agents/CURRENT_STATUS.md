# innertube-rs — Current Status

> **Terakhir Diperbarui**: 24 Agustus 2026  
> **Status Repositori**: `v0.1.0` (Active Development — Full Test Suite, Stream Range & Watch Next Recommendations)  
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`)

---

## 1. Ringkasan Status Fitur (Feature Matrix)

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID → iOS → ANDROID_VR → MWEB, dengan penerusan PO-token & cookie |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`/`720p`/`360p`) |
| **PO-Token & Cookie Support** (`src/bin/cli.rs`) | 🟢 **Ready** | ✅ Passed | Dukungan argumen `--po-token`, `--cookies`, dan env vars |
| **Search Queries** (`src/endpoints/search.rs`) | 🟢 **Ready** | ✅ Passed | Recursive AST renderer parser (Video, Channel, Playlist) |
| **Channel Scraping** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | Metadata, subscribers, avatar, top tracks & playlists |
| **Playlist Tracklist** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | YouTube Music (`WEB_REMIX`) & standard playlist format |
| **Watch Next & Recommendations** (`src/endpoints/next.rs`) | 🟢 **Ready** | ✅ Passed | Rekomendasi video (`lockupViewModel` & `compactVideoRenderer`), autoplay queue, playlist panel, dan token continuation |
| **Stream Download** (`src/bin/cli.rs`) | 🟢 **Ready** | ✅ Passed | Native query param range streaming (`&range=` & `&rn=`), resolusi presisi |
| **Diagnostic Test Suite** (`examples/`) | 🟢 **Ready** | ✅ Passed | 17 script pengujian mandiri untuk seluruh client, rekomendasi, dan mode CDN |
| **CI & Documentation** (`.github/workflows/`) | 🟢 **Ready** | ✅ Passed | `cargo test --doc`, `cargo test`, dan `cargo clippy -D warnings` lulus 100% |

---

## 2. Solusi & Penanganan Streaming CDN YouTube

### Temuan Reverse Engineering CDN & Solusi
1. **Perilaku Kuota Chunk CDN**:
   - Google Video CDN (`googlevideo.com`) memberlakukan proteksi *unauthenticated adaptive stream* (720p/1080p) pada offset **6.2 MB** (`6.291.456 bytes`).
   - Format **Audio-only** (`itag 140/251`) dan **Progressive Video** (`itag 18` 360p) bebas dari batasan ini dan dapat diunduh 100% penuh secara instan oleh `innertube-rs`.
2. **Penanganan Native Range Chunking**:
   - `download_stream_to_file` menggunakan parameter native YouTube (`&range=${start}-${end}&rn=${chunk_index}`) dengan segmen chunk 1MB.
3. **Penyisipan PO-Token & Netscape Cookies**:
   - Menambahkan integrasi penuh `serviceIntegrityDimensions.poToken` di handshake endpoint InnerTube dan penyisipan `&pot=<token>` pada stream URL.
   - Parsing otomatis format file cookies Netscape (`cookies.txt`) dan header `Cookie` pada chunk stream downloader.
4. **Pencegahan Downgrade Resolusi**:
   - `cli.rs` tidak lagi menurunkan kualitas ke progressive 360p secara diam-diam jika pengguna meminta resolusi tinggi (`720p` / `1080p`), melainkan melaporkan restriksi CDN secara jelas agar consumer (seperti `avpull`) dapat melakukan fallback cerdas.

---

## 3. Struktur Codebase & Diagnostic Suite

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
│   ├── constants.rs                      # URLs, API keys, client user agents (WEB, ANDROID, ANDROID_VR, IOS, MWEB)
│   ├── error.rs                          # Typed InnertubeError enum
│   ├── bin/cli.rs                        # CLI binary (info, stream, download commands)
│   ├── core/                             # Session, Player, HttpClient
│   ├── endpoints/                        # Player (with fallback chain), Search, Browse, Next
│   ├── models/                           # Context, Video, Format, Search, Channel, Next models
│   └── utils/                            # QuickJS decipher engine, Protobuf helpers
└── examples/
    ├── get_watch_next.rs                 # Watch Next (/next) recommendations & autoplay tester
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

## 4. Status Integrasi ke Consumer Projects

### A. avpull (`C:\Users\Caya\Desktop\Project\avpull`)
* **Status**: 🟢 **Operational (High-Performance Engine + Seamless Fallback)**
* **Keterangan**:
  - Native binary `innertube.exe` dipakai untuk ekstraksi info super cepat dan pengunduhan stream (audio & video progressive).
  - Download audio (`-f mp3, flac, wav, m4a`) berjalan 100% native tanpa delay (~5ms).
  - Video resolusi tinggi (720p/1080p) otomatis fallback secara transparan ke yt-dlp jika tidak ada PO-Token.
  - Dilengkapi fitur Auto-Detect Browser Cookies (Brave, Chrome, Edge, Firefox).

### B. Noctune (`C:\Users\Caya\Desktop\Project\music-player`)
* **Status**: 🟢 **Ready for Integration**
* **Keterangan**:
  - Model data (`ChannelArtistView`, `YouTubePlaylistView`, `WatchNextResults`, `RelatedVideo`, `TrackInfo`) siap dipakai untuk menggantikan scraper JavaScript di `src-tauri`.
  - Endpoint `/next` dan `/browse` siap menyuplai discography album, track artist, dan antrean radio rekomendasi.
