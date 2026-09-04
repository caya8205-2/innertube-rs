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

use crate::parser::registry::YTNodeVariant;

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
    pub use continuation::{
        ContinuationNode, GridContinuationNode, ItemSectionContinuationNode,
        LiveChatContinuationNode, MusicPlaylistShelfContinuationNode, MusicShelfContinuationNode,
        PlaylistPanelContinuationNode, ReloadContinuationItemsCommandNode,
        SectionListContinuationNode,
    };
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
    AddToPlaylistEndpointNode, AddToPlaylistNode, AddToPlaylistServiceEndpointNode, AlertNode,
    AnimatedThumbnailOverlayViewNode, AttributionViewNode, AudioOnlyPlayabilityNode, AuthorNode,
    AvatarStackViewNode, AvatarViewNode, BackgroundPromoNode, BadgeViewNode, BrowseEndpointNode,
    BrowseFeedActionsNode, ButtonCardViewNode, ButtonNode, ButtonViewNode, C4TabbedHeaderNode,
    CallToActionButtonNode, CardCollectionNode, CardNode, CarouselHeaderNode, CarouselItemNode,
    CarouselItemViewNode, CarouselLockupNode, CarouselTitleViewNode, ChannelOwnerEmptyStateNode,
    ChannelSwitcherPageNode, ChapterNode, ChipBarViewNode, ChipViewNode, ClarificationNode,
    ClientSideToggleMenuItemNode, ClipAdStateNode, ClipCreationNode, ClipCreationScrubberNode,
    ClipCreationTextInputNode, ClipSectionNode, CollaboratorInfoCardContentNode,
    CollectionThumbnailViewNode, CollageHeroImageNode, CommandExecutorCommandNode,
    CommentActionButtonsNode, CommentDialogNode, CommentReplyDialogNode, CommentSimpleboxNode,
    CommentsEntryPointHeaderNode, CommentsHeaderNode, CompactLinkNode, CompactMovieNode,
    CompactStationNode, ConfirmDialogNode, ContentListItemViewNode, ContentMetadataViewNode,
    ContentPreviewImageViewNode, ContinuationEndpointNode, ContinuationItemNode,
    ContinuationItemViewNode, ConversationBarNode, CopyLinkNode, CreateCommentEndpointNode,
    CreatePlaylistDialogFormViewNode, CreatePlaylistDialogNode, CreatePlaylistServiceEndpointNode,
    DecoratedAvatarViewNode, DecoratedPlayerBarNode, DefaultPromoPanelNode, DeletePlaylistEndpointNode,
    DescriptionPreviewViewNode, DialogHeaderViewNode, DialogNode, DialogViewNode, DidYouMeanNode,
    DislikeButtonViewNode, DismissableDialogContentSectionNode, DismissableDialogNode,
    DownloadButtonNode, DownloadListItemViewNode, DropdownItemNode, DropdownNode, DropdownViewNode,
    DynamicTextViewNode, ElementNode, EmojiPickerCategoryButtonNode, EmojiPickerCategoryNode,
    EmojiPickerNode, EmojiPickerUpsellCategoryNode, EndScreenPlaylistNode, EndscreenElementNode,
    EndscreenNode, EngagementPanelSectionListNode, EngagementPanelTitleHeaderNode,
    EomSettingsDisclaimerNode, ExpandableMetadataNode, ExpandableTabNode,
    ExpandedShelfContentsNode, FactoidNode, FancyDismissibleDialogNode, FeedbackEndpointNode,
    FeedNudgeNode, FeedTabbedHeaderNode, FlexibleActionsViewNode, FormFooterViewNode, FormNode,
    FormPopupNode, GameCardNode, GameDetailsNode, GetAccountsListInnertubeEndpointNode,
    GetKidsBlocklistPickerCommandNode, GridHeaderNode, GridNode, GridShelfViewNode,
    GuideCollapsibleEntryNode, GuideCollapsibleSectionEntryNode, GuideDownloadsEntryNode,
    GuideEntryNode, GuideSectionNode, GuideSubscriptionsSectionNode, HashtagHeaderNode,
    HashtagTileNode, HeatMarkerNode, HeatmapNode, HeroPlaylistThumbnailNode, HideEngagementPanelEndpointNode,
    HighlightsCarouselNode, HistorySuggestionNode, HorizontalCardListNode, HorizontalListNode,
    HorizontalMovieListNode, HowThisWasMadeSectionViewNode, HypeFanCreditsSectionViewNode,
    HypePointsFactoidNode, IconLinkNode, ImageBannerViewNode, IncludingResultsForNode,
    InfoPanelContainerNode, InfoPanelContentNode, InfoRowNode, InteractiveTabbedHeaderNode,
    ItemSectionHeaderNode, ItemSectionTabNode, ItemSectionTabbedHeaderNode, KidsCategoriesHeaderNode,
    KidsHomeScreenNode, LikeButtonNode, LikeButtonViewNode, LikeEndpointNode, ListItemViewNode,
    ListViewNode, LiveChatDialogNode, LiveChatItemContextMenuEndpointNode, LockupMetadataViewNode,
    LockupViewNode, MacroMarkersInfoItemNode, MacroMarkersListEntityNode, MacroMarkersListItemNode,
    MacroMarkersListNode, MenuItemNode, MenuNode, MenuTitleNode, MerchandiseItemNode,
    MerchandiseShelfNode, MessageNode, MetadataBadgeNode, MetadataRowContainerNode,
    MetadataRowHeaderNode, MetadataRowNode, MetadataScreenNode, MicroformatDataNode, MixNode,
    ModalWithTitleAndButtonNode, ModifyChannelNotificationPreferenceEndpointNode, MovieNode,
    MovingThumbnailNode, MultiMarkersPlayerBarNode, MusicCardShelfHeaderBasicNode,
    MusicCarouselShelfBasicHeaderNode, MusicLargeCardItemCarouselNode, MusicMultiRowListItemNode,
    MusicPlaylistEditHeaderNode, MusicResponsiveListItemFixedColumnNode,
    MusicResponsiveListItemFlexColumnNode, MusicTastebuilderShelfNode,
    MusicTastebuilderShelfThumbnailNode, NavigateActionNode, NavigationEndpointNode,
    NotificationActionNode, NotificationNode, OpenOnePickAddVideoModalCommandNode, PageHeaderNode,
    PageHeaderViewNode, PageIndicatorViewNode, PageIntroductionNode, PanelFooterViewNode,
    PerformCommentActionEndpointNode, PivotButtonNode, PlayerCaptionsTracklistNode,
    PlayerErrorMessageNode, PlayerLegacyDesktopYpcTrailerNode, PlayerOverlayNode,
    PlayerStoryboardSpecNode, PlaylistAddToOptionNode, PlaylistCollaborationViewNode,
    PlaylistCustomThumbnailNode, PlaylistEditEndpointNode, PlaylistHeaderNode,
    PlaylistInfoCardContentNode, PlaylistPanelVideoWrapperNode, PlaylistSidebarNode,
    PlaylistThumbnailOverlayNode, PlaylistVideoListNode, PlaylistVideoThumbnailNode, PollNode,
    PrefetchWatchCommandNode, PremiereTrailerBadgeNode, ProductListHeaderNode, ProductListItemNode,
    ProductListNode, ProfileColumnNode, ProfileColumnStatsEntryNode, ProfileColumnStatsNode,
    ProfileColumnUserInfoNode, QuizNode, RecognitionShelfNode, ReelWatchEndpointNode,
    RelatedChipCloudNode, RichListHeaderNode, RichMetadataNode, RichMetadataRowNode, SearchBoxNode,
    SearchEndpointNode, SearchFilterGroupNode, SearchFilterNode, SearchFilterOptionsDialogNode,
    SearchHeaderNode, SearchRefinementCardNode, SearchSubMenuNode, SearchSuggestionNode,
    SearchSuggestionsSectionNode, SecondarySearchContainerNode, SectionHeaderViewNode,
    SegmentedLikeDislikeButtonNode, SegmentedLikeDislikeButtonViewNode, SettingBooleanNode,
    SettingsCheckboxNode, SettingsOptionsNode, SettingsSidebarNode, SettingsSwitchNode,
    ShareEndpointNode, ShareEntityEndpointNode, SharePanelHeaderNode, SharePanelTitleV15Node,
    ShareTargetNode, SheetViewNode, ShowCustomThumbnailNode, ShowDialogCommandNode,
    ShowEngagementPanelActionNode, ShowLiveChatActionNode, ShowingResultsForNode,
    SimpleCardContentNode, SimpleCardTeaserNode, SimpleTextSectionNode, SingleColumnBrowseResultsNode,
    SingleColumnMusicWatchNextResultsNode, SingleHeroImageNode, SlimOwnerNode,
    SortFilterHeaderNode, SortFilterSubMenuNode, StartAtNode, StructuredDescriptionContentNode,
    StructuredDescriptionPlaylistLockupNode, SubFeedOptionNode, SubFeedSelectorNode,
    SubscribeButtonNode, SubscribeButtonViewNode, SubscribeEndpointNode,
    SubscriptionNotificationToggleButtonNode, TabbedNode, TabbedSearchResultsNode,
    TextCarouselItemViewNode, TextFieldViewNode, TextHeaderNode, TextNode, TextRunNode,
    ThirdPartyShareTargetSectionNode, ThumbnailBadgeViewNode, ThumbnailBottomOverlayViewNode,
    ThumbnailHoverOverlayToggleActionsViewNode, ThumbnailHoverOverlayViewNode,
    ThumbnailLandscapePortraitNode, ThumbnailListNode, ThumbnailNode,
    ThumbnailOverlayAvatarStackViewNode, ThumbnailOverlayBadgeViewNode,
    ThumbnailOverlayBottomPanelNode, ThumbnailOverlayEndorsementNode,
    ThumbnailOverlayHoverTextNode, ThumbnailOverlayInlineUnplayableNode,
    ThumbnailOverlayLoadingPreviewNode, ThumbnailOverlayNowPlayingNode,
    ThumbnailOverlayPinkingNode, ThumbnailOverlayPlaybackStatusNode,
    ThumbnailOverlayProgressBarNode, ThumbnailOverlayProgressBarViewNode,
    ThumbnailOverlayResumePlaybackNode, ThumbnailOverlaySidePanelNode,
    ThumbnailOverlayTimeStatusNode, ThumbnailOverlayTitleViewNode,
    ThumbnailOverlayToggleButtonNode, ThumbnailViewNode, TicketEventNode, TicketShelfNode,
    TimedMarkerDecorationNode, TitleAndButtonListHeaderNode, ToggleButtonNode, ToggleButtonViewNode,
    ToggleFormFieldNode, ToggleMenuServiceItemNode, TooltipNode, TranscriptFooterNode,
    TranscriptNode, TranscriptSearchBoxNode, TranscriptSearchPanelNode, TranscriptSectionHeaderNode,
    TranscriptSegmentListNode, TranscriptSegmentNode, TwoColumnBrowseResultsNode,
    TwoColumnSearchResultsNode, TwoColumnWatchNextResultsNode, UnifiedSharePanelNode,
    UniversalWatchCardNode, UpdateEngagementPanelActionNode, UploadTimeFactoidNode, UpsellDialogNode,
    VerticalListNode, VerticalWatchCardListNode, VideoAttributesSectionViewNode,
    VideoDescriptionCourseSectionNode, VideoDescriptionInfocardsSectionNode,
    VideoDescriptionMusicSectionNode, VideoDescriptionTranscriptSectionNode,
    VideoDescriptionYouchatSectionViewNode, VideoMetadataCarouselViewNode, VideoOwnerNode,
    ViewCountFactoidNode, ViewCountNode, VoiceReplyContainerViewNode, WatchCardRichHeaderNode,
    WatchCardSectionSequenceNode, WatchEndpointNode, WatchNextEndScreenNode,
    WatchNextTabbedResultsNode, YpcTrailerNode,
    // Batch 15: Menus & Mobile Topbar
    MenuFlexibleItemNode, MenuNavigationItemNode, MenuPopupNode, MenuServiceItemDownloadNode,
    MenuServiceItemNode, MobileTopbarNode, MultiPageMenuNode, MultiPageMenuNotificationSectionNode,
    MultiPageMenuSectionNode, PivotBarItemNode, PivotBarNode, SimpleMenuHeaderNode,
    TopbarMenuButtonNode,
    // Batch 15: Livechat Actions & Collaboration
    BumperUserEduContentViewNode, CommandContextNode, PdgReplyButtonViewNode,
    PlaylistCollaborationFormDataNode, PlaylistCollaborationFormSchemaNode,
    PlaylistCollaborationViewModelPlaylistCollaboratorDataNode, ReplaceLiveChatActionNode,
    SubscriptionButtonNode, UpdateDateTextActionNode, UpdateDescriptionActionNode,
    UpdateTitleActionNode, UpdateToggleButtonTextActionNode, UpdateViewershipActionNode,
    // Batch 15: Endpoints Primitives & Kids
    AccessibilityContextNode, AccessibilityDataNode, AnchoredSectionNode, ChildElementNode,
    EmojiRunNode, KidsBlocklistPickerItemNode, KidsBlocklistPickerNode, RendererContextNode,
    ShareEntityServiceEndpointNode, SignalServiceEndpointNode, UnsubscribeEndpointNode,
    WatchNextEndpointNode,

    BrowserMediaSessionNode,
    ChannelVideoPlayerNode,
    ChildVideoNode,
    EndScreenVideoNode,
    ExpandableVideoDescriptionBodyNode,
    PlayerAnnotationsExpandedNode,
    PlayerCaptchaViewNode,
    PlayerControlsOverlayNode,
    PlayerLegacyDesktopYpcOfferNode,
    PlayerMicroformatNode,
    PlayerOverflowNode,
    PlayerOverlayAutoplayNode,
    PlayerOverlayVideoDetailsNode,
    SlimVideoMetadataNode,
    VideoAttributeViewNode,
    VideoCardNode,
    VideoDescriptionHeaderNode,
    VideoInfoCardContentNode,
    VideoSummaryContentViewNode,
    VideoSummaryParagraphViewNode,
    WatchCardCompactVideoNode,
    WatchCardHeroVideoNode,
    FormatNode,
    VideoDetailsNode,
    LiveChatAuthorBadgeNode,
    LiveChatHeaderNode,
    LiveChatMessageInputNode,
    LiveChatParticipantNode,
    LiveChatBannerChatSummaryNode,
    LiveChatBannerHeaderNode,
    LiveChatBannerRedirectNode,
    LiveChatItemBumperViewNode,
    LiveChatPaidMessageNode,
    LiveChatPlaceholderItemNode,
    LiveChatProductItemNode,
    LiveChatRestrictedParticipationNode,
    LiveChatSponsorshipsGiftPurchaseAnnouncementNode,
    LiveChatSponsorshipsGiftRedemptionAnnouncementNode,
    LiveChatSponsorshipsHeaderNode,
    LiveChatTextMessageNode,
    LiveChatTickerPaidMessageItemNode,
    LiveChatTickerPaidStickerItemNode,
    LiveChatTickerSponsorItemNode,
    ShowLiveChatActionPanelActionNode,
    ShowLiveChatDialogActionNode,
    ShowLiveChatTooltipCommandNode,
    MarkChatItemsByAuthorAsDeletedActionNode,
    LiveChatBannerPollNode,
    AboutChannelNode,
    AboutChannelViewNode,
    AccountChannelNode,
    ChannelNode,
    ChannelAgeGateNode,
    ChannelExternalLinkViewNode,
    ChannelFeaturedContentNode,
    ChannelOptionsNode,
    ChannelTaglineNode,
    ChannelThumbnailWithLinkNode,
    TopicChannelDetailsNode,
    ActiveAccountHeaderNode,
    ChannelHeaderLinksNode,
    ChannelHeaderLinksViewNode,
    ChannelMobileHeaderNode,
    ChannelSwitcherHeaderNode,
    AuthorCommentBadgeNode,
    CommentRepliesNode,
    CommentViewNode,
    CommentsEntryPointTeaserNode,
    CommentsSimpleboxNode,
    PdgCommentChipNode,
    SponsorCommentBadgeNode,
    CommentsContinuationNode,
    MusicDownloadStateBadgeNode,
    MusicElementHeaderNode,
    MusicSortFilterButtonNode,
    MusicThumbnailNode,
    MusicMenuItemDividerNode,
    MusicMultiSelectMenuNode,
    MusicMultiSelectMenuItemNode,
    BackstagePostNode,
    BackstagePostThreadNode,
    SharedPostNode,
    ReelItemNode,
    ReelPlayerHeaderNode,
    ReelPlayerOverlayNode,
    ShortsLockupViewNode,
    AlertWithButtonNode,
    CompositeVideoPrimaryInfoNode,
    EmergencyOneboxNode,
    SingleActionEmergencySupportNode,
    PlayerLiveStoryboardSpecNode,
    PollHeaderNode,
    ChangeEngagementPanelVisibilityActionNode,
    ShowEngagementPanelEndpointNode,
    CreatorHeartViewNode,
    KidsCategoryTabNode,
    AutomixPreviewVideoNode,
    VideoViewCountNode,
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
    SectionListContinuation(SectionListContinuationNode),
    ItemSectionContinuation(ItemSectionContinuationNode),
    GridContinuation(GridContinuationNode),
    MusicShelfContinuation(MusicShelfContinuationNode),
    MusicPlaylistShelfContinuation(MusicPlaylistShelfContinuationNode),
    PlaylistPanelContinuation(PlaylistPanelContinuationNode),
    ReloadContinuationItemsCommand(ReloadContinuationItemsCommandNode),
    LiveChatContinuation(LiveChatContinuationNode),
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
    // Batch 11: Guide & Sections
    GuideEntry(GuideEntryNode),
    GuideSection(GuideSectionNode),
    GuideSubscriptionsSection(GuideSubscriptionsSectionNode),
    HashtagHeader(HashtagHeaderNode),
    HashtagTile(HashtagTileNode),
    HeatMarker(HeatMarkerNode),
    HeroPlaylistThumbnail(HeroPlaylistThumbnailNode),
    HighlightsCarousel(HighlightsCarouselNode),
    HorizontalList(HorizontalListNode),
    HorizontalMovieList(HorizontalMovieListNode),
    HowThisWasMadeSectionView(HowThisWasMadeSectionViewNode),
    HypeFanCreditsSectionView(HypeFanCreditsSectionViewNode),
    // Batch 11: Lists & Headers
    HypePointsFactoid(HypePointsFactoidNode),
    IconLink(IconLinkNode),
    ImageBannerView(ImageBannerViewNode),
    IncludingResultsFor(IncludingResultsForNode),
    InfoPanelContainer(InfoPanelContainerNode),
    InfoPanelContent(InfoPanelContentNode),
    InteractiveTabbedHeader(InteractiveTabbedHeaderNode),
    ItemSectionHeader(ItemSectionHeaderNode),
    ItemSectionTab(ItemSectionTabNode),
    ItemSectionTabbedHeader(ItemSectionTabbedHeaderNode),
    LikeButton(LikeButtonNode),
    LikeButtonView(LikeButtonViewNode),
    // Batch 11: Panels & Lockups
    ListItemView(ListItemViewNode),
    ListView(ListViewNode),
    LiveChatDialog(LiveChatDialogNode),
    LockupMetadataView(LockupMetadataViewNode),
    LockupView(LockupViewNode),
    MacroMarkersInfoItem(MacroMarkersInfoItemNode),
    MacroMarkersListEntity(MacroMarkersListEntityNode),
    MenuTitle(MenuTitleNode),
    MerchandiseItem(MerchandiseItemNode),
    MerchandiseShelf(MerchandiseShelfNode),
    Message(MessageNode),
    MetadataRow(MetadataRowNode),
    // Batch 11: Media & Music Headers
    MetadataRowContainer(MetadataRowContainerNode),
    MetadataRowHeader(MetadataRowHeaderNode),
    MetadataScreen(MetadataScreenNode),
    Mix(MixNode),
    Movie(MovieNode),
    MovingThumbnail(MovingThumbnailNode),
    MultiMarkersPlayerBar(MultiMarkersPlayerBarNode),
    MusicCardShelfHeaderBasic(MusicCardShelfHeaderBasicNode),
    MusicCarouselShelfBasicHeader(MusicCarouselShelfBasicHeaderNode),
    MusicLargeCardItemCarousel(MusicLargeCardItemCarouselNode),
    MusicMultiRowListItem(MusicMultiRowListItemNode),
    MusicPlaylistEditHeader(MusicPlaylistEditHeaderNode),
    // Batch 12: Music & Page Headers
    MusicResponsiveListItemFixedColumn(MusicResponsiveListItemFixedColumnNode),
    MusicResponsiveListItemFlexColumn(MusicResponsiveListItemFlexColumnNode),
    MusicTastebuilderShelf(MusicTastebuilderShelfNode),
    MusicTastebuilderShelfThumbnail(MusicTastebuilderShelfThumbnailNode),
    NotificationAction(NotificationActionNode),
    OpenOnePickAddVideoModalCommand(OpenOnePickAddVideoModalCommandNode),
    PageHeader(PageHeaderNode),
    PageHeaderView(PageHeaderViewNode),
    PageIndicatorView(PageIndicatorViewNode),
    PageIntroduction(PageIntroductionNode),
    PanelFooterView(PanelFooterViewNode),
    PivotButton(PivotButtonNode),
    // Batch 12: Playlists & Products
    PlaylistAddToOption(PlaylistAddToOptionNode),
    PlaylistCollaborationView(PlaylistCollaborationViewNode),
    PlaylistCustomThumbnail(PlaylistCustomThumbnailNode),
    PlaylistHeader(PlaylistHeaderNode),
    PlaylistInfoCardContent(PlaylistInfoCardContentNode),
    PlaylistPanelVideoWrapper(PlaylistPanelVideoWrapperNode),
    PlaylistSidebar(PlaylistSidebarNode),
    PlaylistThumbnailOverlay(PlaylistThumbnailOverlayNode),
    PlaylistVideoList(PlaylistVideoListNode),
    PlaylistVideoThumbnail(PlaylistVideoThumbnailNode),
    PremiereTrailerBadge(PremiereTrailerBadgeNode),
    ProductList(ProductListNode),
    // Batch 12: Products & Metadata
    ProductListHeader(ProductListHeaderNode),
    ProductListItem(ProductListItemNode),
    ProfileColumnStats(ProfileColumnStatsNode),
    ProfileColumnStatsEntry(ProfileColumnStatsEntryNode),
    Quiz(QuizNode),
    RecognitionShelf(RecognitionShelfNode),
    RelatedChipCloud(RelatedChipCloudNode),
    RichListHeader(RichListHeaderNode),
    RichMetadata(RichMetadataNode),
    RichMetadataRow(RichMetadataRowNode),
    SearchBox(SearchBoxNode),
    SearchFilterOptionsDialog(SearchFilterOptionsDialogNode),
    // Batch 12: Search & Settings
    SearchHeader(SearchHeaderNode),
    SearchSuggestion(SearchSuggestionNode),
    SearchSuggestionsSection(SearchSuggestionsSectionNode),
    SecondarySearchContainer(SecondarySearchContainerNode),
    SectionHeaderView(SectionHeaderViewNode),
    SegmentedLikeDislikeButton(SegmentedLikeDislikeButtonNode),
    SegmentedLikeDislikeButtonView(SegmentedLikeDislikeButtonViewNode),
    SettingBoolean(SettingBooleanNode),
    SettingsCheckbox(SettingsCheckboxNode),
    SettingsOptions(SettingsOptionsNode),
    SettingsSidebar(SettingsSidebarNode),
    SettingsSwitch(SettingsSwitchNode),
    // Batch 13: Shares & Columns
    SharePanelHeader(SharePanelHeaderNode),
    SharePanelTitleV15(SharePanelTitleV15Node),
    ShareTarget(ShareTargetNode),
    SheetView(SheetViewNode),
    ShowCustomThumbnail(ShowCustomThumbnailNode),
    SimpleCardContent(SimpleCardContentNode),
    SimpleCardTeaser(SimpleCardTeaserNode),
    SimpleTextSection(SimpleTextSectionNode),
    SingleColumnBrowseResults(SingleColumnBrowseResultsNode),
    SingleColumnMusicWatchNextResults(SingleColumnMusicWatchNextResultsNode),
    SingleHeroImage(SingleHeroImageNode),
    SlimOwner(SlimOwnerNode),
    // Batch 13: Filters & Subscriptions
    SortFilterHeader(SortFilterHeaderNode),
    SortFilterSubMenu(SortFilterSubMenuNode),
    StartAt(StartAtNode),
    StructuredDescriptionContent(StructuredDescriptionContentNode),
    StructuredDescriptionPlaylistLockup(StructuredDescriptionPlaylistLockupNode),
    SubFeedOption(SubFeedOptionNode),
    SubFeedSelector(SubFeedSelectorNode),
    SubscribeButton(SubscribeButtonNode),
    SubscribeButtonView(SubscribeButtonViewNode),
    Tabbed(TabbedNode),
    TabbedSearchResults(TabbedSearchResultsNode),
    TextCarouselItemView(TextCarouselItemViewNode),
    // Batch 13: Thumbnail Overlays & Badges
    TextFieldView(TextFieldViewNode),
    ThirdPartyShareTargetSection(ThirdPartyShareTargetSectionNode),
    ThumbnailBadgeView(ThumbnailBadgeViewNode),
    ThumbnailBottomOverlayView(ThumbnailBottomOverlayViewNode),
    ThumbnailHoverOverlayToggleActionsView(ThumbnailHoverOverlayToggleActionsViewNode),
    ThumbnailHoverOverlayView(ThumbnailHoverOverlayViewNode),
    ThumbnailLandscapePortrait(ThumbnailLandscapePortraitNode),
    ThumbnailOverlayAvatarStackView(ThumbnailOverlayAvatarStackViewNode),
    ThumbnailOverlayBadgeView(ThumbnailOverlayBadgeViewNode),
    ThumbnailOverlayPinking(ThumbnailOverlayPinkingNode),
    ThumbnailOverlayPlaybackStatus(ThumbnailOverlayPlaybackStatusNode),
    ThumbnailOverlayProgressBarView(ThumbnailOverlayProgressBarViewNode),
    // Batch 13: Tickets & Transcripts
    ThumbnailOverlayResumePlayback(ThumbnailOverlayResumePlaybackNode),
    ThumbnailOverlayTitleView(ThumbnailOverlayTitleViewNode),
    ThumbnailView(ThumbnailViewNode),
    TicketEvent(TicketEventNode),
    TicketShelf(TicketShelfNode),
    TitleAndButtonListHeader(TitleAndButtonListHeaderNode),
    ToggleButtonView(ToggleButtonViewNode),
    ToggleFormField(ToggleFormFieldNode),
    ToggleMenuServiceItem(ToggleMenuServiceItemNode),
    Tooltip(TooltipNode),
    Transcript(TranscriptNode),
    TranscriptFooter(TranscriptFooterNode),
    // Batch 14: Transcripts & Watch Columns
    TranscriptSearchBox(TranscriptSearchBoxNode),
    TranscriptSearchPanel(TranscriptSearchPanelNode),
    TranscriptSectionHeader(TranscriptSectionHeaderNode),
    TranscriptSegment(TranscriptSegmentNode),
    TranscriptSegmentList(TranscriptSegmentListNode),
    TwoColumnBrowseResults(TwoColumnBrowseResultsNode),
    TwoColumnSearchResults(TwoColumnSearchResultsNode),
    TwoColumnWatchNextResults(TwoColumnWatchNextResultsNode),
    UnifiedSharePanel(UnifiedSharePanelNode),
    UniversalWatchCard(UniversalWatchCardNode),
    UploadTimeFactoid(UploadTimeFactoidNode),
    UpsellDialog(UpsellDialogNode),
    // Batch 14: Video Sections & Watch Cards
    VerticalWatchCardList(VerticalWatchCardListNode),
    VideoAttributesSectionView(VideoAttributesSectionViewNode),
    VideoDescriptionCourseSection(VideoDescriptionCourseSectionNode),
    VideoDescriptionInfocardsSection(VideoDescriptionInfocardsSectionNode),
    VideoDescriptionMusicSection(VideoDescriptionMusicSectionNode),
    VideoDescriptionTranscriptSection(VideoDescriptionTranscriptSectionNode),
    VideoDescriptionYouchatSectionView(VideoDescriptionYouchatSectionViewNode),
    VideoMetadataCarouselView(VideoMetadataCarouselViewNode),
    ViewCountFactoid(ViewCountFactoidNode),
    WatchCardRichHeader(WatchCardRichHeaderNode),
    WatchCardSectionSequence(WatchCardSectionSequenceNode),
    WatchNextEndScreen(WatchNextEndScreenNode),
    // Batch 14: Comments & Endpoints Extended
    WatchNextTabbedResults(WatchNextTabbedResultsNode),
    YpcTrailer(YpcTrailerNode),
    CommandExecutorCommand(CommandExecutorCommandNode),
    GetKidsBlocklistPickerCommand(GetKidsBlocklistPickerCommandNode),
    ShowDialogCommand(ShowDialogCommandNode),
    CommentDialog(CommentDialogNode),
    CommentReplyDialog(CommentReplyDialogNode),
    EmojiPicker(EmojiPickerNode),
    VoiceReplyContainerView(VoiceReplyContainerViewNode),
    AddToPlaylistEndpoint(AddToPlaylistEndpointNode),
    AddToPlaylistServiceEndpoint(AddToPlaylistServiceEndpointNode),
    CreateCommentEndpoint(CreateCommentEndpointNode),
    // Batch 14: Endpoint Commands
    CreatePlaylistServiceEndpoint(CreatePlaylistServiceEndpointNode),
    DeletePlaylistEndpoint(DeletePlaylistEndpointNode),
    FeedbackEndpoint(FeedbackEndpointNode),
    GetAccountsListInnertubeEndpoint(GetAccountsListInnertubeEndpointNode),
    HideEngagementPanelEndpoint(HideEngagementPanelEndpointNode),
    LiveChatItemContextMenuEndpoint(LiveChatItemContextMenuEndpointNode),
    ModifyChannelNotificationPreferenceEndpoint(ModifyChannelNotificationPreferenceEndpointNode),
    PerformCommentActionEndpoint(PerformCommentActionEndpointNode),
    PlaylistEditEndpoint(PlaylistEditEndpointNode),
    PrefetchWatchCommand(PrefetchWatchCommandNode),
    ShareEndpoint(ShareEndpointNode),
    ShareEntityEndpoint(ShareEntityEndpointNode),
    // Batch 15: Menus & Mobile Topbar
    MenuFlexibleItem(MenuFlexibleItemNode),
    MenuNavigationItem(MenuNavigationItemNode),
    MenuPopup(MenuPopupNode),
    MenuServiceItem(MenuServiceItemNode),
    MenuServiceItemDownload(MenuServiceItemDownloadNode),
    MultiPageMenu(MultiPageMenuNode),
    MultiPageMenuNotificationSection(MultiPageMenuNotificationSectionNode),
    SimpleMenuHeader(SimpleMenuHeaderNode),
    MobileTopbar(MobileTopbarNode),
    MultiPageMenuSection(MultiPageMenuSectionNode),
    PivotBar(PivotBarNode),
    PivotBarItem(PivotBarItemNode),
    TopbarMenuButton(TopbarMenuButtonNode),
    // Batch 15: Livechat Actions & Collaboration
    ReplaceLiveChatAction(ReplaceLiveChatActionNode),
    UpdateDateTextAction(UpdateDateTextActionNode),
    UpdateDescriptionAction(UpdateDescriptionActionNode),
    UpdateTitleAction(UpdateTitleActionNode),
    UpdateToggleButtonTextAction(UpdateToggleButtonTextActionNode),
    UpdateViewershipAction(UpdateViewershipActionNode),
    BumperUserEduContentView(BumperUserEduContentViewNode),
    PdgReplyButtonView(PdgReplyButtonViewNode),
    PlaylistCollaborationFormSchema(PlaylistCollaborationFormSchemaNode),
    PlaylistCollaborationViewModelPlaylistCollaboratorData(PlaylistCollaborationViewModelPlaylistCollaboratorDataNode),
    SubscriptionButton(SubscriptionButtonNode),
    CommandContext(CommandContextNode),
    // Batch 15: Endpoints Primitives & Kids
    ShareEntityServiceEndpoint(ShareEntityServiceEndpointNode),
    SignalServiceEndpoint(SignalServiceEndpointNode),
    UnsubscribeEndpoint(UnsubscribeEndpointNode),
    WatchNextEndpoint(WatchNextEndpointNode),
    AccessibilityContext(AccessibilityContextNode),
    AccessibilityData(AccessibilityDataNode),
    ChildElement(ChildElementNode),
    EmojiRun(EmojiRunNode),
    RendererContext(RendererContextNode),
    AnchoredSection(AnchoredSectionNode),
    KidsBlocklistPicker(KidsBlocklistPickerNode),
    KidsBlocklistPickerItem(KidsBlocklistPickerItemNode),
    // Batch 15: Direct Primitives & Core Variants
    MusicResponsiveListItem(MusicResponsiveListItemNode),
    NavigationEndpoint(NavigationEndpointNode),
    ThumbnailOverlayTimeStatus(ThumbnailOverlayTimeStatusNode),
    BrowseEndpoint(BrowseEndpointNode),
    LikeEndpoint(LikeEndpointNode),
    ReelWatchEndpoint(ReelWatchEndpointNode),
    SearchEndpoint(SearchEndpointNode),
    SubscribeEndpoint(SubscribeEndpointNode),
    WatchEndpoint(WatchEndpointNode),
    Author(AuthorNode),
    Text(TextNode),
    TextRun(TextRunNode),
    Thumbnail(ThumbnailNode),
    AutomixPreviewVideo(AutomixPreviewVideoNode),
    VideoViewCount(VideoViewCountNode),
    // Phase 16: Full 574 Unique AST Nodes
    BrowserMediaSession(BrowserMediaSessionNode),
    ChannelVideoPlayer(ChannelVideoPlayerNode),
    ChildVideo(ChildVideoNode),
    EndScreenVideo(EndScreenVideoNode),
    ExpandableVideoDescriptionBody(ExpandableVideoDescriptionBodyNode),
    PlayerAnnotationsExpanded(PlayerAnnotationsExpandedNode),
    PlayerCaptchaView(PlayerCaptchaViewNode),
    PlayerControlsOverlay(PlayerControlsOverlayNode),
    PlayerLegacyDesktopYpcOffer(PlayerLegacyDesktopYpcOfferNode),
    PlayerMicroformat(PlayerMicroformatNode),
    PlayerOverflow(PlayerOverflowNode),
    PlayerOverlayAutoplay(PlayerOverlayAutoplayNode),
    PlayerOverlayVideoDetails(PlayerOverlayVideoDetailsNode),
    SlimVideoMetadata(SlimVideoMetadataNode),
    VideoAttributeView(VideoAttributeViewNode),
    VideoCard(VideoCardNode),
    VideoDescriptionHeader(VideoDescriptionHeaderNode),
    VideoInfoCardContent(VideoInfoCardContentNode),
    VideoSummaryContentView(VideoSummaryContentViewNode),
    VideoSummaryParagraphView(VideoSummaryParagraphViewNode),
    WatchCardCompactVideo(WatchCardCompactVideoNode),
    WatchCardHeroVideo(WatchCardHeroVideoNode),
    Format(FormatNode),
    VideoDetails(VideoDetailsNode),
    LiveChatAuthorBadge(LiveChatAuthorBadgeNode),
    LiveChatHeader(LiveChatHeaderNode),
    LiveChatMessageInput(LiveChatMessageInputNode),
    LiveChatParticipant(LiveChatParticipantNode),
    LiveChatBannerChatSummary(LiveChatBannerChatSummaryNode),
    LiveChatBannerHeader(LiveChatBannerHeaderNode),
    LiveChatBannerRedirect(LiveChatBannerRedirectNode),
    LiveChatItemBumperView(LiveChatItemBumperViewNode),
    LiveChatPaidMessage(LiveChatPaidMessageNode),
    LiveChatPlaceholderItem(LiveChatPlaceholderItemNode),
    LiveChatProductItem(LiveChatProductItemNode),
    LiveChatRestrictedParticipation(LiveChatRestrictedParticipationNode),
    LiveChatSponsorshipsGiftPurchaseAnnouncement(LiveChatSponsorshipsGiftPurchaseAnnouncementNode),
    LiveChatSponsorshipsGiftRedemptionAnnouncement(LiveChatSponsorshipsGiftRedemptionAnnouncementNode),
    LiveChatSponsorshipsHeader(LiveChatSponsorshipsHeaderNode),
    LiveChatTextMessage(LiveChatTextMessageNode),
    LiveChatTickerPaidMessageItem(LiveChatTickerPaidMessageItemNode),
    LiveChatTickerPaidStickerItem(LiveChatTickerPaidStickerItemNode),
    LiveChatTickerSponsorItem(LiveChatTickerSponsorItemNode),
    ShowLiveChatActionPanelAction(ShowLiveChatActionPanelActionNode),
    ShowLiveChatDialogAction(ShowLiveChatDialogActionNode),
    ShowLiveChatTooltipCommand(ShowLiveChatTooltipCommandNode),
    MarkChatItemsByAuthorAsDeletedAction(MarkChatItemsByAuthorAsDeletedActionNode),
    LiveChatBannerPoll(LiveChatBannerPollNode),
    AboutChannel(AboutChannelNode),
    AboutChannelView(AboutChannelViewNode),
    AccountChannel(AccountChannelNode),
    Channel(ChannelNode),
    ChannelAgeGate(ChannelAgeGateNode),
    ChannelExternalLinkView(ChannelExternalLinkViewNode),
    ChannelFeaturedContent(ChannelFeaturedContentNode),
    ChannelOptions(ChannelOptionsNode),
    ChannelTagline(ChannelTaglineNode),
    ChannelThumbnailWithLink(ChannelThumbnailWithLinkNode),
    TopicChannelDetails(TopicChannelDetailsNode),
    ActiveAccountHeader(ActiveAccountHeaderNode),
    ChannelHeaderLinks(ChannelHeaderLinksNode),
    ChannelHeaderLinksView(ChannelHeaderLinksViewNode),
    ChannelMobileHeader(ChannelMobileHeaderNode),
    ChannelSwitcherHeader(ChannelSwitcherHeaderNode),
    AuthorCommentBadge(AuthorCommentBadgeNode),
    CommentReplies(CommentRepliesNode),
    CommentView(CommentViewNode),
    CommentsEntryPointTeaser(CommentsEntryPointTeaserNode),
    CommentsSimplebox(CommentsSimpleboxNode),
    PdgCommentChip(PdgCommentChipNode),
    SponsorCommentBadge(SponsorCommentBadgeNode),
    CommentsContinuation(CommentsContinuationNode),
    MusicDownloadStateBadge(MusicDownloadStateBadgeNode),
    MusicElementHeader(MusicElementHeaderNode),
    MusicSortFilterButton(MusicSortFilterButtonNode),
    MusicThumbnail(MusicThumbnailNode),
    MusicMenuItemDivider(MusicMenuItemDividerNode),
    MusicMultiSelectMenu(MusicMultiSelectMenuNode),
    MusicMultiSelectMenuItem(MusicMultiSelectMenuItemNode),
    BackstagePost(BackstagePostNode),
    BackstagePostThread(BackstagePostThreadNode),
    SharedPost(SharedPostNode),
    ReelItem(ReelItemNode),
    ReelPlayerHeader(ReelPlayerHeaderNode),
    ReelPlayerOverlay(ReelPlayerOverlayNode),
    ShortsLockupView(ShortsLockupViewNode),
    AlertWithButton(AlertWithButtonNode),
    CompositeVideoPrimaryInfo(CompositeVideoPrimaryInfoNode),
    EmergencyOnebox(EmergencyOneboxNode),
    SingleActionEmergencySupport(SingleActionEmergencySupportNode),
    PlayerLiveStoryboardSpec(PlayerLiveStoryboardSpecNode),
    PollHeader(PollHeaderNode),
    ChangeEngagementPanelVisibilityAction(ChangeEngagementPanelVisibilityActionNode),
    ShowEngagementPanelEndpoint(ShowEngagementPanelEndpointNode),
    CreatorHeartView(CreatorHeartViewNode),
    KidsCategoryTab(KidsCategoryTabNode),

}

