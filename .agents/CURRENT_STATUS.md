# innertube-rs — Current Status

> **Terakhir Diperbarui**: 24 Agustus 2026  
> **Status Repositori**: `v0.4.0` (Active Development — Strategic Modular Parser Phase)  
> **Remote Git**: `https://github.com/caya8205-2/innertube-rs.git` (Branch: `main`, Reference Branch: `reference-youtubejs`)

---

## 1. Status Fitur & Dua Metrik Paritas

| Metrik Paritas | Status | Persentase | Deskripsi |
|---|:---:|:---:|---|
| **Consumer & Streaming Parity** | 🟢 Ready | **~95%** | Seluruh fungsi baca/konsumsi (streaming audio/video, n-token deciphering, search, YT Music suite, full playlists, channel tabs, comments, subtitle SRT/VTT) sudah **100% bekerja dan terverifikasi live**. |
| **Architectural Parity (Component Parser)** | 🟡 In Progress | **~60%** | Sedang dalam proses transisi menuju arsitektur modular **`src/parser/nodes/` (`enum YTNode`)** agar 100% DRY dan tahan lama terhadap perubahan skema UI YouTube di masa depan. |

---

## 2. Feature Matrix & Status Modul

| Modul / Fitur | Status | Verifikasi Live | Catatan |
|---|---|---|---|
| **Session Bootstrap** (`src/core/session.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi `visitor_data` & API key dari `sw.js_data`, multi-client `post_innertube_client` |
| **Protobuf Visitor Data** (`src/utils/proto.rs`) | 🟢 **Ready** | ✅ Passed | Encode/decode Base64 URL-safe dengan padding `%3D` |
| **Player Decipher Engine** (`src/utils/decipher.rs`) | 🟢 **Ready** | ✅ Passed | Sandbox QuickJS (`rquickjs`), eksekusi n-token & sig (<5ms) |
| **Video Metadata & Info** (`src/models/video.rs`) | 🟢 **Ready** | ✅ Passed | Title, author, duration, view count, formats count |
| **Client Fallback Chain** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | WEB → ANDROID → iOS → ANDROID_VR → MWEB, dengan penerusan PO-token & cookie |
| **Stream URL Resolution** (`src/endpoints/player.rs`) | 🟢 **Ready** | ✅ Passed | Audio-only (`AAC`/`Opus`) & Video (`1080p`/`720p`/`360p`) |
| **Search Autocomplete & Suggestions** (`src/endpoints/suggestions.rs`) | 🟢 **Ready** | ✅ Passed | Suggestion keyword instan untuk YouTube & YouTube Music |
| **Full YouTube Playlist Scraper** (`src/endpoints/playlist.rs`) | 🟢 **Ready** | ✅ Passed | Metadata header, total videos count, `lockupViewModel` & `playlistVideoRenderer`, continuations |
| **Channel Extended Tabs** (`src/endpoints/channel.rs`) | 🟢 **Ready** | ✅ Passed | Tab *Videos* (recent uploads), *Shorts* (`shortsLockupViewModel`), and Channel *About* metadata |
| **YouTube Music Search & Filters** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Filter khusus: *Songs, Albums, Artists, Playlists, Videos* via `WEB_REMIX` context |
| **YouTube Music Albums & Tracklist** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Ekstraksi album, header cover, artist, tahun rilis, dan seluruh tracklist video IDs |
| **YouTube Music Lyrics Engine** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Resolusi tab `MPLY...` dan ekstraksi lirik lagu (LyricFind/Musixmatch) |
| **YouTube Music Explore & Charts** (`src/endpoints/music.rs`) | 🟢 **Ready** | ✅ Passed | Top Trending Songs, Top Videos, New Album Releases, dan Moods/Genres |
| **Watch Next & Recommendations** (`src/endpoints/next.rs`) | 🟢 **Ready** | ✅ Passed | Rekomendasi video (`lockupViewModel` & `compactVideoRenderer`), autoplay queue, playlist panel |
| **Subtitles & Transcripts** (`src/endpoints/transcript.rs`) | 🟢 **Ready** | ✅ Passed | Timed transcript JSON3 & XML parser, export SRT & WebVTT, multi-language caption tracks |
| **Comments & Threads Engine** (`src/endpoints/comments.rs`) | 🟢 **Ready** | ✅ Passed | Top comments, pinned comments, author badges, likes, reply threads (`entityBatchUpdate` support) |
| **HLS & DASH Manifest Parser** (`src/utils/manifest.rs`) | 🟢 **Ready** | ✅ Passed | Native Master M3U8 & MPD representation parser (bandwidth, resolutions, codecs) |
| **CI/CD Auto-Build & Release** (`.github/workflows/release.yml`) | 🟢 **Ready** | ✅ Passed | Multi-platform binary auto-build (Windows, Linux, macOS universal) & GitHub Release saat bump versi |
| **Diagnostic Test Suite** (`examples/`) | 🟢 **Ready** | ✅ Passed | **25 script pengujian mandiri** di folder `examples/` |
| **Unit Test & Linter Standards** | 🟢 **Ready** | ✅ Passed | 15 unit tests passing 100%, 3 doc tests passing, 0 clippy warnings |

---

## 3. Rencana Arsitektur Jangka Panjang: Modular Component Parser

Untuk memperkuat codebase agar tahan lama dan mudah di-*maintain* saat YouTube merilis skema baru:
1. **Buat `src/parser/nodes/`**:
   - `nodes/text.rs`: Ekstraksi teks dari `runs` dan `simpleText`.
   - `nodes/thumbnail.rs`: Ekstraksi list thumbnail resolution.
   - `nodes/video.rs`: Parser seragam untuk `videoRenderer`, `compactVideoRenderer`, `lockupViewModel`.
   - `nodes/short.rs`: Parser seragam untuk `reelItemRenderer`, `shortsLockupViewModel`.
   - `nodes/playlist.rs`: Parser seragam untuk `playlistVideoRenderer`, `playlistRenderer`, `lockupViewModel`.
   - `nodes/channel.rs`: Parser seragam untuk `channelRenderer`, `c4TabbedHeaderRenderer`, `pageHeaderRenderer`.
   - `nodes/continuation.rs`: Parser seragam untuk `continuationItemRenderer`, `continuationItemViewModel`.
2. **Refactor Endpoint Parsers**:
   - Endpoint `playlist.rs`, `channel.rs`, `next.rs`, `search.rs` menggunakan parser komponen bersama (`YTNode`).
3. **Port Fitur Sisa 5% (Interaksi & Live Chat)**:
   - OAuth2 Flow Generator (`src/core/OAuth2.ts` $\rightarrow$ `src/core/oauth.rs`).
   - Account Mutations (`src/core/Actions.ts` $\rightarrow$ `src/core/actions.rs`).
   - Live Chat Event Poller (`src/parser/youtube/LiveChat.ts` $\rightarrow$ `src/endpoints/live_chat.rs`).
