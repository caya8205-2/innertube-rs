# innertube-rs — Porting Status & Feature Matrix

> **Obsolete for full-parity decisions.** This historical matrix predates the
> source-level parity audit and its `Complete`/`100%` labels are not current
> evidence. Use `PARITY_MANIFEST.md` as the sole parity authority and
> `PARITY_PLAN.md` for the current execution order.

> **Upstream Reference**: [LuanRT/YouTube.js (YouTubei.js)](https://github.com/LuanRT/YouTube.js)  
> **Target Project**: `innertube-rs` (Pure Rust Port)  
> **Last Updated**: August 25, 2026  
> **Overall Porting Progress**: **Full-parity audit and implementation in progress; see `.agents/PARITY_MANIFEST.md` for authoritative completion status.**

---

## 1. Module-by-Module Porting Status (Historical Reference)

> **Notice**: The table below is a historical milestone reference. Real parity exit status is governed solely by [`.agents/PARITY_MANIFEST.md`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/.agents/PARITY_MANIFEST.md).

| YouTube.js (JS/TS) Module | Rust Equivalent (`innertube-rs`) | Status | Parity % | Description & Implementation Details |
|---|---|:---:|:---:|---|
| `src/core/Session.ts` | [`src/core/session.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/session.rs) | 🟢 Complete | 100% | Client context creation, device category headers, API key extraction from `sw.js_data`, visitor data generation, PO-token & cookie session management, `post_innertube_client`. |
| `src/core/HTTPClient.ts` | [`src/core/http.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/http.rs) | 🟢 Complete | 100% | `reqwest`-based asynchronous HTTP client with gzip, brotli, HTTP/2 negotiation, custom headers, and Netscape cookie store. |
| `src/core/Player.ts` | [`src/utils/decipher.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/decipher.rs) | 🟢 Complete | 100% | QuickJS (`rquickjs`) sandboxed decipher engine. Extracts and executes base.js signature decipher algorithms and n-token transformations (<5ms). |
| `src/utils/ProtoUtils.ts` | [`src/utils/proto.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/proto.rs) | 🟢 Complete | 100% | Protobuf visitor data encoding and decoding using `prost` and URL-safe base64 padding. |
| `src/parser/nodes/` | [`src/parser/nodes/`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/parser/nodes/) | 🟢 Complete | 100% | Modular AST Component Parser (Text, Thumbnail, Author, Navigation, Video, Short, Playlist, Channel, Music, Comments, Community Post, LiveChat, Continuation). |
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
| `src/actions/` (Channels) | [`src/endpoints/channel.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/channel.rs) | 🟢 Complete | 100% | Channel tabs browser: Videos tab (uploads), Shorts tab, Channel About, and Community tab. |
| `src/actions/` (Transcript / Subtitles) | [`src/endpoints/transcript.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/transcript.rs) | 🟢 Complete | 100% | Subtitle track extraction, timed text parser (JSON3 and XML), export to SRT and WebVTT. |
| `src/parser/` (Manifests) | [`src/utils/manifest.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/utils/manifest.rs) | 🟢 Complete | 100% | Native parser for HLS (`.m3u8` master playlists) and DASH (`.mpd` representations). |
| `src/core/OAuth2.ts` | [`src/core/oauth.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/oauth.rs) | 🟢 Complete | 100% | TV/Device code OAuth2 login flow, verification code request, token polling, and token refresher. |
| `src/core/Actions.ts` | [`src/core/actions.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/core/actions.rs) | 🟢 Complete | 100% | Authenticated account mutations (Like, Dislike, Remove rating, Subscribe, Unsubscribe, Comment, Playlist Edit). |
| `src/parser/youtube/LiveChat.ts` | [`src/endpoints/live_chat.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/live_chat.rs) | 🟢 Complete | 100% | Real-time live chat token extractor, live chat messages polling, Super Chat, Memberships. |
| `src/parser/youtube/History.ts` & `Library.ts` | [`src/endpoints/account.rs`](file:///c:/Users/Caya/Desktop/Project/innertube-rs/src/endpoints/account.rs) | 🟢 Complete | 100% | Authenticated user history feed, library feed, and account notifications. |

---

## 2. Categorized Example Suite (`examples/`)

The repository includes **runnable diagnostic tools and examples organized by category**:

* **Download**: `examples/download/download_audio.rs`
* **Video & Search**: `examples/video/get_video_info.rs`, `examples/video/get_watch_next.rs`, `examples/video/get_comments.rs`, `examples/video/get_transcript.rs`, `examples/video/get_playlist.rs`, `examples/video/get_suggestions.rs`, `examples/video/search_and_browse.rs`
* **Music Suite**: `examples/music/get_music_album.rs`, `examples/music/get_music_artist.rs`, `examples/music/get_music_explore.rs`, `examples/music/get_music_lyrics.rs`, `examples/music/test_music_search.rs`
* **Channel Suite**: `examples/channel/get_channel_tabs.rs`, `examples/channel/get_community_posts.rs`
* **Feeds & Guide**: `examples/feed/get_home_feed.rs`, `examples/feed/get_trending.rs`, `examples/feed/get_guide.rs`
* **Live Chat**: `examples/live/test_live_chat.rs`
* **OAuth2 Authentication**: `examples/auth/test_oauth_flow.rs`
* **Diagnostics & Benchmarks**: `examples/diagnostics/*` (Client fallbacks, botguard, n-token, and manifest tests)