impl YTNode {

    /// Return the corresponding `YTNodeVariant` for this `YTNode` instance.
    pub fn variant(&self) -> YTNodeVariant {
        match self {
            YTNode::Video(_) => YTNodeVariant::Video,
            YTNode::VideoPrimaryInfo(_) => YTNodeVariant::VideoPrimaryInfo,
            YTNode::VideoSecondaryInfo(_) => YTNodeVariant::VideoSecondaryInfo,
            YTNode::Short(_) => YTNodeVariant::ReelItem,
            YTNode::ReelShelf(_) => YTNodeVariant::ReelShelf,
            YTNode::Playlist(_) => YTNodeVariant::Playlist,
            YTNode::PlaylistVideo(_) => YTNodeVariant::PlaylistVideo,
            YTNode::PlaylistPanel(_) => YTNodeVariant::PlaylistPanel,
            YTNode::PlaylistPanelVideo(_) => YTNodeVariant::PlaylistPanelVideo,
            YTNode::PlaylistMetadata(_) => YTNodeVariant::PlaylistMetadata,
            YTNode::PlaylistSidebarPrimaryInfo(_) => YTNodeVariant::PlaylistSidebarPrimaryInfo,
            YTNode::PlaylistSidebarSecondaryInfo(_) => YTNodeVariant::PlaylistSidebarSecondaryInfo,
            YTNode::ChannelHeader(_) => YTNodeVariant::C4TabbedHeader,
            YTNode::ChannelCard(_) => YTNodeVariant::Channel,
            YTNode::ChannelAboutFullMetadata(_) => YTNodeVariant::ChannelAboutFullMetadata,
            YTNode::ChannelMetadata(_) => YTNodeVariant::ChannelMetadata,
            YTNode::ChannelSubMenu(_) => YTNodeVariant::ChannelSubMenu,
            YTNode::MusicItem(_) => YTNodeVariant::MusicResponsiveListItem,
            YTNode::MusicCard(_) => YTNodeVariant::MusicTwoRowItem,
            YTNode::MusicDescriptionShelf(_) => YTNodeVariant::MusicDescriptionShelf,
            YTNode::MusicHeader(_) => YTNodeVariant::MusicHeader,
            YTNode::MusicInlineBadge(_) => YTNodeVariant::MusicInlineBadge,
            YTNode::MusicNavigationButton(_) => YTNodeVariant::MusicNavigationButton,
            YTNode::MusicQueue(_) => YTNodeVariant::MusicQueue,
            YTNode::MusicPlayButton(_) => YTNodeVariant::MusicPlayButton,
            YTNode::Comment(_) => YTNodeVariant::CommentView,
            YTNode::CommentThread(_) => YTNodeVariant::CommentThread,
            YTNode::CreatorHeart(_) => YTNodeVariant::CreatorHeart,
            YTNode::Post(_) => YTNodeVariant::Post,
            YTNode::BackstageImage(_) => YTNodeVariant::BackstageImage,
            YTNode::PostMultiImage(_) => YTNodeVariant::PostMultiImage,
            YTNode::Continuation(_) => YTNodeVariant::ContinuationItem,
            YTNode::SectionListContinuation(_) => YTNodeVariant::SectionListContinuation,
            YTNode::ItemSectionContinuation(_) => YTNodeVariant::ItemSectionContinuation,
            YTNode::GridContinuation(_) => YTNodeVariant::GridContinuation,
            YTNode::MusicShelfContinuation(_) => YTNodeVariant::MusicShelfContinuation,
            YTNode::MusicPlaylistShelfContinuation(_) => YTNodeVariant::MusicPlaylistShelfContinuation,
            YTNode::PlaylistPanelContinuation(_) => YTNodeVariant::PlaylistPanelContinuation,
            YTNode::ReloadContinuationItemsCommand(_) => YTNodeVariant::ReloadContinuationItemsCommand,
            YTNode::LiveChatContinuation(_) => YTNodeVariant::LiveChatContinuation,
            YTNode::SectionList(_) => YTNodeVariant::SectionList,
            YTNode::ItemSection(_) => YTNodeVariant::ItemSection,
            YTNode::RichGrid(_) => YTNodeVariant::RichGrid,
            YTNode::Shelf(_) => YTNodeVariant::Shelf,
            YTNode::RichShelf(_) => YTNodeVariant::RichShelf,
            YTNode::Tab(_) => YTNodeVariant::Tab,
            YTNode::ChipCloud(_) => YTNodeVariant::ChipCloud,
            YTNode::ChipCloudChip(_) => YTNodeVariant::ChipCloudChip,
            YTNode::FeedFilterChipBar(_) => YTNodeVariant::FeedFilterChipBar,
            YTNode::LiveChat(_) => YTNodeVariant::LiveChat,
            YTNode::LiveChatPaidSticker(_) => YTNodeVariant::LiveChatPaidSticker,
            YTNode::LiveChatMembershipItem(_) => YTNodeVariant::LiveChatMembershipItem,
            YTNode::LiveChatViewerEngagementMessage(_) => YTNodeVariant::LiveChatViewerEngagementMessage,
            YTNode::LiveChatBanner(_) => YTNodeVariant::LiveChatBanner,
            YTNode::AddChatItemAction(_) => YTNodeVariant::AddChatItemAction,
            YTNode::MarkChatItemAsDeletedAction(_) => YTNodeVariant::MarkChatItemAsDeletedAction,
            YTNode::LiveChatAutoModMessage(_) => YTNodeVariant::LiveChatAutoModMessage,
            YTNode::LiveChatModeChangeMessage(_) => YTNodeVariant::LiveChatModeChangeMessage,
            YTNode::ShowEngagementPanelAction(_) => YTNodeVariant::ShowEngagementPanelEndpoint,
            YTNode::UpdateEngagementPanelAction(_) => YTNodeVariant::UpdateEngagementPanelAction,
            YTNode::NavigateAction(_) => YTNodeVariant::NavigationEndpoint,
            YTNode::ShowLiveChatAction(_) => YTNodeVariant::ShowLiveChatDialogAction,
            YTNode::Button(_) => YTNodeVariant::Button,
            YTNode::ToggleButton(_) => YTNodeVariant::ToggleButton,
            YTNode::Menu(_) => YTNodeVariant::Menu,
            YTNode::DidYouMean(_) => YTNodeVariant::DidYouMean,
            YTNode::ShowingResultsFor(_) => YTNodeVariant::ShowingResultsFor,
            YTNode::SearchSubMenu(_) => YTNodeVariant::SearchSubMenu,
            YTNode::SearchFilterGroup(_) => YTNodeVariant::SearchFilterGroup,
            YTNode::SearchFilter(_) => YTNodeVariant::SearchFilter,
            YTNode::Endscreen(_) => YTNodeVariant::Endscreen,
            YTNode::EndscreenElement(_) => YTNodeVariant::EndscreenElement,
            YTNode::MetadataBadge(_) => YTNodeVariant::MetadataBadge,
            YTNode::ViewCount(_) => YTNodeVariant::VideoViewCount,
            YTNode::VideoOwner(_) => YTNodeVariant::VideoOwner,
            YTNode::MicroformatData(_) => YTNodeVariant::MicroformatData,
            YTNode::Alert(_) => YTNodeVariant::Alert,
            YTNode::Card(_) => YTNodeVariant::Card,
            YTNode::Clarification(_) => YTNodeVariant::SingleActionEmergencySupport,
            YTNode::Poll(_) => YTNodeVariant::Poll,
            YTNode::PlayerOverlay(_) => YTNodeVariant::PlayerOverlay,
            YTNode::PlayerStoryboardSpec(_) => YTNodeVariant::PlayerStoryboardSpec,
            YTNode::TimedMarkerDecoration(_) => YTNodeVariant::TimedMarkerDecoration,
            YTNode::PlayerCaptionsTracklist(_) => YTNodeVariant::PlayerCaptionsTracklist,
            YTNode::PlayerErrorMessage(_) => YTNodeVariant::PlayerErrorMessage,
            YTNode::PlayerLegacyDesktopYpcTrailer(_) => YTNodeVariant::PlayerLegacyDesktopYpcTrailer,
            YTNode::ProfileColumn(_) => YTNodeVariant::ProfileColumn,
            YTNode::ProfileColumnUserInfo(_) => YTNodeVariant::ProfileColumnUserInfo,
            YTNode::VerticalList(_) => YTNodeVariant::VerticalList,
            YTNode::Chapter(_) => YTNodeVariant::Chapter,
            YTNode::Heatmap(_) => YTNodeVariant::Heatmap,
            YTNode::MacroMarkersList(_) => YTNodeVariant::MacroMarkersList,
            YTNode::MacroMarkersListItem(_) => YTNodeVariant::MacroMarkersListItem,
            YTNode::SearchRefinementCard(_) => YTNodeVariant::SearchRefinementCard,
            YTNode::HorizontalCardList(_) => YTNodeVariant::HorizontalCardList,
            YTNode::ExpandableTab(_) => YTNodeVariant::ExpandableTab,
            YTNode::Notification(_) => YTNodeVariant::Notification,
            YTNode::HistorySuggestion(_) => YTNodeVariant::HistorySuggestion,
            YTNode::AccountSectionList(_) => YTNodeVariant::AccountSectionList,
            YTNode::AccountItem(_) => YTNodeVariant::AccountItem,
            YTNode::AccountItemSection(_) => YTNodeVariant::AccountItemSection,
            YTNode::AccountItemSectionHeader(_) => YTNodeVariant::AccountItemSectionHeader,
            YTNode::KidsCategoriesHeader(_) => YTNodeVariant::KidsCategoriesHeader,
            YTNode::KidsHomeScreen(_) => YTNodeVariant::KidsHomeScreen,
            YTNode::ClipCreation(_) => YTNodeVariant::ClipCreation,
            YTNode::ClipCreationScrubber(_) => YTNodeVariant::ClipCreationScrubber,
            YTNode::BadgeView(_) => YTNodeVariant::BadgeView,
            YTNode::CallToActionButton(_) => YTNodeVariant::CallToActionButton,
            YTNode::ButtonCardView(_) => YTNodeVariant::ButtonCardView,
            YTNode::AvatarView(_) => YTNodeVariant::AvatarView,
            YTNode::CompactLink(_) => YTNodeVariant::CompactLink,
            YTNode::GridVideo(_) => YTNodeVariant::GridVideo,
            YTNode::GridChannel(_) => YTNodeVariant::GridChannel,
            YTNode::GridPlaylist(_) => YTNodeVariant::GridPlaylist,
            YTNode::GridMix(_) => YTNodeVariant::GridMix,
            YTNode::GridMovie(_) => YTNodeVariant::GridMovie,
            YTNode::GridShow(_) => YTNodeVariant::GridShow,
            YTNode::CompactVideo(_) => YTNodeVariant::CompactVideo,
            YTNode::CompactChannel(_) => YTNodeVariant::CompactChannel,
            YTNode::CompactPlaylist(_) => YTNodeVariant::CompactPlaylist,
            YTNode::CompactMix(_) => YTNodeVariant::CompactMix,
            YTNode::RichItem(_) => YTNodeVariant::RichItem,
            YTNode::RichSection(_) => YTNodeVariant::RichSection,
            YTNode::MusicCarouselShelf(_) => YTNodeVariant::MusicCarouselShelf,
            YTNode::MusicShelf(_) => YTNodeVariant::MusicShelf,
            YTNode::MusicSideAlignedItem(_) => YTNodeVariant::MusicSideAlignedItem,
            YTNode::MusicVisualHeader(_) => YTNodeVariant::MusicVisualHeader,
            YTNode::MusicItemThumbnailOverlay(_) => YTNodeVariant::MusicItemThumbnailOverlay,
            YTNode::MusicPlaylistShelf(_) => YTNodeVariant::MusicPlaylistShelf,
            YTNode::MusicCardShelf(_) => YTNodeVariant::MusicCardShelf,
            YTNode::MusicImmersiveHeader(_) => YTNodeVariant::MusicImmersiveHeader,
            YTNode::MusicDetailHeader(_) => YTNodeVariant::MusicDetailHeader,
            YTNode::MusicEditablePlaylistDetailHeader(_) => YTNodeVariant::MusicEditablePlaylistDetailHeader,
            YTNode::MusicResponsiveHeader(_) => YTNodeVariant::MusicResponsiveHeader,
            YTNode::MusicAutoplay(_) => YTNodeVariant::AutomixPreviewVideo,
            YTNode::ThumbnailOverlayHoverText(_) => YTNodeVariant::ThumbnailOverlayHoverText,
            YTNode::ThumbnailOverlayEndorsement(_) => YTNodeVariant::ThumbnailOverlayEndorsement,
            YTNode::ThumbnailOverlayNowPlaying(_) => YTNodeVariant::ThumbnailOverlayNowPlaying,
            YTNode::ThumbnailOverlayLoadingPreview(_) => YTNodeVariant::ThumbnailOverlayLoadingPreview,
            YTNode::ThumbnailOverlayInlineUnplayable(_) => YTNodeVariant::ThumbnailOverlayInlineUnplayable,
            YTNode::ThumbnailOverlayBottomPanel(_) => YTNodeVariant::ThumbnailOverlayBottomPanel,
            YTNode::ThumbnailOverlaySidePanel(_) => YTNodeVariant::ThumbnailOverlaySidePanel,
            YTNode::ThumbnailOverlayToggleButton(_) => YTNodeVariant::ThumbnailOverlayToggleButton,
            YTNode::DecoratedPlayerBar(_) => YTNodeVariant::DecoratedPlayerBar,
            YTNode::ConfirmDialog(_) => YTNodeVariant::ConfirmDialog,
            YTNode::Dialog(_) => YTNodeVariant::ConfirmDialog,
            YTNode::ModalWithTitleAndButton(_) => YTNodeVariant::ModalWithTitleAndButton,
            YTNode::EngagementPanelSectionList(_) => YTNodeVariant::EngagementPanelSectionList,
            YTNode::EngagementPanelTitleHeader(_) => YTNodeVariant::EngagementPanelTitleHeader,
            YTNode::CommentsHeader(_) => YTNodeVariant::CommentsHeader,
            YTNode::CommentsEntryPointHeader(_) => YTNodeVariant::CommentsEntryPointHeader,
            YTNode::CommentActionButtons(_) => YTNodeVariant::CommentActionButtons,
            YTNode::CommentSimplebox(_) => YTNodeVariant::CommentSimplebox,
            YTNode::SubscriptionNotificationToggleButton(_) => YTNodeVariant::SubscriptionNotificationToggleButton,
            YTNode::InfoRow(_) => YTNodeVariant::InfoRow,
            YTNode::CollageHeroImage(_) => YTNodeVariant::CollageHeroImage,
            YTNode::FeedNudge(_) => YTNodeVariant::FeedNudge,
            YTNode::ChannelOwnerEmptyState(_) => YTNodeVariant::ChannelOwnerEmptyState,
            YTNode::TextHeader(_) => YTNodeVariant::TextHeader,
            YTNode::CarouselHeader(_) => YTNodeVariant::CarouselHeader,
            YTNode::CarouselItem(_) => YTNodeVariant::CarouselItem,
            YTNode::CarouselItemView(_) => YTNodeVariant::CarouselItemView,
            YTNode::CarouselLockup(_) => YTNodeVariant::CarouselLockup,
            YTNode::CarouselTitleView(_) => YTNodeVariant::CarouselTitleView,
            YTNode::ChipBarView(_) => YTNodeVariant::ChipBarView,
            YTNode::ChipView(_) => YTNodeVariant::ChipView,
            YTNode::ContentListItemView(_) => YTNodeVariant::ContentListItemView,
            YTNode::BackgroundPromo(_) => YTNodeVariant::BackgroundPromo,
            YTNode::AttributionView(_) => YTNodeVariant::AttributionView,
            YTNode::AvatarStackView(_) => YTNodeVariant::AvatarStackView,
            YTNode::AnimatedThumbnailOverlayView(_) => YTNodeVariant::AnimatedThumbnailOverlayView,
            YTNode::CardCollection(_) => YTNodeVariant::CardCollection,
            YTNode::CollaboratorInfoCardContent(_) => YTNodeVariant::CollaboratorInfoCardContent,
            YTNode::CollectionThumbnailView(_) => YTNodeVariant::CollectionThumbnailView,
            YTNode::ClipAdState(_) => YTNodeVariant::ClipAdState,
            YTNode::ClipCreationTextInput(_) => YTNodeVariant::ClipCreationTextInput,
            YTNode::ClientSideToggleMenuItem(_) => YTNodeVariant::ClientSideToggleMenuItem,
            YTNode::AudioOnlyPlayability(_) => YTNodeVariant::AudioOnlyPlayability,
            YTNode::CompactMovie(_) => YTNodeVariant::CompactMovie,
            YTNode::CompactStation(_) => YTNodeVariant::CompactStation,
            YTNode::AddToPlaylist(_) => YTNodeVariant::AddToPlaylist,
            YTNode::C4TabbedHeader(_) => YTNodeVariant::C4TabbedHeader,
            YTNode::ChannelSwitcherPage(_) => YTNodeVariant::ChannelSwitcherPage,
            YTNode::LiveChatItemList(_) => YTNodeVariant::LiveChatItemList,
            YTNode::LiveChatParticipantsList(_) => YTNodeVariant::LiveChatParticipantsList,
            YTNode::LiveChatActionPanel(_) => YTNodeVariant::LiveChatActionPanel,
            YTNode::AddBannerToLiveChatCommand(_) => YTNodeVariant::AddBannerToLiveChatCommand,
            YTNode::RemoveBannerForLiveChatCommand(_) => YTNodeVariant::RemoveBannerForLiveChatCommand,
            YTNode::AddLiveChatTickerItemAction(_) => YTNodeVariant::AddLiveChatTickerItemAction,
            YTNode::DimChatItemAction(_) => YTNodeVariant::DimChatItemAction,
            YTNode::RemoveChatItemAction(_) => YTNodeVariant::RemoveChatItemAction,
            YTNode::RemoveChatItemByAuthorAction(_) => YTNodeVariant::RemoveChatItemByAuthorAction,
            YTNode::ReplaceChatItemAction(_) => YTNodeVariant::ReplaceChatItemAction,
            YTNode::ReplayChatItemAction(_) => YTNodeVariant::ReplayChatItemAction,
            YTNode::UpdateLiveChatPollAction(_) => YTNodeVariant::UpdateLiveChatPollAction,
            YTNode::AppendContinuationItemsAction(_) => YTNodeVariant::AppendContinuationItemsAction,
            YTNode::GetMultiPageMenuAction(_) => YTNodeVariant::GetMultiPageMenuAction,
            YTNode::OpenPopupAction(_) => YTNodeVariant::OpenPopupAction,
            YTNode::SendFeedbackAction(_) => YTNodeVariant::SendFeedbackAction,
            YTNode::SignalAction(_) => YTNodeVariant::SignalAction,
            YTNode::UpdateChannelSwitcherPageAction(_) => YTNodeVariant::UpdateChannelSwitcherPageAction,
            YTNode::UpdateSubscribeButtonAction(_) => YTNodeVariant::UpdateSubscribeButtonAction,
            YTNode::AddToPlaylistCommand(_) => YTNodeVariant::AddToPlaylistCommand,
            YTNode::ContinuationCommand(_) => YTNodeVariant::ContinuationCommand,
            YTNode::ShowSheetCommand(_) => YTNodeVariant::ShowSheetCommand,
            YTNode::UpdateEngagementPanelContentCommand(_) => YTNodeVariant::UpdateEngagementPanelContentCommand,
            YTNode::RunAttestationCommand(_) => YTNodeVariant::RunAttestationCommand,
            YTNode::BrowseFeedActions(_) => YTNodeVariant::BrowseFeedActions,
            YTNode::ButtonView(_) => YTNodeVariant::ButtonView,
            YTNode::ClipSection(_) => YTNodeVariant::ClipSection,
            YTNode::ContentMetadataView(_) => YTNodeVariant::ContentMetadataView,
            YTNode::ContentPreviewImageView(_) => YTNodeVariant::ContentPreviewImageView,
            YTNode::ContinuationItem(_) => YTNodeVariant::ContinuationItem,
            YTNode::ContinuationItemView(_) => YTNodeVariant::ContinuationItemView,
            YTNode::ConversationBar(_) => YTNodeVariant::ConversationBar,
            YTNode::CopyLink(_) => YTNodeVariant::CopyLink,
            YTNode::CreatePlaylistDialog(_) => YTNodeVariant::CreatePlaylistDialog,
            YTNode::CreatePlaylistDialogFormView(_) => YTNodeVariant::CreatePlaylistDialogFormView,
            YTNode::DecoratedAvatarView(_) => YTNodeVariant::DecoratedAvatarView,
            YTNode::DefaultPromoPanel(_) => YTNodeVariant::DefaultPromoPanel,
            YTNode::DescriptionPreviewView(_) => YTNodeVariant::DescriptionPreviewView,
            YTNode::DialogHeaderView(_) => YTNodeVariant::DialogHeaderView,
            YTNode::DialogView(_) => YTNodeVariant::DialogView,
            YTNode::DislikeButtonView(_) => YTNodeVariant::DislikeButtonView,
            YTNode::DismissableDialog(_) => YTNodeVariant::DismissableDialog,
            YTNode::DismissableDialogContentSection(_) => YTNodeVariant::DismissableDialogContentSection,
            YTNode::DownloadButton(_) => YTNodeVariant::DownloadButton,
            YTNode::DownloadListItemView(_) => YTNodeVariant::DownloadListItemView,
            YTNode::Dropdown(_) => YTNodeVariant::Dropdown,
            YTNode::DropdownItem(_) => YTNodeVariant::DropdownItem,
            YTNode::DropdownView(_) => YTNodeVariant::DropdownView,
            YTNode::DynamicTextView(_) => YTNodeVariant::DynamicTextView,
            YTNode::Element(_) => YTNodeVariant::Element,
            YTNode::EmojiPickerCategory(_) => YTNodeVariant::EmojiPickerCategory,
            YTNode::EmojiPickerCategoryButton(_) => YTNodeVariant::EmojiPickerCategoryButton,
            YTNode::EmojiPickerUpsellCategory(_) => YTNodeVariant::EmojiPickerUpsellCategory,
            YTNode::EndScreenPlaylist(_) => YTNodeVariant::EndScreenPlaylist,
            YTNode::EomSettingsDisclaimer(_) => YTNodeVariant::EomSettingsDisclaimer,
            YTNode::ExpandableMetadata(_) => YTNodeVariant::ExpandableMetadata,
            YTNode::ExpandedShelfContents(_) => YTNodeVariant::ExpandedShelfContents,
            YTNode::Factoid(_) => YTNodeVariant::Factoid,
            YTNode::FancyDismissibleDialog(_) => YTNodeVariant::FancyDismissibleDialog,
            YTNode::FeedTabbedHeader(_) => YTNodeVariant::FeedTabbedHeader,
            YTNode::FlexibleActionsView(_) => YTNodeVariant::FlexibleActionsView,
            YTNode::Form(_) => YTNodeVariant::Form,
            YTNode::FormFooterView(_) => YTNodeVariant::FormFooterView,
            YTNode::FormPopup(_) => YTNodeVariant::FormPopup,
            YTNode::GameCard(_) => YTNodeVariant::GameCard,
            YTNode::GameDetails(_) => YTNodeVariant::GameDetails,
            YTNode::Grid(_) => YTNodeVariant::Grid,
            YTNode::GridHeader(_) => YTNodeVariant::GridHeader,
            YTNode::GridShelfView(_) => YTNodeVariant::GridShelfView,
            YTNode::GuideCollapsibleEntry(_) => YTNodeVariant::GuideCollapsibleEntry,
            YTNode::GuideCollapsibleSectionEntry(_) => YTNodeVariant::GuideCollapsibleSectionEntry,
            YTNode::GuideDownloadsEntry(_) => YTNodeVariant::GuideDownloadsEntry,
            YTNode::GuideEntry(_) => YTNodeVariant::GuideEntry,
            YTNode::GuideSection(_) => YTNodeVariant::GuideSection,
            YTNode::GuideSubscriptionsSection(_) => YTNodeVariant::GuideSubscriptionsSection,
            YTNode::HashtagHeader(_) => YTNodeVariant::HashtagHeader,
            YTNode::HashtagTile(_) => YTNodeVariant::HashtagTile,
            YTNode::HeatMarker(_) => YTNodeVariant::HeatMarker,
            YTNode::HeroPlaylistThumbnail(_) => YTNodeVariant::HeroPlaylistThumbnail,
            YTNode::HighlightsCarousel(_) => YTNodeVariant::HighlightsCarousel,
            YTNode::HorizontalList(_) => YTNodeVariant::HorizontalList,
            YTNode::HorizontalMovieList(_) => YTNodeVariant::HorizontalMovieList,
            YTNode::HowThisWasMadeSectionView(_) => YTNodeVariant::HowThisWasMadeSectionView,
            YTNode::HypeFanCreditsSectionView(_) => YTNodeVariant::HypeFanCreditsSectionView,
            YTNode::HypePointsFactoid(_) => YTNodeVariant::HypePointsFactoid,
            YTNode::IconLink(_) => YTNodeVariant::IconLink,
            YTNode::ImageBannerView(_) => YTNodeVariant::ImageBannerView,
            YTNode::IncludingResultsFor(_) => YTNodeVariant::IncludingResultsFor,
            YTNode::InfoPanelContainer(_) => YTNodeVariant::InfoPanelContainer,
            YTNode::InfoPanelContent(_) => YTNodeVariant::InfoPanelContent,
            YTNode::InteractiveTabbedHeader(_) => YTNodeVariant::InteractiveTabbedHeader,
            YTNode::ItemSectionHeader(_) => YTNodeVariant::ItemSectionHeader,
            YTNode::ItemSectionTab(_) => YTNodeVariant::ItemSectionTab,
            YTNode::ItemSectionTabbedHeader(_) => YTNodeVariant::ItemSectionTabbedHeader,
            YTNode::LikeButton(_) => YTNodeVariant::LikeButton,
            YTNode::LikeButtonView(_) => YTNodeVariant::LikeButtonView,
            YTNode::ListItemView(_) => YTNodeVariant::ListItemView,
            YTNode::ListView(_) => YTNodeVariant::ListView,
            YTNode::LiveChatDialog(_) => YTNodeVariant::LiveChatDialog,
            YTNode::LockupMetadataView(_) => YTNodeVariant::LockupMetadataView,
            YTNode::LockupView(_) => YTNodeVariant::LockupView,
            YTNode::MacroMarkersInfoItem(_) => YTNodeVariant::MacroMarkersInfoItem,
            YTNode::MacroMarkersListEntity(_) => YTNodeVariant::MacroMarkersListEntity,
            YTNode::MenuTitle(_) => YTNodeVariant::MenuTitle,
            YTNode::MerchandiseItem(_) => YTNodeVariant::MerchandiseItem,
            YTNode::MerchandiseShelf(_) => YTNodeVariant::MerchandiseShelf,
            YTNode::Message(_) => YTNodeVariant::Message,
            YTNode::MetadataRow(_) => YTNodeVariant::MetadataRow,
            YTNode::MetadataRowContainer(_) => YTNodeVariant::MetadataRowContainer,
            YTNode::MetadataRowHeader(_) => YTNodeVariant::MetadataRowHeader,
            YTNode::MetadataScreen(_) => YTNodeVariant::MetadataScreen,
            YTNode::Mix(_) => YTNodeVariant::Mix,
            YTNode::Movie(_) => YTNodeVariant::Movie,
            YTNode::MovingThumbnail(_) => YTNodeVariant::MovingThumbnail,
            YTNode::MultiMarkersPlayerBar(_) => YTNodeVariant::MultiMarkersPlayerBar,
            YTNode::MusicCardShelfHeaderBasic(_) => YTNodeVariant::MusicCardShelfHeaderBasic,
            YTNode::MusicCarouselShelfBasicHeader(_) => YTNodeVariant::MusicCarouselShelfBasicHeader,
            YTNode::MusicLargeCardItemCarousel(_) => YTNodeVariant::MusicLargeCardItemCarousel,
            YTNode::MusicMultiRowListItem(_) => YTNodeVariant::MusicMultiRowListItem,
            YTNode::MusicPlaylistEditHeader(_) => YTNodeVariant::MusicPlaylistEditHeader,
            YTNode::MusicResponsiveListItemFixedColumn(_) => YTNodeVariant::MusicResponsiveListItemFixedColumn,
            YTNode::MusicResponsiveListItemFlexColumn(_) => YTNodeVariant::MusicResponsiveListItemFlexColumn,
            YTNode::MusicTastebuilderShelf(_) => YTNodeVariant::MusicTastebuilderShelf,
            YTNode::MusicTastebuilderShelfThumbnail(_) => YTNodeVariant::MusicTastebuilderShelfThumbnail,
            YTNode::NotificationAction(_) => YTNodeVariant::NotificationAction,
            YTNode::OpenOnePickAddVideoModalCommand(_) => YTNodeVariant::OpenOnePickAddVideoModalCommand,
            YTNode::PageHeader(_) => YTNodeVariant::PageHeader,
            YTNode::PageHeaderView(_) => YTNodeVariant::PageHeaderView,
            YTNode::PageIndicatorView(_) => YTNodeVariant::PageIndicatorView,
            YTNode::PageIntroduction(_) => YTNodeVariant::PageIntroduction,
            YTNode::PanelFooterView(_) => YTNodeVariant::PanelFooterView,
            YTNode::PivotButton(_) => YTNodeVariant::PivotButton,
            YTNode::PlaylistAddToOption(_) => YTNodeVariant::PlaylistAddToOption,
            YTNode::PlaylistCollaborationView(_) => YTNodeVariant::PlaylistCollaborationView,
            YTNode::PlaylistCustomThumbnail(_) => YTNodeVariant::PlaylistCustomThumbnail,
            YTNode::PlaylistHeader(_) => YTNodeVariant::PlaylistHeader,
            YTNode::PlaylistInfoCardContent(_) => YTNodeVariant::PlaylistInfoCardContent,
            YTNode::PlaylistPanelVideoWrapper(_) => YTNodeVariant::PlaylistPanelVideoWrapper,
            YTNode::PlaylistSidebar(_) => YTNodeVariant::PlaylistSidebar,
            YTNode::PlaylistThumbnailOverlay(_) => YTNodeVariant::PlaylistThumbnailOverlay,
            YTNode::PlaylistVideoList(_) => YTNodeVariant::PlaylistVideoList,
            YTNode::PlaylistVideoThumbnail(_) => YTNodeVariant::PlaylistVideoThumbnail,
            YTNode::PremiereTrailerBadge(_) => YTNodeVariant::PremiereTrailerBadge,
            YTNode::ProductList(_) => YTNodeVariant::ProductList,
            YTNode::ProductListHeader(_) => YTNodeVariant::ProductListHeader,
            YTNode::ProductListItem(_) => YTNodeVariant::ProductListItem,
            YTNode::ProfileColumnStats(_) => YTNodeVariant::ProfileColumnStats,
            YTNode::ProfileColumnStatsEntry(_) => YTNodeVariant::ProfileColumnStatsEntry,
            YTNode::Quiz(_) => YTNodeVariant::Quiz,
            YTNode::RecognitionShelf(_) => YTNodeVariant::RecognitionShelf,
            YTNode::RelatedChipCloud(_) => YTNodeVariant::RelatedChipCloud,
            YTNode::RichListHeader(_) => YTNodeVariant::RichListHeader,
            YTNode::RichMetadata(_) => YTNodeVariant::RichMetadata,
            YTNode::RichMetadataRow(_) => YTNodeVariant::RichMetadataRow,
            YTNode::SearchBox(_) => YTNodeVariant::SearchBox,
            YTNode::SearchFilterOptionsDialog(_) => YTNodeVariant::SearchFilterOptionsDialog,
            YTNode::SearchHeader(_) => YTNodeVariant::SearchHeader,
            YTNode::SearchSuggestion(_) => YTNodeVariant::SearchSuggestion,
            YTNode::SearchSuggestionsSection(_) => YTNodeVariant::SearchSuggestionsSection,
            YTNode::SecondarySearchContainer(_) => YTNodeVariant::SecondarySearchContainer,
            YTNode::SectionHeaderView(_) => YTNodeVariant::SectionHeaderView,
            YTNode::SegmentedLikeDislikeButton(_) => YTNodeVariant::SegmentedLikeDislikeButton,
            YTNode::SegmentedLikeDislikeButtonView(_) => YTNodeVariant::SegmentedLikeDislikeButtonView,
            YTNode::SettingBoolean(_) => YTNodeVariant::SettingBoolean,
            YTNode::SettingsCheckbox(_) => YTNodeVariant::SettingsCheckbox,
            YTNode::SettingsOptions(_) => YTNodeVariant::SettingsOptions,
            YTNode::SettingsSidebar(_) => YTNodeVariant::SettingsSidebar,
            YTNode::SettingsSwitch(_) => YTNodeVariant::SettingsSwitch,
            YTNode::SharePanelHeader(_) => YTNodeVariant::SharePanelHeader,
            YTNode::SharePanelTitleV15(_) => YTNodeVariant::SharePanelTitleV15,
            YTNode::ShareTarget(_) => YTNodeVariant::ShareTarget,
            YTNode::SheetView(_) => YTNodeVariant::SheetView,
            YTNode::ShowCustomThumbnail(_) => YTNodeVariant::ShowCustomThumbnail,
            YTNode::SimpleCardContent(_) => YTNodeVariant::SimpleCardContent,
            YTNode::SimpleCardTeaser(_) => YTNodeVariant::SimpleCardTeaser,
            YTNode::SimpleTextSection(_) => YTNodeVariant::SimpleTextSection,
            YTNode::SingleColumnBrowseResults(_) => YTNodeVariant::SingleColumnBrowseResults,
            YTNode::SingleColumnMusicWatchNextResults(_) => YTNodeVariant::SingleColumnMusicWatchNextResults,
            YTNode::SingleHeroImage(_) => YTNodeVariant::SingleHeroImage,
            YTNode::SlimOwner(_) => YTNodeVariant::SlimOwner,
            YTNode::SortFilterHeader(_) => YTNodeVariant::SortFilterHeader,
            YTNode::SortFilterSubMenu(_) => YTNodeVariant::SortFilterSubMenu,
            YTNode::StartAt(_) => YTNodeVariant::StartAt,
            YTNode::StructuredDescriptionContent(_) => YTNodeVariant::StructuredDescriptionContent,
            YTNode::StructuredDescriptionPlaylistLockup(_) => YTNodeVariant::StructuredDescriptionPlaylistLockup,
            YTNode::SubFeedOption(_) => YTNodeVariant::SubFeedOption,
            YTNode::SubFeedSelector(_) => YTNodeVariant::SubFeedSelector,
            YTNode::SubscribeButton(_) => YTNodeVariant::SubscribeButton,
            YTNode::SubscribeButtonView(_) => YTNodeVariant::SubscribeButtonView,
            YTNode::Tabbed(_) => YTNodeVariant::Tabbed,
            YTNode::TabbedSearchResults(_) => YTNodeVariant::TabbedSearchResults,
            YTNode::TextCarouselItemView(_) => YTNodeVariant::TextCarouselItemView,
            YTNode::TextFieldView(_) => YTNodeVariant::TextFieldView,
            YTNode::ThirdPartyShareTargetSection(_) => YTNodeVariant::ThirdPartyShareTargetSection,
            YTNode::ThumbnailBadgeView(_) => YTNodeVariant::ThumbnailBadgeView,
            YTNode::ThumbnailBottomOverlayView(_) => YTNodeVariant::ThumbnailBottomOverlayView,
            YTNode::ThumbnailHoverOverlayToggleActionsView(_) => YTNodeVariant::ThumbnailHoverOverlayToggleActionsView,
            YTNode::ThumbnailHoverOverlayView(_) => YTNodeVariant::ThumbnailHoverOverlayView,
            YTNode::ThumbnailLandscapePortrait(_) => YTNodeVariant::ThumbnailLandscapePortrait,
            YTNode::ThumbnailOverlayAvatarStackView(_) => YTNodeVariant::ThumbnailOverlayAvatarStackView,
            YTNode::ThumbnailOverlayBadgeView(_) => YTNodeVariant::ThumbnailOverlayBadgeView,
            YTNode::ThumbnailOverlayPinking(_) => YTNodeVariant::ThumbnailOverlayPinking,
            YTNode::ThumbnailOverlayPlaybackStatus(_) => YTNodeVariant::ThumbnailOverlayPlaybackStatus,
            YTNode::ThumbnailOverlayProgressBarView(_) => YTNodeVariant::ThumbnailOverlayProgressBarView,
            YTNode::ThumbnailOverlayResumePlayback(_) => YTNodeVariant::ThumbnailOverlayResumePlayback,
            YTNode::ThumbnailOverlayTitleView(_) => YTNodeVariant::ThumbnailOverlayTitleView,
            YTNode::ThumbnailView(_) => YTNodeVariant::ThumbnailView,
            YTNode::TicketEvent(_) => YTNodeVariant::TicketEvent,
            YTNode::TicketShelf(_) => YTNodeVariant::TicketShelf,
            YTNode::TitleAndButtonListHeader(_) => YTNodeVariant::TitleAndButtonListHeader,
            YTNode::ToggleButtonView(_) => YTNodeVariant::ToggleButtonView,
            YTNode::ToggleFormField(_) => YTNodeVariant::ToggleFormField,
            YTNode::ToggleMenuServiceItem(_) => YTNodeVariant::ToggleMenuServiceItem,
            YTNode::Tooltip(_) => YTNodeVariant::Tooltip,
            YTNode::Transcript(_) => YTNodeVariant::Transcript,
            YTNode::TranscriptFooter(_) => YTNodeVariant::TranscriptFooter,
            YTNode::TranscriptSearchBox(_) => YTNodeVariant::TranscriptSearchBox,
            YTNode::TranscriptSearchPanel(_) => YTNodeVariant::TranscriptSearchPanel,
            YTNode::TranscriptSectionHeader(_) => YTNodeVariant::TranscriptSectionHeader,
            YTNode::TranscriptSegment(_) => YTNodeVariant::TranscriptSegment,
            YTNode::TranscriptSegmentList(_) => YTNodeVariant::TranscriptSegmentList,
            YTNode::TwoColumnBrowseResults(_) => YTNodeVariant::TwoColumnBrowseResults,
            YTNode::TwoColumnSearchResults(_) => YTNodeVariant::TwoColumnSearchResults,
            YTNode::TwoColumnWatchNextResults(_) => YTNodeVariant::TwoColumnWatchNextResults,
            YTNode::UnifiedSharePanel(_) => YTNodeVariant::UnifiedSharePanel,
            YTNode::UniversalWatchCard(_) => YTNodeVariant::UniversalWatchCard,
            YTNode::UploadTimeFactoid(_) => YTNodeVariant::UploadTimeFactoid,
            YTNode::UpsellDialog(_) => YTNodeVariant::UpsellDialog,
            YTNode::VerticalWatchCardList(_) => YTNodeVariant::VerticalWatchCardList,
            YTNode::VideoAttributesSectionView(_) => YTNodeVariant::VideoAttributesSectionView,
            YTNode::VideoDescriptionCourseSection(_) => YTNodeVariant::VideoDescriptionCourseSection,
            YTNode::VideoDescriptionInfocardsSection(_) => YTNodeVariant::VideoDescriptionInfocardsSection,
            YTNode::VideoDescriptionMusicSection(_) => YTNodeVariant::VideoDescriptionMusicSection,
            YTNode::VideoDescriptionTranscriptSection(_) => YTNodeVariant::VideoDescriptionTranscriptSection,
            YTNode::VideoDescriptionYouchatSectionView(_) => YTNodeVariant::VideoDescriptionYouchatSectionView,
            YTNode::VideoMetadataCarouselView(_) => YTNodeVariant::VideoMetadataCarouselView,
            YTNode::ViewCountFactoid(_) => YTNodeVariant::ViewCountFactoid,
            YTNode::WatchCardRichHeader(_) => YTNodeVariant::WatchCardRichHeader,
            YTNode::WatchCardSectionSequence(_) => YTNodeVariant::WatchCardSectionSequence,
            YTNode::WatchNextEndScreen(_) => YTNodeVariant::WatchNextEndScreen,
            YTNode::WatchNextTabbedResults(_) => YTNodeVariant::WatchNextTabbedResults,
            YTNode::YpcTrailer(_) => YTNodeVariant::YpcTrailer,
            YTNode::CommandExecutorCommand(_) => YTNodeVariant::CommandExecutorCommand,
            YTNode::GetKidsBlocklistPickerCommand(_) => YTNodeVariant::GetKidsBlocklistPickerCommand,
            YTNode::ShowDialogCommand(_) => YTNodeVariant::ShowDialogCommand,
            YTNode::CommentDialog(_) => YTNodeVariant::CommentDialog,
            YTNode::CommentReplyDialog(_) => YTNodeVariant::CommentReplyDialog,
            YTNode::EmojiPicker(_) => YTNodeVariant::EmojiPicker,
            YTNode::VoiceReplyContainerView(_) => YTNodeVariant::VoiceReplyContainerView,
            YTNode::AddToPlaylistEndpoint(_) => YTNodeVariant::AddToPlaylistEndpoint,
            YTNode::AddToPlaylistServiceEndpoint(_) => YTNodeVariant::AddToPlaylistServiceEndpoint,
            YTNode::CreateCommentEndpoint(_) => YTNodeVariant::CreateCommentEndpoint,
            YTNode::CreatePlaylistServiceEndpoint(_) => YTNodeVariant::CreatePlaylistServiceEndpoint,
            YTNode::DeletePlaylistEndpoint(_) => YTNodeVariant::DeletePlaylistEndpoint,
            YTNode::FeedbackEndpoint(_) => YTNodeVariant::FeedbackEndpoint,
            YTNode::GetAccountsListInnertubeEndpoint(_) => YTNodeVariant::GetAccountsListInnertubeEndpoint,
            YTNode::HideEngagementPanelEndpoint(_) => YTNodeVariant::HideEngagementPanelEndpoint,
            YTNode::LiveChatItemContextMenuEndpoint(_) => YTNodeVariant::LiveChatItemContextMenuEndpoint,
            YTNode::ModifyChannelNotificationPreferenceEndpoint(_) => YTNodeVariant::ModifyChannelNotificationPreferenceEndpoint,
            YTNode::PerformCommentActionEndpoint(_) => YTNodeVariant::PerformCommentActionEndpoint,
            YTNode::PlaylistEditEndpoint(_) => YTNodeVariant::PlaylistEditEndpoint,
            YTNode::PrefetchWatchCommand(_) => YTNodeVariant::PrefetchWatchCommand,
            YTNode::ShareEndpoint(_) => YTNodeVariant::ShareEndpoint,
            YTNode::ShareEntityEndpoint(_) => YTNodeVariant::ShareEntityEndpoint,
            YTNode::MenuFlexibleItem(_) => YTNodeVariant::MenuFlexibleItem,
            YTNode::MenuNavigationItem(_) => YTNodeVariant::MenuNavigationItem,
            YTNode::MenuPopup(_) => YTNodeVariant::MenuPopup,
            YTNode::MenuServiceItem(_) => YTNodeVariant::MenuServiceItem,
            YTNode::MenuServiceItemDownload(_) => YTNodeVariant::MenuServiceItemDownload,
            YTNode::MultiPageMenu(_) => YTNodeVariant::MultiPageMenu,
            YTNode::MultiPageMenuNotificationSection(_) => YTNodeVariant::MultiPageMenuNotificationSection,
            YTNode::SimpleMenuHeader(_) => YTNodeVariant::SimpleMenuHeader,
            YTNode::MobileTopbar(_) => YTNodeVariant::MobileTopbar,
            YTNode::MultiPageMenuSection(_) => YTNodeVariant::MultiPageMenuSection,
            YTNode::PivotBar(_) => YTNodeVariant::PivotBar,
            YTNode::PivotBarItem(_) => YTNodeVariant::PivotBarItem,
            YTNode::TopbarMenuButton(_) => YTNodeVariant::TopbarMenuButton,
            YTNode::ReplaceLiveChatAction(_) => YTNodeVariant::ReplaceLiveChatAction,
            YTNode::UpdateDateTextAction(_) => YTNodeVariant::UpdateDateTextAction,
            YTNode::UpdateDescriptionAction(_) => YTNodeVariant::UpdateDescriptionAction,
            YTNode::UpdateTitleAction(_) => YTNodeVariant::UpdateTitleAction,
            YTNode::UpdateToggleButtonTextAction(_) => YTNodeVariant::UpdateToggleButtonTextAction,
            YTNode::UpdateViewershipAction(_) => YTNodeVariant::UpdateViewershipAction,
            YTNode::BumperUserEduContentView(_) => YTNodeVariant::BumperUserEduContentView,
            YTNode::PdgReplyButtonView(_) => YTNodeVariant::PdgReplyButtonView,
            YTNode::PlaylistCollaborationFormSchema(_) => YTNodeVariant::PlaylistCollaborationFormSchema,
            YTNode::PlaylistCollaborationViewModelPlaylistCollaboratorData(_) => YTNodeVariant::PlaylistCollaborationViewModelPlaylistCollaboratorData,
            YTNode::SubscriptionButton(_) => YTNodeVariant::SubscriptionButton,
            YTNode::CommandContext(_) => YTNodeVariant::CommandContext,
            YTNode::ShareEntityServiceEndpoint(_) => YTNodeVariant::ShareEntityServiceEndpoint,
            YTNode::SignalServiceEndpoint(_) => YTNodeVariant::SignalServiceEndpoint,
            YTNode::UnsubscribeEndpoint(_) => YTNodeVariant::UnsubscribeEndpoint,
            YTNode::WatchNextEndpoint(_) => YTNodeVariant::WatchNextEndpoint,
            YTNode::AccessibilityContext(_) => YTNodeVariant::AccessibilityContext,
            YTNode::AccessibilityData(_) => YTNodeVariant::AccessibilityData,
            YTNode::ChildElement(_) => YTNodeVariant::ChildElement,
            YTNode::EmojiRun(_) => YTNodeVariant::EmojiRun,
            YTNode::RendererContext(_) => YTNodeVariant::RendererContext,
            YTNode::AnchoredSection(_) => YTNodeVariant::AnchoredSection,
            YTNode::KidsBlocklistPicker(_) => YTNodeVariant::KidsBlocklistPicker,
            YTNode::KidsBlocklistPickerItem(_) => YTNodeVariant::KidsBlocklistPickerItem,
            YTNode::MusicResponsiveListItem(_) => YTNodeVariant::MusicResponsiveListItem,
            YTNode::NavigationEndpoint(_) => YTNodeVariant::NavigationEndpoint,
            YTNode::ThumbnailOverlayTimeStatus(_) => YTNodeVariant::ThumbnailOverlayTimeStatus,
            YTNode::BrowseEndpoint(_) => YTNodeVariant::BrowseEndpoint,
            YTNode::LikeEndpoint(_) => YTNodeVariant::LikeEndpoint,
            YTNode::ReelWatchEndpoint(_) => YTNodeVariant::ReelWatchEndpoint,
            YTNode::SearchEndpoint(_) => YTNodeVariant::SearchEndpoint,
            YTNode::SubscribeEndpoint(_) => YTNodeVariant::SubscribeEndpoint,
            YTNode::WatchEndpoint(_) => YTNodeVariant::WatchEndpoint,
            YTNode::Author(_) => YTNodeVariant::Author,
            YTNode::Text(_) => YTNodeVariant::Text,
            YTNode::TextRun(_) => YTNodeVariant::TextRun,
            YTNode::Thumbnail(_) => YTNodeVariant::Thumbnail,
            YTNode::AutomixPreviewVideo(_) => YTNodeVariant::AutomixPreviewVideo,
            YTNode::VideoViewCount(_) => YTNodeVariant::VideoViewCount,
            YTNode::BrowserMediaSession(_) => YTNodeVariant::BrowserMediaSession,
            YTNode::ChannelVideoPlayer(_) => YTNodeVariant::ChannelVideoPlayer,
            YTNode::ChildVideo(_) => YTNodeVariant::ChildVideo,
            YTNode::EndScreenVideo(_) => YTNodeVariant::EndScreenVideo,
            YTNode::ExpandableVideoDescriptionBody(_) => YTNodeVariant::ExpandableVideoDescriptionBody,
            YTNode::PlayerAnnotationsExpanded(_) => YTNodeVariant::PlayerAnnotationsExpanded,
            YTNode::PlayerCaptchaView(_) => YTNodeVariant::PlayerCaptchaView,
            YTNode::PlayerControlsOverlay(_) => YTNodeVariant::PlayerControlsOverlay,
            YTNode::PlayerLegacyDesktopYpcOffer(_) => YTNodeVariant::PlayerLegacyDesktopYpcOffer,
            YTNode::PlayerMicroformat(_) => YTNodeVariant::PlayerMicroformat,
            YTNode::PlayerOverflow(_) => YTNodeVariant::PlayerOverflow,
            YTNode::PlayerOverlayAutoplay(_) => YTNodeVariant::PlayerOverlayAutoplay,
            YTNode::PlayerOverlayVideoDetails(_) => YTNodeVariant::PlayerOverlayVideoDetails,
            YTNode::SlimVideoMetadata(_) => YTNodeVariant::SlimVideoMetadata,
            YTNode::VideoAttributeView(_) => YTNodeVariant::VideoAttributeView,
            YTNode::VideoCard(_) => YTNodeVariant::VideoCard,
            YTNode::VideoDescriptionHeader(_) => YTNodeVariant::VideoDescriptionHeader,
            YTNode::VideoInfoCardContent(_) => YTNodeVariant::VideoInfoCardContent,
            YTNode::VideoSummaryContentView(_) => YTNodeVariant::VideoSummaryContentView,
            YTNode::VideoSummaryParagraphView(_) => YTNodeVariant::VideoSummaryParagraphView,
            YTNode::WatchCardCompactVideo(_) => YTNodeVariant::WatchCardCompactVideo,
            YTNode::WatchCardHeroVideo(_) => YTNodeVariant::WatchCardHeroVideo,
            YTNode::Format(_) => YTNodeVariant::Format,
            YTNode::VideoDetails(_) => YTNodeVariant::VideoDetails,
            YTNode::LiveChatAuthorBadge(_) => YTNodeVariant::LiveChatAuthorBadge,
            YTNode::LiveChatHeader(_) => YTNodeVariant::LiveChatHeader,
            YTNode::LiveChatMessageInput(_) => YTNodeVariant::LiveChatMessageInput,
            YTNode::LiveChatParticipant(_) => YTNodeVariant::LiveChatParticipant,
            YTNode::LiveChatBannerChatSummary(_) => YTNodeVariant::LiveChatBannerChatSummary,
            YTNode::LiveChatBannerHeader(_) => YTNodeVariant::LiveChatBannerHeader,
            YTNode::LiveChatBannerRedirect(_) => YTNodeVariant::LiveChatBannerRedirect,
            YTNode::LiveChatItemBumperView(_) => YTNodeVariant::LiveChatItemBumperView,
            YTNode::LiveChatPaidMessage(_) => YTNodeVariant::LiveChatPaidMessage,
            YTNode::LiveChatPlaceholderItem(_) => YTNodeVariant::LiveChatPlaceholderItem,
            YTNode::LiveChatProductItem(_) => YTNodeVariant::LiveChatProductItem,
            YTNode::LiveChatRestrictedParticipation(_) => YTNodeVariant::LiveChatRestrictedParticipation,
            YTNode::LiveChatSponsorshipsGiftPurchaseAnnouncement(_) => YTNodeVariant::LiveChatSponsorshipsGiftPurchaseAnnouncement,
            YTNode::LiveChatSponsorshipsGiftRedemptionAnnouncement(_) => YTNodeVariant::LiveChatSponsorshipsGiftRedemptionAnnouncement,
            YTNode::LiveChatSponsorshipsHeader(_) => YTNodeVariant::LiveChatSponsorshipsHeader,
            YTNode::LiveChatTextMessage(_) => YTNodeVariant::LiveChatTextMessage,
            YTNode::LiveChatTickerPaidMessageItem(_) => YTNodeVariant::LiveChatTickerPaidMessageItem,
            YTNode::LiveChatTickerPaidStickerItem(_) => YTNodeVariant::LiveChatTickerPaidStickerItem,
            YTNode::LiveChatTickerSponsorItem(_) => YTNodeVariant::LiveChatTickerSponsorItem,
            YTNode::ShowLiveChatActionPanelAction(_) => YTNodeVariant::ShowLiveChatActionPanelAction,
            YTNode::ShowLiveChatDialogAction(_) => YTNodeVariant::ShowLiveChatDialogAction,
            YTNode::ShowLiveChatTooltipCommand(_) => YTNodeVariant::ShowLiveChatTooltipCommand,
            YTNode::MarkChatItemsByAuthorAsDeletedAction(_) => YTNodeVariant::MarkChatItemsByAuthorAsDeletedAction,
            YTNode::LiveChatBannerPoll(_) => YTNodeVariant::LiveChatBannerPoll,
            YTNode::AboutChannel(_) => YTNodeVariant::AboutChannel,
            YTNode::AboutChannelView(_) => YTNodeVariant::AboutChannelView,
            YTNode::AccountChannel(_) => YTNodeVariant::AccountChannel,
            YTNode::Channel(_) => YTNodeVariant::Channel,
            YTNode::ChannelAgeGate(_) => YTNodeVariant::ChannelAgeGate,
            YTNode::ChannelExternalLinkView(_) => YTNodeVariant::ChannelExternalLinkView,
            YTNode::ChannelFeaturedContent(_) => YTNodeVariant::ChannelFeaturedContent,
            YTNode::ChannelOptions(_) => YTNodeVariant::ChannelOptions,
            YTNode::ChannelTagline(_) => YTNodeVariant::ChannelTagline,
            YTNode::ChannelThumbnailWithLink(_) => YTNodeVariant::ChannelThumbnailWithLink,
            YTNode::TopicChannelDetails(_) => YTNodeVariant::TopicChannelDetails,
            YTNode::ActiveAccountHeader(_) => YTNodeVariant::ActiveAccountHeader,
            YTNode::ChannelHeaderLinks(_) => YTNodeVariant::ChannelHeaderLinks,
            YTNode::ChannelHeaderLinksView(_) => YTNodeVariant::ChannelHeaderLinksView,
            YTNode::ChannelMobileHeader(_) => YTNodeVariant::ChannelMobileHeader,
            YTNode::ChannelSwitcherHeader(_) => YTNodeVariant::ChannelSwitcherHeader,
            YTNode::AuthorCommentBadge(_) => YTNodeVariant::AuthorCommentBadge,
            YTNode::CommentReplies(_) => YTNodeVariant::CommentReplies,
            YTNode::CommentView(_) => YTNodeVariant::CommentView,
            YTNode::CommentsEntryPointTeaser(_) => YTNodeVariant::CommentsEntryPointTeaser,
            YTNode::CommentsSimplebox(_) => YTNodeVariant::CommentsSimplebox,
            YTNode::PdgCommentChip(_) => YTNodeVariant::PdgCommentChip,
            YTNode::SponsorCommentBadge(_) => YTNodeVariant::SponsorCommentBadge,
            YTNode::CommentsContinuation(_) => YTNodeVariant::CommentsContinuation,
            YTNode::MusicDownloadStateBadge(_) => YTNodeVariant::MusicDownloadStateBadge,
            YTNode::MusicElementHeader(_) => YTNodeVariant::MusicElementHeader,
            YTNode::MusicSortFilterButton(_) => YTNodeVariant::MusicSortFilterButton,
            YTNode::MusicThumbnail(_) => YTNodeVariant::MusicThumbnail,
            YTNode::MusicMenuItemDivider(_) => YTNodeVariant::MusicMenuItemDivider,
            YTNode::MusicMultiSelectMenu(_) => YTNodeVariant::MusicMultiSelectMenu,
            YTNode::MusicMultiSelectMenuItem(_) => YTNodeVariant::MusicMultiSelectMenuItem,
            YTNode::BackstagePost(_) => YTNodeVariant::BackstagePost,
            YTNode::BackstagePostThread(_) => YTNodeVariant::BackstagePostThread,
            YTNode::SharedPost(_) => YTNodeVariant::SharedPost,
            YTNode::ReelItem(_) => YTNodeVariant::ReelItem,
            YTNode::ReelPlayerHeader(_) => YTNodeVariant::ReelPlayerHeader,
            YTNode::ReelPlayerOverlay(_) => YTNodeVariant::ReelPlayerOverlay,
            YTNode::ShortsLockupView(_) => YTNodeVariant::ShortsLockupView,
            YTNode::AlertWithButton(_) => YTNodeVariant::AlertWithButton,
            YTNode::CompositeVideoPrimaryInfo(_) => YTNodeVariant::CompositeVideoPrimaryInfo,
            YTNode::EmergencyOnebox(_) => YTNodeVariant::EmergencyOnebox,
            YTNode::SingleActionEmergencySupport(_) => YTNodeVariant::SingleActionEmergencySupport,
            YTNode::PlayerLiveStoryboardSpec(_) => YTNodeVariant::PlayerLiveStoryboardSpec,
            YTNode::PollHeader(_) => YTNodeVariant::PollHeader,
            YTNode::ChangeEngagementPanelVisibilityAction(_) => YTNodeVariant::ChangeEngagementPanelVisibilityAction,
            YTNode::ShowEngagementPanelEndpoint(_) => YTNodeVariant::ShowEngagementPanelEndpoint,
            YTNode::CreatorHeartView(_) => YTNodeVariant::CreatorHeartView,
            YTNode::KidsCategoryTab(_) => YTNodeVariant::KidsCategoryTab,
        }
    }

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

