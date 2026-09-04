pub mod nodes;
pub mod registry;
pub mod response;

pub use nodes::*;
pub use registry::*;
pub use response::*;

use serde_json::Value;

/// Central Recursive AST Parser Engine (1:1 port of `src/parser/parser.ts` and `generator.ts`).
pub struct Parser;

impl Parser {
    /// Recursively traverse a YouTube InnerTube JSON response and extract all known `YTNode` instances.
    pub fn parse_tree(val: &Value) -> Vec<YTNode> {
        Self::parse_tree_reporting(val)
    }

    /// Like [`Parser::parse_tree`], additionally skipping legacy
    /// `IGNORED_LIST` renderers and reporting unknown renderer keys to the
    /// installed parser error handler.
    pub fn parse_tree_reporting(val: &Value) -> Vec<YTNode> {
        let mut results = Vec::new();
        Self::traverse_recursive(val, &mut results);
        results
    }

    /// Attempt to parse a single JSON value into a `YTNode`.
    pub fn parse_node(val: &Value) -> Option<YTNode> {
        YTNode::parse(val)
    }

    /// Parse the response body only (excluding the top-level `header`),
    /// mirroring legacy `Feed.#getBodyContinuations` header-memo exclusion.
    pub fn parse_body_tree(val: &Value) -> Vec<YTNode> {
        if let Value::Object(map) = val {
            let body: serde_json::Map<String, Value> = map
                .iter()
                .filter(|(k, _)| k.as_str() != "header")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            return Self::parse_tree_reporting(&Value::Object(body));
        }
        Self::parse_tree_reporting(val)
    }

    fn traverse_recursive(val: &Value, results: &mut Vec<YTNode>) {
        if val.is_null() {
            return;
        }

        // Skip ignored renderers silently (legacy IGNORED_LIST).
        if let Value::Object(map) = val {
            if map.len() == 1 {
                if let Some(key) = map.keys().next() {
                    let class_name = response::sanitize_class_name(key);
                    if response::IGNORED_LIST.contains(&class_name.as_str()) {
                        return;
                    }
                }
            }
        }

        // Try parsing current node first
        if let Some(node) = YTNode::parse(val) {
            results.push(node);
            // Even if matched, certain nodes like SectionList / RichGrid might contain nested items
        } else if let Value::Object(map) = val {
            // Report unknown single-key renderer/view payloads.
            if map.len() == 1 {
                if let Some(key) = map.keys().next() {
                    if key.ends_with("Renderer") || key.ends_with("Model") || key.ends_with("View") {
                        response::report_parser_error(response::ParserError {
                            error_type: "class_not_found".to_string(),
                            class_name: response::sanitize_class_name(key),
                            detail: None,
                        });
                    }
                }
            }
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
    fn find_channels(&self) -> Vec<&ChannelNode>;
    fn find_music_items(&self) -> Vec<&MusicResponsiveListItemNode>;
    fn find_comments(&self) -> Vec<&CommentThreadNode>;
    fn find_posts(&self) -> Vec<&PostNode>;
    /// Community posts (`BackstagePost`), legacy `Feed.posts`.
    fn find_backstage_posts(&self) -> Vec<&crate::parser::nodes::misc::music_shorts_misc::BackstagePostNode>;
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

    fn find_channels(&self) -> Vec<&ChannelNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::Channel(c) => Some(c),
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

    fn find_backstage_posts(
        &self,
    ) -> Vec<&crate::parser::nodes::misc::music_shorts_misc::BackstagePostNode> {
        self.iter()
            .filter_map(|n| match n {
                YTNode::BackstagePost(p) => Some(p),
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
