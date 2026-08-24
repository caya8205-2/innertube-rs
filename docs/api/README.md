# innertube-rs API Reference

`innertube-rs` provides a structured, high-performance async Rust API.

---

## 1. Core Modules

| Module / Struct | Description | Reference Link |
|---|---|---|
| [`Innertube`](core/innertube.md) | Top-level client containing high-level wrappers for all YouTube features | [core/innertube.md](core/innertube.md) |
| [`Session`](core/session.md) | InnerTube HTTP session, protobuf visitor data, and multi-client dispatch | [core/session.md](core/session.md) |
| [`Player`](core/player.md) | Signature and n-token decipher engine using embedded QuickJS | [core/player.md](core/player.md) |
| [`OAuth2`](core/oauth.md) | Google TV device login flow & token refresh manager | [core/oauth.md](core/oauth.md) |
| [`Actions`](core/actions.md) | Authenticated account mutations (like, subscribe, comment, playlist) | [core/actions.md](core/actions.md) |

---

## 2. Endpoints & Features

| Feature Domain | Endpoint Reference | Models & Payloads Reference |
|---|---|---|
| **Video & Streaming** | [endpoints/player.md](endpoints/player.md) | [models/video.md](models/video.md), [models/format.md](models/format.md) |
| **Search & Autocomplete** | [endpoints/search.md](endpoints/search.md) | [models/search.md](models/search.md) |
| **Channels** | [endpoints/channel.md](endpoints/channel.md) | [models/channel.md](models/channel.md), [models/post.md](models/post.md) |
| **Playlists** | [endpoints/playlist.md](endpoints/playlist.md) | [models/playlist.md](models/playlist.md) |
| **Watch Next & Related** | [endpoints/next.md](endpoints/next.md) | [models/video.md](models/video.md) |
| **Feeds & Navigation** | [endpoints/feed.md](endpoints/feed.md), [endpoints/guide.md](endpoints/guide.md) | [models/account.md](models/account.md) |
| **YouTube Music** | [endpoints/music.md](endpoints/music.md) | [models/music.md](models/music.md) |
| **Comments** | [endpoints/comments.md](endpoints/comments.md) | [models/comments.md](models/comments.md) |
| **Live Chat** | [endpoints/live_chat.md](endpoints/live_chat.md) | [models/live_chat.md](models/live_chat.md) |
| **Transcripts** | [endpoints/transcript.md](endpoints/transcript.md) | [models/transcript.md](models/transcript.md) |
| **Account Feeds** | [endpoints/account.md](endpoints/account.md) | [models/account.md](models/account.md) |
| **Manifest Parsers** | [models/manifest.md](models/manifest.md) | [models/manifest.md](models/manifest.md) |

---

## 3. Modular AST Parser

All InnerTube polymorphic JSON trees are parsed via `Parser::parse_tree(val)` into strongly typed `YTNode` variants:

* **AST Architecture & Traversal**: [parser/ast.md](parser/ast.md)
* **Polymorphic Node Types**: [parser/nodes.md](parser/nodes.md)
