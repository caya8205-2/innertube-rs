# innertube-rs — Current Status

> **Terakhir Diperbarui**: 23 Agustus 2026  
> **Status Repositori**: `v0.1.0` (Active Development — CDN Download & Format Selection)  
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
| **Stream Download** (`src/bin/cli.rs`) | 🟢 **Ready** | ✅ Passed | Native query param range streaming (`&range=` & `&rn=`), resolusi presisi |
| **Library Documentation** (`src/lib.rs`, `README.md`) | 🟢 **Ready** | ✅ Passed | `cargo test --doc` 100% pass, `cargo doc` HTML site |

---

## 2. Solusi & Perbaikan Masalah CDN Error 403 & Resolusi Stream

### Root Cause & Resolved Solution
1. **Penyebab Utama 403 & Throttling CDN**:
   - Google Video CDN memberlakukan limit data per segmen (~1MB) dan total transfer per session (~7-10MB) untuk stream adaptif tanpa Proof of Origin Token (PO-Token).
   - Penggunaan query parameter native YouTube (`&range=${start}-${end}&rn=${chunk_index}`) menggantikan HTTP Header `Range: bytes=` untuk menghindari pemutusan stream di tengah jalan.
2. **PO-Token & Netscape Cookies**:
   - Menambahkan integrasi penuh `serviceIntegrityDimensions.poToken` di handshake endpoint InnerTube dan penyisipan `&pot=<token>` pada stream URL.
   - Parsing otomatis format file cookies Netscape (`cookies.txt`) dan header `Cookie` pada chunk stream downloader.
3. **Pencegahan Downgrade Resolusi**:
   - `cli.rs` tidak lagi menurunkan kualitas ke progressive 360p secara diam-diam jika pengguna meminta resolusi tinggi (`720p` / `1080p`), melainkan melaporkan restriksi CDN secara jelas agar consumer (seperti `avpull`) dapat melakukan fallback cerdas.

---

## 3. Struktur Codebase

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
│   ├── constants.rs                 # URLs, API keys, client user agents (WEB, ANDROID, ANDROID_VR, IOS, MWEB)
│   ├── error.rs                     # Typed InnertubeError enum
│   ├── bin/cli.rs                   # CLI binary (info, stream, download commands)
│   ├── core/                        # Session, Player, HttpClient
│   ├── endpoints/                   # Player (with fallback chain), Search, Browse
│   ├── models/                      # Context, Video, Format, Search, Channel models
│   └── utils/                       # QuickJS decipher engine, Protobuf helpers
└── examples/
    ├── download_audio.rs            # Test audio stream range chunk download
    └── test_playability.rs          # Diagnostic test for CDN download strategies
```

---

## 4. Status Integrasi ke Consumer Projects

### A. avpull (`C:\Users\Caya\Desktop\Project\avpull`)
* **Status**: 🟢 **Operational (100% Native Engine)**
* **Keterangan**:
  - Native binary `innertube.exe` dipakai untuk ekstraksi info, streaming, dan pengunduhan stream mentah (audio & video).
  - Berhasil mengunduh dan mengonversi audio (`-f mp3`) dan video (`-f mp4`) secara langsung tanpa fallback ke yt-dlp.

### B. Noctune (`C:\Users\Caya\Desktop\Project\music-player`)
* **Status**: 🟢 **Ready for Integration**
* **Keterangan**:
  - Model data (`ChannelArtistView`, `YouTubePlaylistView`) sudah disesuaikan untuk kebutuhan player Noctune.
