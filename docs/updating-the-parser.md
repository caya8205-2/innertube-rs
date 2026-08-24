# Updating the InnerTube AST Parser

YouTube frequently updates its JSON renderers and view model schemas. `innertube-rs` implements a strongly typed, resilient recursive AST parser in pure Rust (`src/parser/`) designed for zero allocation overhead and high resilience against YouTube UI mutations.

---

## 1. Modular AST Architecture

The parser traverses JSON payload trees recursively and constructs strongly-typed `YTNode` variants:

```
src/parser/
├── mod.rs                        # Parser::parse_tree, NodeListExt trait
└── nodes/
    ├── mod.rs                    # YTNode polymorphic enum dispatch
    ├── video.rs                  # VideoNode (videoRenderer, compactVideoRenderer, lockupViewModel)
    ├── short.rs                  # ShortNode (reelItemRenderer, shortsLockupViewModel)
    ├── playlist.rs               # PlaylistNode, PlaylistVideoNode
    ├── channel.rs                # ChannelHeaderNode, ChannelCardNode
    ├── music.rs                  # MusicResponsiveListItemNode, MusicTwoRowItemNode
    ├── comments.rs               # CommentNode, CommentThreadNode
    ├── post.rs                   # PostNode (BackstagePost, Polls)
    ├── livechat.rs               # LiveChatMessageNode (LiveChat, SuperChat, Memberships)
    ├── continuation.rs           # ContinuationNode (pagination tokens)
    ├── containers.rs             # TabNode, SectionList, RichGrid
    └── misc/
        ├── text.rs               # TextNode, TextRunNode
        ├── thumbnail.rs          # ThumbnailNode, ThumbnailListNode
        ├── navigation.rs         # NavigationEndpointNode
        └── author.rs             # AuthorNode
```

---

## 2. Adding or Extending a Node Parser in Rust

When YouTube introduces a new renderer or view model:

### Step 1: Create or Update Node Struct (`src/parser/nodes/<domain>.rs`)
Define the strongly typed struct with `serde` support:

```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::parser::nodes::misc::text::TextNode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomItemNode {
    pub id: String,
    pub title: String,
}

impl CustomItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("customItemRenderer")
            .or_else(|| val.get("customItemViewModel"))?;

        let id = node.get("id")?.as_str()?.to_string();
        let title = TextNode::from_value(node.get("title")?)?.text;

        Some(Self { id, title })
    }
}
```

### Step 2: Register in `YTNode` Enum (`src/parser/nodes/mod.rs`)
Add the variant to `pub enum YTNode` and dispatch in `YTNode::parse(val: &Value)`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum YTNode {
    // ... existing nodes
    CustomItem(CustomItemNode),
}

impl YTNode {
    pub fn parse(val: &Value) -> Option<Self> {
        // ...
        if val.get("customItemRenderer").is_some() || val.get("customItemViewModel").is_some() {
            if let Some(c) = CustomItemNode::from_value(val) {
                return Some(YTNode::CustomItem(c));
            }
        }
        None
    }
}
```

### Step 3: Add Ergonomic Helper to `NodeListExt` (`src/parser/mod.rs`)

```rust
pub trait NodeListExt {
    // ...
    fn find_custom_items(&self) -> Vec<&CustomItemNode>;
}

impl NodeListExt for [YTNode] {
    fn find_custom_items(&self) -> Vec<&CustomItemNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::CustomItem(c) => Some(c),
                _ => None,
            })
            .collect()
    }
}
```

---

## 3. Querying Nodes in Endpoints

Once registered, any endpoint can parse trees with zero boilerplate:

```rust
let parsed_tree = Parser::parse_tree(&raw_json);

for video in parsed_tree.find_videos() {
    println!("Found video: {} ({})", video.title, video.id);
}

if let Some(token) = parsed_tree.find_continuation_token() {
    println!("Next page token: {}", token);
}
```
