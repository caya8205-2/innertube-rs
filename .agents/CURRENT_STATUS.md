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
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID_VR → iOS → MWEB, termasuk update `playability_status` |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`/`720p`/`360p`) |
| **Search Queries** (`src/endpoints/search.rs`) | 🟢 **Ready** | ✅ Passed | Recursive AST renderer parser (Video, Channel, Playlist) |
| **Channel Scraping** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | Metadata, subscribers, avatar, top tracks & playlists |
| **Playlist Tracklist** (`src/endpoints/browse.rs`) | 🟢 **Ready** | ✅ Passed | YouTube Music (`WEB_REMIX`) & standard playlist format |
| **Stream Download** (`src/bin/cli.rs`) | 🟡 **In Progress** | ⚠️ Partial | Investigasi format selection (Progressive itag 18 vs Adaptive itag 140) |
| **Library Documentation** (`src/lib.rs`, `README.md`) | 🟢 **Ready** | ✅ Passed | `cargo test --doc` 100% pass, `cargo doc` HTML site |

---

## 2. Analisis Masalah: Error 403 pada Download Stream

### Root Cause Analysis
1. **Perbedaan Pemilihan Format (Format Selection Strategy)**:
   - Pada saat user meminta `-f mp4 -q 360`, CLI `innertube` memecah unduhan menjadi 2 stream: **Adaptive Video** (itag 134/137) + **Adaptive Audio** (itag 140).
   - Di sisi lain, `yt-dlp` memilih **Progressive Format (itag 18)** yang sudah mencakup Video 360p dan Audio AAC dalam satu file/stream tunggal. Format ini tidak memerlukan penggabungan stream dan tidak diblokir oleh CDN Google Video.
2. **Restriksi Stream Adaptive Mobile (`ANDROID_VR`)**:
   - Stream adaptive audio (itag 140 / 139) yang berasal dari endpoint `ANDROID_VR` memerlukan otorisasi/session range tertentu pada CDN, sehingga request chunk Range lanjutan menghasilkan `403 Forbidden`.
   - Stream progressive (itag 18 / 22) dan Web deciphered formats (dengan n-token yang valid) tidak memiliki restriksi ini.
3. **Kesimpulan**:
   - **Masalah berada di library `innertube-rs`** (khususnya strategi pemilihan format dan mekanisme stream download di `src/bin/cli.rs`), bukan pada aplikasi consumer `avpull`.

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

## 4. Status Integrasi ke Consumer Projects

### A. avpull (`C:\Users\Caya\Desktop\Project\avpull`)
* **Status**: 🟢 **Operational with Fallback**
* **Keterangan**:
  - `youtubei.js` sudah dihapus sepenuhnya.
  - Native binary `innertube.exe` dipakai untuk ekstraksi info & streaming awal.
  - Jika native download menemui error 403, avpull otomatis fallback ke `yt-dlp` dan berhasil menyelesaikan download sampai `[OK]`.

### B. Noctune (`C:\Users\Caya\Desktop\Project\music-player`)
* **Status**: 🟢 **Ready for Integration**
* **Keterangan**:
  - Model data (`ChannelArtistView`, `YouTubePlaylistView`) sudah disesuaikan untuk kebutuhan player Noctune.

---

## 5. Rencana Perbaikan (Next Steps)

1. **Prioritaskan Progressive Formats di `src/bin/cli.rs`**:
   - Jika format target adalah `mp4` dan kualitas yang diminta cocok dengan format progressive (itag 18 untuk 360p, itag 22 untuk 720p), langsung download stream progressive tersebut tanpa memecah ke audio/video terpisah.
2. **Web Decipher Fallback untuk Audio Adaptive**:
   - Untuk stream audio murni (MP3/FLAC/M4A), gunakan format audio dari client Web/MWEB yang di-decipher via `rquickjs` agar terbebas dari restriksi stream mobile.
3. **Rebuild & Sync**:
   - Build binary release `innertube.exe` dan copy ke `avpull/bin/`.
