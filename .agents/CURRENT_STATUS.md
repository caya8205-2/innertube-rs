# innertube-rs — Current Status

> **Terakhir Diperbarui**: 23 Agustus 2026  
> **Status Repositori**: `v0.1.0` (Ready for Downstream Integration)  
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`)

---

## 1. Ringkasan Status Fitur (Feature Matrix)

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`) `200 OK` |
| **Stream Chunk Download** (`examples/download_audio.rs`) | 🟢 **Ready** | ✅ Passed | HTTP Range chunk download (`206 Partial Content`) |
| **Search Queries** (`src/endpoints/search.rs`) | 🟢 **Ready** | ✅ Passed | Recursive AST renderer parser (Video, Channel, Playlist) |
| **Channel Scraping** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | Metadata, subscribers, avatar, top tracks & playlists |
| **Playlist Tracklist** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | YouTube Music (`WEB_REMIX`) & standard playlist format |
| **Library Documentation** (`src/lib.rs`, `README.md`) | 🟢 **Ready** | ✅ Passed | `cargo test --doc` 100% pass, `cargo doc` HTML site |

---

## 2. Struktur Codebase

```
innertube-rs/
├── .agents/
│   ├── AGENTS.md                    # Panduan context agent & path downstream
│   ├── CURRENT_STATUS.md            # Status aktif project saat ini
│   ├── PORTING_GUIDE.md             # Panduan teknis porting TypeScript -> Rust
│   └── archived/
│       └── PORTING_PLAN.md          # Arsip master plan porting (Phase 0–6)
├── Cargo.toml                       # Dependencies: tokio, reqwest, rquickjs, prost, serde
├── build.rs                         # Protobuf build automation (protoc-bin-vendored)
├── protos/                          # Protobuf schemas (params.proto, common.proto)
├── src/
│   ├── lib.rs                       # Top-level Innertube client & public re-exports
│   ├── constants.rs                 # URLs, API keys, client user agents
│   ├── error.rs                     # Typed InnertubeError enum
│   ├── core/                        # Session, Player, HttpClient
│   ├── endpoints/                   # Player, Search, Browse endpoints
│   ├── models/                      # Context, Video, Format, Search, Channel models
│   └── utils/                       # QuickJS decipher engine, Protobuf helpers
└── examples/
    ├── test_session.rs              # Test session bootstrap & decipher execution
    ├── get_video_info.rs            # Test video metadata & stream URL resolution
    ├── search_and_browse.rs         # Test search query & channel/playlist scraping
    └── download_audio.rs            # Test audio stream range chunk download
```

---

## 3. Status Build & Pengujian

* **`cargo check --all-targets`**: **0 errors / 0 warnings**
* **`cargo test`**: **100% passing** (unit tests & integration tests)
* **`cargo test --doc`**: **3/3 passing**
* **`cargo doc --no-deps`**: Berhasil di-generate di `target/doc/innertube_rs/index.html`

---

## 4. Status Integrasi ke Consumer Projects

### A. Noctune (`C:\Users\Caya\Desktop\Project\music-player`)
* **Tujuan**: Menggantikan backend Node.js sidecar (`youtubei.js`) untuk streaming audio playback.
* **Kesiapan**:
  - `innertube_rs::Innertube::get_stream_url()` siap dipanggil langsung dari Tauri backend (`src-tauri`).
  - Model `ChannelArtistView` dan `YouTubePlaylistView` di `src/models/channel.rs` sudah disesuaikan dengan schema yang digunakan oleh Noctune.

### B. avpull (`C:\Users\Caya\Desktop\Project\avpull`)
* **Tujuan**: Menyediakan engine native untuk download video/audio tanpa dependensi `yt-dlp` / Node.js.
* **Kesiapan**:
  - `FormatFilter` mendukung pemilihan kualitas tertinggi (`AudioOnly`, `VideoOnly`, `AudioVideo`).
  - Stream URL yang di-resolve siap diunduh secara multi-chunk (HTTP Range).

---

## 5. Roadmap & Langkah Mendatang

1. **Integrasi Downstream**:
   - Pasang `innertube-rs` di `music-player/src-tauri/Cargo.toml` sebagai local path dependency atau git dependency.
   - Sambungkan ke `avpull`.
2. **Hardening**:
   - Pantau pembaruan obfuscation script YouTube (`base.js`) jika ada perubahan pattern.
   - Optimasi pooling / reusable runtime QuickJS untuk high-throughput concurrent requests jika diperlukan.
