# innertube-rs — Porting Status & Feature Matrix

> **Upstream Reference**: [LuanRT/YouTube.js (YouTubei.js)](https://github.com/LuanRT/YouTube.js)  
> **Target Project**: `innertube-rs` (Pure Rust Port)  
> **Last Updated**: August 24, 2026  
> **Overall Porting Progress (Read Operations & Media Streaming)**: **100% Complete**  
> **Overall Porting Progress (Total YouTube.js Feature Parity)**: **~95%** (Remaining 5%: Account Mutations, OAuth2, LiveChat)

---

## 1. Module-by-Module Porting Status

| YouTube.js (JS/TS) Module | Rust Equivalent (`innertube-rs`) | Status | Parity % | Description & Implementation Details |
|---|---|:---:|:---:|---|
| `src/core/Session.ts` | [`src/core/session.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/session.rs) | 🟢 Complete | 100% | Client context creation, device category headers, API key extraction from `sw.js_data`, visitor data generation, PO-token & cookie session management, `post_innertube_client`. |
| `src/core/HTTPClient.ts` | [`src/core/http.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/http.rs) | 🟢 Complete | 100% | `reqwest`-based asynchronous HTTP client with gzip, brotli, HTTP/2 negotiation, custom headers, and Netscape cookie store. |
| `src/core/Player.ts` | [`src/utils/decipher.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/decipher.rs) | 🟢 Complete | 100% | QuickJS (`rquickjs`) sandboxed decipher engine. Extracts and executes base.js signature decipher algorithms and n-token transformations (<5ms). |
| `src/utils/ProtoUtils.ts` | [`src/utils/proto.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/proto.rs) | 🟢 Complete | 100% | Protobuf visitor data encoding and decoding using `prost` and URL-safe base64 padding. |
| `src/parser/nodes/` | [`src/parser/nodes/`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/parser/nodes/) | 🟢 Complete | 100% | Modular AST Component Parser (Text, Thumbnail, Author, Navigation, Video, Short, Playlist, Channel, Music, Comments, Continuation). |
| `src/core/endpoints/Player.ts` | [`src/endpoints/player.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/player.rs) | 🟢 Complete | 100% | InnerTube `/player` endpoint with automatic multi-client fallback chain (**WEB → ANDROID → iOS → ANDROID_VR → MWEB**). |
| `src/core/endpoints/Search.ts` | [`src/endpoints/search.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/search.rs) | 🟢 Complete | 100% | InnerTube `/search` endpoint delegating extraction to `Parser::parse_tree` (Video, ChannelCard, Playlist). |
| `src/core/endpoints/Browse.ts` | [`src/endpoints/browse.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/browse.rs) | 🟢 Complete | 100% | InnerTube `/browse` endpoint. Supports channel profile scraping, top tracks, and playlist extraction. |
| `src/core/endpoints/Next.ts` | [`src/endpoints/next.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/next.rs) | 🟢 Complete | 100% | InnerTube `/next` endpoint. Video recommendations (`lockupViewModel`, `compactVideoRenderer`), autoplay suggestions, playlist queue panels, and continuation tokens. |
| `src/core/endpoints/Comments.ts` | [`src/endpoints/comments.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/comments.rs) | 🟢 Complete | 100% | InnerTube comment threads, pinned comments, author creator badges, replies, likes count, and `entityBatchUpdate` resolution. |
| `src/parser/youtube/HomeFeed.ts` | [`src/endpoints/feed.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/feed.rs) | 🟢 Complete | 100% | Home Feed (`FEwhat_to_watch`) with category filter chips and continuation tokens. |
| `src/parser/youtube/Explore.ts` | [`src/endpoints/feed.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/feed.rs) | 🟢 Complete | 100% | Trending Feed (`FEtrending`) with category tabs (*Now, Music, Gaming, Movies*). |
| `src/parser/youtube/HashtagFeed.ts` | [`src/endpoints/feed.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/feed.rs) | 🟢 Complete | 100% | Hashtag Feed (`FEhashtag`) with header metadata and video lists. |
| `src/parser/youtube/Guide.ts` | [`src/endpoints/guide.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/guide.rs) | 🟢 Complete | 100% | Navigation sidebar endpoint (`/guide`) with menu sections and explore items. |
| `src/parser/ytmusic/` | [`src/endpoints/music.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/music.rs) | 🟢 Complete | 100% | YouTube Music dedicated suite: Filtered search, Albums, Dedicated Artist Page, Home Feed, Lyrics, Explore/Charts. |
| `src/actions/` (Suggestions) | [`src/endpoints/suggestions.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/suggestions.rs) | 🟢 Complete | 100% | Search autocomplete query endpoint supporting both YouTube and YouTube Music. |
| `src/actions/` (Playlists) | [`src/endpoints/playlist.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/playlist.rs) | 🟢 Complete | 100% | Full playlist scraper supporting metadata headers, video lists, and continuation pagination. |
| `src/actions/` (Channels) | [`src/endpoints/channel.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/channel.rs) | 🟢 Complete | 100% | Channel tabs browser: Videos tab (uploads), Shorts tab (`shortsLockupViewModel`), and Channel About metadata. |
| `src/actions/` (Transcript / Subtitles) | [`src/endpoints/transcript.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/transcript.rs) | 🟢 Complete | 100% | Subtitle track extraction, timed text parser (JSON3 and XML), export to SRT and WebVTT. |
| `src/parser/` (Manifests) | [`src/utils/manifest.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/manifest.rs) | 🟢 Complete | 100% | Native parser for HLS (`.m3u8` master playlists) and DASH (`.mpd` representations). |
| `src/core/OAuth2.ts` | `src/core/oauth.rs` | ⚪ Remaining 5% | 0% | TV/Device code OAuth2 login flow. |
| `src/core/Actions.ts` | `src/core/actions.rs` | ⚪ Remaining 5% | 0% | Authenticated account mutations (Like, Dislike, Subscribe, Unsubscribe, Comment, Playlist Edit). |
| `src/parser/youtube/LiveChat.ts` | `src/endpoints/live_chat.rs` | ⚪ Remaining 5% | 0% | Real-time live chat polling and parser. |

---

## 2. Diagnostic & Example Suite (`examples/`)

The repository includes **29 standalone runnable diagnostic tools and examples**:

* `examples/get_suggestions.rs`
* `examples/get_playlist.rs`
* `examples/get_channel_tabs.rs`
* `examples/get_home_feed.rs`
* `examples/get_trending.rs`
* `examples/get_guide.rs`
* `examples/test_music_search.rs`
* `examples/get_music_album.rs`
* `examples/get_music_artist.rs`
* `examples/get_music_explore.rs`
* `examples/get_music_lyrics.rs`
* `examples/get_comments.rs`
* `examples/get_transcript.rs`
* `examples/get_watch_next.rs`
* `examples/test_manifest_parser.rs`
* `examples/test_clients.rs`
* `examples/test_mweb_stream.rs`
* `examples/test_cdn_modes.rs`
* `examples/test_ntoken_standalone.rs`
* `examples/test_native_botguard.rs`
* `examples/test_native_botguard_full.rs`
* `examples/test_android_vr_stream.rs`
* `examples/test_android_testsuite.rs`
* `examples/test_tv_client.rs`
* `examples/test_web_creator.rs`
* `examples/test_web_embedded.rs`
* `examples/test_cpn_streaming.rs`
* `examples/test_hls.rs`
* `examples/test_http2_cdn.rs`
* `examples/download_audio.rs`
* `examples/get_video_info.rs`
* `examples/search_and_browse.rs`
