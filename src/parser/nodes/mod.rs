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
    ChannelSubMenuNode,
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
    AccountItemNode, AccountSectionListNode, AlertNode, AuthorNode, BrowseEndpointNode, ButtonNode,
    CardNode, ChapterNode, ClarificationNode, ContinuationEndpointNode, DidYouMeanNode,
    EndscreenElementNode, EndscreenNode, ExpandableTabNode, HeatmapNode, HistorySuggestionNode,
    HorizontalCardListNode, KidsCategoriesHeaderNode, KidsHomeScreenNode, LikeEndpointNode,
    MacroMarkersListItemNode, MacroMarkersListNode, MenuItemNode, MenuNode, MetadataBadgeNode,
    MicroformatDataNode, NavigateActionNode, NavigationEndpointNode, NotificationNode,
    PlayerCaptionsTracklistNode, PlayerErrorMessageNode, PlayerLegacyDesktopYpcTrailerNode,
    PlayerOverlayNode, PlayerStoryboardSpecNode, PollNode, ProfileColumnNode,
    ProfileColumnUserInfoNode, ReelWatchEndpointNode, SearchEndpointNode, SearchFilterGroupNode,
    SearchFilterNode, SearchRefinementCardNode, SearchSubMenuNode, ShowingResultsForNode,
    ShowEngagementPanelActionNode, ShowLiveChatActionNode, SubscribeEndpointNode, TextNode,
    TextRunNode, ThumbnailListNode, ThumbnailNode, ThumbnailOverlayProgressBarNode,
    ThumbnailOverlayTimeStatusNode, TimedMarkerDecorationNode, ToggleButtonNode,
    UpdateEngagementPanelActionNode, VerticalListNode, VideoOwnerNode, ViewCountNode,
    WatchEndpointNode,
};
pub use music::{
    MusicDescriptionShelfNode, MusicHeaderNode, MusicInlineBadgeNode, MusicNavigationButtonNode,
    MusicPlayButtonNode, MusicQueueNode, MusicResponsiveListItemNode, MusicTwoRowItemNode,
};
pub use playlist::{
    PlaylistMetadataNode, PlaylistNode, PlaylistPanelNode, PlaylistPanelVideoNode,
    PlaylistSidebarPrimaryInfoNode, PlaylistSidebarSecondaryInfoNode, PlaylistVideoNode,
};
pub use post::{BackstageImageNode, PostMultiImageNode, PostNode};
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
    PlaylistMetadata(PlaylistMetadataNode),
    PlaylistSidebarPrimaryInfo(PlaylistSidebarPrimaryInfoNode),
    PlaylistSidebarSecondaryInfo(PlaylistSidebarSecondaryInfoNode),
    ChannelHeader(ChannelHeaderNode),
    ChannelCard(ChannelCardNode),
    ChannelAboutFullMetadata(ChannelAboutFullMetadataNode),
    ChannelMetadata(ChannelMetadataNode),
    ChannelSubMenu(ChannelSubMenuNode),
    MusicItem(MusicResponsiveListItemNode),
    MusicCard(MusicTwoRowItemNode),
    MusicDescriptionShelf(MusicDescriptionShelfNode),
    MusicHeader(MusicHeaderNode),
    MusicInlineBadge(MusicInlineBadgeNode),
    MusicNavigationButton(MusicNavigationButtonNode),
    MusicQueue(MusicQueueNode),
    MusicPlayButton(MusicPlayButtonNode),
    Comment(CommentNode),
    CommentThread(CommentThreadNode),
    CreatorHeart(CreatorHeartNode),
    Post(PostNode),
    BackstageImage(BackstageImageNode),
    PostMultiImage(PostMultiImageNode),
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
    ShowEngagementPanelAction(ShowEngagementPanelActionNode),
    UpdateEngagementPanelAction(UpdateEngagementPanelActionNode),
    NavigateAction(NavigateActionNode),
    ShowLiveChatAction(ShowLiveChatActionNode),
    Button(ButtonNode),
    ToggleButton(ToggleButtonNode),
    Menu(MenuNode),
    DidYouMean(DidYouMeanNode),
    ShowingResultsFor(ShowingResultsForNode),
    SearchSubMenu(SearchSubMenuNode),
    SearchFilterGroup(SearchFilterGroupNode),
    SearchFilter(SearchFilterNode),
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
    PlayerCaptionsTracklist(PlayerCaptionsTracklistNode),
    PlayerErrorMessage(PlayerErrorMessageNode),
    PlayerLegacyDesktopYpcTrailer(PlayerLegacyDesktopYpcTrailerNode),
    ProfileColumn(ProfileColumnNode),
    ProfileColumnUserInfo(ProfileColumnUserInfoNode),
    VerticalList(VerticalListNode),
    Chapter(ChapterNode),
    Heatmap(HeatmapNode),
    MacroMarkersList(MacroMarkersListNode),
    MacroMarkersListItem(MacroMarkersListItemNode),
    SearchRefinementCard(SearchRefinementCardNode),
    HorizontalCardList(HorizontalCardListNode),
    ExpandableTab(ExpandableTabNode),
    Notification(NotificationNode),
    HistorySuggestion(HistorySuggestionNode),
    AccountSectionList(AccountSectionListNode),
    AccountItem(AccountItemNode),
    KidsCategoriesHeader(KidsCategoriesHeaderNode),
    KidsHomeScreen(KidsHomeScreenNode),
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

        // 23. Check for Markers, Chapters & Heatmaps
        if val.get("chapterRenderer").is_some() {
            if let Some(ch) = ChapterNode::from_value(val) {
                return Some(YTNode::Chapter(ch));
            }
        }
        if val.get("heatmapRenderer").is_some() {
            if let Some(hm) = HeatmapNode::from_value(val) {
                return Some(YTNode::Heatmap(hm));
            }
        }
        if val.get("macroMarkersListRenderer").is_some() {
            if let Some(mml) = MacroMarkersListNode::from_value(val) {
                return Some(YTNode::MacroMarkersList(mml));
            }
        }
        if val.get("macroMarkersListItemRenderer").is_some() {
            if let Some(mmli) = MacroMarkersListItemNode::from_value(val) {
                return Some(YTNode::MacroMarkersListItem(mmli));
            }
        }

        // 24. Check for Cards & Tabs
        if val.get("searchRefinementCardRenderer").is_some() {
            if let Some(src) = SearchRefinementCardNode::from_value(val) {
                return Some(YTNode::SearchRefinementCard(src));
            }
        }
        if val.get("horizontalCardListRenderer").is_some() {
            if let Some(hcl) = HorizontalCardListNode::from_value(val) {
                return Some(YTNode::HorizontalCardList(hcl));
            }
        }
        if val.get("expandableTabRenderer").is_some() {
            if let Some(et) = ExpandableTabNode::from_value(val) {
                return Some(YTNode::ExpandableTab(et));
            }
        }

        // 25. Check for Post Media & Channel SubMenus
        if val.get("backstageImageRenderer").is_some() {
            if let Some(bi) = BackstageImageNode::from_value(val) {
                return Some(YTNode::BackstageImage(bi));
            }
        }
        if val.get("postMultiImageRenderer").is_some() {
            if let Some(pmi) = PostMultiImageNode::from_value(val) {
                return Some(YTNode::PostMultiImage(pmi));
            }
        }
        if val.get("channelSubMenuRenderer").is_some() {
            if let Some(csm) = ChannelSubMenuNode::from_value(val) {
                return Some(YTNode::ChannelSubMenu(csm));
            }
        }

        // 26. Check for Engagement & Live Actions
        if val.get("showEngagementPanelEndpoint").is_some()
            || val.get("showEngagementPanelAction").is_some()
        {
            if let Some(se) = ShowEngagementPanelActionNode::from_value(val) {
                return Some(YTNode::ShowEngagementPanelAction(se));
            }
        }
        if val.get("updateEngagementPanelAction").is_some() {
            if let Some(ue) = UpdateEngagementPanelActionNode::from_value(val) {
                return Some(YTNode::UpdateEngagementPanelAction(ue));
            }
        }
        if val.get("navigateAction").is_some() {
            if let Some(na) = NavigateActionNode::from_value(val) {
                return Some(YTNode::NavigateAction(na));
            }
        }
        if val.get("showLiveChatAction").is_some() || val.get("showLiveChatItemEndpoint").is_some() {
            if let Some(slc) = ShowLiveChatActionNode::from_value(val) {
                return Some(YTNode::ShowLiveChatAction(slc));
            }
        }

        // 27. Check for Player Media & Error Messages
        if val.get("playerCaptionsTracklistRenderer").is_some() {
            if let Some(pct) = PlayerCaptionsTracklistNode::from_value(val) {
                return Some(YTNode::PlayerCaptionsTracklist(pct));
            }
        }
        if val.get("playerErrorMessageRenderer").is_some() {
            if let Some(pem) = PlayerErrorMessageNode::from_value(val) {
                return Some(YTNode::PlayerErrorMessage(pem));
            }
        }
        if val.get("playerLegacyDesktopYpcTrailerRenderer").is_some() {
            if let Some(ypc) = PlayerLegacyDesktopYpcTrailerNode::from_value(val) {
                return Some(YTNode::PlayerLegacyDesktopYpcTrailer(ypc));
            }
        }

        // 28. Check for Playlist Metadata & Sidebar Info
        if val.get("playlistMetadataRenderer").is_some() {
            if let Some(pm) = PlaylistMetadataNode::from_value(val) {
                return Some(YTNode::PlaylistMetadata(pm));
            }
        }
        if val.get("playlistSidebarPrimaryInfoRenderer").is_some() {
            if let Some(spi) = PlaylistSidebarPrimaryInfoNode::from_value(val) {
                return Some(YTNode::PlaylistSidebarPrimaryInfo(spi));
            }
        }
        if val.get("playlistSidebarSecondaryInfoRenderer").is_some() {
            if let Some(ssi) = PlaylistSidebarSecondaryInfoNode::from_value(val) {
                return Some(YTNode::PlaylistSidebarSecondaryInfo(ssi));
            }
        }

        // 29. Check for Notifications & Account Components
        if val.get("notificationRenderer").is_some() {
            if let Some(notif) = NotificationNode::from_value(val) {
                return Some(YTNode::Notification(notif));
            }
        }
        if val.get("historySuggestionRenderer").is_some() {
            if let Some(hs) = HistorySuggestionNode::from_value(val) {
                return Some(YTNode::HistorySuggestion(hs));
            }
        }
        if val.get("accountSectionListRenderer").is_some() {
            if let Some(asl) = AccountSectionListNode::from_value(val) {
                return Some(YTNode::AccountSectionList(asl));
            }
        }
        if val.get("accountItemRenderer").is_some() {
            if let Some(ai) = AccountItemNode::from_value(val) {
                return Some(YTNode::AccountItem(ai));
            }
        }

        // 30. Check for Search Filters & Groups
        if val.get("searchFilterGroupRenderer").is_some() {
            if let Some(sfg) = SearchFilterGroupNode::from_value(val) {
                return Some(YTNode::SearchFilterGroup(sfg));
            }
        }
        if val.get("searchFilterRenderer").is_some() {
            if let Some(sf) = SearchFilterNode::from_value(val) {
                return Some(YTNode::SearchFilter(sf));
            }
        }

        // 31. Check for Kids & Music Specialty Renderers
        if val.get("kidsCategoriesHeaderRenderer").is_some()
            || val.get("kidsCategoryTabRenderer").is_some()
        {
            if let Some(kch) = KidsCategoriesHeaderNode::from_value(val) {
                return Some(YTNode::KidsCategoriesHeader(kch));
            }
        }
        if val.get("kidsHomeScreenRenderer").is_some() {
            if let Some(khs) = KidsHomeScreenNode::from_value(val) {
                return Some(YTNode::KidsHomeScreen(khs));
            }
        }
        if val.get("musicQueueRenderer").is_some() {
            if let Some(mq) = MusicQueueNode::from_value(val) {
                return Some(YTNode::MusicQueue(mq));
            }
        }
        if val.get("musicPlayButtonRenderer").is_some() {
            if let Some(mpb) = MusicPlayButtonNode::from_value(val) {
                return Some(YTNode::MusicPlayButton(mpb));
            }
        }

        None
    }
}
