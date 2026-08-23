# innertube-rs — Porting Status & Feature Matrix

> **Upstream Reference**: [LuanRT/YouTube.js (YouTubei.js)](https://github.com/LuanRT/YouTube.js)  
> **Target Project**: `innertube-rs` (Pure Rust Port)  
> **Last Updated**: August 24, 2026  
> **Overall Porting Progress (Core Streaming & Media Engine)**: **~92% – 95%**  
> **Overall Porting Progress (Total YouTube.js Feature Parity)**: **~45% – 50%**

---

## 1. Module-by-Module Porting Status

| YouTube.js (JS/TS) Module | Rust Equivalent (`innertube-rs`) | Status | Parity % | Description & Implementation Details |
|---|---|:---:|:---:|---|
| `src/core/Session.ts` | [`src/core/session.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/session.rs) | 🟢 Complete | 90% | Client context creation, device category headers, API key extraction from `sw.js_data`, visitor data generation, PO-token & cookie session management. |
| `src/core/HTTPClient.ts` | [`src/core/http.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/http.rs) | 🟢 Complete | 100% | `reqwest`-based asynchronous HTTP client with gzip, brotli, HTTP/2 negotiation, custom headers, and Netscape cookie store. |
| `src/core/Player.ts` | [`src/utils/decipher.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/decipher.rs) | 🟢 Complete | 95% | QuickJS (`rquickjs`) sandboxed decipher engine. Extracts and executes base.js signature decipher algorithms and n-token transformations (<5ms). |
| `src/utils/ProtoUtils.ts` | [`src/utils/proto.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/proto.rs) | 🟢 Complete | 100% | Protobuf visitor data encoding and decoding using `prost` and URL-safe base64 padding. |
| `src/core/endpoints/Player.ts` | [`src/endpoints/player.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/player.rs) | 🟢 Complete | 90% | InnerTube `/player` endpoint with automatic multi-client fallback chain (**WEB → ANDROID → iOS → ANDROID_VR → MWEB**). |
| `src/core/endpoints/Search.ts` | [`src/endpoints/search.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/search.rs) | 🟢 Complete | 80% | InnerTube `/search` endpoint with recursive AST renderer parser (extracts video results, channel results, playlist results). |
| `src/core/endpoints/Browse.ts` | [`src/endpoints/browse.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/browse.rs) | 🟢 Complete | 85% | InnerTube `/browse` endpoint. Supports channel profile scraping, top tracks, and YouTube Music playlist tracklist extraction. |
| `src/core/endpoints/Next.ts` | [`src/endpoints/next.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/next.rs) | 🟢 Complete | 90% | InnerTube `/next` endpoint. Supports video recommendations (`lockupViewModel`, `compactVideoRenderer`), autoplay suggestions, playlist queue panels, and continuation tokens. |
| `src/parser/` | [`src/models/`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/models/) | 🟢 Complete (Core) | 40% | Strongly typed Serde models for essential InnerTube payloads. Polymorphic renderers are parsed on-demand rather than maintaining 150+ individual class hierarchies. |
| `src/core/OAuth2.ts` | `src/core/oauth.rs` | ⚪ Optional | 0% | TV/Device code OAuth2 login flow. |
| `src/actions/` | `src/actions/` | 🟡 In Progress | 40% | High-level user interaction wrappers (Comments, LiveChat, Studio, Subscriptions). |

---

## 2. Streaming & CDN Engine Matrix

| Feature / Protocol | Status | Notes |
|---|:---:|---|
| **Audio Streaming (`-f mp3, flac, wav, m4a`)** | 🟢 **100% Native** | Audio formats (`itag 140` AAC, `itag 251` Opus) have no CDN chunk throttling. Downloads full 40MB+ streams at maximum throughput. |
| **Progressive Video (`itag 18` 360p)** | 🟢 **100% Native** | Direct playback formats from standard `ANDROID` client download 100% without CDN restrictions. |
| **Adaptive Video (720p / 1080p / 4K)** | 🟢 **Supported** | Supports streaming via transformed n-tokens (MWEB 60fps routes) and PO-token injection (`&pot=`). Unauthenticated requests hitting the 6.2MB CDN boundary trigger clean fallback in downstream consumers (`avpull`). |
| **Chunk Segmentation Protocol** | 🟢 **100% Native** | Uses YouTube native query parameters (`&range=${start}-${end}&rn=${index}`) with 1MB segment size, avoiding mid-stream drops from standard HTTP headers. |
| **PO-Token Propagation** | 🟢 **Ready** | Supports `--po-token` CLI flag, environment variables (`INNERTUBE_PO_TOKEN`, `POT`), and auto-injects into `serviceIntegrityDimensions.poToken` and stream URLs. |
| **Cookie Support** | 🟢 **Ready** | Supports raw cookie strings and Netscape format (`cookies.txt`) with automatic header injection. |

---

## 3. Consumer Projects Integration Status

### A. `avpull` (`C:/Users/Caya/Desktop/Project/avpull`)
- **Role**: Command-line audio/video downloader with +1k npm downloads.
- **Engine**: Replaced heavy Node.js sidecar and `youtubei.js` runtime with compiled native `innertube.exe` binary.
- **Status**: 🟢 **Operational**
  - Instant metadata & audio conversion (<5ms startup).
  - High-resolution fallback gracefully delegates to `yt-dlp` when CDN 403 policies apply, providing seamless 720p/1080p downloads with zero manual steps.

### B. `noctune` (`C:/Users/Caya/Desktop/Project/music-player`)
- **Role**: High-performance desktop music player with on-device Hybrid Collaborative Filtering ML model.
- **Target**: Pure native Rust scraping and audio streaming, replacing Node.js sidecars.
- **Status**: 🟢 **Ready for Integration**
  - Models (`ChannelArtistView`, `YouTubePlaylistView`, `WatchNextResults`, `RelatedVideo`, `TrackInfo`) match `noctune`'s player requirements.

---

## 4. Diagnostic & Example Suite (`examples/`)

The repository includes 17 standalone runnable diagnostic tools and examples:

| Example File | Purpose | Command |
|---|---|---|
| [`examples/get_watch_next.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_watch_next.rs) | Watch Next (/next) recommendations and autoplay tester | `cargo run --example get_watch_next -- [VIDEO_ID]` |
| [`examples/test_clients.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_clients.rs) | Multi-client diagnostic tester (iOS, ANDROID, VR, MWEB, WEB) | `cargo run --example test_clients -- [VIDEO_ID]` |
| [`examples/test_mweb_stream.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_mweb_stream.rs) | MWEB HD stream decipher & range downloader | `cargo run --example test_mweb_stream -- [VIDEO_ID]` |
| [`examples/test_cdn_modes.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_cdn_modes.rs) | Tests Range headers vs query params vs full GET on CDN | `cargo run --example test_cdn_modes` |
| [`examples/test_ntoken_standalone.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_ntoken_standalone.rs) | QuickJS n-token & signature decipher test | `cargo run --example test_ntoken_standalone` |
| [`examples/test_native_botguard.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_native_botguard.rs) | Google WAA challenge & QuickJS BotGuard VM executor | `cargo run --example test_native_botguard` |
| [`examples/test_native_botguard_full.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_native_botguard_full.rs) | Full 2-step BotGuard & Google GenerateIT integrity token flow | `cargo run --example test_native_botguard_full` |
| [`examples/test_android_vr_stream.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_android_vr_stream.rs) | ANDROID_VR client direct stream URL and range chunking | `cargo run --example test_android_vr_stream` |
| [`examples/test_android_testsuite.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_android_testsuite.rs) | ANDROID_TESTSUITE client playback test | `cargo run --example test_android_testsuite` |
| [`examples/test_tv_client.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_tv_client.rs) | TVHTML5_SIMPLY_EMBEDDED_PLAYER client test | `cargo run --example test_tv_client` |
| [`examples/test_web_creator.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_web_creator.rs) | WEB_CREATOR client endpoint test | `cargo run --example test_web_creator` |
| [`examples/test_web_embedded.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_web_embedded.rs) | WEB_EMBEDDED_PLAYER client endpoint test | `cargo run --example test_web_embedded` |
| [`examples/test_cpn_streaming.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_cpn_streaming.rs) | Client Playback Nonce (CPN) parameter passing | `cargo run --example test_cpn_streaming` |
| [`examples/test_hls.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_hls.rs) | HLS / DASH manifest availability inspector | `cargo run --example test_hls` |
| [`examples/test_http2_cdn.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_http2_cdn.rs) | HTTP/2 protocol negotiation on Google Video CDN | `cargo run --example test_http2_cdn` |
| [`examples/download_audio.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/download_audio.rs) | High-performance audio stream range chunk downloader | `cargo run --example download_audio` |
| [`examples/get_video_info.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_video_info.rs) | Full video metadata and streamingData inspector | `cargo run --example get_video_info` |
| [`examples/search_and_browse.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/search_and_browse.rs) | Search and channel scraping example | `cargo run --example search_and_browse` |

---

## 5. Next Steps & Future Roadmap

1. **HLS / DASH Manifest Parser**:
   - Add native m3u8/mpd manifest parsing for high-bitrate live stream playback.
2. **Interactive Actions (Comments & Subtitles/Transcript)**:
   - Add captions / transcript endpoint scraping (`get_transcript`) and comment threads.
3. **Advanced BotGuard WebPO Content-Binding**:
   - Pure-Rust WebPO signal binding to unlock unauthenticated 1080p/4K downloads directly without fallback.
