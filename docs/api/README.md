# innertube-rs API Reference

`innertube-rs` provides a structured, high-performance async Rust API.

---

## 1. Core Modules

| Module / Struct | Description | Link |
|---|---|---|
| [`Innertube`](classes/Innertube.md) | Top-level client containing high-level wrappers for all YouTube features | [Innertube.md](classes/Innertube.md) |
| [`Session`](classes/Session.md) | InnerTube HTTP session, protobuf visitor data, and multi-client dispatch | [Session.md](classes/Session.md) |
| [`Player`](classes/Player.md) | Signature and n-token decipher engine using embedded QuickJS | [Player.md](classes/Player.md) |
| [`OAuth2`](classes/OAuth2.md) | Google TV device login flow & token refresh manager | [OAuth2.md](classes/OAuth2.md) |
| [`Actions`](classes/Actions.md) | Authenticated account mutations (like, subscribe, comment, playlist) | [Actions.md](classes/Actions.md) |

---

## 2. Endpoints & Features

| Feature Domain | Endpoint Functions | Models & Payloads |
|---|---|---|
| **Video & Streaming** | `get_video_info`, `get_stream_url`, `select_format` | `VideoInfo`, `Format`, `FormatFilter` |
| **Search & Autocomplete** | `search`, `get_search_suggestions` | `SearchResults`, `SearchSuggestion` |
| **Channels** | `get_channel_about`, `get_channel_videos`, `get_channel_shorts`, `get_channel_community` | `ChannelAbout`, `ChannelVideosResponse`, `CommunityPostsResponse` |
| **Playlists** | `get_playlist`, `get_playlist_continuation` | `PlaylistView`, `PlaylistVideoItem` |
| **Watch Next & Related** | `get_watch_next`, `get_related_videos` | `WatchNextResults`, `RelatedVideo` |
| **Feeds & Navigation** | `get_home_feed`, `get_trending`, `get_hashtag_feed`, `get_guide` | `HomeFeed`, `TrendingFeed`, `GuideResponse` |
| **YouTube Music** | `search_music`, `get_music_artist`, `get_music_album`, `get_music_lyrics`, `get_music_explore`, `get_music_home` | `MusicArtistPage`, `MusicAlbumView`, `MusicLyrics` |
| **Comments** | `get_comments`, `get_comment_replies` | `CommentsResult`, `CommentThread`, `Comment` |
| **Live Chat** | `get_live_chat_token`, `get_live_chat` | `LiveChatResponse`, `LiveChatMessage` |
| **Transcripts** | `get_transcript_tracks`, `get_transcript` | `Transcript`, `TranscriptTrack`, `TranscriptSegment` |
| **Account Feeds** | `get_history`, `get_library`, `get_notifications` | `HistoryFeed`, `LibraryFeed`, `AccountNotificationsResponse` |

---

## 3. Modular AST Parser

All InnerTube polymorphic JSON trees are parsed via `Parser::parse_tree(val)` into strongly typed `YTNode` variants:

* **Video & Shorts**: `VideoNode`, `ShortNode`
* **Playlists**: `PlaylistNode`, `PlaylistVideoNode`
* **Channels**: `ChannelHeaderNode`, `ChannelCardNode`
* **Music**: `MusicResponsiveListItemNode`, `MusicTwoRowItemNode`, `MusicDescriptionShelfNode`
* **Community**: `PostNode`
* **Live Chat**: `LiveChatMessageNode`
* **Comments**: `CommentNode`, `CommentThreadNode`
* **Continuations**: `ContinuationNode`
