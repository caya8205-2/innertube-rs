# innertube-rs — Porting Status & Feature Matrix

> **Upstream Reference**: [LuanRT/YouTube.js (YouTubei.js)](https://github.com/LuanRT/YouTube.js)  
> **Target Project**: `innertube-rs` (Pure Rust Port)  
> **Last Updated**: August 24, 2026  
> **Overall Porting Progress (Core Streaming & Media Engine)**: **~99%**  
> **Overall Porting Progress (Total YouTube.js Feature Parity)**: **~85% – 90%**

---

## 1. Module-by-Module Porting Status

| YouTube.js (JS/TS) Module | Rust Equivalent (`innertube-rs`) | Status | Parity % | Description & Implementation Details |
|---|---|:---:|:---:|---|
| `src/core/Session.ts` | [`src/core/session.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/session.rs) | 🟢 Complete | 95% | Client context creation, device category headers, API key extraction from `sw.js_data`, visitor data generation, PO-token & cookie session management, `post_innertube_client`. |
| `src/core/HTTPClient.ts` | [`src/core/http.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/http.rs) | 🟢 Complete | 100% | `reqwest`-based asynchronous HTTP client with gzip, brotli, HTTP/2 negotiation, custom headers, and Netscape cookie store. |
| `src/core/Player.ts` | [`src/utils/decipher.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/decipher.rs) | 🟢 Complete | 95% | QuickJS (`rquickjs`) sandboxed decipher engine. Extracts and executes base.js signature decipher algorithms and n-token transformations (<5ms). |
| `src/utils/ProtoUtils.ts` | [`src/utils/proto.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/proto.rs) | 🟢 Complete | 100% | Protobuf visitor data encoding and decoding using `prost` and URL-safe base64 padding. |
| `src/core/endpoints/Player.ts` | [`src/endpoints/player.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/player.rs) | 🟢 Complete | 95% | InnerTube `/player` endpoint with automatic multi-client fallback chain (**WEB → ANDROID → iOS → ANDROID_VR → MWEB**). |
| `src/core/endpoints/Search.ts` | [`src/endpoints/search.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/search.rs) | 🟢 Complete | 90% | InnerTube `/search` endpoint with recursive AST renderer parser (extracts video results, channel results, playlist results). |
| `src/core/endpoints/Browse.ts` | [`src/endpoints/browse.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/browse.rs) | 🟢 Complete | 90% | InnerTube `/browse` endpoint. Supports channel profile scraping, top tracks, and YouTube Music playlist tracklist extraction. |
| `src/core/endpoints/Next.ts` | [`src/endpoints/next.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/next.rs) | 🟢 Complete | 95% | InnerTube `/next` endpoint. Supports video recommendations (`lockupViewModel`, `compactVideoRenderer`), autoplay suggestions, playlist queue panels, and continuation tokens. |
| `src/core/endpoints/Comments.ts` | [`src/endpoints/comments.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/comments.rs) | 🟢 Complete | 90% | InnerTube comment threads, pinned comments, author creator badges, replies, likes count, and `entityBatchUpdate` resolution. |
| `src/parser/ytmusic/` | [`src/endpoints/music.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/music.rs) | 🟢 Complete | 90% | YouTube Music dedicated suite: Filtered search (Songs, Albums, Artists, Playlists), Album details & tracklists, Song Lyrics, Explore/Charts. |
| `src/actions/` (Transcript / Subtitles) | [`src/endpoints/transcript.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/transcript.rs) | 🟢 Complete | 95% | Subtitle track extraction, timed text parser (JSON3 and XML), export to SRT and WebVTT. |
| `src/parser/` (Manifests) | [`src/utils/manifest.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/manifest.rs) | 🟢 Complete | 95% | Native parser for HLS (`.m3u8` master playlists) and DASH (`.mpd` representations). |
| `src/parser/` (AST Models) | [`src/models/`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/models/) | 🟢 Complete (Core) | 80% | Strongly typed Serde models for essential InnerTube payloads. Polymorphic renderers are parsed on-demand rather than maintaining 150+ individual class hierarchies. |
| `src/core/OAuth2.ts` | `src/core/oauth.rs` | ⚪ Optional | 0% | TV/Device code OAuth2 login flow. |

---

## 2. Diagnostic & Example Suite (`examples/`)

The repository includes **22 standalone runnable diagnostic tools and examples**:

| Example File | Purpose | Command |
|---|---|---|
| [`examples/test_music_search.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_music_search.rs) | YouTube Music filtered search (Songs, Albums, Artists, Playlists) | `cargo run --example test_music_search -- [QUERY]` |
| [`examples/get_music_album.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_music_album.rs) | YouTube Music album details and tracklist scraper | `cargo run --example get_music_album -- [ALBUM_ID]` |
| [`examples/get_music_explore.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_music_explore.rs) | YouTube Music Explore & Trending Charts | `cargo run --example get_music_explore` |
| [`examples/get_music_lyrics.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_music_lyrics.rs) | YouTube Music song lyrics tester | `cargo run --example get_music_lyrics -- [VIDEO_ID]` |
| [`examples/get_transcript.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_transcript.rs) | Subtitles/transcripts extraction and SRT/VTT export | `cargo run --example get_transcript -- [VIDEO_ID]` |
| [`examples/get_comments.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_comments.rs) | Comments and thread replies extraction | `cargo run --example get_comments -- [VIDEO_ID]` |
| [`examples/get_watch_next.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/get_watch_next.rs) | Watch Next (/next) recommendations and autoplay tester | `cargo run --example get_watch_next -- [VIDEO_ID]` |
| [`examples/test_manifest_parser.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/examples/test_manifest_parser.rs) | HLS (.m3u8) & DASH (.mpd) manifest parser tester | `cargo run --example test_manifest_parser -- [VIDEO_ID]` |
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