        // 1b. Continuation wrappers (legacy parser/continuations.ts)
        if val.get("reloadContinuationItemsCommand").is_some() {
            if let Some(n) = ReloadContinuationItemsCommandNode::from_value(val) {
                return Some(YTNode::ReloadContinuationItemsCommand(n));
            }
        }
        if val.get("playlistPanelContinuation").is_some() {
            if let Some(n) = PlaylistPanelContinuationNode::from_value(val) {
                return Some(YTNode::PlaylistPanelContinuation(n));
            }
        }
        if val.get("liveChatContinuation").is_some() {
            if let Some(n) = LiveChatContinuationNode::from_value(val) {
                return Some(YTNode::LiveChatContinuation(n));
            }
        }
        if val.get("sectionListContinuation").is_some() {
            if let Some(n) = SectionListContinuationNode::from_value(val) {
                return Some(YTNode::SectionListContinuation(n));
            }
        }
        if val.get("itemSectionContinuation").is_some() {
            if let Some(n) = ItemSectionContinuationNode::from_value(val) {
                return Some(YTNode::ItemSectionContinuation(n));
            }
        }
        if val.get("gridContinuation").is_some() {
            if let Some(n) = GridContinuationNode::from_value(val) {
                return Some(YTNode::GridContinuation(n));
            }
        }
        if val.get("musicPlaylistShelfContinuation").is_some() {
            if let Some(n) = MusicPlaylistShelfContinuationNode::from_value(val) {
                return Some(YTNode::MusicPlaylistShelfContinuation(n));
            }
        }
        if val.get("musicShelfContinuation").is_some() {
            if let Some(n) = MusicShelfContinuationNode::from_value(val) {
                return Some(YTNode::MusicShelfContinuation(n));
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
        if val.get("videoRenderer").is_some() {
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
        if val.get("playlistRenderer").is_some() {
            if let Some(p) = PlaylistNode::from_value(val) {
                return Some(YTNode::Playlist(p));
            }
        }

        // 6. Check for Channel Renderers
        if val.get("channelCardRenderer").is_some() {
            if let Some(c) = ChannelCardNode::from_value(val) {
                return Some(YTNode::ChannelCard(c));
            }
        }
        if val.get("channelHeaderRenderer").is_some() {
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
        if val.get("postRenderer").is_some() {
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
        if val.get("liveChatRenderer").is_some() || val.get("liveChatMessageRenderer").is_some() {
            if let Some(lc) = LiveChatMessageNode::from_value(val) {
                return Some(YTNode::LiveChat(lc));
            }
        }

        // 12. Check for Buttons and Menus
        if val.get("toggleButtonRenderer").is_some() {
            if let Some(tb) = ToggleButtonNode::from_value(val) {
                return Some(YTNode::ToggleButton(tb));
            }
        }
        if val.get("buttonRenderer").is_some() {
            if let Some(b) = ButtonNode::from_value(val) {
                return Some(YTNode::Button(b));
            }
        }
        if val.get("menuRenderer").is_some() {
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
        if val.get("viewCountRenderer").is_some() {
            if let Some(vc) = ViewCountNode::from_value(val) {
                return Some(YTNode::ViewCount(vc));
            }
        }
        if val.get("videoViewCountRenderer").is_some() {
            if let Some(vc) = VideoViewCountNode::from_value(val) {
                return Some(YTNode::VideoViewCount(vc));
            }
        }
        if val.get("automixPreviewVideoRenderer").is_some() || val.get("automixPreviewVideo").is_some() {
            if let Some(ap) = AutomixPreviewVideoNode::from_value(val) {
                return Some(YTNode::AutomixPreviewVideo(ap));
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
        if val.get("musicHeaderRenderer").is_some() {
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
        if val.get("clarificationRenderer").is_some() {
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
        if val.get("showEngagementPanelAction").is_some() {
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
        if val.get("showLiveChatAction").is_some() {
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

        // 47. Guide & Sections (Batch 11)
        if val.get("guideEntryRenderer").is_some() {
            if let Some(n) = GuideEntryNode::from_value(val) { return Some(YTNode::GuideEntry(n)); }
        }
        if val.get("guideSectionRenderer").is_some() {
            if let Some(n) = GuideSectionNode::from_value(val) { return Some(YTNode::GuideSection(n)); }
        }
        if val.get("guideSubscriptionsSectionRenderer").is_some() {
            if let Some(n) = GuideSubscriptionsSectionNode::from_value(val) { return Some(YTNode::GuideSubscriptionsSection(n)); }
        }
        if val.get("hashtagHeaderRenderer").is_some() {
            if let Some(n) = HashtagHeaderNode::from_value(val) { return Some(YTNode::HashtagHeader(n)); }
        }
        if val.get("hashtagTileRenderer").is_some() {
            if let Some(n) = HashtagTileNode::from_value(val) { return Some(YTNode::HashtagTile(n)); }
        }
        if val.get("heatMarkerRenderer").is_some() {
            if let Some(n) = HeatMarkerNode::from_value(val) { return Some(YTNode::HeatMarker(n)); }
        }
        if val.get("heroPlaylistThumbnailRenderer").is_some() {
            if let Some(n) = HeroPlaylistThumbnailNode::from_value(val) { return Some(YTNode::HeroPlaylistThumbnail(n)); }
        }
        if val.get("highlightsCarouselRenderer").is_some() {
            if let Some(n) = HighlightsCarouselNode::from_value(val) { return Some(YTNode::HighlightsCarousel(n)); }
        }
        if val.get("horizontalListRenderer").is_some() {
            if let Some(n) = HorizontalListNode::from_value(val) { return Some(YTNode::HorizontalList(n)); }
        }
        if val.get("horizontalMovieListRenderer").is_some() {
            if let Some(n) = HorizontalMovieListNode::from_value(val) { return Some(YTNode::HorizontalMovieList(n)); }
        }
        if val.get("howThisWasMadeSectionView").is_some() {
            if let Some(n) = HowThisWasMadeSectionViewNode::from_value(val) { return Some(YTNode::HowThisWasMadeSectionView(n)); }
        }
        if val.get("hypeFanCreditsSectionView").is_some() {
            if let Some(n) = HypeFanCreditsSectionViewNode::from_value(val) { return Some(YTNode::HypeFanCreditsSectionView(n)); }
        }

        // 48. Lists & Headers (Batch 11)
        if val.get("hypePointsFactoidRenderer").is_some() {
            if let Some(n) = HypePointsFactoidNode::from_value(val) { return Some(YTNode::HypePointsFactoid(n)); }
        }
        if val.get("iconLinkRenderer").is_some() {
            if let Some(n) = IconLinkNode::from_value(val) { return Some(YTNode::IconLink(n)); }
        }
        if val.get("imageBannerView").is_some() {
            if let Some(n) = ImageBannerViewNode::from_value(val) { return Some(YTNode::ImageBannerView(n)); }
        }
        if val.get("includingResultsForRenderer").is_some() {
            if let Some(n) = IncludingResultsForNode::from_value(val) { return Some(YTNode::IncludingResultsFor(n)); }
        }
        if val.get("infoPanelContainerRenderer").is_some() {
            if let Some(n) = InfoPanelContainerNode::from_value(val) { return Some(YTNode::InfoPanelContainer(n)); }
        }
        if val.get("infoPanelContentRenderer").is_some() {
            if let Some(n) = InfoPanelContentNode::from_value(val) { return Some(YTNode::InfoPanelContent(n)); }
        }
        if val.get("interactiveTabbedHeaderRenderer").is_some() {
            if let Some(n) = InteractiveTabbedHeaderNode::from_value(val) { return Some(YTNode::InteractiveTabbedHeader(n)); }
        }
        if val.get("itemSectionHeaderRenderer").is_some() {
            if let Some(n) = ItemSectionHeaderNode::from_value(val) { return Some(YTNode::ItemSectionHeader(n)); }
        }
        if val.get("itemSectionTabRenderer").is_some() {
            if let Some(n) = ItemSectionTabNode::from_value(val) { return Some(YTNode::ItemSectionTab(n)); }
        }
        if val.get("itemSectionTabbedHeaderRenderer").is_some() {
            if let Some(n) = ItemSectionTabbedHeaderNode::from_value(val) { return Some(YTNode::ItemSectionTabbedHeader(n)); }
        }
        if val.get("likeButtonRenderer").is_some() {
            if let Some(n) = LikeButtonNode::from_value(val) { return Some(YTNode::LikeButton(n)); }
        }
        if val.get("likeButtonView").is_some() {
            if let Some(n) = LikeButtonViewNode::from_value(val) { return Some(YTNode::LikeButtonView(n)); }
        }

        // 49. Panels & Lockups (Batch 11)
        if val.get("listItemView").is_some() {
            if let Some(n) = ListItemViewNode::from_value(val) { return Some(YTNode::ListItemView(n)); }
        }
        if val.get("listView").is_some() {
            if let Some(n) = ListViewNode::from_value(val) { return Some(YTNode::ListView(n)); }
        }
        if val.get("liveChatDialogRenderer").is_some() {
            if let Some(n) = LiveChatDialogNode::from_value(val) { return Some(YTNode::LiveChatDialog(n)); }
        }
        if val.get("lockupMetadataView").is_some() {
            if let Some(n) = LockupMetadataViewNode::from_value(val) { return Some(YTNode::LockupMetadataView(n)); }
        }
        if val.get("lockupView").is_some() {
            if let Some(n) = LockupViewNode::from_value(val) { return Some(YTNode::LockupView(n)); }
        }
        if val.get("macroMarkersInfoItemRenderer").is_some() {
            if let Some(n) = MacroMarkersInfoItemNode::from_value(val) { return Some(YTNode::MacroMarkersInfoItem(n)); }
        }
        if val.get("macroMarkersListEntity").is_some() {
            if let Some(n) = MacroMarkersListEntityNode::from_value(val) { return Some(YTNode::MacroMarkersListEntity(n)); }
        }
        if val.get("menuTitleRenderer").is_some() {
            if let Some(n) = MenuTitleNode::from_value(val) { return Some(YTNode::MenuTitle(n)); }
        }
        if val.get("merchandiseItemRenderer").is_some() {
            if let Some(n) = MerchandiseItemNode::from_value(val) { return Some(YTNode::MerchandiseItem(n)); }
        }
        if val.get("merchandiseShelfRenderer").is_some() {
            if let Some(n) = MerchandiseShelfNode::from_value(val) { return Some(YTNode::MerchandiseShelf(n)); }
        }
        if val.get("messageRenderer").is_some() {
            if let Some(n) = MessageNode::from_value(val) { return Some(YTNode::Message(n)); }
        }
        if val.get("metadataRowRenderer").is_some() {
            if let Some(n) = MetadataRowNode::from_value(val) { return Some(YTNode::MetadataRow(n)); }
        }

        // 50. Media & Music Headers (Batch 11)
        if val.get("metadataRowContainerRenderer").is_some() {
            if let Some(n) = MetadataRowContainerNode::from_value(val) { return Some(YTNode::MetadataRowContainer(n)); }
        }
        if val.get("metadataRowHeaderRenderer").is_some() {
            if let Some(n) = MetadataRowHeaderNode::from_value(val) { return Some(YTNode::MetadataRowHeader(n)); }
        }
        if val.get("metadataScreenRenderer").is_some() {
            if let Some(n) = MetadataScreenNode::from_value(val) { return Some(YTNode::MetadataScreen(n)); }
        }
        if val.get("mixRenderer").is_some() {
            if let Some(n) = MixNode::from_value(val) { return Some(YTNode::Mix(n)); }
        }
        if val.get("movieRenderer").is_some() {
            if let Some(n) = MovieNode::from_value(val) { return Some(YTNode::Movie(n)); }
        }
        if val.get("movingThumbnailRenderer").is_some() {
            if let Some(n) = MovingThumbnailNode::from_value(val) { return Some(YTNode::MovingThumbnail(n)); }
        }
        if val.get("multiMarkersPlayerBarRenderer").is_some() {
            if let Some(n) = MultiMarkersPlayerBarNode::from_value(val) { return Some(YTNode::MultiMarkersPlayerBar(n)); }
        }
        if val.get("musicCardShelfHeaderBasicRenderer").is_some() {
            if let Some(n) = MusicCardShelfHeaderBasicNode::from_value(val) { return Some(YTNode::MusicCardShelfHeaderBasic(n)); }
        }
        if val.get("musicCarouselShelfBasicHeaderRenderer").is_some() {
            if let Some(n) = MusicCarouselShelfBasicHeaderNode::from_value(val) { return Some(YTNode::MusicCarouselShelfBasicHeader(n)); }
        }
        if val.get("musicLargeCardItemCarouselRenderer").is_some() {
            if let Some(n) = MusicLargeCardItemCarouselNode::from_value(val) { return Some(YTNode::MusicLargeCardItemCarousel(n)); }
        }
        if val.get("musicMultiRowListItemRenderer").is_some() {
            if let Some(n) = MusicMultiRowListItemNode::from_value(val) { return Some(YTNode::MusicMultiRowListItem(n)); }
        }
        if val.get("musicPlaylistEditHeaderRenderer").is_some() {
            if let Some(n) = MusicPlaylistEditHeaderNode::from_value(val) { return Some(YTNode::MusicPlaylistEditHeader(n)); }
        }

        // 51. Music & Page Headers (Batch 12)
        if val.get("musicResponsiveListItemFixedColumnRenderer").is_some() {
            if let Some(n) = MusicResponsiveListItemFixedColumnNode::from_value(val) { return Some(YTNode::MusicResponsiveListItemFixedColumn(n)); }
        }
        if val.get("musicResponsiveListItemFlexColumnRenderer").is_some() {
            if let Some(n) = MusicResponsiveListItemFlexColumnNode::from_value(val) { return Some(YTNode::MusicResponsiveListItemFlexColumn(n)); }
        }
        if val.get("musicTastebuilderShelfRenderer").is_some() {
            if let Some(n) = MusicTastebuilderShelfNode::from_value(val) { return Some(YTNode::MusicTastebuilderShelf(n)); }
        }
        if val.get("musicTastebuilderShelfThumbnailRenderer").is_some() {
            if let Some(n) = MusicTastebuilderShelfThumbnailNode::from_value(val) { return Some(YTNode::MusicTastebuilderShelfThumbnail(n)); }
        }
        if val.get("notificationActionRenderer").is_some() {
            if let Some(n) = NotificationActionNode::from_value(val) { return Some(YTNode::NotificationAction(n)); }
        }
        if val.get("openOnePickAddVideoModalCommand").is_some() {
            if let Some(n) = OpenOnePickAddVideoModalCommandNode::from_value(val) { return Some(YTNode::OpenOnePickAddVideoModalCommand(n)); }
        }
        if val.get("pageHeaderRenderer").is_some() {
            if let Some(n) = PageHeaderNode::from_value(val) { return Some(YTNode::PageHeader(n)); }
        }
        if val.get("pageHeaderView").is_some() {
            if let Some(n) = PageHeaderViewNode::from_value(val) { return Some(YTNode::PageHeaderView(n)); }
        }
        if val.get("pageIndicatorView").is_some() {
            if let Some(n) = PageIndicatorViewNode::from_value(val) { return Some(YTNode::PageIndicatorView(n)); }
        }
        if val.get("pageIntroductionRenderer").is_some() {
            if let Some(n) = PageIntroductionNode::from_value(val) { return Some(YTNode::PageIntroduction(n)); }
        }
        if val.get("panelFooterView").is_some() {
            if let Some(n) = PanelFooterViewNode::from_value(val) { return Some(YTNode::PanelFooterView(n)); }
        }
        if val.get("pivotButtonRenderer").is_some() {
            if let Some(n) = PivotButtonNode::from_value(val) { return Some(YTNode::PivotButton(n)); }
        }

        // 52. Playlists & Products (Batch 12)
        if val.get("playlistAddToOptionRenderer").is_some() {
            if let Some(n) = PlaylistAddToOptionNode::from_value(val) { return Some(YTNode::PlaylistAddToOption(n)); }
        }
        if val.get("playlistCollaborationView").is_some() {
            if let Some(n) = PlaylistCollaborationViewNode::from_value(val) { return Some(YTNode::PlaylistCollaborationView(n)); }
        }
        if val.get("playlistCustomThumbnailRenderer").is_some() {
            if let Some(n) = PlaylistCustomThumbnailNode::from_value(val) { return Some(YTNode::PlaylistCustomThumbnail(n)); }
        }
        if val.get("playlistHeaderRenderer").is_some() {
            if let Some(n) = PlaylistHeaderNode::from_value(val) { return Some(YTNode::PlaylistHeader(n)); }
        }
        if val.get("playlistInfoCardContentRenderer").is_some() {
            if let Some(n) = PlaylistInfoCardContentNode::from_value(val) { return Some(YTNode::PlaylistInfoCardContent(n)); }
        }
        if val.get("playlistPanelVideoWrapperRenderer").is_some() {
            if let Some(n) = PlaylistPanelVideoWrapperNode::from_value(val) { return Some(YTNode::PlaylistPanelVideoWrapper(n)); }
        }
        if val.get("playlistSidebarRenderer").is_some() {
            if let Some(n) = PlaylistSidebarNode::from_value(val) { return Some(YTNode::PlaylistSidebar(n)); }
        }
        if val.get("playlistThumbnailOverlayRenderer").is_some() {
            if let Some(n) = PlaylistThumbnailOverlayNode::from_value(val) { return Some(YTNode::PlaylistThumbnailOverlay(n)); }
        }
        if val.get("playlistVideoListRenderer").is_some() {
            if let Some(n) = PlaylistVideoListNode::from_value(val) { return Some(YTNode::PlaylistVideoList(n)); }
        }
        if val.get("playlistVideoThumbnailRenderer").is_some() {
            if let Some(n) = PlaylistVideoThumbnailNode::from_value(val) { return Some(YTNode::PlaylistVideoThumbnail(n)); }
        }
        if val.get("premiereTrailerBadgeRenderer").is_some() {
            if let Some(n) = PremiereTrailerBadgeNode::from_value(val) { return Some(YTNode::PremiereTrailerBadge(n)); }
        }
        if val.get("productListRenderer").is_some() {
            if let Some(n) = ProductListNode::from_value(val) { return Some(YTNode::ProductList(n)); }
        }

        // 53. Products & Metadata (Batch 12)
        if val.get("productListHeaderRenderer").is_some() {
            if let Some(n) = ProductListHeaderNode::from_value(val) { return Some(YTNode::ProductListHeader(n)); }
        }
        if val.get("productListItemRenderer").is_some() {
            if let Some(n) = ProductListItemNode::from_value(val) { return Some(YTNode::ProductListItem(n)); }
        }
        if val.get("profileColumnStatsRenderer").is_some() {
            if let Some(n) = ProfileColumnStatsNode::from_value(val) { return Some(YTNode::ProfileColumnStats(n)); }
        }
        if val.get("profileColumnStatsEntryRenderer").is_some() {
            if let Some(n) = ProfileColumnStatsEntryNode::from_value(val) { return Some(YTNode::ProfileColumnStatsEntry(n)); }
        }
        if val.get("quizRenderer").is_some() {
            if let Some(n) = QuizNode::from_value(val) { return Some(YTNode::Quiz(n)); }
        }
        if val.get("recognitionShelfRenderer").is_some() {
            if let Some(n) = RecognitionShelfNode::from_value(val) { return Some(YTNode::RecognitionShelf(n)); }
        }
        if val.get("relatedChipCloudRenderer").is_some() {
            if let Some(n) = RelatedChipCloudNode::from_value(val) { return Some(YTNode::RelatedChipCloud(n)); }
        }
        if val.get("richListHeaderRenderer").is_some() {
            if let Some(n) = RichListHeaderNode::from_value(val) { return Some(YTNode::RichListHeader(n)); }
        }
        if val.get("richMetadataRenderer").is_some() {
            if let Some(n) = RichMetadataNode::from_value(val) { return Some(YTNode::RichMetadata(n)); }
        }
        if val.get("richMetadataRowRenderer").is_some() {
            if let Some(n) = RichMetadataRowNode::from_value(val) { return Some(YTNode::RichMetadataRow(n)); }
        }
        if val.get("searchBoxRenderer").is_some() {
            if let Some(n) = SearchBoxNode::from_value(val) { return Some(YTNode::SearchBox(n)); }
        }
        if val.get("searchFilterOptionsDialogRenderer").is_some() {
            if let Some(n) = SearchFilterOptionsDialogNode::from_value(val) { return Some(YTNode::SearchFilterOptionsDialog(n)); }
        }

        // 54. Search & Settings (Batch 12)
        if val.get("searchHeaderRenderer").is_some() {
            if let Some(n) = SearchHeaderNode::from_value(val) { return Some(YTNode::SearchHeader(n)); }
        }
        if val.get("searchSuggestionRenderer").is_some() {
            if let Some(n) = SearchSuggestionNode::from_value(val) { return Some(YTNode::SearchSuggestion(n)); }
        }
        if val.get("searchSuggestionsSectionRenderer").is_some() {
            if let Some(n) = SearchSuggestionsSectionNode::from_value(val) { return Some(YTNode::SearchSuggestionsSection(n)); }
        }
        if val.get("secondarySearchContainerRenderer").is_some() {
            if let Some(n) = SecondarySearchContainerNode::from_value(val) { return Some(YTNode::SecondarySearchContainer(n)); }
        }
        if val.get("sectionHeaderView").is_some() {
            if let Some(n) = SectionHeaderViewNode::from_value(val) { return Some(YTNode::SectionHeaderView(n)); }
        }
        if val.get("segmentedLikeDislikeButtonRenderer").is_some() {
            if let Some(n) = SegmentedLikeDislikeButtonNode::from_value(val) { return Some(YTNode::SegmentedLikeDislikeButton(n)); }
        }
        if val.get("segmentedLikeDislikeButtonView").is_some() {
            if let Some(n) = SegmentedLikeDislikeButtonViewNode::from_value(val) { return Some(YTNode::SegmentedLikeDislikeButtonView(n)); }
        }
        if val.get("settingBooleanRenderer").is_some() {
            if let Some(n) = SettingBooleanNode::from_value(val) { return Some(YTNode::SettingBoolean(n)); }
        }
        if val.get("settingsCheckboxRenderer").is_some() {
            if let Some(n) = SettingsCheckboxNode::from_value(val) { return Some(YTNode::SettingsCheckbox(n)); }
        }
        if val.get("settingsOptionsRenderer").is_some() {
            if let Some(n) = SettingsOptionsNode::from_value(val) { return Some(YTNode::SettingsOptions(n)); }
        }
        if val.get("settingsSidebarRenderer").is_some() {
            if let Some(n) = SettingsSidebarNode::from_value(val) { return Some(YTNode::SettingsSidebar(n)); }
        }
        if val.get("settingsSwitchRenderer").is_some() {
            if let Some(n) = SettingsSwitchNode::from_value(val) { return Some(YTNode::SettingsSwitch(n)); }
        }

        // 55. Shares & Columns (Batch 13)
        if val.get("sharePanelHeaderRenderer").is_some() {
            if let Some(n) = SharePanelHeaderNode::from_value(val) { return Some(YTNode::SharePanelHeader(n)); }
        }
        if val.get("sharePanelTitleV15Renderer").is_some() {
            if let Some(n) = SharePanelTitleV15Node::from_value(val) { return Some(YTNode::SharePanelTitleV15(n)); }
        }
        if val.get("shareTargetRenderer").is_some() {
            if let Some(n) = ShareTargetNode::from_value(val) { return Some(YTNode::ShareTarget(n)); }
        }
        if val.get("sheetView").is_some() {
            if let Some(n) = SheetViewNode::from_value(val) { return Some(YTNode::SheetView(n)); }
        }
        if val.get("showCustomThumbnailRenderer").is_some() {
            if let Some(n) = ShowCustomThumbnailNode::from_value(val) { return Some(YTNode::ShowCustomThumbnail(n)); }
        }
        if val.get("simpleCardContentRenderer").is_some() {
            if let Some(n) = SimpleCardContentNode::from_value(val) { return Some(YTNode::SimpleCardContent(n)); }
        }
        if val.get("simpleCardTeaserRenderer").is_some() {
            if let Some(n) = SimpleCardTeaserNode::from_value(val) { return Some(YTNode::SimpleCardTeaser(n)); }
        }
        if val.get("simpleTextSectionRenderer").is_some() {
            if let Some(n) = SimpleTextSectionNode::from_value(val) { return Some(YTNode::SimpleTextSection(n)); }
        }
        if val.get("singleColumnBrowseResultsRenderer").is_some() {
            if let Some(n) = SingleColumnBrowseResultsNode::from_value(val) { return Some(YTNode::SingleColumnBrowseResults(n)); }
        }
        if val.get("singleColumnMusicWatchNextResultsRenderer").is_some() {
            if let Some(n) = SingleColumnMusicWatchNextResultsNode::from_value(val) { return Some(YTNode::SingleColumnMusicWatchNextResults(n)); }
        }
        if val.get("singleHeroImageRenderer").is_some() {
            if let Some(n) = SingleHeroImageNode::from_value(val) { return Some(YTNode::SingleHeroImage(n)); }
        }
        if val.get("slimOwnerRenderer").is_some() {
            if let Some(n) = SlimOwnerNode::from_value(val) { return Some(YTNode::SlimOwner(n)); }
        }

        // 56. Filters & Subscriptions (Batch 13)
        if val.get("sortFilterHeaderRenderer").is_some() {
            if let Some(n) = SortFilterHeaderNode::from_value(val) { return Some(YTNode::SortFilterHeader(n)); }
        }
        if val.get("sortFilterSubMenuRenderer").is_some() {
            if let Some(n) = SortFilterSubMenuNode::from_value(val) { return Some(YTNode::SortFilterSubMenu(n)); }
        }
        if val.get("startAtRenderer").is_some() {
            if let Some(n) = StartAtNode::from_value(val) { return Some(YTNode::StartAt(n)); }
        }
        if val.get("structuredDescriptionContentRenderer").is_some() {
            if let Some(n) = StructuredDescriptionContentNode::from_value(val) { return Some(YTNode::StructuredDescriptionContent(n)); }
        }
        if val.get("structuredDescriptionPlaylistLockupRenderer").is_some() {
            if let Some(n) = StructuredDescriptionPlaylistLockupNode::from_value(val) { return Some(YTNode::StructuredDescriptionPlaylistLockup(n)); }
        }
        if val.get("subFeedOptionRenderer").is_some() {
            if let Some(n) = SubFeedOptionNode::from_value(val) { return Some(YTNode::SubFeedOption(n)); }
        }
        if val.get("subFeedSelectorRenderer").is_some() {
            if let Some(n) = SubFeedSelectorNode::from_value(val) { return Some(YTNode::SubFeedSelector(n)); }
        }
        if val.get("subscribeButtonRenderer").is_some() {
            if let Some(n) = SubscribeButtonNode::from_value(val) { return Some(YTNode::SubscribeButton(n)); }
        }
        if val.get("subscribeButtonView").is_some() {
            if let Some(n) = SubscribeButtonViewNode::from_value(val) { return Some(YTNode::SubscribeButtonView(n)); }
        }
        if val.get("tabbedRenderer").is_some() {
            if let Some(n) = TabbedNode::from_value(val) { return Some(YTNode::Tabbed(n)); }
        }
        if val.get("tabbedSearchResultsRenderer").is_some() {
            if let Some(n) = TabbedSearchResultsNode::from_value(val) { return Some(YTNode::TabbedSearchResults(n)); }
        }
        if val.get("textCarouselItemView").is_some() {
            if let Some(n) = TextCarouselItemViewNode::from_value(val) { return Some(YTNode::TextCarouselItemView(n)); }
        }

        // 57. Thumbnail Overlays & Badges (Batch 13)
        if val.get("textFieldView").is_some() {
            if let Some(n) = TextFieldViewNode::from_value(val) { return Some(YTNode::TextFieldView(n)); }
        }
        if val.get("thirdPartyShareTargetSectionRenderer").is_some() {
            if let Some(n) = ThirdPartyShareTargetSectionNode::from_value(val) { return Some(YTNode::ThirdPartyShareTargetSection(n)); }
        }
        if val.get("thumbnailBadgeView").is_some() {
            if let Some(n) = ThumbnailBadgeViewNode::from_value(val) { return Some(YTNode::ThumbnailBadgeView(n)); }
        }
        if val.get("thumbnailBottomOverlayView").is_some() {
            if let Some(n) = ThumbnailBottomOverlayViewNode::from_value(val) { return Some(YTNode::ThumbnailBottomOverlayView(n)); }
        }
        if val.get("thumbnailHoverOverlayToggleActionsView").is_some() {
            if let Some(n) = ThumbnailHoverOverlayToggleActionsViewNode::from_value(val) { return Some(YTNode::ThumbnailHoverOverlayToggleActionsView(n)); }
        }
        if val.get("thumbnailHoverOverlayView").is_some() {
            if let Some(n) = ThumbnailHoverOverlayViewNode::from_value(val) { return Some(YTNode::ThumbnailHoverOverlayView(n)); }
        }
        if val.get("thumbnailLandscapePortraitRenderer").is_some() {
            if let Some(n) = ThumbnailLandscapePortraitNode::from_value(val) { return Some(YTNode::ThumbnailLandscapePortrait(n)); }
        }
        if val.get("thumbnailOverlayAvatarStackView").is_some() {
            if let Some(n) = ThumbnailOverlayAvatarStackViewNode::from_value(val) { return Some(YTNode::ThumbnailOverlayAvatarStackView(n)); }
        }
        if val.get("thumbnailOverlayBadgeView").is_some() {
            if let Some(n) = ThumbnailOverlayBadgeViewNode::from_value(val) { return Some(YTNode::ThumbnailOverlayBadgeView(n)); }
        }
        if val.get("thumbnailOverlayPinkingRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayPinkingNode::from_value(val) { return Some(YTNode::ThumbnailOverlayPinking(n)); }
        }
        if val.get("thumbnailOverlayPlaybackStatusRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayPlaybackStatusNode::from_value(val) { return Some(YTNode::ThumbnailOverlayPlaybackStatus(n)); }
        }
        if val.get("thumbnailOverlayProgressBarView").is_some() {
            if let Some(n) = ThumbnailOverlayProgressBarViewNode::from_value(val) { return Some(YTNode::ThumbnailOverlayProgressBarView(n)); }
        }

        // 58. Tickets & Transcripts (Batch 13)
        if val.get("thumbnailOverlayResumePlaybackRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayResumePlaybackNode::from_value(val) { return Some(YTNode::ThumbnailOverlayResumePlayback(n)); }
        }
        if val.get("thumbnailOverlayTitleView").is_some() {
            if let Some(n) = ThumbnailOverlayTitleViewNode::from_value(val) { return Some(YTNode::ThumbnailOverlayTitleView(n)); }
        }
        if val.get("thumbnailView").is_some() {
            if let Some(n) = ThumbnailViewNode::from_value(val) { return Some(YTNode::ThumbnailView(n)); }
        }
        if val.get("ticketEventRenderer").is_some() {
            if let Some(n) = TicketEventNode::from_value(val) { return Some(YTNode::TicketEvent(n)); }
        }
        if val.get("ticketShelfRenderer").is_some() {
            if let Some(n) = TicketShelfNode::from_value(val) { return Some(YTNode::TicketShelf(n)); }
        }
        if val.get("titleAndButtonListHeaderRenderer").is_some() {
            if let Some(n) = TitleAndButtonListHeaderNode::from_value(val) { return Some(YTNode::TitleAndButtonListHeader(n)); }
        }
        if val.get("toggleButtonView").is_some() {
            if let Some(n) = ToggleButtonViewNode::from_value(val) { return Some(YTNode::ToggleButtonView(n)); }
        }
        if val.get("toggleFormFieldRenderer").is_some() {
            if let Some(n) = ToggleFormFieldNode::from_value(val) { return Some(YTNode::ToggleFormField(n)); }
        }
        if val.get("toggleMenuServiceItemRenderer").is_some() {
            if let Some(n) = ToggleMenuServiceItemNode::from_value(val) { return Some(YTNode::ToggleMenuServiceItem(n)); }
        }
        if val.get("tooltipRenderer").is_some() {
            if let Some(n) = TooltipNode::from_value(val) { return Some(YTNode::Tooltip(n)); }
        }
        if val.get("transcriptRenderer").is_some() {
            if let Some(n) = TranscriptNode::from_value(val) { return Some(YTNode::Transcript(n)); }
        }
        if val.get("transcriptFooterRenderer").is_some() {
            if let Some(n) = TranscriptFooterNode::from_value(val) { return Some(YTNode::TranscriptFooter(n)); }
        }

        // 59. Transcripts & Watch Columns (Batch 14)
        if val.get("transcriptSearchBoxRenderer").is_some() {
            if let Some(n) = TranscriptSearchBoxNode::from_value(val) { return Some(YTNode::TranscriptSearchBox(n)); }
        }
        if val.get("transcriptSearchPanelRenderer").is_some() {
            if let Some(n) = TranscriptSearchPanelNode::from_value(val) { return Some(YTNode::TranscriptSearchPanel(n)); }
        }
        if val.get("transcriptSectionHeaderRenderer").is_some() {
            if let Some(n) = TranscriptSectionHeaderNode::from_value(val) { return Some(YTNode::TranscriptSectionHeader(n)); }
        }
        if val.get("transcriptSegmentRenderer").is_some() {
            if let Some(n) = TranscriptSegmentNode::from_value(val) { return Some(YTNode::TranscriptSegment(n)); }
        }
        if val.get("transcriptSegmentListRenderer").is_some() {
            if let Some(n) = TranscriptSegmentListNode::from_value(val) { return Some(YTNode::TranscriptSegmentList(n)); }
        }
        if val.get("twoColumnBrowseResultsRenderer").is_some() {
            if let Some(n) = TwoColumnBrowseResultsNode::from_value(val) { return Some(YTNode::TwoColumnBrowseResults(n)); }
        }
        if val.get("twoColumnSearchResultsRenderer").is_some() {
            if let Some(n) = TwoColumnSearchResultsNode::from_value(val) { return Some(YTNode::TwoColumnSearchResults(n)); }
        }
        if val.get("twoColumnWatchNextResults").is_some() {
            if let Some(n) = TwoColumnWatchNextResultsNode::from_value(val) { return Some(YTNode::TwoColumnWatchNextResults(n)); }
        }
        if val.get("unifiedSharePanelRenderer").is_some() {
            if let Some(n) = UnifiedSharePanelNode::from_value(val) { return Some(YTNode::UnifiedSharePanel(n)); }
        }
        if val.get("universalWatchCardRenderer").is_some() {
            if let Some(n) = UniversalWatchCardNode::from_value(val) { return Some(YTNode::UniversalWatchCard(n)); }
        }
        if val.get("uploadTimeFactoidRenderer").is_some() {
            if let Some(n) = UploadTimeFactoidNode::from_value(val) { return Some(YTNode::UploadTimeFactoid(n)); }
        }
        if val.get("upsellDialogRenderer").is_some() {
            if let Some(n) = UpsellDialogNode::from_value(val) { return Some(YTNode::UpsellDialog(n)); }
        }

        // 60. Video Sections & Watch Cards (Batch 14)
        if val.get("verticalWatchCardListRenderer").is_some() {
            if let Some(n) = VerticalWatchCardListNode::from_value(val) { return Some(YTNode::VerticalWatchCardList(n)); }
        }
        if val.get("videoAttributesSectionView").is_some() {
            if let Some(n) = VideoAttributesSectionViewNode::from_value(val) { return Some(YTNode::VideoAttributesSectionView(n)); }
        }
        if val.get("videoDescriptionCourseSectionRenderer").is_some() {
            if let Some(n) = VideoDescriptionCourseSectionNode::from_value(val) { return Some(YTNode::VideoDescriptionCourseSection(n)); }
        }
        if val.get("videoDescriptionInfocardsSectionRenderer").is_some() {
            if let Some(n) = VideoDescriptionInfocardsSectionNode::from_value(val) { return Some(YTNode::VideoDescriptionInfocardsSection(n)); }
        }
        if val.get("videoDescriptionMusicSectionRenderer").is_some() {
            if let Some(n) = VideoDescriptionMusicSectionNode::from_value(val) { return Some(YTNode::VideoDescriptionMusicSection(n)); }
        }
        if val.get("videoDescriptionTranscriptSectionRenderer").is_some() {
            if let Some(n) = VideoDescriptionTranscriptSectionNode::from_value(val) { return Some(YTNode::VideoDescriptionTranscriptSection(n)); }
        }
        if val.get("videoDescriptionYouchatSectionView").is_some() {
            if let Some(n) = VideoDescriptionYouchatSectionViewNode::from_value(val) { return Some(YTNode::VideoDescriptionYouchatSectionView(n)); }
        }
        if val.get("videoMetadataCarouselView").is_some() {
            if let Some(n) = VideoMetadataCarouselViewNode::from_value(val) { return Some(YTNode::VideoMetadataCarouselView(n)); }
        }
        if val.get("viewCountFactoidRenderer").is_some() {
            if let Some(n) = ViewCountFactoidNode::from_value(val) { return Some(YTNode::ViewCountFactoid(n)); }
        }
        if val.get("watchCardRichHeaderRenderer").is_some() {
            if let Some(n) = WatchCardRichHeaderNode::from_value(val) { return Some(YTNode::WatchCardRichHeader(n)); }
        }
        if val.get("watchCardSectionSequenceRenderer").is_some() {
            if let Some(n) = WatchCardSectionSequenceNode::from_value(val) { return Some(YTNode::WatchCardSectionSequence(n)); }
        }
        if val.get("watchNextEndScreenRenderer").is_some() {
            if let Some(n) = WatchNextEndScreenNode::from_value(val) { return Some(YTNode::WatchNextEndScreen(n)); }
        }

        // 61. Comments & Endpoints Extended (Batch 14)
        if val.get("watchNextTabbedResultsRenderer").is_some() {
            if let Some(n) = WatchNextTabbedResultsNode::from_value(val) { return Some(YTNode::WatchNextTabbedResults(n)); }
        }
        if val.get("ypcTrailerRenderer").is_some() {
            if let Some(n) = YpcTrailerNode::from_value(val) { return Some(YTNode::YpcTrailer(n)); }
        }
        if val.get("commandExecutorCommand").is_some() {
            if let Some(n) = CommandExecutorCommandNode::from_value(val) { return Some(YTNode::CommandExecutorCommand(n)); }
        }
        if val.get("getKidsBlocklistPickerCommand").is_some() {
            if let Some(n) = GetKidsBlocklistPickerCommandNode::from_value(val) { return Some(YTNode::GetKidsBlocklistPickerCommand(n)); }
        }
        if val.get("showDialogCommand").is_some() {
            if let Some(n) = ShowDialogCommandNode::from_value(val) { return Some(YTNode::ShowDialogCommand(n)); }
        }
        if val.get("commentDialogRenderer").is_some() {
            if let Some(n) = CommentDialogNode::from_value(val) { return Some(YTNode::CommentDialog(n)); }
        }
        if val.get("commentReplyDialogRenderer").is_some() {
            if let Some(n) = CommentReplyDialogNode::from_value(val) { return Some(YTNode::CommentReplyDialog(n)); }
        }
        if val.get("emojiPickerRenderer").is_some() {
            if let Some(n) = EmojiPickerNode::from_value(val) { return Some(YTNode::EmojiPicker(n)); }
        }
        if val.get("voiceReplyContainerView").is_some() {
            if let Some(n) = VoiceReplyContainerViewNode::from_value(val) { return Some(YTNode::VoiceReplyContainerView(n)); }
        }
        if val.get("addToPlaylistEndpoint").is_some() {
            if let Some(n) = AddToPlaylistEndpointNode::from_value(val) { return Some(YTNode::AddToPlaylistEndpoint(n)); }
        }
        if val.get("addToPlaylistServiceEndpoint").is_some() {
            if let Some(n) = AddToPlaylistServiceEndpointNode::from_value(val) { return Some(YTNode::AddToPlaylistServiceEndpoint(n)); }
        }
        if val.get("createCommentEndpoint").is_some() {
            if let Some(n) = CreateCommentEndpointNode::from_value(val) { return Some(YTNode::CreateCommentEndpoint(n)); }
        }

        // 62. Endpoint Commands (Batch 14)
        if val.get("createPlaylistServiceEndpoint").is_some() {
            if let Some(n) = CreatePlaylistServiceEndpointNode::from_value(val) { return Some(YTNode::CreatePlaylistServiceEndpoint(n)); }
        }
        if val.get("deletePlaylistEndpoint").is_some() {
            if let Some(n) = DeletePlaylistEndpointNode::from_value(val) { return Some(YTNode::DeletePlaylistEndpoint(n)); }
        }
        if val.get("feedbackEndpoint").is_some() {
            if let Some(n) = FeedbackEndpointNode::from_value(val) { return Some(YTNode::FeedbackEndpoint(n)); }
        }
        if val.get("getAccountsListInnertubeEndpoint").is_some() {
            if let Some(n) = GetAccountsListInnertubeEndpointNode::from_value(val) { return Some(YTNode::GetAccountsListInnertubeEndpoint(n)); }
        }
        if val.get("hideEngagementPanelEndpoint").is_some() {
            if let Some(n) = HideEngagementPanelEndpointNode::from_value(val) { return Some(YTNode::HideEngagementPanelEndpoint(n)); }
        }
        if val.get("liveChatItemContextMenuEndpoint").is_some() {
            if let Some(n) = LiveChatItemContextMenuEndpointNode::from_value(val) { return Some(YTNode::LiveChatItemContextMenuEndpoint(n)); }
        }
        if val.get("modifyChannelNotificationPreferenceEndpoint").is_some() {
            if let Some(n) = ModifyChannelNotificationPreferenceEndpointNode::from_value(val) { return Some(YTNode::ModifyChannelNotificationPreferenceEndpoint(n)); }
        }
        if val.get("performCommentActionEndpoint").is_some() {
            if let Some(n) = PerformCommentActionEndpointNode::from_value(val) { return Some(YTNode::PerformCommentActionEndpoint(n)); }
        }
        if val.get("playlistEditEndpoint").is_some() {
            if let Some(n) = PlaylistEditEndpointNode::from_value(val) { return Some(YTNode::PlaylistEditEndpoint(n)); }
        }
        if val.get("prefetchWatchCommand").is_some() {
            if let Some(n) = PrefetchWatchCommandNode::from_value(val) { return Some(YTNode::PrefetchWatchCommand(n)); }
        }
        if val.get("shareEndpoint").is_some() {
            if let Some(n) = ShareEndpointNode::from_value(val) { return Some(YTNode::ShareEndpoint(n)); }
        }
        if val.get("shareEntityEndpoint").is_some() {
            if let Some(n) = ShareEntityEndpointNode::from_value(val) { return Some(YTNode::ShareEntityEndpoint(n)); }
        }

        // 63. Menus & Mobile Topbar (Batch 15)
        if val.get("menuFlexibleItemRenderer").is_some() {
            if let Some(n) = MenuFlexibleItemNode::from_value(val) { return Some(YTNode::MenuFlexibleItem(n)); }
        }
        if val.get("menuNavigationItemRenderer").is_some() {
            if let Some(n) = MenuNavigationItemNode::from_value(val) { return Some(YTNode::MenuNavigationItem(n)); }
        }
        if val.get("menuPopupRenderer").is_some() {
            if let Some(n) = MenuPopupNode::from_value(val) { return Some(YTNode::MenuPopup(n)); }
        }
        if val.get("menuServiceItemRenderer").is_some() {
            if let Some(n) = MenuServiceItemNode::from_value(val) { return Some(YTNode::MenuServiceItem(n)); }
        }
        if val.get("menuServiceItemDownloadRenderer").is_some() {
            if let Some(n) = MenuServiceItemDownloadNode::from_value(val) { return Some(YTNode::MenuServiceItemDownload(n)); }
        }
        if val.get("multiPageMenuRenderer").is_some() {
            if let Some(n) = MultiPageMenuNode::from_value(val) { return Some(YTNode::MultiPageMenu(n)); }
        }
        if val.get("multiPageMenuNotificationSectionRenderer").is_some() {
            if let Some(n) = MultiPageMenuNotificationSectionNode::from_value(val) { return Some(YTNode::MultiPageMenuNotificationSection(n)); }
        }
        if val.get("simpleMenuHeaderRenderer").is_some() {
            if let Some(n) = SimpleMenuHeaderNode::from_value(val) { return Some(YTNode::SimpleMenuHeader(n)); }
        }
        if val.get("mobileTopbarRenderer").is_some() {
            if let Some(n) = MobileTopbarNode::from_value(val) { return Some(YTNode::MobileTopbar(n)); }
        }
        if val.get("multiPageMenuSectionRenderer").is_some() {
            if let Some(n) = MultiPageMenuSectionNode::from_value(val) { return Some(YTNode::MultiPageMenuSection(n)); }
        }
        if val.get("pivotBarRenderer").is_some() {
            if let Some(n) = PivotBarNode::from_value(val) { return Some(YTNode::PivotBar(n)); }
        }
        if val.get("pivotBarItemRenderer").is_some() {
            if let Some(n) = PivotBarItemNode::from_value(val) { return Some(YTNode::PivotBarItem(n)); }
        }
        if val.get("topbarMenuButtonRenderer").is_some() {
            if let Some(n) = TopbarMenuButtonNode::from_value(val) { return Some(YTNode::TopbarMenuButton(n)); }
        }

        // 64. Livechat Actions & Collaboration (Batch 15)
        if val.get("replaceLiveChatAction").is_some() {
            if let Some(n) = ReplaceLiveChatActionNode::from_value(val) { return Some(YTNode::ReplaceLiveChatAction(n)); }
        }
        if val.get("updateDateTextAction").is_some() {
            if let Some(n) = UpdateDateTextActionNode::from_value(val) { return Some(YTNode::UpdateDateTextAction(n)); }
        }
        if val.get("updateDescriptionAction").is_some() {
            if let Some(n) = UpdateDescriptionActionNode::from_value(val) { return Some(YTNode::UpdateDescriptionAction(n)); }
        }
        if val.get("updateTitleAction").is_some() {
            if let Some(n) = UpdateTitleActionNode::from_value(val) { return Some(YTNode::UpdateTitleAction(n)); }
        }
        if val.get("updateToggleButtonTextAction").is_some() {
            if let Some(n) = UpdateToggleButtonTextActionNode::from_value(val) { return Some(YTNode::UpdateToggleButtonTextAction(n)); }
        }
        if val.get("updateViewershipAction").is_some() {
            if let Some(n) = UpdateViewershipActionNode::from_value(val) { return Some(YTNode::UpdateViewershipAction(n)); }
        }
        if val.get("bumperUserEduContentView").is_some() {
            if let Some(n) = BumperUserEduContentViewNode::from_value(val) { return Some(YTNode::BumperUserEduContentView(n)); }
        }
        if val.get("pdgReplyButtonView").is_some() {
            if let Some(n) = PdgReplyButtonViewNode::from_value(val) { return Some(YTNode::PdgReplyButtonView(n)); }
        }
        if val.get("playlistCollaborationFormSchema").is_some() {
            if let Some(n) = PlaylistCollaborationFormSchemaNode::from_value(val) { return Some(YTNode::PlaylistCollaborationFormSchema(n)); }
        }
        if val.get("playlistCollaborationViewModelPlaylistCollaboratorData").is_some() {
            if let Some(n) = PlaylistCollaborationViewModelPlaylistCollaboratorDataNode::from_value(val) { return Some(YTNode::PlaylistCollaborationViewModelPlaylistCollaboratorData(n)); }
        }
        if val.get("subscriptionButton").is_some() {
            if let Some(n) = SubscriptionButtonNode::from_value(val) { return Some(YTNode::SubscriptionButton(n)); }
        }
        if val.get("commandContext").is_some() {
            if let Some(n) = CommandContextNode::from_value(val) { return Some(YTNode::CommandContext(n)); }
        }

        // 65. Endpoints Primitives & Kids (Batch 15)
        if val.get("shareEntityServiceEndpoint").is_some() {
            if let Some(n) = ShareEntityServiceEndpointNode::from_value(val) { return Some(YTNode::ShareEntityServiceEndpoint(n)); }
        }
        if val.get("signalServiceEndpoint").is_some() {
            if let Some(n) = SignalServiceEndpointNode::from_value(val) { return Some(YTNode::SignalServiceEndpoint(n)); }
        }
        if val.get("unsubscribeEndpoint").is_some() {
            if let Some(n) = UnsubscribeEndpointNode::from_value(val) { return Some(YTNode::UnsubscribeEndpoint(n)); }
        }
        if val.get("watchNextEndpoint").is_some() {
            if let Some(n) = WatchNextEndpointNode::from_value(val) { return Some(YTNode::WatchNextEndpoint(n)); }
        }
        if val.get("accessibilityContext").is_some() {
            if let Some(n) = AccessibilityContextNode::from_value(val) { return Some(YTNode::AccessibilityContext(n)); }
        }
        if val.get("accessibilityData").is_some() {
            if let Some(n) = AccessibilityDataNode::from_value(val) { return Some(YTNode::AccessibilityData(n)); }
        }
        if val.get("childElement").is_some() {
            if let Some(n) = ChildElementNode::from_value(val) { return Some(YTNode::ChildElement(n)); }
        }
        if val.get("emojiRun").is_some() {
            if let Some(n) = EmojiRunNode::from_value(val) { return Some(YTNode::EmojiRun(n)); }
        }
        if val.get("rendererContext").is_some() {
            if let Some(n) = RendererContextNode::from_value(val) { return Some(YTNode::RendererContext(n)); }
        }
        if val.get("anchoredSectionRenderer").is_some() {
            if let Some(n) = AnchoredSectionNode::from_value(val) { return Some(YTNode::AnchoredSection(n)); }
        }
        if val.get("kidsBlocklistPickerRenderer").is_some() {
            if let Some(n) = KidsBlocklistPickerNode::from_value(val) { return Some(YTNode::KidsBlocklistPicker(n)); }
        }
        if val.get("kidsBlocklistPickerItemRenderer").is_some() {
            if let Some(n) = KidsBlocklistPickerItemNode::from_value(val) { return Some(YTNode::KidsBlocklistPickerItem(n)); }
        }

        // 66. Direct Primitives & Core Variants (Batch 15)
        if val.get("musicResponsiveListItemRenderer").is_some() {
            if let Some(n) = MusicResponsiveListItemNode::from_value(val) { return Some(YTNode::MusicResponsiveListItem(n)); }
        }
        if val.get("navigationEndpoint").is_some() {
            if let Some(n) = NavigationEndpointNode::from_value(val) { return Some(YTNode::NavigationEndpoint(n)); }
        }
        if val.get("thumbnailOverlayTimeStatusRenderer").is_some() {
            if let Some(n) = ThumbnailOverlayTimeStatusNode::from_value(val) { return Some(YTNode::ThumbnailOverlayTimeStatus(n)); }
        }
        if val.get("browseEndpoint").is_some() {
            if let Some(n) = BrowseEndpointNode::from_value(val) { return Some(YTNode::BrowseEndpoint(n)); }
        }
        if val.get("likeEndpoint").is_some() {
            if let Some(n) = LikeEndpointNode::from_value(val) { return Some(YTNode::LikeEndpoint(n)); }
        }
        if val.get("reelWatchEndpoint").is_some() {
            if let Some(n) = ReelWatchEndpointNode::from_value(val) { return Some(YTNode::ReelWatchEndpoint(n)); }
        }
        if val.get("searchEndpoint").is_some() {
            if let Some(n) = SearchEndpointNode::from_value(val) { return Some(YTNode::SearchEndpoint(n)); }
        }
        if val.get("subscribeEndpoint").is_some() {
            if let Some(n) = SubscribeEndpointNode::from_value(val) { return Some(YTNode::SubscribeEndpoint(n)); }
        }
        if val.get("watchEndpoint").is_some() {
            if let Some(n) = WatchEndpointNode::from_value(val) { return Some(YTNode::WatchEndpoint(n)); }
        }
        if val.get("author").is_some() {
            if let Some(n) = AuthorNode::from_value(val) { return Some(YTNode::Author(n)); }
        }
        if val.get("textRun").is_some() {
            if let Some(n) = TextRunNode::from_value(val) { return Some(YTNode::TextRun(n)); }
        }
        if val.get("text").is_some() || val.get("simpleText").is_some() || val.get("runs").is_some() {
            if let Some(n) = TextNode::from_value(val) { return Some(YTNode::Text(n)); }
        }
        if val.get("thumbnail").is_some() || val.get("thumbnails").is_some() {
            if let Some(n) = ThumbnailNode::from_value(val) { return Some(YTNode::Thumbnail(n)); }
        }

        // 67. Full 574 Unique AST Nodes (Phase 16)
        if val.get("browserMediaSession").is_some() {
            if let Some(n) = BrowserMediaSessionNode::from_value(val) { return Some(YTNode::BrowserMediaSession(n)); }
        }
        if val.get("channelVideoPlayerRenderer").is_some() {
            if let Some(n) = ChannelVideoPlayerNode::from_value(val) { return Some(YTNode::ChannelVideoPlayer(n)); }
        }
        if val.get("childVideoRenderer").is_some() {
            if let Some(n) = ChildVideoNode::from_value(val) { return Some(YTNode::ChildVideo(n)); }
        }
        if val.get("endScreenVideoRenderer").is_some() {
            if let Some(n) = EndScreenVideoNode::from_value(val) { return Some(YTNode::EndScreenVideo(n)); }
        }
        if val.get("expandableVideoDescriptionBodyRenderer").is_some() {
            if let Some(n) = ExpandableVideoDescriptionBodyNode::from_value(val) { return Some(YTNode::ExpandableVideoDescriptionBody(n)); }
        }
        if val.get("playerAnnotationsExpandedRenderer").is_some() {
            if let Some(n) = PlayerAnnotationsExpandedNode::from_value(val) { return Some(YTNode::PlayerAnnotationsExpanded(n)); }
        }
        if val.get("playerCaptchaView").is_some() {
            if let Some(n) = PlayerCaptchaViewNode::from_value(val) { return Some(YTNode::PlayerCaptchaView(n)); }
        }
        if val.get("playerControlsOverlayRenderer").is_some() {
            if let Some(n) = PlayerControlsOverlayNode::from_value(val) { return Some(YTNode::PlayerControlsOverlay(n)); }
        }
        if val.get("playerLegacyDesktopYpcOfferRenderer").is_some() {
            if let Some(n) = PlayerLegacyDesktopYpcOfferNode::from_value(val) { return Some(YTNode::PlayerLegacyDesktopYpcOffer(n)); }
        }
        if val.get("playerMicroformatRenderer").is_some() {
            if let Some(n) = PlayerMicroformatNode::from_value(val) { return Some(YTNode::PlayerMicroformat(n)); }
        }
        if val.get("playerOverflowRenderer").is_some() {
            if let Some(n) = PlayerOverflowNode::from_value(val) { return Some(YTNode::PlayerOverflow(n)); }
        }
        if val.get("playerOverlayAutoplayRenderer").is_some() {
            if let Some(n) = PlayerOverlayAutoplayNode::from_value(val) { return Some(YTNode::PlayerOverlayAutoplay(n)); }
        }
        if val.get("playerOverlayVideoDetailsRenderer").is_some() {
            if let Some(n) = PlayerOverlayVideoDetailsNode::from_value(val) { return Some(YTNode::PlayerOverlayVideoDetails(n)); }
        }
        if val.get("slimVideoMetadataRenderer").is_some() {
            if let Some(n) = SlimVideoMetadataNode::from_value(val) { return Some(YTNode::SlimVideoMetadata(n)); }
        }
        if val.get("videoAttributeView").is_some() {
            if let Some(n) = VideoAttributeViewNode::from_value(val) { return Some(YTNode::VideoAttributeView(n)); }
        }
        if val.get("videoCardRenderer").is_some() {
            if let Some(n) = VideoCardNode::from_value(val) { return Some(YTNode::VideoCard(n)); }
        }
        if val.get("videoDescriptionHeaderRenderer").is_some() {
            if let Some(n) = VideoDescriptionHeaderNode::from_value(val) { return Some(YTNode::VideoDescriptionHeader(n)); }
        }
        if val.get("videoInfoCardContentRenderer").is_some() {
            if let Some(n) = VideoInfoCardContentNode::from_value(val) { return Some(YTNode::VideoInfoCardContent(n)); }
        }
        if val.get("videoSummaryContentView").is_some() {
            if let Some(n) = VideoSummaryContentViewNode::from_value(val) { return Some(YTNode::VideoSummaryContentView(n)); }
        }
        if val.get("videoSummaryParagraphView").is_some() {
            if let Some(n) = VideoSummaryParagraphViewNode::from_value(val) { return Some(YTNode::VideoSummaryParagraphView(n)); }
        }
        if val.get("watchCardCompactVideoRenderer").is_some() {
            if let Some(n) = WatchCardCompactVideoNode::from_value(val) { return Some(YTNode::WatchCardCompactVideo(n)); }
        }
        if val.get("watchCardHeroVideoRenderer").is_some() {
            if let Some(n) = WatchCardHeroVideoNode::from_value(val) { return Some(YTNode::WatchCardHeroVideo(n)); }
        }
        if val.get("format").is_some() {
            if let Some(n) = FormatNode::from_value(val) { return Some(YTNode::Format(n)); }
        }
        if val.get("videoDetails").is_some() {
            if let Some(n) = VideoDetailsNode::from_value(val) { return Some(YTNode::VideoDetails(n)); }
        }
        if val.get("liveChatAuthorBadgeRenderer").is_some() {
            if let Some(n) = LiveChatAuthorBadgeNode::from_value(val) { return Some(YTNode::LiveChatAuthorBadge(n)); }
        }
        if val.get("liveChatHeaderRenderer").is_some() {
            if let Some(n) = LiveChatHeaderNode::from_value(val) { return Some(YTNode::LiveChatHeader(n)); }
        }
        if val.get("liveChatMessageInputRenderer").is_some() {
            if let Some(n) = LiveChatMessageInputNode::from_value(val) { return Some(YTNode::LiveChatMessageInput(n)); }
        }
        if val.get("liveChatParticipantRenderer").is_some() {
            if let Some(n) = LiveChatParticipantNode::from_value(val) { return Some(YTNode::LiveChatParticipant(n)); }
        }
        if val.get("liveChatBannerChatSummaryRenderer").is_some() {
            if let Some(n) = LiveChatBannerChatSummaryNode::from_value(val) { return Some(YTNode::LiveChatBannerChatSummary(n)); }
        }
        if val.get("liveChatBannerHeaderRenderer").is_some() {
            if let Some(n) = LiveChatBannerHeaderNode::from_value(val) { return Some(YTNode::LiveChatBannerHeader(n)); }
        }
        if val.get("liveChatBannerRedirectRenderer").is_some() {
            if let Some(n) = LiveChatBannerRedirectNode::from_value(val) { return Some(YTNode::LiveChatBannerRedirect(n)); }
        }
        if val.get("liveChatItemBumperView").is_some() {
            if let Some(n) = LiveChatItemBumperViewNode::from_value(val) { return Some(YTNode::LiveChatItemBumperView(n)); }
        }
        if val.get("liveChatPaidMessageRenderer").is_some() {
            if let Some(n) = LiveChatPaidMessageNode::from_value(val) { return Some(YTNode::LiveChatPaidMessage(n)); }
        }
        if val.get("liveChatPlaceholderItemRenderer").is_some() {
            if let Some(n) = LiveChatPlaceholderItemNode::from_value(val) { return Some(YTNode::LiveChatPlaceholderItem(n)); }
        }
        if val.get("liveChatProductItemRenderer").is_some() {
            if let Some(n) = LiveChatProductItemNode::from_value(val) { return Some(YTNode::LiveChatProductItem(n)); }
        }
        if val.get("liveChatRestrictedParticipationRenderer").is_some() {
            if let Some(n) = LiveChatRestrictedParticipationNode::from_value(val) { return Some(YTNode::LiveChatRestrictedParticipation(n)); }
        }
        if val.get("liveChatSponsorshipsGiftPurchaseAnnouncementRenderer").is_some() {
            if let Some(n) = LiveChatSponsorshipsGiftPurchaseAnnouncementNode::from_value(val) { return Some(YTNode::LiveChatSponsorshipsGiftPurchaseAnnouncement(n)); }
        }
        if val.get("liveChatSponsorshipsGiftRedemptionAnnouncementRenderer").is_some() {
            if let Some(n) = LiveChatSponsorshipsGiftRedemptionAnnouncementNode::from_value(val) { return Some(YTNode::LiveChatSponsorshipsGiftRedemptionAnnouncement(n)); }
        }
        if val.get("liveChatSponsorshipsHeaderRenderer").is_some() {
            if let Some(n) = LiveChatSponsorshipsHeaderNode::from_value(val) { return Some(YTNode::LiveChatSponsorshipsHeader(n)); }
        }
        if val.get("liveChatTextMessageRenderer").is_some() {
            if let Some(n) = LiveChatTextMessageNode::from_value(val) { return Some(YTNode::LiveChatTextMessage(n)); }
        }
        if val.get("liveChatTickerPaidMessageItemRenderer").is_some() {
            if let Some(n) = LiveChatTickerPaidMessageItemNode::from_value(val) { return Some(YTNode::LiveChatTickerPaidMessageItem(n)); }
        }
        if val.get("liveChatTickerPaidStickerItemRenderer").is_some() {
            if let Some(n) = LiveChatTickerPaidStickerItemNode::from_value(val) { return Some(YTNode::LiveChatTickerPaidStickerItem(n)); }
        }
        if val.get("liveChatTickerSponsorItemRenderer").is_some() {
            if let Some(n) = LiveChatTickerSponsorItemNode::from_value(val) { return Some(YTNode::LiveChatTickerSponsorItem(n)); }
        }
        if val.get("showLiveChatActionPanelAction").is_some() {
            if let Some(n) = ShowLiveChatActionPanelActionNode::from_value(val) { return Some(YTNode::ShowLiveChatActionPanelAction(n)); }
        }
        if val.get("showLiveChatDialogAction").is_some() {
            if let Some(n) = ShowLiveChatDialogActionNode::from_value(val) { return Some(YTNode::ShowLiveChatDialogAction(n)); }
        }
        if val.get("showLiveChatTooltipCommand").is_some() {
            if let Some(n) = ShowLiveChatTooltipCommandNode::from_value(val) { return Some(YTNode::ShowLiveChatTooltipCommand(n)); }
        }
        if val.get("markChatItemsByAuthorAsDeletedAction").is_some() {
            if let Some(n) = MarkChatItemsByAuthorAsDeletedActionNode::from_value(val) { return Some(YTNode::MarkChatItemsByAuthorAsDeletedAction(n)); }
        }
        if val.get("liveChatBannerPollRenderer").is_some() {
            if let Some(n) = LiveChatBannerPollNode::from_value(val) { return Some(YTNode::LiveChatBannerPoll(n)); }
        }
        if val.get("aboutChannelRenderer").is_some() {
            if let Some(n) = AboutChannelNode::from_value(val) { return Some(YTNode::AboutChannel(n)); }
        }
        if val.get("aboutChannelViewModel").is_some() {
            if let Some(n) = AboutChannelViewNode::from_value(val) { return Some(YTNode::AboutChannelView(n)); }
        }
        if val.get("accountChannelRenderer").is_some() {
            if let Some(n) = AccountChannelNode::from_value(val) { return Some(YTNode::AccountChannel(n)); }
        }
        if val.get("channelRenderer").is_some() {
            if let Some(n) = ChannelNode::from_value(val) { return Some(YTNode::Channel(n)); }
        }
        if val.get("channelAgeGateRenderer").is_some() {
            if let Some(n) = ChannelAgeGateNode::from_value(val) { return Some(YTNode::ChannelAgeGate(n)); }
        }
        if val.get("channelExternalLinkViewModel").is_some() {
            if let Some(n) = ChannelExternalLinkViewNode::from_value(val) { return Some(YTNode::ChannelExternalLinkView(n)); }
        }
        if val.get("channelFeaturedContentRenderer").is_some() {
            if let Some(n) = ChannelFeaturedContentNode::from_value(val) { return Some(YTNode::ChannelFeaturedContent(n)); }
        }
        if val.get("channelOptionsRenderer").is_some() {
            if let Some(n) = ChannelOptionsNode::from_value(val) { return Some(YTNode::ChannelOptions(n)); }
        }
        if val.get("channelTaglineRenderer").is_some() {
            if let Some(n) = ChannelTaglineNode::from_value(val) { return Some(YTNode::ChannelTagline(n)); }
        }
        if val.get("channelThumbnailWithLinkRenderer").is_some() {
            if let Some(n) = ChannelThumbnailWithLinkNode::from_value(val) { return Some(YTNode::ChannelThumbnailWithLink(n)); }
        }
        if val.get("topicChannelDetailsRenderer").is_some() {
            if let Some(n) = TopicChannelDetailsNode::from_value(val) { return Some(YTNode::TopicChannelDetails(n)); }
        }
        if val.get("activeAccountHeaderRenderer").is_some() {
            if let Some(n) = ActiveAccountHeaderNode::from_value(val) { return Some(YTNode::ActiveAccountHeader(n)); }
        }
        if val.get("channelHeaderLinksRenderer").is_some() {
            if let Some(n) = ChannelHeaderLinksNode::from_value(val) { return Some(YTNode::ChannelHeaderLinks(n)); }
        }
        if val.get("channelHeaderLinksViewModel").is_some() {
            if let Some(n) = ChannelHeaderLinksViewNode::from_value(val) { return Some(YTNode::ChannelHeaderLinksView(n)); }
        }
        if val.get("channelMobileHeaderRenderer").is_some() {
            if let Some(n) = ChannelMobileHeaderNode::from_value(val) { return Some(YTNode::ChannelMobileHeader(n)); }
        }
        if val.get("channelSwitcherHeaderRenderer").is_some() {
            if let Some(n) = ChannelSwitcherHeaderNode::from_value(val) { return Some(YTNode::ChannelSwitcherHeader(n)); }
        }
        if val.get("authorCommentBadgeRenderer").is_some() {
            if let Some(n) = AuthorCommentBadgeNode::from_value(val) { return Some(YTNode::AuthorCommentBadge(n)); }
        }
        if val.get("commentRepliesRenderer").is_some() {
            if let Some(n) = CommentRepliesNode::from_value(val) { return Some(YTNode::CommentReplies(n)); }
        }
        if val.get("commentViewModel").is_some() {
            if let Some(n) = CommentViewNode::from_value(val) { return Some(YTNode::CommentView(n)); }
        }
        if val.get("commentsEntryPointTeaserRenderer").is_some() {
            if let Some(n) = CommentsEntryPointTeaserNode::from_value(val) { return Some(YTNode::CommentsEntryPointTeaser(n)); }
        }
        if val.get("commentsSimpleboxRenderer").is_some() {
            if let Some(n) = CommentsSimpleboxNode::from_value(val) { return Some(YTNode::CommentsSimplebox(n)); }
        }
        if val.get("pdgCommentChipRenderer").is_some() {
            if let Some(n) = PdgCommentChipNode::from_value(val) { return Some(YTNode::PdgCommentChip(n)); }
        }
        if val.get("sponsorCommentBadgeRenderer").is_some() {
            if let Some(n) = SponsorCommentBadgeNode::from_value(val) { return Some(YTNode::SponsorCommentBadge(n)); }
        }
        if val.get("commentsContinuation").is_some() {
            if let Some(n) = CommentsContinuationNode::from_value(val) { return Some(YTNode::CommentsContinuation(n)); }
        }
        if val.get("musicDownloadStateBadgeRenderer").is_some() {
            if let Some(n) = MusicDownloadStateBadgeNode::from_value(val) { return Some(YTNode::MusicDownloadStateBadge(n)); }
        }
        if val.get("musicElementHeaderRenderer").is_some() {
            if let Some(n) = MusicElementHeaderNode::from_value(val) { return Some(YTNode::MusicElementHeader(n)); }
        }
        if val.get("musicSortFilterButtonRenderer").is_some() {
            if let Some(n) = MusicSortFilterButtonNode::from_value(val) { return Some(YTNode::MusicSortFilterButton(n)); }
        }
        if val.get("musicThumbnailRenderer").is_some() {
            if let Some(n) = MusicThumbnailNode::from_value(val) { return Some(YTNode::MusicThumbnail(n)); }
        }
        if val.get("musicMenuItemDividerRenderer").is_some() {
            if let Some(n) = MusicMenuItemDividerNode::from_value(val) { return Some(YTNode::MusicMenuItemDivider(n)); }
        }
        if val.get("musicMultiSelectMenuRenderer").is_some() {
            if let Some(n) = MusicMultiSelectMenuNode::from_value(val) { return Some(YTNode::MusicMultiSelectMenu(n)); }
        }
        if val.get("musicMultiSelectMenuItemRenderer").is_some() {
            if let Some(n) = MusicMultiSelectMenuItemNode::from_value(val) { return Some(YTNode::MusicMultiSelectMenuItem(n)); }
        }
        if val.get("backstagePostRenderer").is_some() {
            if let Some(n) = BackstagePostNode::from_value(val) { return Some(YTNode::BackstagePost(n)); }
        }
        if val.get("backstagePostThreadRenderer").is_some() {
            if let Some(n) = BackstagePostThreadNode::from_value(val) { return Some(YTNode::BackstagePostThread(n)); }
        }
        if val.get("sharedPostRenderer").is_some() {
            if let Some(n) = SharedPostNode::from_value(val) { return Some(YTNode::SharedPost(n)); }
        }
        if val.get("reelItemRenderer").is_some() {
            if let Some(n) = ReelItemNode::from_value(val) { return Some(YTNode::ReelItem(n)); }
        }
        if val.get("reelPlayerHeaderRenderer").is_some() {
            if let Some(n) = ReelPlayerHeaderNode::from_value(val) { return Some(YTNode::ReelPlayerHeader(n)); }
        }
        if val.get("reelPlayerOverlayRenderer").is_some() {
            if let Some(n) = ReelPlayerOverlayNode::from_value(val) { return Some(YTNode::ReelPlayerOverlay(n)); }
        }
        if val.get("shortsLockupViewModel").is_some() {
            if let Some(n) = ShortsLockupViewNode::from_value(val) { return Some(YTNode::ShortsLockupView(n)); }
        }
        if val.get("alertWithButtonRenderer").is_some() {
            if let Some(n) = AlertWithButtonNode::from_value(val) { return Some(YTNode::AlertWithButton(n)); }
        }
        if val.get("compositeVideoPrimaryInfoRenderer").is_some() {
            if let Some(n) = CompositeVideoPrimaryInfoNode::from_value(val) { return Some(YTNode::CompositeVideoPrimaryInfo(n)); }
        }
        if val.get("emergencyOneboxRenderer").is_some() {
            if let Some(n) = EmergencyOneboxNode::from_value(val) { return Some(YTNode::EmergencyOnebox(n)); }
        }
        if val.get("singleActionEmergencySupportRenderer").is_some() {
            if let Some(n) = SingleActionEmergencySupportNode::from_value(val) { return Some(YTNode::SingleActionEmergencySupport(n)); }
        }
        if val.get("playerLiveStoryboardSpecRenderer").is_some() {
            if let Some(n) = PlayerLiveStoryboardSpecNode::from_value(val) { return Some(YTNode::PlayerLiveStoryboardSpec(n)); }
        }
        if val.get("pollHeaderRenderer").is_some() {
            if let Some(n) = PollHeaderNode::from_value(val) { return Some(YTNode::PollHeader(n)); }
        }
        if val.get("changeEngagementPanelVisibilityAction").is_some() {
            if let Some(n) = ChangeEngagementPanelVisibilityActionNode::from_value(val) { return Some(YTNode::ChangeEngagementPanelVisibilityAction(n)); }
        }
        if val.get("showEngagementPanelEndpoint").is_some() {
            if let Some(n) = ShowEngagementPanelEndpointNode::from_value(val) { return Some(YTNode::ShowEngagementPanelEndpoint(n)); }
        }
        if val.get("creatorHeartViewModel").is_some() {
            if let Some(n) = CreatorHeartViewNode::from_value(val) { return Some(YTNode::CreatorHeartView(n)); }
        }
        if val.get("kidsCategoryTabRenderer").is_some() {
            if let Some(n) = KidsCategoryTabNode::from_value(val) { return Some(YTNode::KidsCategoryTab(n)); }
        }

        None
    }
}

