# innertube-rs — Current Status

> **Terakhir Diperbarui**: 23 Agustus 2026  
> **Status Repositori**: `v0.1.0` (Active Development — CDN Download Iteration)  
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`)

---

## 1. Ringkasan Status Fitur (Feature Matrix)

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID_VR → iOS → MWEB, termasuk playability status |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`) resolved URLs |
| **Stream Download** (`src/bin/cli.rs`) | 🟡 **In Progress** | ⚠️ Partial | `&range=` + `&rn=` pattern, Chrome UA. CDN rate-limit masih terjadi |
| **Search Queries** (`src/endpoints/search.rs`) | 🟢 **Ready** | ✅ Passed | Recursive AST renderer parser (Video, Channel, Playlist) |
| **Channel Scraping** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | Metadata, subscribers, avatar, top tracks & playlists |
| **Playlist Tracklist** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | YouTube Music (`WEB_REMIX`) & standard playlist format |
| **Library Documentation** (`src/lib.rs`, `README.md`) | 🟢 **Ready** | ✅ Passed | `cargo test --doc` 100% pass, `cargo doc` HTML site |

---

## 2. Status Download Stream (Known Issues)

### Masalah Aktif: Google CDN Rate Limiting
- **Gejala**: HTTP `403 Forbidden` pada request download stream ke Google Video CDN.
- **Root Cause**: Google CDN menerapkan per-IP rate limiting. Setelah banyak request stream dalam waktu singkat, semua subsequent download request ditolak.
- **Perilaku yang Dikonfirmasi**:
  - Chunk pertama (Range `bytes=0-1048575`) selalu berhasil (`206 Partial Content`).
  - Chunk kedua ke URL yang sama selalu ditolak (`403 Forbidden`), terlepas dari metode (Range header, `&range=` query param, client baru per chunk).
  - File sangat kecil (<300KB) yang muat dalam satu chunk tetap berhasil.
  - Bahkan `yt-dlp` mengalami hal yang sama ketika rate-limit aktif.
- **Mitigasi Saat Ini**:
  - `avpull` memiliki fallback otomatis ke `yt-dlp` jika `innertube` download gagal.
  - Download function sudah diubah ke pola `&range=` + `&rn=` (sesuai yt-dlp) yang lebih robust setelah rate-limit reset.

### Pendekatan Download Saat Ini (`download_stream_to_file`)
- Menggunakan **`&range=` query param + `&rn=` request number** (bukan Range header).
- **Chrome User-Agent** untuk CDN download (sesuai perilaku yt-dlp).
- **Chunk size 10MB** (matching yt-dlp `http_chunk_size`).
- **Dedicated download client** terpisah dari session HTTP client.

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
│   ├── constants.rs                 # URLs, API keys, client user agents (WEB, ANDROID_VR, IOS, MWEB)
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

## 4. Status Build & Pengujian

* **`cargo check --all-targets`**: **0 errors / 0 warnings**
* **`cargo test`**: **100% passing** (unit tests & integration tests)
* **`cargo test --doc`**: **3/3 passing**
* **`cargo doc --no-deps`**: Berhasil di-generate di `target/doc/innertube_rs/index.html`

---

## 5. Status Integrasi ke Consumer Projects

### A. Noctune (`C:\Users\Caya\Desktop\Project\music-player`)
* **Tujuan**: Menggantikan backend Node.js sidecar (`youtubei.js`) untuk streaming audio playback.
* **Kesiapan**:
  - `innertube_rs::Innertube::get_stream_url()` siap dipanggil langsung dari Tauri backend (`src-tauri`).
  - Model `ChannelArtistView` dan `YouTubePlaylistView` di `src/models/channel.rs` sudah disesuaikan dengan schema yang digunakan oleh Noctune.

### B. avpull (`C:\Users\Caya\Desktop\Project\avpull`)
* **Status**: 🟡 **Partially Working** (CDN rate-limit masih mengganggu file besar)
* **Implementasi**:
  - `youtubei.js` dihapus sepenuhnya dari `package.json`.
  - Menggunakan native binary `innertube` (Rust) untuk metadata extraction dan stream download.
  - Fallback otomatis ke `yt-dlp` jika `innertube` download gagal.
  - Video kecil (<300KB) berhasil didownload langsung via `innertube`.

---

## 6. Roadmap & Langkah Mendatang

1. **Fix CDN Download (Prioritas Tinggi)**:
   - Investigasi lebih lanjut setelah IP rate-limit reset (biasanya beberapa jam).
   - Kemungkinan solusi: PoToken/Proof of Origin Token, visitor data yang valid, atau session cookie yang lebih lengkap.
2. **Integrasi Downstream**:
   - Pasang `innertube-rs` di `music-player/src-tauri/Cargo.toml` sebagai local path dependency atau git dependency.
3. **Hardening**:
   - Pantau pembaruan obfuscation script YouTube (`base.js`) jika ada perubahan pattern.
   - Optimasi pooling / reusable runtime QuickJS untuk high-throughput concurrent requests jika diperlukan.
