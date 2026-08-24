pub mod nodes;
pub mod registry;

pub use nodes::*;
pub use registry::*;

use serde_json::Value;

/// Central Recursive AST Parser Engine (1:1 port of `src/parser/parser.ts` and `generator.ts`).
pub struct Parser;

impl Parser {
    /// Recursively traverse a YouTube InnerTube JSON response and extract all known `YTNode` instances.
    pub fn parse_tree(val: &Value) -> Vec<YTNode> {
        let mut results = Vec::new();
        Self::traverse_recursive(val, &mut results);
        results
    }

    /// Attempt to parse a single JSON value into a `YTNode`.
    pub fn parse_node(val: &Value) -> Option<YTNode> {
        YTNode::parse(val)
    }

    fn traverse_recursive(val: &Value, results: &mut Vec<YTNode>) {
        if val.is_null() {
            return;
        }

        // Try parsing current node first
        if let Some(node) = YTNode::parse(val) {
            results.push(node);
            // Even if matched, certain nodes like SectionList / RichGrid might contain nested items
        }

        match val {
            Value::Object(map) => {
                for (_, v) in map {
                    Self::traverse_recursive(v, results);
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    Self::traverse_recursive(item, results);
                }
            }
            _ => {}
        }
    }
}

/// Extension trait for ergonomic querying of `Vec<YTNode>`.
pub trait NodeListExt {
    fn find_videos(&self) -> Vec<&VideoNode>;
    fn find_shorts(&self) -> Vec<&ShortNode>;
    fn find_playlists(&self) -> Vec<&PlaylistNode>;
    fn find_playlist_videos(&self) -> Vec<&PlaylistVideoNode>;
    fn find_channels(&self) -> Vec<&ChannelCardNode>;
    fn find_music_items(&self) -> Vec<&MusicResponsiveListItemNode>;
    fn find_comments(&self) -> Vec<&CommentThreadNode>;
    fn find_posts(&self) -> Vec<&PostNode>;
    fn find_continuation_token(&self) -> Option<String>;
    fn find_shelves(&self) -> Vec<&ShelfNode>;
    fn find_tabs(&self) -> Vec<&TabNode>;
    fn find_buttons(&self) -> Vec<&ButtonNode>;
    fn find_menus(&self) -> Vec<&MenuNode>;
}

impl NodeListExt for [YTNode] {
    fn find_videos(&self) -> Vec<&VideoNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Video(v) => Some(v),
                _ => None,
            })
            .collect()
    }

    fn find_shorts(&self) -> Vec<&ShortNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Short(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    fn find_playlists(&self) -> Vec<&PlaylistNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Playlist(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    fn find_playlist_videos(&self) -> Vec<&PlaylistVideoNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::PlaylistVideo(pv) => Some(pv),
                _ => None,
            })
            .collect()
    }

    fn find_channels(&self) -> Vec<&ChannelCardNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::ChannelCard(c) => Some(c),
                _ => None,
            })
            .collect()
    }

    fn find_music_items(&self) -> Vec<&MusicResponsiveListItemNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::MusicItem(m) => Some(m),
                _ => None,
            })
            .collect()
    }

    fn find_comments(&self) -> Vec<&CommentThreadNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::CommentThread(ct) => Some(ct),
                _ => None,
            })
            .collect()
    }

    fn find_posts(&self) -> Vec<&PostNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Post(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    fn find_continuation_token(&self) -> Option<String> {
        self.iter().find_map(|n| match n {
            YTNode::Continuation(c) => Some(c.token.clone()),
            _ => None,
        })
    }

    fn find_shelves(&self) -> Vec<&ShelfNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Shelf(s) => Some(s),
                _ => None,
            })
            .collect()
    }

    fn find_tabs(&self) -> Vec<&TabNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Tab(t) => Some(t),
                _ => None,
            })
            .collect()
    }

    fn find_buttons(&self) -> Vec<&ButtonNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Button(b) => Some(b),
                _ => None,
            })
            .collect()
    }

    fn find_menus(&self) -> Vec<&MenuNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Menu(m) => Some(m),
                _ => None,
            })
            .collect()
    }
}
