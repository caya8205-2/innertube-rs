pub mod channel;
pub mod comments;
pub mod containers;
pub mod continuation;
pub mod livechat;
pub mod misc;
pub mod music;
pub mod playlist;
pub mod post;
pub mod short;
pub mod video;

pub use channel::{ChannelCardNode, ChannelHeaderNode};
pub use comments::{CommentNode, CommentThreadNode};
pub use containers::{
    ItemSectionNode, RichGridNode, RichShelfNode, SectionListNode, ShelfNode, TabNode,
};
pub use continuation::ContinuationNode;
pub use livechat::LiveChatMessageNode;
pub use misc::{
    AuthorNode, BrowseEndpointNode, ContinuationEndpointNode, LikeEndpointNode,
    NavigationEndpointNode, ReelWatchEndpointNode, SearchEndpointNode, SubscribeEndpointNode,
    TextNode, TextRunNode, ThumbnailListNode, ThumbnailNode, WatchEndpointNode,
};
pub use music::{MusicDescriptionShelfNode, MusicResponsiveListItemNode, MusicTwoRowItemNode};
pub use playlist::{PlaylistNode, PlaylistVideoNode};
pub use post::PostNode;
pub use short::ShortNode;
pub use video::VideoNode;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Central strongly typed Polymorphic InnerTube Node representation (1:1 port of `YTNode.ts`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum YTNode {
    Video(VideoNode),
    Short(ShortNode),
    Playlist(PlaylistNode),
    PlaylistVideo(PlaylistVideoNode),
    ChannelHeader(ChannelHeaderNode),
    ChannelCard(ChannelCardNode),
    MusicItem(MusicResponsiveListItemNode),
    MusicCard(MusicTwoRowItemNode),
    MusicDescriptionShelf(MusicDescriptionShelfNode),
    Comment(CommentNode),
    CommentThread(CommentThreadNode),
    Post(PostNode),
    Continuation(ContinuationNode),
    SectionList(SectionListNode),
    ItemSection(ItemSectionNode),
    RichGrid(RichGridNode),
    Shelf(ShelfNode),
    RichShelf(RichShelfNode),
    Tab(TabNode),
    LiveChat(LiveChatMessageNode),
}

impl YTNode {
    /// Attempt to parse a single JSON value into a known `YTNode` variant.
    pub fn parse(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // 1. Check for Continuation
        if val.get("continuationItemRenderer").is_some()
            || val.get("continuationItemViewModel").is_some()
        {
            if let Some(c) = ContinuationNode::from_value(val) {
                return Some(YTNode::Continuation(c));
            }
        }

        // 2. Check for Video Renderers
        if val.get("videoRenderer").is_some()
            || val.get("compactVideoRenderer").is_some()
            || val.get("gridVideoRenderer").is_some()
        {
            if let Some(v) = VideoNode::from_value(val) {
                return Some(YTNode::Video(v));
            }
        }

        // 3. Check for Shorts Renderers
        if val.get("reelItemRenderer").is_some() || val.get("shortsLockupViewModel").is_some() {
            if let Some(s) = ShortNode::from_value(val) {
                return Some(YTNode::Short(s));
            }
        }

        // 4. Check for lockupViewModel (Polymorphic: Video vs Short vs Playlist)
        if let Some(lvm) = val.get("lockupViewModel") {
            let content_type = lvm
                .get("contentType")
                .and_then(|c| c.as_str())
                .unwrap_or("");
            if content_type.contains("SHORT")
                || lvm
                    .pointer("/rendererContext/commandContext/onTap/innertubeCommand/reelWatchEndpoint")
                    .is_some()
            {
                if let Some(s) = ShortNode::from_value(lvm) {
                    return Some(YTNode::Short(s));
                }
            } else if content_type.contains("PLAYLIST") {
                if let Some(p) = PlaylistNode::from_value(lvm) {
                    return Some(YTNode::Playlist(p));
                }
            } else {
                if let Some(v) = VideoNode::from_value(lvm) {
                    return Some(YTNode::Video(v));
                }
            }
        }

        // 5. Check for Playlist Renderers
        if val.get("playlistVideoRenderer").is_some() {
            if let Some(pv) = PlaylistVideoNode::from_value(val) {
                return Some(YTNode::PlaylistVideo(pv));
            }
        }
        if val.get("playlistRenderer").is_some()
            || val.get("gridPlaylistRenderer").is_some()
            || val.get("playlistHeaderRenderer").is_some()
        {
            if let Some(p) = PlaylistNode::from_value(val) {
                return Some(YTNode::Playlist(p));
            }
        }

        // 6. Check for Channel Renderers
        if val.get("channelRenderer").is_some() || val.get("gridChannelRenderer").is_some() {
            if let Some(c) = ChannelCardNode::from_value(val) {
                return Some(YTNode::ChannelCard(c));
            }
        }
        if val.get("c4TabbedHeaderRenderer").is_some()
            || val.get("pageHeaderRenderer").is_some()
        {
            if let Some(ch) = ChannelHeaderNode::from_value(val) {
                return Some(YTNode::ChannelHeader(ch));
            }
        }

        // 7. Check for Music Renderers
        if val.get("musicResponsiveListItemRenderer").is_some() {
            if let Some(m) = MusicResponsiveListItemNode::from_value(val) {
                return Some(YTNode::MusicItem(m));
            }
        }
        if val.get("musicTwoRowItemRenderer").is_some() {
            if let Some(m) = MusicTwoRowItemNode::from_value(val) {
                return Some(YTNode::MusicCard(m));
            }
        }
        if val.get("musicDescriptionShelfRenderer").is_some() {
            if let Some(l) = MusicDescriptionShelfNode::from_value(val) {
                return Some(YTNode::MusicDescriptionShelf(l));
            }
        }

        // 8. Check for Comments Renderers
        if val.get("commentThreadRenderer").is_some() {
            if let Some(ct) = CommentThreadNode::from_value(val) {
                return Some(YTNode::CommentThread(ct));
            }
        }
        if val.get("commentRenderer").is_some() {
            if let Some(c) = CommentNode::from_value(val) {
                return Some(YTNode::Comment(c));
            }
        }

        // 9. Check for Community Post Renderers
        if val.get("backstagePostRenderer").is_some()
            || val.get("postRenderer").is_some()
            || val.get("sharedPostRenderer").is_some()
            || val.get("backstagePostThreadRenderer").is_some()
        {
            if let Some(p) = PostNode::from_value(val) {
                return Some(YTNode::Post(p));
            }
        }

        // 10. Check for Containers & Layouts
        if val.get("sectionListRenderer").is_some() {
            if let Some(s) = SectionListNode::from_value(val) {
                return Some(YTNode::SectionList(s));
            }
        }
        if val.get("itemSectionRenderer").is_some() {
            if let Some(i) = ItemSectionNode::from_value(val) {
                return Some(YTNode::ItemSection(i));
            }
        }
        if val.get("richGridRenderer").is_some() {
            if let Some(r) = RichGridNode::from_value(val) {
                return Some(YTNode::RichGrid(r));
            }
        }
        if val.get("richShelfRenderer").is_some() {
            if let Some(rs) = RichShelfNode::from_value(val) {
                return Some(YTNode::RichShelf(rs));
            }
        }
        if val.get("shelfRenderer").is_some() {
            if let Some(s) = ShelfNode::from_value(val) {
                return Some(YTNode::Shelf(s));
            }
        }
        if val.get("tabRenderer").is_some() {
            if let Some(t) = TabNode::from_value(val) {
                return Some(YTNode::Tab(t));
            }
        }

        // 11. Check for Live Chat
        if val.get("liveChatTextMessageRenderer").is_some()
            || val.get("liveChatPaidMessageRenderer").is_some()
            || val.get("liveChatMembershipItemRenderer").is_some()
        {
            if let Some(lc) = LiveChatMessageNode::from_value(val) {
                return Some(YTNode::LiveChat(lc));
            }
        }

        None
    }
}
