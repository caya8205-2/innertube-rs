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

pub use channel::{
    ChannelAboutFullMetadataNode, ChannelCardNode, ChannelHeaderNode, ChannelMetadataNode,
};
pub use comments::{CommentNode, CommentThreadNode, CreatorHeartNode};
pub use containers::{
    ChipCloudChipNode, ChipCloudNode, ItemSectionNode, RichGridNode, RichShelfNode, SectionListNode,
    ShelfNode, TabNode,
};
pub use continuation::ContinuationNode;
pub use livechat::{
    AddChatItemActionNode, LiveChatAutoModMessageNode, LiveChatBannerNode,
    LiveChatMembershipItemNode, LiveChatMessageNode, LiveChatModeChangeMessageNode,
    LiveChatPaidStickerNode, LiveChatViewerEngagementMessageNode,
    MarkChatItemAsDeletedActionNode,
};
pub use misc::{
    AlertNode, AuthorNode, BrowseEndpointNode, ButtonNode, CardNode, ClarificationNode,
    ContinuationEndpointNode, DidYouMeanNode, EndscreenElementNode, EndscreenNode, LikeEndpointNode,
    MenuItemNode, MenuNode, MetadataBadgeNode, MicroformatDataNode, NavigationEndpointNode,
    PlayerOverlayNode, PlayerStoryboardSpecNode, PollNode, ProfileColumnNode,
    ProfileColumnUserInfoNode, ReelWatchEndpointNode, SearchEndpointNode, SearchSubMenuNode,
    ShowingResultsForNode, SubscribeEndpointNode, TextNode, TextRunNode, ThumbnailListNode,
    ThumbnailNode, ThumbnailOverlayProgressBarNode, ThumbnailOverlayTimeStatusNode,
    TimedMarkerDecorationNode, ToggleButtonNode, VerticalListNode, VideoOwnerNode, ViewCountNode,
    WatchEndpointNode,
};
pub use music::{
    MusicDescriptionShelfNode, MusicHeaderNode, MusicInlineBadgeNode, MusicNavigationButtonNode,
    MusicResponsiveListItemNode, MusicTwoRowItemNode,
};
pub use playlist::{PlaylistNode, PlaylistPanelNode, PlaylistPanelVideoNode, PlaylistVideoNode};
pub use post::PostNode;
pub use short::{ReelShelfNode, ShortNode};
pub use video::{VideoNode, VideoPrimaryInfoNode, VideoSecondaryInfoNode};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Central strongly typed Polymorphic InnerTube Node representation (1:1 port of `YTNode.ts`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum YTNode {
    Video(VideoNode),
    VideoPrimaryInfo(VideoPrimaryInfoNode),
    VideoSecondaryInfo(VideoSecondaryInfoNode),
    Short(ShortNode),
    ReelShelf(ReelShelfNode),
    Playlist(PlaylistNode),
    PlaylistVideo(PlaylistVideoNode),
    PlaylistPanel(PlaylistPanelNode),
    PlaylistPanelVideo(PlaylistPanelVideoNode),
    ChannelHeader(ChannelHeaderNode),
    ChannelCard(ChannelCardNode),
    ChannelAboutFullMetadata(ChannelAboutFullMetadataNode),
    ChannelMetadata(ChannelMetadataNode),
    MusicItem(MusicResponsiveListItemNode),
    MusicCard(MusicTwoRowItemNode),
    MusicDescriptionShelf(MusicDescriptionShelfNode),
    MusicHeader(MusicHeaderNode),
    MusicInlineBadge(MusicInlineBadgeNode),
    MusicNavigationButton(MusicNavigationButtonNode),
    Comment(CommentNode),
    CommentThread(CommentThreadNode),
    CreatorHeart(CreatorHeartNode),
    Post(PostNode),
    Continuation(ContinuationNode),
    SectionList(SectionListNode),
    ItemSection(ItemSectionNode),
    RichGrid(RichGridNode),
    Shelf(ShelfNode),
    RichShelf(RichShelfNode),
    Tab(TabNode),
    ChipCloud(ChipCloudNode),
    ChipCloudChip(ChipCloudChipNode),
    LiveChat(LiveChatMessageNode),
    LiveChatPaidSticker(LiveChatPaidStickerNode),
    LiveChatMembershipItem(LiveChatMembershipItemNode),
    LiveChatViewerEngagementMessage(LiveChatViewerEngagementMessageNode),
    LiveChatBanner(LiveChatBannerNode),
    AddChatItemAction(AddChatItemActionNode),
    MarkChatItemAsDeletedAction(MarkChatItemAsDeletedActionNode),
    LiveChatAutoModMessage(LiveChatAutoModMessageNode),
    LiveChatModeChangeMessage(LiveChatModeChangeMessageNode),
    Button(ButtonNode),
    ToggleButton(ToggleButtonNode),
    Menu(MenuNode),
    DidYouMean(DidYouMeanNode),
    ShowingResultsFor(ShowingResultsForNode),
    SearchSubMenu(SearchSubMenuNode),
    Endscreen(EndscreenNode),
    EndscreenElement(EndscreenElementNode),
    MetadataBadge(MetadataBadgeNode),
    ViewCount(ViewCountNode),
    VideoOwner(VideoOwnerNode),
    MicroformatData(MicroformatDataNode),
    Alert(AlertNode),
    Card(CardNode),
    Clarification(ClarificationNode),
    Poll(PollNode),
    PlayerOverlay(PlayerOverlayNode),
    PlayerStoryboardSpec(PlayerStoryboardSpecNode),
    TimedMarkerDecoration(TimedMarkerDecorationNode),
    ProfileColumn(ProfileColumnNode),
    ProfileColumnUserInfo(ProfileColumnUserInfoNode),
    VerticalList(VerticalListNode),
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
        if val.get("videoPrimaryInfoRenderer").is_some() {
            if let Some(vpi) = VideoPrimaryInfoNode::from_value(val) {
                return Some(YTNode::VideoPrimaryInfo(vpi));
            }
        }
        if val.get("videoSecondaryInfoRenderer").is_some() {
            if let Some(vsi) = VideoSecondaryInfoNode::from_value(val) {
                return Some(YTNode::VideoSecondaryInfo(vsi));
            }
        }
        if val.get("videoRenderer").is_some()
            || val.get("compactVideoRenderer").is_some()
            || val.get("gridVideoRenderer").is_some()
        {
            if let Some(v) = VideoNode::from_value(val) {
                return Some(YTNode::Video(v));
            }
        }

        // 3. Check for Shorts Renderers
        if val.get("reelShelfRenderer").is_some() {
            if let Some(rs) = ReelShelfNode::from_value(val) {
                return Some(YTNode::ReelShelf(rs));
            }
        }
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
        if val.get("playlistPanelRenderer").is_some() {
            if let Some(pp) = PlaylistPanelNode::from_value(val) {
                return Some(YTNode::PlaylistPanel(pp));
            }
        }
        if val.get("playlistPanelVideoRenderer").is_some() {
            if let Some(ppv) = PlaylistPanelVideoNode::from_value(val) {
                return Some(YTNode::PlaylistPanelVideo(ppv));
            }
        }
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
        if val.get("creatorHeartRenderer").is_some() {
            if let Some(ch) = CreatorHeartNode::from_value(val) {
                return Some(YTNode::CreatorHeart(ch));
            }
        }
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
        if val.get("chipCloudRenderer").is_some() {
            if let Some(cc) = ChipCloudNode::from_value(val) {
                return Some(YTNode::ChipCloud(cc));
            }
        }
        if val.get("chipCloudChipRenderer").is_some() {
            if let Some(ccc) = ChipCloudChipNode::from_value(val) {
                return Some(YTNode::ChipCloudChip(ccc));
            }
        }
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

        // 11. Check for Live Chat Messages
        if val.get("liveChatTextMessageRenderer").is_some()
            || val.get("liveChatPaidMessageRenderer").is_some()
        {
            if let Some(lc) = LiveChatMessageNode::from_value(val) {
                return Some(YTNode::LiveChat(lc));
            }
        }

        // 12. Check for Buttons and Menus
        if val.get("toggleButtonRenderer").is_some() || val.get("toggleButtonViewModel").is_some() {
            if let Some(tb) = ToggleButtonNode::from_value(val) {
                return Some(YTNode::ToggleButton(tb));
            }
        }
        if val.get("buttonRenderer").is_some() || val.get("buttonViewModel").is_some() {
            if let Some(b) = ButtonNode::from_value(val) {
                return Some(YTNode::Button(b));
            }
        }
        if val.get("menuRenderer").is_some() || val.get("menuPopupRenderer").is_some() {
            if let Some(m) = MenuNode::from_value(val) {
                return Some(YTNode::Menu(m));
            }
        }

        // 13. Check for Search Modifiers & Submenus
        if val.get("didYouMeanRenderer").is_some() {
            if let Some(dym) = DidYouMeanNode::from_value(val) {
                return Some(YTNode::DidYouMean(dym));
            }
        }
        if val.get("showingResultsForRenderer").is_some() {
            if let Some(srf) = ShowingResultsForNode::from_value(val) {
                return Some(YTNode::ShowingResultsFor(srf));
            }
        }
        if val.get("searchSubMenuRenderer").is_some() {
            if let Some(ssm) = SearchSubMenuNode::from_value(val) {
                return Some(YTNode::SearchSubMenu(ssm));
            }
        }

        // 14. Check for Endscreens & Overlays
        if val.get("endscreenRenderer").is_some() {
            if let Some(es) = EndscreenNode::from_value(val) {
                return Some(YTNode::Endscreen(es));
            }
        }
        if val.get("endscreenElementRenderer").is_some() {
            if let Some(ese) = EndscreenElementNode::from_value(val) {
                return Some(YTNode::EndscreenElement(ese));
            }
        }

        // 15. Check for Metadata & Badges
        if val.get("metadataBadgeRenderer").is_some() {
            if let Some(mb) = MetadataBadgeNode::from_value(val) {
                return Some(YTNode::MetadataBadge(mb));
            }
        }
        if val.get("viewCountRenderer").is_some() || val.get("videoViewCountRenderer").is_some() {
            if let Some(vc) = ViewCountNode::from_value(val) {
                return Some(YTNode::ViewCount(vc));
            }
        }
        if val.get("videoOwnerRenderer").is_some() {
            if let Some(vo) = VideoOwnerNode::from_value(val) {
                return Some(YTNode::VideoOwner(vo));
            }
        }
        if val.get("microformatDataRenderer").is_some() {
            if let Some(md) = MicroformatDataNode::from_value(val) {
                return Some(YTNode::MicroformatData(md));
            }
        }

        // 16. Check for Channel About / Metadata
        if val.get("channelAboutFullMetadataRenderer").is_some() {
            if let Some(cafm) = ChannelAboutFullMetadataNode::from_value(val) {
                return Some(YTNode::ChannelAboutFullMetadata(cafm));
            }
        }
        if val.get("channelMetadataRenderer").is_some() {
            if let Some(cm) = ChannelMetadataNode::from_value(val) {
                return Some(YTNode::ChannelMetadata(cm));
            }
        }

        // 17. Check for Additional Live Chat Renderers
        if val.get("liveChatPaidStickerRenderer").is_some() {
            if let Some(ps) = LiveChatPaidStickerNode::from_value(val) {
                return Some(YTNode::LiveChatPaidSticker(ps));
            }
        }
        if val.get("liveChatMembershipItemRenderer").is_some() {
            if let Some(mi) = LiveChatMembershipItemNode::from_value(val) {
                return Some(YTNode::LiveChatMembershipItem(mi));
            }
        }
        if val.get("liveChatViewerEngagementMessageRenderer").is_some() {
            if let Some(ve) = LiveChatViewerEngagementMessageNode::from_value(val) {
                return Some(YTNode::LiveChatViewerEngagementMessage(ve));
            }
        }
        if val.get("liveChatBannerRenderer").is_some() {
            if let Some(b) = LiveChatBannerNode::from_value(val) {
                return Some(YTNode::LiveChatBanner(b));
            }
        }

        // 18. Check for Music Header / Badges / Buttons
        if val.get("musicHeaderRenderer").is_some() || val.get("musicVisualHeaderRenderer").is_some() {
            if let Some(mh) = MusicHeaderNode::from_value(val) {
                return Some(YTNode::MusicHeader(mh));
            }
        }
        if val.get("musicInlineBadgeRenderer").is_some() {
            if let Some(mib) = MusicInlineBadgeNode::from_value(val) {
                return Some(YTNode::MusicInlineBadge(mib));
            }
        }
        if val.get("musicNavigationButtonRenderer").is_some() {
            if let Some(mnb) = MusicNavigationButtonNode::from_value(val) {
                return Some(YTNode::MusicNavigationButton(mnb));
            }
        }

        // 19. Check for Alerts, Cards, Clarifications, and Polls
        if val.get("alertRenderer").is_some() || val.get("alertWithActionsRenderer").is_some() {
            if let Some(a) = AlertNode::from_value(val) {
                return Some(YTNode::Alert(a));
            }
        }
        if val.get("cardRenderer").is_some() {
            if let Some(c) = CardNode::from_value(val) {
                return Some(YTNode::Card(c));
            }
        }
        if val.get("clarificationRenderer").is_some() || val.get("emergencyOneboxRenderer").is_some() {
            if let Some(cl) = ClarificationNode::from_value(val) {
                return Some(YTNode::Clarification(cl));
            }
        }
        if val.get("pollRenderer").is_some() {
            if let Some(p) = PollNode::from_value(val) {
                return Some(YTNode::Poll(p));
            }
        }

        // 20. Check for Live Chat Actions & Moderation
        if val.get("addChatItemAction").is_some() {
            if let Some(a) = AddChatItemActionNode::from_value(val) {
                return Some(YTNode::AddChatItemAction(a));
            }
        }
        if val.get("markChatItemAsDeletedAction").is_some()
            || val.get("markChatItemsByAuthorAsDeletedAction").is_some()
        {
            if let Some(m) = MarkChatItemAsDeletedActionNode::from_value(val) {
                return Some(YTNode::MarkChatItemAsDeletedAction(m));
            }
        }
        if val.get("liveChatAutoModMessageRenderer").is_some() {
            if let Some(am) = LiveChatAutoModMessageNode::from_value(val) {
                return Some(YTNode::LiveChatAutoModMessage(am));
            }
        }
        if val.get("liveChatModeChangeMessageRenderer").is_some() {
            if let Some(mc) = LiveChatModeChangeMessageNode::from_value(val) {
                return Some(YTNode::LiveChatModeChangeMessage(mc));
            }
        }

        // 21. Check for Player Overlays & Storyboards
        if val.get("playerOverlayRenderer").is_some() {
            if let Some(po) = PlayerOverlayNode::from_value(val) {
                return Some(YTNode::PlayerOverlay(po));
            }
        }
        if val.get("playerStoryboardSpecRenderer").is_some()
            || val.get("playerLiveStoryboardSpecRenderer").is_some()
        {
            if let Some(pss) = PlayerStoryboardSpecNode::from_value(val) {
                return Some(YTNode::PlayerStoryboardSpec(pss));
            }
        }
        if val.get("timedMarkerDecorationRenderer").is_some() {
            if let Some(tmd) = TimedMarkerDecorationNode::from_value(val) {
                return Some(YTNode::TimedMarkerDecoration(tmd));
            }
        }

        // 22. Check for Profile & Vertical List Components
        if val.get("profileColumnRenderer").is_some() {
            if let Some(pc) = ProfileColumnNode::from_value(val) {
                return Some(YTNode::ProfileColumn(pc));
            }
        }
        if val.get("profileColumnUserInfoRenderer").is_some() {
            if let Some(pcu) = ProfileColumnUserInfoNode::from_value(val) {
                return Some(YTNode::ProfileColumnUserInfo(pcu));
            }
        }
        if val.get("verticalListRenderer").is_some() {
            if let Some(vl) = VerticalListNode::from_value(val) {
                return Some(YTNode::VerticalList(vl));
            }
        }

        None
    }
}
