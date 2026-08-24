# Parser: AST & Traversals (`src/parser/`)

```rust
use innertube_rs::parser::{Parser, NodeListExt, YTNode};
```

---

## 1. `Parser`

### `Parser::parse_tree(val: &Value) -> Vec<YTNode>`
Recursively walks an arbitrary InnerTube JSON tree, inspecting keys and transforming every recognized renderer or view model into its typed `YTNode` equivalent.

---

## 2. `NodeListExt` Trait

Provides convenient filtering methods on `[YTNode]`:

```rust
pub trait NodeListExt {
    fn find_videos(&self) -> Vec<&VideoNode>;
    fn find_shorts(&self) -> Vec<&ShortNode>;
    fn find_playlists(&self) -> Vec<&PlaylistNode>;
    fn find_playlist_videos(&self) -> Vec<&PlaylistVideoNode>;
    fn find_channels(&self) -> Vec<&ChannelCardNode>;
    fn find_music_items(&self) -> Vec<&MusicResponsiveListItemNode>;
    fn find_comments(&self) -> Vec<&CommentNode>;
    fn find_comment_threads(&self) -> Vec<&CommentThreadNode>;
    fn find_posts(&self) -> Vec<&PostNode>;
    fn find_live_chat_messages(&self) -> Vec<&LiveChatMessageNode>;
    fn find_continuation_token(&self) -> Option<&str>;
}
```
