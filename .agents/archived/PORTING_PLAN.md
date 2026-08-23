# innertube-rs — Original Porting Plan & Implementation Blueprint

> **Status**: COMPLETED (Phase 0–6 Implemented & Live Verified)  
> **Archived Date**: 2026-08-23  
> **Purpose**: Referensi arsitektur dan blueprint awal untuk kebutuhan maintenance, hardening, dan audit di kemudian hari.

---

## 1. Overview & Context

Porting [LuanRT/YouTube.js](https://github.com/LuanRT/YouTube.js) ke pure Rust sebagai engine native untuk **Noctune** (`C:\Users\Caya\Desktop\Project\music-player`) dan **avpull** (`C:\Users\Caya\Desktop\Project\avpull`).

### Consumer Requirements
1. **Noctune (Desktop Music Player)**:
   - Metadata & Playable Audio Stream URL resolution (Opus / AAC / itag 140/251) tanpa sidecar Node.js.
   - Channel scraper & playlist tracklist extractor (kompatibel dengan schema Noctune).
2. **avpull (Video/Audio CLI Downloader)**:
   - Direct audio/video stream URL resolution (hingga 1080p / 4K) dengan HTTP Range download.

---

## 2. Scope & Technical Architecture

| Komponen | Implementasi di Rust | Lokasi File |
|---|---|---|
| **Session & Visitor Data** | Bootstrap via `sw.js_data` + Protobuf URL-Safe Base64 | `src/core/session.rs`, `src/utils/proto.rs` |
| **Player Decipher Engine** | QuickJS sandbox (`rquickjs`) + Closure Export Hook | `src/utils/decipher.rs`, `src/core/player.rs` |
| **Player Endpoint** | `/youtubei/v1/player` + iOS adaptive stream fallback | `src/endpoints/player.rs`, `src/models/format.rs` |
| **Search Endpoint** | `/youtubei/v1/search` + recursive AST renderer parser | `src/endpoints/search.rs`, `src/models/search.rs` |
| **Browse Endpoint** | Concurrent HTML scraping (`ytInitialData`) & `/browse` | `src/endpoints/browse.rs`, `src/models/channel.rs` |
| **Protobuf** | `prost` + `protoc-bin-vendored` + `prost-build` | `build.rs`, `protos/misc/` |

---

## 3. Implementation Phases Summary

### Phase 0 — Project Bootstrap & Build Setup
- `Cargo.toml`: `reqwest` (rustls), `tokio`, `serde`, `serde_json`, `thiserror`, `prost`, `rand`, `base64`, `regex`, `rquickjs` (default + parallel).
- `build.rs`: Otomatisasi download protoc via `protoc-bin-vendored` dan compile `protos/misc/params.proto` + `common.proto`.

### Phase 1 — Constants, Error Types & HTTP Client
- `src/constants.rs`: Base URLs, API keys, client versions, dan client configurations (`WEB`, `WEB_REMIX`, `iOS`, `ANDROID`).
- `src/error.rs`: Typed `InnertubeError` enum dengan `thiserror` (Network, Json, Proto, Player, Api, NotFound, Restricted, Format).
- `src/models/context.rs`: Struct `InnerTubeContext` serializable ke camelCase JSON.
- `src/core/http_client.rs`: Wrapper `reqwest::Client` dengan header injection otomatis (`X-Youtube-Client-*`, `X-Goog-Visitor-Id`, `Origin`).

### Phase 2 — Session & Visitor Data (Protobuf)
- `src/core/session.rs`: Sesi bootstrap otomatis dari `https://www.youtube.com/sw.js_data` untuk mengambil `visitor_data` token dan API key terkini (fallback ke local generator jika offline).
- `src/utils/proto.rs`: Encoding & decoding `VisitorData` Protobuf ke Base64 URL-safe dengan percent-encoded padding `%3D`.

### Phase 3 — Player Decipher Engine (QuickJS) ⚠️ Critical
- `src/core/player.rs`: Fetch `iframe_api` untuk player ID, download `base.js`, dan ekstrak `signatureTimestamp` (STS).
- `src/utils/decipher.rs`:
  - QuickJS runtime dengan browser shims (`window`, `document`, `location`, `navigator`, `Intl`, `XMLHttpRequest`, `fetch`, `Event`, `MessageChannel`).
  - **Closure Hook Technique**: Menyuntikkan `window.__nsig_fn = <fn>;` sebelum IIFE `})(_yt_player);` selesai dievaluasi agar function `ji` (nsig) yang ter-scope di dalam closure bisa diakses langsung oleh runtime.
  - Transformasi `sig` dan `n-token` berjalan instan (<5ms).

### Phase 4 — Player Endpoint & Format Selection
- `src/models/format.rs`: `StreamingFormat`, `FormatFilter` (`AudioOnly`, `VideoOnly`, `AudioVideo`), `QualityPreference`.
- `src/models/video.rs`: `PlayerResponse`, `VideoDetails`, `StreamingData`.
- `src/endpoints/player.rs`:
  - POST ke `/youtubei/v1/player`.
  - **Adaptive Fallback**: Jika client `WEB` tidak menyertakan cipher/url untuk adaptive stream (format 1080p / 251), otomatis fallback ke client `iOS` untuk mengambil pre-signed direct stream URLs.

### Phase 5 — Browse & Search Endpoints
- `src/models/search.rs`: `SearchResultItem` (`Video`, `Channel`, `Playlist`).
- `src/models/channel.rs`: `ChannelArtistView`, `ChannelTrack`, `YouTubePlaylistView` (kompatibel 1:1 dengan data model Noctune).
- `src/endpoints/search.rs`: Eksekusi `/search` dan recursive traversal untuk mengekstrak `videoRenderer`, `channelRenderer`, `playlistRenderer`.
- `src/endpoints/browse.rs`: Scraping concurrent `home`, `videos`, `releases` untuk channel YouTube & parsing browse playlist via `WEB_REMIX`.

### Phase 6 — Public API & Examples
- `src/lib.rs`: Top-level `Innertube` struct dengan method:
  - `Innertube::new() / with_options()`
  - `get_video_info(video_id)`
  - `get_stream_url(video_id, filter)`
  - `search(query, continuation)`
  - `get_channel(channel_id_or_handle)`
  - `get_playlist(playlist_id)`
- `examples/`:
  - `test_session.rs`
  - `get_video_info.rs`
  - `search_and_browse.rs`
  - `download_audio.rs`

---

## 4. Known Edge Cases & Hardening Notes (Catatan untuk Masa Depan)

1. **QuickJS Closure Function Binding**:
   - Jika YouTube mengubah penutup IIFE `base.js` dari `})(_yt_player);` ke identifier lain, hook string replacement di `src/utils/decipher.rs` perlu disesuaikan dengan regex closing matcher.
2. **Visitor Data Protobuf**:
   - Struktur Visitor Data di YouTube internal berubah sangat jarang. Jika ada perubahan format payload, sesuaikan field di `protos/misc/params.proto`.
3. **Adaptive Formats & Client Fallback**:
   - Fallback `iOS` client saat ini adalah cara paling stabil dan cepat untuk bypass throttling tanpa memerlukan generation PO Token (Proof of Origin) yang berat di browser.
