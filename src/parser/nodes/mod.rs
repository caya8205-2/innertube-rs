pub mod channel;
pub mod commands_ext;
pub mod comments;
pub mod containers;
pub mod continuation;
pub mod grid;
pub mod livechat;
pub mod livechat_ext;
pub mod misc;
pub mod music;
pub mod music_extended;
pub mod playlist;
pub mod post;
pub mod short;
pub mod video;

pub use channel::{
    ChannelAboutFullMetadataNode, ChannelCardNode, ChannelHeaderNode, ChannelMetadataNode,
    ChannelSubMenuNode,
};
pub use commands_ext::{
    AddToPlaylistCommandNode, AppendContinuationItemsActionNode, ContinuationCommandNode,
    GetMultiPageMenuActionNode, OpenPopupActionNode, RunAttestationCommandNode, SendFeedbackActionNode,
    ShowSheetCommandNode, SignalActionNode, UpdateChannelSwitcherPageActionNode,
    UpdateEngagementPanelContentCommandNode, UpdateSubscribeButtonActionNode,
};
pub use comments::{CommentNode, CommentThreadNode, CreatorHeartNode};
pub use containers::{
    ChipCloudChipNode, ChipCloudNode, FeedFilterChipBarNode, ItemSectionNode, RichGridNode,
    RichShelfNode, SectionListNode, ShelfNode, TabNode,
};
pub use continuation::ContinuationNode;
pub use grid::{
    CompactChannelNode, CompactMixNode, CompactPlaylistNode, CompactVideoNode, GridChannelNode,
    GridMixNode, GridMovieNode, GridPlaylistNode, GridShowNode, GridVideoNode, RichItemNode,
    RichSectionNode,
};
pub use livechat::{
    AddChatItemActionNode, LiveChatAutoModMessageNode, LiveChatBannerNode,
    LiveChatMembershipItemNode, LiveChatMessageNode, LiveChatModeChangeMessageNode,
    LiveChatPaidStickerNode, LiveChatViewerEngagementMessageNode,
    MarkChatItemAsDeletedActionNode,
};
pub use livechat_ext::{
    AddBannerToLiveChatCommandNode, AddLiveChatTickerItemActionNode, DimChatItemActionNode,
    LiveChatActionPanelNode, LiveChatItemListNode, LiveChatParticipantsListNode,
    RemoveBannerForLiveChatCommandNode, RemoveChatItemActionNode, RemoveChatItemByAuthorActionNode,
    ReplaceChatItemActionNode, ReplayChatItemActionNode, UpdateLiveChatPollActionNode,
};
pub use misc::{
    AccountItemNode, AccountItemSectionHeaderNode, AccountItemSectionNode, AccountSectionListNode,
    AddToPlaylistNode, AlertNode, AnimatedThumbnailOverlayViewNode, AttributionViewNode,
    AudioOnlyPlayabilityNode, AuthorNode, AvatarStackViewNode, AvatarViewNode, BackgroundPromoNode,
    BadgeViewNode, BrowseEndpointNode, BrowseFeedActionsNode, ButtonCardViewNode, ButtonNode,
    ButtonViewNode, C4TabbedHeaderNode, CallToActionButtonNode, CardCollectionNode, CardNode,
    CarouselHeaderNode, CarouselItemNode, CarouselItemViewNode, CarouselLockupNode,
    CarouselTitleViewNode, ChannelOwnerEmptyStateNode, ChannelSwitcherPageNode, ChapterNode,
    ChipBarViewNode, ChipViewNode, ClarificationNode, ClientSideToggleMenuItemNode, ClipAdStateNode,
    ClipCreationNode, ClipCreationScrubberNode, ClipCreationTextInputNode, ClipSectionNode,
    CollaboratorInfoCardContentNode, CollectionThumbnailViewNode, CollageHeroImageNode,
    CommentActionButtonsNode, CommentSimpleboxNode, CommentsEntryPointHeaderNode,
    CommentsHeaderNode, CompactLinkNode, CompactMovieNode, CompactStationNode, ConfirmDialogNode,
    ContentListItemViewNode, ContentMetadataViewNode, ContentPreviewImageViewNode,
    ContinuationEndpointNode, ContinuationItemNode, ContinuationItemViewNode, ConversationBarNode,
    CopyLinkNode, CreatePlaylistDialogFormViewNode, CreatePlaylistDialogNode, DecoratedAvatarViewNode,
    DecoratedPlayerBarNode, DefaultPromoPanelNode, DescriptionPreviewViewNode, DialogHeaderViewNode,
    DialogNode, DialogViewNode, DidYouMeanNode, DislikeButtonViewNode,
    DismissableDialogContentSectionNode, DismissableDialogNode, DownloadButtonNode,
    DownloadListItemViewNode, DropdownItemNode, DropdownNode, DropdownViewNode, DynamicTextViewNode,
    ElementNode, EmojiPickerCategoryButtonNode, EmojiPickerCategoryNode,
    EmojiPickerUpsellCategoryNode, EndScreenPlaylistNode, EndscreenElementNode, EndscreenNode,
    EngagementPanelSectionListNode, EngagementPanelTitleHeaderNode, EomSettingsDisclaimerNode,
    ExpandableMetadataNode, ExpandableTabNode, ExpandedShelfContentsNode, FactoidNode,
    FancyDismissibleDialogNode, FeedNudgeNode, FeedTabbedHeaderNode, FlexibleActionsViewNode,
    FormFooterViewNode, FormNode, FormPopupNode, GameCardNode, GameDetailsNode, GridHeaderNode,
    GridNode, GridShelfViewNode, GuideCollapsibleEntryNode, GuideCollapsibleSectionEntryNode,
    GuideDownloadsEntryNode, HeatmapNode, HistorySuggestionNode, HorizontalCardListNode,
    InfoRowNode, KidsCategoriesHeaderNode, KidsHomeScreenNode, LikeEndpointNode,
    MacroMarkersListItemNode, MacroMarkersListNode, MenuItemNode, MenuNode, MetadataBadgeNode,
    MicroformatDataNode, ModalWithTitleAndButtonNode, NavigateActionNode, NavigationEndpointNode,
    NotificationNode, PlayerCaptionsTracklistNode, PlayerErrorMessageNode,
    PlayerLegacyDesktopYpcTrailerNode, PlayerOverlayNode, PlayerStoryboardSpecNode, PollNode,
    ProfileColumnNode, ProfileColumnUserInfoNode, ReelWatchEndpointNode, SearchEndpointNode,
    SearchFilterGroupNode, SearchFilterNode, SearchRefinementCardNode, SearchSubMenuNode,
    ShowEngagementPanelActionNode, ShowLiveChatActionNode, ShowingResultsForNode,
    SubscribeEndpointNode, SubscriptionNotificationToggleButtonNode, TextHeaderNode, TextNode,
    TextRunNode, ThumbnailListNode, ThumbnailNode, ThumbnailOverlayBottomPanelNode,
    ThumbnailOverlayEndorsementNode, ThumbnailOverlayHoverTextNode,
    ThumbnailOverlayInlineUnplayableNode, ThumbnailOverlayLoadingPreviewNode,
    ThumbnailOverlayNowPlayingNode, ThumbnailOverlayProgressBarNode,
    ThumbnailOverlaySidePanelNode, ThumbnailOverlayTimeStatusNode,
    ThumbnailOverlayToggleButtonNode, TimedMarkerDecorationNode, ToggleButtonNode,
    UpdateEngagementPanelActionNode, VerticalListNode, VideoOwnerNode, ViewCountNode,
    WatchEndpointNode,
};
pub use music::{
    MusicDescriptionShelfNode, MusicHeaderNode, MusicInlineBadgeNode, MusicNavigationButtonNode,
    MusicPlayButtonNode, MusicQueueNode, MusicResponsiveListItemNode, MusicTwoRowItemNode,
};
pub use music_extended::{
    MusicAutoplayNode, MusicCardShelfNode, MusicCarouselShelfNode, MusicDetailHeaderNode,
    MusicEditablePlaylistDetailHeaderNode, MusicImmersiveHeaderNode,
    MusicItemThumbnailOverlayNode, MusicPlaylistShelfNode, MusicResponsiveHeaderNode,
    MusicShelfNode, MusicSideAlignedItemNode, MusicVisualHeaderNode,
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
    FeedFilterChipBar(FeedFilterChipBarNode),
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
    AccountItemSection(AccountItemSectionNode),
    AccountItemSectionHeader(AccountItemSectionHeaderNode),
    KidsCategoriesHeader(KidsCategoriesHeaderNode),
    KidsHomeScreen(KidsHomeScreenNode),
    ClipCreation(ClipCreationNode),
    ClipCreationScrubber(ClipCreationScrubberNode),
    BadgeView(BadgeViewNode),
    CallToActionButton(CallToActionButtonNode),
    ButtonCardView(ButtonCardViewNode),
    AvatarView(AvatarViewNode),
    CompactLink(CompactLinkNode),
    // Batch 8: Grid & Compact renderers
    GridVideo(GridVideoNode),
    GridChannel(GridChannelNode),
    GridPlaylist(GridPlaylistNode),
    GridMix(GridMixNode),
    GridMovie(GridMovieNode),
    GridShow(GridShowNode),
    CompactVideo(CompactVideoNode),
    CompactChannel(CompactChannelNode),
    CompactPlaylist(CompactPlaylistNode),
    CompactMix(CompactMixNode),
    RichItem(RichItemNode),
    RichSection(RichSectionNode),
    // Batch 8: Music Extended renderers
    MusicCarouselShelf(MusicCarouselShelfNode),
    MusicShelf(MusicShelfNode),
    MusicSideAlignedItem(MusicSideAlignedItemNode),
    MusicVisualHeader(MusicVisualHeaderNode),
    MusicItemThumbnailOverlay(MusicItemThumbnailOverlayNode),
    MusicPlaylistShelf(MusicPlaylistShelfNode),
    MusicCardShelf(MusicCardShelfNode),
    MusicImmersiveHeader(MusicImmersiveHeaderNode),
    MusicDetailHeader(MusicDetailHeaderNode),
    MusicEditablePlaylistDetailHeader(MusicEditablePlaylistDetailHeaderNode),
    MusicResponsiveHeader(MusicResponsiveHeaderNode),
    MusicAutoplay(MusicAutoplayNode),
    // Batch 8: Overlay & Dialog renderers
    ThumbnailOverlayHoverText(ThumbnailOverlayHoverTextNode),
    ThumbnailOverlayEndorsement(ThumbnailOverlayEndorsementNode),
    ThumbnailOverlayNowPlaying(ThumbnailOverlayNowPlayingNode),
    ThumbnailOverlayLoadingPreview(ThumbnailOverlayLoadingPreviewNode),
    ThumbnailOverlayInlineUnplayable(ThumbnailOverlayInlineUnplayableNode),
    ThumbnailOverlayBottomPanel(ThumbnailOverlayBottomPanelNode),
    ThumbnailOverlaySidePanel(ThumbnailOverlaySidePanelNode),
    ThumbnailOverlayToggleButton(ThumbnailOverlayToggleButtonNode),
    DecoratedPlayerBar(DecoratedPlayerBarNode),
    ConfirmDialog(ConfirmDialogNode),
    Dialog(DialogNode),
    ModalWithTitleAndButton(ModalWithTitleAndButtonNode),
    // Batch 8: Engagement & Comments renderers
    EngagementPanelSectionList(EngagementPanelSectionListNode),
    EngagementPanelTitleHeader(EngagementPanelTitleHeaderNode),
    CommentsHeader(CommentsHeaderNode),
    CommentsEntryPointHeader(CommentsEntryPointHeaderNode),
    CommentActionButtons(CommentActionButtonsNode),
    CommentSimplebox(CommentSimpleboxNode),
    SubscriptionNotificationToggleButton(SubscriptionNotificationToggleButtonNode),
    InfoRow(InfoRowNode),
    CollageHeroImage(CollageHeroImageNode),
    FeedNudge(FeedNudgeNode),
    ChannelOwnerEmptyState(ChannelOwnerEmptyStateNode),
    TextHeader(TextHeaderNode),
    // Batch 9: Carousels & Views
    CarouselHeader(CarouselHeaderNode),
    CarouselItem(CarouselItemNode),
    CarouselItemView(CarouselItemViewNode),
    CarouselLockup(CarouselLockupNode),
    CarouselTitleView(CarouselTitleViewNode),
    ChipBarView(ChipBarViewNode),
    ChipView(ChipViewNode),
    ContentListItemView(ContentListItemViewNode),
    BackgroundPromo(BackgroundPromoNode),
    AttributionView(AttributionViewNode),
    AvatarStackView(AvatarStackViewNode),
    AnimatedThumbnailOverlayView(AnimatedThumbnailOverlayViewNode),
    // Batch 9: Cards & Interactive Items
    CardCollection(CardCollectionNode),
    CollaboratorInfoCardContent(CollaboratorInfoCardContentNode),
    CollectionThumbnailView(CollectionThumbnailViewNode),
    ClipAdState(ClipAdStateNode),
    ClipCreationTextInput(ClipCreationTextInputNode),
    ClientSideToggleMenuItem(ClientSideToggleMenuItemNode),
    AudioOnlyPlayability(AudioOnlyPlayabilityNode),
    CompactMovie(CompactMovieNode),
    CompactStation(CompactStationNode),
    AddToPlaylist(AddToPlaylistNode),
    C4TabbedHeader(C4TabbedHeaderNode),
    ChannelSwitcherPage(ChannelSwitcherPageNode),
    // Batch 9: LiveChat Extras
    LiveChatItemList(LiveChatItemListNode),
    LiveChatParticipantsList(LiveChatParticipantsListNode),
    LiveChatActionPanel(LiveChatActionPanelNode),
    AddBannerToLiveChatCommand(AddBannerToLiveChatCommandNode),
    RemoveBannerForLiveChatCommand(RemoveBannerForLiveChatCommandNode),
    AddLiveChatTickerItemAction(AddLiveChatTickerItemActionNode),
    DimChatItemAction(DimChatItemActionNode),
    RemoveChatItemAction(RemoveChatItemActionNode),
    RemoveChatItemByAuthorAction(RemoveChatItemByAuthorActionNode),
    ReplaceChatItemAction(ReplaceChatItemActionNode),
    ReplayChatItemAction(ReplayChatItemActionNode),
    UpdateLiveChatPollAction(UpdateLiveChatPollActionNode),
    // Batch 9: Commands & Actions
    AppendContinuationItemsAction(AppendContinuationItemsActionNode),
    GetMultiPageMenuAction(GetMultiPageMenuActionNode),
    OpenPopupAction(OpenPopupActionNode),
    SendFeedbackAction(SendFeedbackActionNode),
    SignalAction(SignalActionNode),
    UpdateChannelSwitcherPageAction(UpdateChannelSwitcherPageActionNode),
    UpdateSubscribeButtonAction(UpdateSubscribeButtonActionNode),
    AddToPlaylistCommand(AddToPlaylistCommandNode),
    ContinuationCommand(ContinuationCommandNode),
    ShowSheetCommand(ShowSheetCommandNode),
    UpdateEngagementPanelContentCommand(UpdateEngagementPanelContentCommandNode),
    RunAttestationCommand(RunAttestationCommandNode),
    // Batch 10: Dialogs & Views
    BrowseFeedActions(BrowseFeedActionsNode),
    ButtonView(ButtonViewNode),
    ClipSection(ClipSectionNode),
    ContentMetadataView(ContentMetadataViewNode),
    ContentPreviewImageView(ContentPreviewImageViewNode),
    ContinuationItem(ContinuationItemNode),
    ContinuationItemView(ContinuationItemViewNode),
    ConversationBar(ConversationBarNode),
    CopyLink(CopyLinkNode),
    CreatePlaylistDialog(CreatePlaylistDialogNode),
    CreatePlaylistDialogFormView(CreatePlaylistDialogFormViewNode),
    DecoratedAvatarView(DecoratedAvatarViewNode),
    // Batch 10: Previews & Dropdowns
    DefaultPromoPanel(DefaultPromoPanelNode),
    DescriptionPreviewView(DescriptionPreviewViewNode),
    DialogHeaderView(DialogHeaderViewNode),
    DialogView(DialogViewNode),
    DislikeButtonView(DislikeButtonViewNode),
    DismissableDialog(DismissableDialogNode),
    DismissableDialogContentSection(DismissableDialogContentSectionNode),
    DownloadButton(DownloadButtonNode),
    DownloadListItemView(DownloadListItemViewNode),
    Dropdown(DropdownNode),
    DropdownItem(DropdownItemNode),
    DropdownView(DropdownViewNode),
    // Batch 10: Forms & Emojis
    DynamicTextView(DynamicTextViewNode),
    Element(ElementNode),
    EmojiPickerCategory(EmojiPickerCategoryNode),
    EmojiPickerCategoryButton(EmojiPickerCategoryButtonNode),
    EmojiPickerUpsellCategory(EmojiPickerUpsellCategoryNode),
    EndScreenPlaylist(EndScreenPlaylistNode),
    EomSettingsDisclaimer(EomSettingsDisclaimerNode),
    ExpandableMetadata(ExpandableMetadataNode),
    ExpandedShelfContents(ExpandedShelfContentsNode),
    Factoid(FactoidNode),
    FancyDismissibleDialog(FancyDismissibleDialogNode),
    FeedTabbedHeader(FeedTabbedHeaderNode),
    // Batch 10: Headers & Grids
    FlexibleActionsView(FlexibleActionsViewNode),
    Form(FormNode),
    FormFooterView(FormFooterViewNode),
    FormPopup(FormPopupNode),
    GameCard(GameCardNode),
    GameDetails(GameDetailsNode),
    Grid(GridNode),
    GridHeader(GridHeaderNode),
    GridShelfView(GridShelfViewNode),
    GuideCollapsibleEntry(GuideCollapsibleEntryNode),
    GuideCollapsibleSectionEntry(GuideCollapsibleSectionEntryNode),
    GuideDownloadsEntry(GuideDownloadsEntryNode),
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

        // 32. Check for Clip Creation Components
        if val.get("clipCreationRenderer").is_some() {
            if let Some(cc) = ClipCreationNode::from_value(val) {
                return Some(YTNode::ClipCreation(cc));
            }
        }
        if val.get("clipCreationScrubberRenderer").is_some() {
            if let Some(ccs) = ClipCreationScrubberNode::from_value(val) {
                return Some(YTNode::ClipCreationScrubber(ccs));
            }
        }

        // 33. Check for Interactive Views & Buttons
        if val.get("badgeView").is_some() {
            if let Some(bv) = BadgeViewNode::from_value(val) {
                return Some(YTNode::BadgeView(bv));
            }
        }
        if val.get("callToActionButtonRenderer").is_some() {
            if let Some(cta) = CallToActionButtonNode::from_value(val) {
                return Some(YTNode::CallToActionButton(cta));
            }
        }
        if val.get("buttonCardView").is_some() {
            if let Some(bcv) = ButtonCardViewNode::from_value(val) {
                return Some(YTNode::ButtonCardView(bcv));
            }
        }
        if val.get("avatarView").is_some() {
            if let Some(av) = AvatarViewNode::from_value(val) {
                return Some(YTNode::AvatarView(av));
            }
        }
        if val.get("compactLinkRenderer").is_some() {
            if let Some(cl) = CompactLinkNode::from_value(val) {
                return Some(YTNode::CompactLink(cl));
            }
        }

        // 34. Check for Account Sections & Feed Filter Bars
        if val.get("accountItemSectionRenderer").is_some() {
            if let Some(ais) = AccountItemSectionNode::from_value(val) {
                return Some(YTNode::AccountItemSection(ais));
            }
        }
        if val.get("accountItemSectionHeaderRenderer").is_some() {
            if let Some(aish) = AccountItemSectionHeaderNode::from_value(val) {
                return Some(YTNode::AccountItemSectionHeader(aish));
            }
        }
        if val.get("feedFilterChipBarRenderer").is_some() {
            if let Some(ffcb) = FeedFilterChipBarNode::from_value(val) {
                return Some(YTNode::FeedFilterChipBar(ffcb));
            }
        }

        // 35. Grid & Compact renderers
        if val.get("gridVideoRenderer").is_some() {
            if let Some(n) = GridVideoNode::from_value(val) { return Some(YTNode::GridVideo(n)); }
        }
        if val.get("gridChannelRenderer").is_some() {
            if let Some(n) = GridChannelNode::from_value(val) { return Some(YTNode::GridChannel(n)); }
        }
        if val.get("gridPlaylistRenderer").is_some() {
            if let Some(n) = GridPlaylistNode::from_value(val) { return Some(YTNode::GridPlaylist(n)); }
        }
        if val.get("gridRadioRenderer").is_some() {
            if let Some(n) = GridMixNode::from_value(val) { return Some(YTNode::GridMix(n)); }
        }
        if val.get("gridMovieRenderer").is_some() {
            if let Some(n) = GridMovieNode::from_value(val) { return Some(YTNode::GridMovie(n)); }
        }
        if val.get("gridShowRenderer").is_some() {
            if let Some(n) = GridShowNode::from_value(val) { return Some(YTNode::GridShow(n)); }
        }
        if val.get("compactVideoRenderer").is_some() {
            if let Some(n) = CompactVideoNode::from_value(val) { return Some(YTNode::CompactVideo(n)); }
        }
        if val.get("compactChannelRenderer").is_some() {
            if let Some(n) = CompactChannelNode::from_value(val) { return Some(YTNode::CompactChannel(n)); }
        }
        if val.get("compactPlaylistRenderer").is_some() {
            if let Some(n) = CompactPlaylistNode::from_value(val) { return Some(YTNode::CompactPlaylist(n)); }
        }
        if val.get("compactRadioRenderer").is_some() {
            if let Some(n) = CompactMixNode::from_value(val) { return Some(YTNode::CompactMix(n)); }
        }
        if val.get("richItemRenderer").is_some() {
            if let Some(n) = RichItemNode::from_value(val) { return Some(YTNode::RichItem(n)); }
        }
        if val.get("richSectionRenderer").is_some() {
            if let Some(n) = RichSectionNode::from_value(val) { return Some(YTNode::RichSection(n)); }
        }

        // 36. Music Extended renderers
        if val.get("musicCarouselShelfRenderer").is_some() {
            if let Some(n) = MusicCarouselShelfNode::from_value(val) { return Some(YTNode::MusicCarouselShelf(n)); }
        }
        if val.get("musicShelfRenderer").is_some() {
            if let Some(n) = MusicShelfNode::from_value(val) { return Some(YTNode::MusicShelf(n)); }
        }
        if val.get("musicSideAlignedItemRenderer").is_some() {
            if let Some(n) = MusicSideAlignedItemNode::from_value(val) { return Some(YTNode::MusicSideAlignedItem(n)); }
        }
        if val.get("musicVisualHeaderRenderer").is_some() {
            if let Some(n) = MusicVisualHeaderNode::from_value(val) { return Some(YTNode::MusicVisualHeader(n)); }
        }
        if val.get("musicItemThumbnailOverlayRenderer").is_some() {
            if let Some(n) = MusicItemThumbnailOverlayNode::from_value(val) { return Some(YTNode::MusicItemThumbnailOverlay(n)); }
        }
        if val.get("musicPlaylistShelfRenderer").is_some() {
            if let Some(n) = MusicPlaylistShelfNode::from_value(val) { return Some(YTNode::MusicPlaylistShelf(n)); }
        }
        if val.get("musicCardShelfRenderer").is_some() {
            if let Some(n) = MusicCardShelfNode::from_value(val) { return Some(YTNode::MusicCardShelf(n)); }
        }
        if val.get("musicImmersiveHeaderRenderer").is_some() {
            if let Some(n) = MusicImmersiveHeaderNode::from_value(val) { return Some(YTNode::MusicImmersiveHeader(n)); }
        }
        if val.get("musicDetailHeaderRenderer").is_some() {
            if let Some(n) = MusicDetailHeaderNode::from_value(val) { return Some(YTNode::MusicDetailHeader(n)); }
        }
        if val.get("musicEditablePlaylistDetailHeaderRenderer").is_some() {
            if let Some(n) = MusicEditablePlaylistDetailHeaderNode::from_value(val) { return Some(YTNode::MusicEditablePlaylistDetailHeader(n)); }
        }
        if val.get("musicResponsiveHeaderRenderer").is_some() {
            if let Some(n) = MusicResponsiveHeaderNode::from_value(val) { return Some(YTNode::MusicResponsiveHeader(n)); }
        }
        if val.get("automixPreviewVideoRenderer").is_some() {
            if let Some(n) = MusicAutoplayNode::from_value(val) { return Some(YTNode::MusicAutoplay(n)); }
        }

        // 37. Overlay & Dialog renderers
        if val.get("thumbnailOverlayHoverTextRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayHoverTextNode::from_value(val) { return Some(YTNode::ThumbnailOverlayHoverText(n)); }
        }
        if val.get("thumbnailOverlayEndorsementRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayEndorsementNode::from_value(val) { return Some(YTNode::ThumbnailOverlayEndorsement(n)); }
        }
        if val.get("thumbnailOverlayNowPlayingRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayNowPlayingNode::from_value(val) { return Some(YTNode::ThumbnailOverlayNowPlaying(n)); }
        }
        if val.get("thumbnailOverlayLoadingPreviewRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayLoadingPreviewNode::from_value(val) { return Some(YTNode::ThumbnailOverlayLoadingPreview(n)); }
        }
        if val.get("thumbnailOverlayInlineUnplayableRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayInlineUnplayableNode::from_value(val) { return Some(YTNode::ThumbnailOverlayInlineUnplayable(n)); }
        }
        if val.get("thumbnailOverlayBottomPanelRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayBottomPanelNode::from_value(val) { return Some(YTNode::ThumbnailOverlayBottomPanel(n)); }
        }
        if val.get("thumbnailOverlaySidePanelRenderer").is_some() {
            if let Some(n) = ThumbnailOverlaySidePanelNode::from_value(val) { return Some(YTNode::ThumbnailOverlaySidePanel(n)); }
        }
        if val.get("thumbnailOverlayToggleButtonRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayToggleButtonNode::from_value(val) { return Some(YTNode::ThumbnailOverlayToggleButton(n)); }
        }
        if val.get("decoratedPlayerBarRenderer").is_some() {
            if let Some(n) = DecoratedPlayerBarNode::from_value(val) { return Some(YTNode::DecoratedPlayerBar(n)); }
        }
        if val.get("confirmDialogRenderer").is_some() {
            if let Some(n) = ConfirmDialogNode::from_value(val) { return Some(YTNode::ConfirmDialog(n)); }
        }
        if val.get("dialogRenderer").is_some() {
            if let Some(n) = DialogNode::from_value(val) { return Some(YTNode::Dialog(n)); }
        }
        if val.get("modalWithTitleAndButtonRenderer").is_some() {
            if let Some(n) = ModalWithTitleAndButtonNode::from_value(val) { return Some(YTNode::ModalWithTitleAndButton(n)); }
        }

        // 38. Engagement & Comments renderers
        if val.get("engagementPanelSectionListRenderer").is_some() {
            if let Some(n) = EngagementPanelSectionListNode::from_value(val) { return Some(YTNode::EngagementPanelSectionList(n)); }
        }
        if val.get("engagementPanelTitleHeaderRenderer").is_some() {
            if let Some(n) = EngagementPanelTitleHeaderNode::from_value(val) { return Some(YTNode::EngagementPanelTitleHeader(n)); }
        }
        if val.get("commentsHeaderRenderer").is_some() {
            if let Some(n) = CommentsHeaderNode::from_value(val) { return Some(YTNode::CommentsHeader(n)); }
        }
        if val.get("commentsEntryPointHeaderRenderer").is_some() {
            if let Some(n) = CommentsEntryPointHeaderNode::from_value(val) { return Some(YTNode::CommentsEntryPointHeader(n)); }
        }
        if val.get("commentActionButtonsRenderer").is_some() {
            if let Some(n) = CommentActionButtonsNode::from_value(val) { return Some(YTNode::CommentActionButtons(n)); }
        }
        if val.get("commentSimpleboxRenderer").is_some() {
            if let Some(n) = CommentSimpleboxNode::from_value(val) { return Some(YTNode::CommentSimplebox(n)); }
        }
        if val.get("subscriptionNotificationToggleButtonRenderer").is_some() {
            if let Some(n) = SubscriptionNotificationToggleButtonNode::from_value(val) { return Some(YTNode::SubscriptionNotificationToggleButton(n)); }
        }
        if val.get("infoRowRenderer").is_some() {
            if let Some(n) = InfoRowNode::from_value(val) { return Some(YTNode::InfoRow(n)); }
        }
        if val.get("collageHeroImageRenderer").is_some() {
            if let Some(n) = CollageHeroImageNode::from_value(val) { return Some(YTNode::CollageHeroImage(n)); }
        }
        if val.get("feedNudgeRenderer").is_some() {
            if let Some(n) = FeedNudgeNode::from_value(val) { return Some(YTNode::FeedNudge(n)); }
        }
        if val.get("channelOwnerEmptyStateRenderer").is_some() {
            if let Some(n) = ChannelOwnerEmptyStateNode::from_value(val) { return Some(YTNode::ChannelOwnerEmptyState(n)); }
        }
        if val.get("textHeaderRenderer").is_some() {
            if let Some(n) = TextHeaderNode::from_value(val) { return Some(YTNode::TextHeader(n)); }
        }

        // 39. Carousels & Views (Batch 9)
        if val.get("carouselHeaderRenderer").is_some() {
            if let Some(n) = CarouselHeaderNode::from_value(val) { return Some(YTNode::CarouselHeader(n)); }
        }
        if val.get("carouselItemRenderer").is_some() {
            if let Some(n) = CarouselItemNode::from_value(val) { return Some(YTNode::CarouselItem(n)); }
        }
        if val.get("carouselItemView").is_some() {
            if let Some(n) = CarouselItemViewNode::from_value(val) { return Some(YTNode::CarouselItemView(n)); }
        }
        if val.get("carouselLockupRenderer").is_some() {
            if let Some(n) = CarouselLockupNode::from_value(val) { return Some(YTNode::CarouselLockup(n)); }
        }
        if val.get("carouselTitleView").is_some() {
            if let Some(n) = CarouselTitleViewNode::from_value(val) { return Some(YTNode::CarouselTitleView(n)); }
        }
        if val.get("chipBarView").is_some() {
            if let Some(n) = ChipBarViewNode::from_value(val) { return Some(YTNode::ChipBarView(n)); }
        }
        if val.get("chipView").is_some() {
            if let Some(n) = ChipViewNode::from_value(val) { return Some(YTNode::ChipView(n)); }
        }
        if val.get("contentListItemView").is_some() {
            if let Some(n) = ContentListItemViewNode::from_value(val) { return Some(YTNode::ContentListItemView(n)); }
        }
        if val.get("backgroundPromoRenderer").is_some() {
            if let Some(n) = BackgroundPromoNode::from_value(val) { return Some(YTNode::BackgroundPromo(n)); }
        }
        if val.get("attributionView").is_some() {
            if let Some(n) = AttributionViewNode::from_value(val) { return Some(YTNode::AttributionView(n)); }
        }
        if val.get("avatarStackView").is_some() {
            if let Some(n) = AvatarStackViewNode::from_value(val) { return Some(YTNode::AvatarStackView(n)); }
        }
        if val.get("animatedThumbnailOverlayView").is_some() {
            if let Some(n) = AnimatedThumbnailOverlayViewNode::from_value(val) { return Some(YTNode::AnimatedThumbnailOverlayView(n)); }
        }

        // 40. Cards & Interactive Items (Batch 9)
        if val.get("cardCollectionRenderer").is_some() {
            if let Some(n) = CardCollectionNode::from_value(val) { return Some(YTNode::CardCollection(n)); }
        }
        if val.get("collaboratorInfoCardContentRenderer").is_some() {
            if let Some(n) = CollaboratorInfoCardContentNode::from_value(val) { return Some(YTNode::CollaboratorInfoCardContent(n)); }
        }
        if val.get("collectionThumbnailView").is_some() {
            if let Some(n) = CollectionThumbnailViewNode::from_value(val) { return Some(YTNode::CollectionThumbnailView(n)); }
        }
        if val.get("clipAdStateRenderer").is_some() {
            if let Some(n) = ClipAdStateNode::from_value(val) { return Some(YTNode::ClipAdState(n)); }
        }
        if val.get("clipCreationTextInputRenderer").is_some() {
            if let Some(n) = ClipCreationTextInputNode::from_value(val) { return Some(YTNode::ClipCreationTextInput(n)); }
        }
        if val.get("clientSideToggleMenuItemRenderer").is_some() {
            if let Some(n) = ClientSideToggleMenuItemNode::from_value(val) { return Some(YTNode::ClientSideToggleMenuItem(n)); }
        }
        if val.get("audioOnlyPlayabilityRenderer").is_some() {
            if let Some(n) = AudioOnlyPlayabilityNode::from_value(val) { return Some(YTNode::AudioOnlyPlayability(n)); }
        }
        if val.get("compactMovieRenderer").is_some() {
            if let Some(n) = CompactMovieNode::from_value(val) { return Some(YTNode::CompactMovie(n)); }
        }
        if val.get("compactStationRenderer").is_some() {
            if let Some(n) = CompactStationNode::from_value(val) { return Some(YTNode::CompactStation(n)); }
        }
        if val.get("addToPlaylistRenderer").is_some() {
            if let Some(n) = AddToPlaylistNode::from_value(val) { return Some(YTNode::AddToPlaylist(n)); }
        }
        if val.get("c4TabbedHeaderRenderer").is_some() {
            if let Some(n) = C4TabbedHeaderNode::from_value(val) { return Some(YTNode::C4TabbedHeader(n)); }
        }
        if val.get("channelSwitcherPageRenderer").is_some() {
            if let Some(n) = ChannelSwitcherPageNode::from_value(val) { return Some(YTNode::ChannelSwitcherPage(n)); }
        }

        // 41. LiveChat Extras (Batch 9)
        if val.get("liveChatItemListRenderer").is_some() {
            if let Some(n) = LiveChatItemListNode::from_value(val) { return Some(YTNode::LiveChatItemList(n)); }
        }
        if val.get("liveChatParticipantsListRenderer").is_some() {
            if let Some(n) = LiveChatParticipantsListNode::from_value(val) { return Some(YTNode::LiveChatParticipantsList(n)); }
        }
        if val.get("liveChatActionPanelRenderer").is_some() {
            if let Some(n) = LiveChatActionPanelNode::from_value(val) { return Some(YTNode::LiveChatActionPanel(n)); }
        }
        if val.get("addBannerToLiveChatCommand").is_some() {
            if let Some(n) = AddBannerToLiveChatCommandNode::from_value(val) { return Some(YTNode::AddBannerToLiveChatCommand(n)); }
        }
        if val.get("removeBannerForLiveChatCommand").is_some() {
            if let Some(n) = RemoveBannerForLiveChatCommandNode::from_value(val) { return Some(YTNode::RemoveBannerForLiveChatCommand(n)); }
        }
        if val.get("addLiveChatTickerItemAction").is_some() {
            if let Some(n) = AddLiveChatTickerItemActionNode::from_value(val) { return Some(YTNode::AddLiveChatTickerItemAction(n)); }
        }
        if val.get("dimChatItemAction").is_some() {
            if let Some(n) = DimChatItemActionNode::from_value(val) { return Some(YTNode::DimChatItemAction(n)); }
        }
        if val.get("removeChatItemAction").is_some() {
            if let Some(n) = RemoveChatItemActionNode::from_value(val) { return Some(YTNode::RemoveChatItemAction(n)); }
        }
        if val.get("removeChatItemByAuthorAction").is_some() {
            if let Some(n) = RemoveChatItemByAuthorActionNode::from_value(val) { return Some(YTNode::RemoveChatItemByAuthorAction(n)); }
        }
        if val.get("replaceChatItemAction").is_some() {
            if let Some(n) = ReplaceChatItemActionNode::from_value(val) { return Some(YTNode::ReplaceChatItemAction(n)); }
        }
        if val.get("replayChatItemAction").is_some() {
            if let Some(n) = ReplayChatItemActionNode::from_value(val) { return Some(YTNode::ReplayChatItemAction(n)); }
        }
        if val.get("updateLiveChatPollAction").is_some() {
            if let Some(n) = UpdateLiveChatPollActionNode::from_value(val) { return Some(YTNode::UpdateLiveChatPollAction(n)); }
        }

        // 42. Commands & Actions (Batch 9)
        if val.get("appendContinuationItemsAction").is_some() {
            if let Some(n) = AppendContinuationItemsActionNode::from_value(val) { return Some(YTNode::AppendContinuationItemsAction(n)); }
        }
        if val.get("getMultiPageMenuAction").is_some() {
            if let Some(n) = GetMultiPageMenuActionNode::from_value(val) { return Some(YTNode::GetMultiPageMenuAction(n)); }
        }
        if val.get("openPopupAction").is_some() {
            if let Some(n) = OpenPopupActionNode::from_value(val) { return Some(YTNode::OpenPopupAction(n)); }
        }
        if val.get("sendFeedbackAction").is_some() {
            if let Some(n) = SendFeedbackActionNode::from_value(val) { return Some(YTNode::SendFeedbackAction(n)); }
        }
        if val.get("signalAction").is_some() {
            if let Some(n) = SignalActionNode::from_value(val) { return Some(YTNode::SignalAction(n)); }
        }
        if val.get("updateChannelSwitcherPageAction").is_some() {
            if let Some(n) = UpdateChannelSwitcherPageActionNode::from_value(val) { return Some(YTNode::UpdateChannelSwitcherPageAction(n)); }
        }
        if val.get("updateSubscribeButtonAction").is_some() {
            if let Some(n) = UpdateSubscribeButtonActionNode::from_value(val) { return Some(YTNode::UpdateSubscribeButtonAction(n)); }
        }
        if val.get("addToPlaylistCommand").is_some() {
            if let Some(n) = AddToPlaylistCommandNode::from_value(val) { return Some(YTNode::AddToPlaylistCommand(n)); }
        }
        if val.get("continuationCommand").is_some() {
            if let Some(n) = ContinuationCommandNode::from_value(val) { return Some(YTNode::ContinuationCommand(n)); }
        }
        if val.get("showSheetCommand").is_some() {
            if let Some(n) = ShowSheetCommandNode::from_value(val) { return Some(YTNode::ShowSheetCommand(n)); }
        }
        if val.get("updateEngagementPanelContentCommand").is_some() {
            if let Some(n) = UpdateEngagementPanelContentCommandNode::from_value(val) { return Some(YTNode::UpdateEngagementPanelContentCommand(n)); }
        }
        if val.get("runAttestationCommand").is_some() {
            if let Some(n) = RunAttestationCommandNode::from_value(val) { return Some(YTNode::RunAttestationCommand(n)); }
        }

        // 43. Dialogs & Views (Batch 10)
        if val.get("browseFeedActionsRenderer").is_some() || val.get("browseFeedActions").is_some() {
            if let Some(n) = BrowseFeedActionsNode::from_value(val) { return Some(YTNode::BrowseFeedActions(n)); }
        }
        if val.get("buttonView").is_some() {
            if let Some(n) = ButtonViewNode::from_value(val) { return Some(YTNode::ButtonView(n)); }
        }
        if val.get("clipSectionRenderer").is_some() {
            if let Some(n) = ClipSectionNode::from_value(val) { return Some(YTNode::ClipSection(n)); }
        }
        if val.get("contentMetadataView").is_some() {
            if let Some(n) = ContentMetadataViewNode::from_value(val) { return Some(YTNode::ContentMetadataView(n)); }
        }
        if val.get("contentPreviewImageView").is_some() {
            if let Some(n) = ContentPreviewImageViewNode::from_value(val) { return Some(YTNode::ContentPreviewImageView(n)); }
        }
        if val.get("continuationItemRenderer").is_some() {
            if let Some(n) = ContinuationItemNode::from_value(val) { return Some(YTNode::ContinuationItem(n)); }
        }
        if val.get("continuationItemView").is_some() {
            if let Some(n) = ContinuationItemViewNode::from_value(val) { return Some(YTNode::ContinuationItemView(n)); }
        }
        if val.get("conversationBarRenderer").is_some() {
            if let Some(n) = ConversationBarNode::from_value(val) { return Some(YTNode::ConversationBar(n)); }
        }
        if val.get("copyLinkRenderer").is_some() {
            if let Some(n) = CopyLinkNode::from_value(val) { return Some(YTNode::CopyLink(n)); }
        }
        if val.get("createPlaylistDialogRenderer").is_some() {
            if let Some(n) = CreatePlaylistDialogNode::from_value(val) { return Some(YTNode::CreatePlaylistDialog(n)); }
        }
        if val.get("createPlaylistDialogFormView").is_some() {
            if let Some(n) = CreatePlaylistDialogFormViewNode::from_value(val) { return Some(YTNode::CreatePlaylistDialogFormView(n)); }
        }
        if val.get("decoratedAvatarView").is_some() {
            if let Some(n) = DecoratedAvatarViewNode::from_value(val) { return Some(YTNode::DecoratedAvatarView(n)); }
        }

        // 44. Previews & Dropdowns (Batch 10)
        if val.get("defaultPromoPanelRenderer").is_some() {
            if let Some(n) = DefaultPromoPanelNode::from_value(val) { return Some(YTNode::DefaultPromoPanel(n)); }
        }
        if val.get("descriptionPreviewView").is_some() {
            if let Some(n) = DescriptionPreviewViewNode::from_value(val) { return Some(YTNode::DescriptionPreviewView(n)); }
        }
        if val.get("dialogHeaderView").is_some() {
            if let Some(n) = DialogHeaderViewNode::from_value(val) { return Some(YTNode::DialogHeaderView(n)); }
        }
        if val.get("dialogView").is_some() {
            if let Some(n) = DialogViewNode::from_value(val) { return Some(YTNode::DialogView(n)); }
        }
        if val.get("dislikeButtonView").is_some() {
            if let Some(n) = DislikeButtonViewNode::from_value(val) { return Some(YTNode::DislikeButtonView(n)); }
        }
        if val.get("dismissableDialogRenderer").is_some() {
            if let Some(n) = DismissableDialogNode::from_value(val) { return Some(YTNode::DismissableDialog(n)); }
        }
        if val.get("dismissableDialogContentSectionRenderer").is_some() {
            if let Some(n) = DismissableDialogContentSectionNode::from_value(val) { return Some(YTNode::DismissableDialogContentSection(n)); }
        }
        if val.get("downloadButtonRenderer").is_some() {
            if let Some(n) = DownloadButtonNode::from_value(val) { return Some(YTNode::DownloadButton(n)); }
        }
        if val.get("downloadListItemView").is_some() {
            if let Some(n) = DownloadListItemViewNode::from_value(val) { return Some(YTNode::DownloadListItemView(n)); }
        }
        if val.get("dropdownRenderer").is_some() {
            if let Some(n) = DropdownNode::from_value(val) { return Some(YTNode::Dropdown(n)); }
        }
        if val.get("dropdownItemRenderer").is_some() {
            if let Some(n) = DropdownItemNode::from_value(val) { return Some(YTNode::DropdownItem(n)); }
        }
        if val.get("dropdownView").is_some() {
            if let Some(n) = DropdownViewNode::from_value(val) { return Some(YTNode::DropdownView(n)); }
        }

        // 45. Forms & Emojis (Batch 10)
        if val.get("dynamicTextView").is_some() {
            if let Some(n) = DynamicTextViewNode::from_value(val) { return Some(YTNode::DynamicTextView(n)); }
        }
        if val.get("elementRenderer").is_some() {
            if let Some(n) = ElementNode::from_value(val) { return Some(YTNode::Element(n)); }
        }
        if val.get("emojiPickerCategoryRenderer").is_some() {
            if let Some(n) = EmojiPickerCategoryNode::from_value(val) { return Some(YTNode::EmojiPickerCategory(n)); }
        }
        if val.get("emojiPickerCategoryButtonRenderer").is_some() {
            if let Some(n) = EmojiPickerCategoryButtonNode::from_value(val) { return Some(YTNode::EmojiPickerCategoryButton(n)); }
        }
        if val.get("emojiPickerUpsellCategoryRenderer").is_some() {
            if let Some(n) = EmojiPickerUpsellCategoryNode::from_value(val) { return Some(YTNode::EmojiPickerUpsellCategory(n)); }
        }
        if val.get("endScreenPlaylistRenderer").is_some() {
            if let Some(n) = EndScreenPlaylistNode::from_value(val) { return Some(YTNode::EndScreenPlaylist(n)); }
        }
        if val.get("eomSettingsDisclaimerRenderer").is_some() {
            if let Some(n) = EomSettingsDisclaimerNode::from_value(val) { return Some(YTNode::EomSettingsDisclaimer(n)); }
        }
        if val.get("expandableMetadataRenderer").is_some() {
            if let Some(n) = ExpandableMetadataNode::from_value(val) { return Some(YTNode::ExpandableMetadata(n)); }
        }
        if val.get("expandedShelfContentsRenderer").is_some() {
            if let Some(n) = ExpandedShelfContentsNode::from_value(val) { return Some(YTNode::ExpandedShelfContents(n)); }
        }
        if val.get("factoidRenderer").is_some() {
            if let Some(n) = FactoidNode::from_value(val) { return Some(YTNode::Factoid(n)); }
        }
        if val.get("fancyDismissibleDialogRenderer").is_some() {
            if let Some(n) = FancyDismissibleDialogNode::from_value(val) { return Some(YTNode::FancyDismissibleDialog(n)); }
        }
        if val.get("feedTabbedHeaderRenderer").is_some() {
            if let Some(n) = FeedTabbedHeaderNode::from_value(val) { return Some(YTNode::FeedTabbedHeader(n)); }
        }

        // 46. Headers & Grids (Batch 10)
        if val.get("flexibleActionsView").is_some() {
            if let Some(n) = FlexibleActionsViewNode::from_value(val) { return Some(YTNode::FlexibleActionsView(n)); }
        }
        if val.get("formRenderer").is_some() {
            if let Some(n) = FormNode::from_value(val) { return Some(YTNode::Form(n)); }
        }
        if val.get("formFooterView").is_some() {
            if let Some(n) = FormFooterViewNode::from_value(val) { return Some(YTNode::FormFooterView(n)); }
        }
        if val.get("formPopupRenderer").is_some() {
            if let Some(n) = FormPopupNode::from_value(val) { return Some(YTNode::FormPopup(n)); }
        }
        if val.get("gameCardRenderer").is_some() {
            if let Some(n) = GameCardNode::from_value(val) { return Some(YTNode::GameCard(n)); }
        }
        if val.get("gameDetailsRenderer").is_some() {
            if let Some(n) = GameDetailsNode::from_value(val) { return Some(YTNode::GameDetails(n)); }
        }
        if val.get("gridRenderer").is_some() {
            if let Some(n) = GridNode::from_value(val) { return Some(YTNode::Grid(n)); }
        }
        if val.get("gridHeaderRenderer").is_some() {
            if let Some(n) = GridHeaderNode::from_value(val) { return Some(YTNode::GridHeader(n)); }
        }
        if val.get("gridShelfView").is_some() {
            if let Some(n) = GridShelfViewNode::from_value(val) { return Some(YTNode::GridShelfView(n)); }
        }
        if val.get("guideCollapsibleEntryRenderer").is_some() {
            if let Some(n) = GuideCollapsibleEntryNode::from_value(val) { return Some(YTNode::GuideCollapsibleEntry(n)); }
        }
        if val.get("guideCollapsibleSectionEntryRenderer").is_some() {
            if let Some(n) = GuideCollapsibleSectionEntryNode::from_value(val) { return Some(YTNode::GuideCollapsibleSectionEntry(n)); }
        }
        if val.get("guideDownloadsEntryRenderer").is_some() {
            if let Some(n) = GuideDownloadsEntryNode::from_value(val) { return Some(YTNode::GuideDownloadsEntry(n)); }
        }

        None
    }
}

