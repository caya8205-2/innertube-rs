# Parser: YTNodes (`src/parser/nodes/`)

```rust
use innertube_rs::parser::YTNode;
```

---

## 1. `YTNode` Enum

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum YTNode {
    Video(VideoNode),
    Short(ShortNode),
    Playlist(PlaylistNode),
    PlaylistVideo(PlaylistVideoNode),
    ChannelHeader(ChannelHeaderNode),
    ChannelCard(ChannelCardNode),
    MusicResponsiveListItem(MusicResponsiveListItemNode),
    MusicTwoRowItem(MusicTwoRowItemNode),
    MusicDescriptionShelf(MusicDescriptionShelfNode),
    Comment(CommentNode),
    CommentThread(CommentThreadNode),
    Post(PostNode),
    LiveChatMessage(LiveChatMessageNode),
    Continuation(ContinuationNode),
    Tab(TabNode),
    Raw(Value),
}
```

---

## 2. Primitive Nodes (`src/parser/nodes/misc/`)

* **`TextNode`**: Polymorphic text extraction (`runs`, `simpleText`, `accessibilityData`).
* **`ThumbnailNode` & `ThumbnailListNode`**: Thumbnail URLs with `best_url()` and `first_url()` helpers.
* **`AuthorNode`**: Channel owner details, badges, and avatars.
* **`NavigationEndpointNode`**: Browse IDs, video IDs, and continuation command tokens.
