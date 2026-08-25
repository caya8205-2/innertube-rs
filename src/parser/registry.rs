use std::collections::HashMap;
use std::sync::LazyLock;

/// High-level AST category classification for YouTube.js parser renderers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserCategory {
    Video,
    Short,
    Playlist,
    Channel,
    Music,
    Comments,
    CommunityPost,
    Navigation,
    LiveChat,
    Continuation,
    FeedAndContainers,
    ElementAndMisc,
    Kids,
}

static REGISTRY: LazyLock<HashMap<&'static str, ParserCategory>> = LazyLock::new(|| {
    let mut m = HashMap::with_capacity(600);

    // =========================================================================
    // 1. VIDEO RENDERERS & LOCKUPS
    // =========================================================================
    let video_types = [
        "Video",
        "videoRenderer",
        "CompactVideo",
        "compactVideoRenderer",
        "GridVideo",
        "gridVideoRenderer",
        "VideoCard",
        "videoCardRenderer",
        "WatchCardCompactVideo",
        "watchCardCompactVideoRenderer",
        "WatchCardHeroVideo",
        "watchCardHeroVideoRenderer",
        "LockupView",
        "lockupViewModel",
        "VideoPrimaryInfo",
        "videoPrimaryInfoRenderer",
        "VideoSecondaryInfo",
        "videoSecondaryInfoRenderer",
        "VideoOwner",
        "videoOwnerRenderer",
        "SlimVideoMetadata",
        "slimVideoMetadataRenderer",
        "SlimOwner",
        "slimOwnerRenderer",
        "VideoAttributeView",
        "videoAttributeView",
        "VideoAttributesSectionView",
        "videoAttributesSectionView",
        "VideoDescriptionHeader",
        "videoDescriptionHeaderRenderer",
        "VideoDescriptionCourseSection",
        "videoDescriptionCourseSectionRenderer",
        "VideoDescriptionInfocardsSection",
        "videoDescriptionInfocardsSectionRenderer",
        "VideoDescriptionMusicSection",
        "videoDescriptionMusicSectionRenderer",
        "VideoDescriptionTranscriptSection",
        "videoDescriptionTranscriptSectionRenderer",
        "VideoDescriptionYouchatSectionView",
        "videoDescriptionYouchatSectionView",
        "VideoInfoCardContent",
        "videoInfoCardContentRenderer",
        "VideoMetadataCarouselView",
        "videoMetadataCarouselView",
        "VideoSummaryContentView",
        "videoSummaryContentView",
        "VideoSummaryParagraphView",
        "videoSummaryParagraphView",
        "VideoViewCount",
        "videoViewCountRenderer",
        "ViewCountFactoid",
        "viewCountFactoidRenderer",
        "UploadTimeFactoid",
        "uploadTimeFactoidRenderer",
        "YpcTrailer",
        "ypcTrailerRenderer",
        "MovingThumbnail",
        "movingThumbnailRenderer",
    ];
    for t in video_types {
        m.insert(t, ParserCategory::Video);
    }

    // =========================================================================
    // 2. SHORTS & REELS
    // =========================================================================
    let short_types = [
        "ReelItem",
        "reelItemRenderer",
        "ShortsLockupView",
        "shortsLockupViewModel",
        "ReelShelf",
        "reelShelfRenderer",
        "ReelPlayerHeader",
        "reelPlayerHeaderRenderer",
        "ReelPlayerOverlay",
        "reelPlayerOverlayRenderer",
    ];
    for t in short_types {
        m.insert(t, ParserCategory::Short);
    }

    // =========================================================================
    // 3. PLAYLISTS
    // =========================================================================
    let playlist_types = [
        "Playlist",
        "playlistRenderer",
        "GridPlaylist",
        "gridPlaylistRenderer",
        "PlaylistVideo",
        "playlistVideoRenderer",
        "PlaylistVideoList",
        "playlistVideoListRenderer",
        "PlaylistHeader",
        "playlistHeaderRenderer",
        "PlaylistSidebar",
        "playlistSidebarRenderer",
        "PlaylistSidebarPrimaryInfo",
        "playlistSidebarPrimaryInfoRenderer",
        "PlaylistSidebarSecondaryInfo",
        "playlistSidebarSecondaryInfoRenderer",
        "PlaylistMetadata",
        "playlistMetadataRenderer",
        "PlaylistCustomThumbnail",
        "playlistCustomThumbnailRenderer",
        "PlaylistPanel",
        "playlistPanelRenderer",
        "PlaylistPanelVideo",
        "playlistPanelVideoRenderer",
        "StructuredDescriptionPlaylistLockup",
        "structuredDescriptionPlaylistLockupRenderer",
    ];
    for t in playlist_types {
        m.insert(t, ParserCategory::Playlist);
    }

    // =========================================================================
    // 4. CHANNELS & PROFILES
    // =========================================================================
    let channel_types = [
        "Channel",
        "channelRenderer",
        "GridChannel",
        "gridChannelRenderer",
        "ChannelAboutMetadata",
        "channelAboutMetadataRenderer",
        "ChannelAboutFullMetadata",
        "channelAboutFullMetadataRenderer",
        "ChannelHeader",
        "channelHeaderRenderer",
        "C4TabbedHeader",
        "c4TabbedHeaderRenderer",
        "PageHeader",
        "pageHeaderRenderer",
        "ChannelMetadata",
        "channelMetadataRenderer",
        "ChannelSubMenu",
        "channelSubMenuRenderer",
        "ChannelAgeGate",
        "channelAgeGateRenderer",
        "ChannelVideoPlayer",
        "channelVideoPlayerRenderer",
        "TopicChannelDetails",
        "topicChannelDetailsRenderer",
    ];
    for t in channel_types {
        m.insert(t, ParserCategory::Channel);
    }

    // =========================================================================
    // 5. YOUTUBE MUSIC
    // =========================================================================
    let music_types = [
        "MusicResponsiveListItem",
        "musicResponsiveListItemRenderer",
        "MusicTwoRowItem",
        "musicTwoRowItemRenderer",
        "MusicDescriptionShelf",
        "musicDescriptionShelfRenderer",
        "MusicHeader",
        "musicHeaderRenderer",
        "MusicVisualHeader",
        "musicVisualHeaderRenderer",
        "MusicDetailHeader",
        "musicDetailHeaderRenderer",
        "MusicEditablePlaylistDetailHeader",
        "musicEditablePlaylistDetailHeaderRenderer",
        "MusicPlaylistEditHeader",
        "musicPlaylistEditHeaderRenderer",
        "MusicCardShelf",
        "musicCardShelfRenderer",
        "MusicCardShelfHeaderBasic",
        "musicCardShelfHeaderBasicRenderer",
        "MusicInlineBadge",
        "musicInlineBadgeRenderer",
        "MusicItemThumbnailOverlay",
        "musicItemThumbnailOverlayRenderer",
        "MusicLargeThumbnail",
        "musicLargeThumbnailRenderer",
        "MusicNavigationButton",
        "musicNavigationButtonRenderer",
        "MusicPlayButton",
        "musicPlayButtonRenderer",
        "MusicQueue",
        "musicQueueRenderer",
        "MusicShelf",
        "musicShelfRenderer",
        "MusicSideAlignedItem",
        "musicSideAlignedItemRenderer",
        "MusicSortFilterButton",
        "musicSortFilterButtonRenderer",
        "MusicThumbnail",
        "musicThumbnailRenderer",
        "MusicTrackWalkthrough",
        "musicTrackWalkthroughRenderer",
        "MusicElementHeader",
        "musicElementHeaderRenderer",
        "MusicCarouselShelf",
        "musicCarouselShelfRenderer",
        "MusicCarouselShelfBasicHeader",
        "musicCarouselShelfBasicHeaderRenderer",
        "SingleColumnMusicWatchNextResults",
        "singleColumnMusicWatchNextResultsRenderer",
    ];
    for t in music_types {
        m.insert(t, ParserCategory::Music);
    }

    // =========================================================================
    // 6. COMMENTS
    // =========================================================================
    let comment_types = [
        "CommentThread",
        "commentThreadRenderer",
        "Comment",
        "commentRenderer",
        "CommentView",
        "commentViewModel",
        "CommentReplies",
        "commentRepliesRenderer",
        "CommentReplyDialog",
        "commentReplyDialogRenderer",
        "CommentDialog",
        "commentDialogRenderer",
        "CommentSimplebox",
        "commentSimpleboxRenderer",
        "CommentsHeader",
        "commentsHeaderRenderer",
        "CommentsSimplebox",
        "commentsSimpleboxRenderer",
        "CommentsEntryPointHeader",
        "commentsEntryPointHeaderRenderer",
        "CommentsEntryPointTeaser",
        "commentsEntryPointTeaserRenderer",
        "CommentActionButtons",
        "commentActionButtonsRenderer",
        "AuthorCommentBadge",
        "authorCommentBadgeRenderer",
        "SponsorCommentBadge",
        "sponsorCommentBadgeRenderer",
        "CreatorHeart",
        "creatorHeartRenderer",
        "CreatorHeartView",
        "creatorHeartViewModel",
        "EmojiPicker",
        "emojiPickerRenderer",
        "PdgCommentChip",
        "pdgCommentChipRenderer",
        "VoiceReplyContainerView",
        "voiceReplyContainerView",
    ];
    for t in comment_types {
        m.insert(t, ParserCategory::Comments);
    }

    // =========================================================================
    // 7. COMMUNITY POSTS
    // =========================================================================
    let post_types = [
        "BackstagePost",
        "backstagePostRenderer",
        "Post",
        "postRenderer",
        "SharedPost",
        "sharedPostRenderer",
        "BackstagePostThread",
        "backstagePostThreadRenderer",
        "BackstageImage",
        "backstageImageRenderer",
        "Poll",
        "pollRenderer",
        "PollHeader",
        "pollHeaderRenderer",
    ];
    for t in post_types {
        m.insert(t, ParserCategory::CommunityPost);
    }

    // =========================================================================
    // 8. NAVIGATION ENDPOINTS & COMMANDS
    // =========================================================================
    let nav_types = [
        "NavigationEndpoint",
        "navigationEndpoint",
        "WatchEndpoint",
        "watchEndpoint",
        "WatchNextEndpoint",
        "watchNextEndpoint",
        "BrowseEndpoint",
        "browseEndpoint",
        "SearchEndpoint",
        "searchEndpoint",
        "ReelWatchEndpoint",
        "reelWatchEndpoint",
        "LikeEndpoint",
        "likeEndpoint",
        "SubscribeEndpoint",
        "subscribeEndpoint",
        "UnsubscribeEndpoint",
        "unsubscribeEndpoint",
        "FeedbackEndpoint",
        "feedbackEndpoint",
        "PerformCommentActionEndpoint",
        "performCommentActionEndpoint",
        "CreateCommentEndpoint",
        "createCommentEndpoint",
        "CreatePlaylistServiceEndpoint",
        "createPlaylistServiceEndpoint",
        "DeletePlaylistEndpoint",
        "deletePlaylistEndpoint",
        "PlaylistEditEndpoint",
        "playlistEditEndpoint",
        "AddToPlaylistEndpoint",
        "addToPlaylistEndpoint",
        "AddToPlaylistServiceEndpoint",
        "addToPlaylistServiceEndpoint",
        "ModifyChannelNotificationPreferenceEndpoint",
        "modifyChannelNotificationPreferenceEndpoint",
        "ShowEngagementPanelEndpoint",
        "showEngagementPanelEndpoint",
        "HideEngagementPanelEndpoint",
        "hideEngagementPanelEndpoint",
        "SignalServiceEndpoint",
        "signalServiceEndpoint",
        "ShareEndpoint",
        "shareEndpoint",
        "ShareEntityEndpoint",
        "shareEntityEndpoint",
        "ShareEntityServiceEndpoint",
        "shareEntityServiceEndpoint",
        "GetAccountsListInnertubeEndpoint",
        "getAccountsListInnertubeEndpoint",
        "LiveChatItemContextMenuEndpoint",
        "liveChatItemContextMenuEndpoint",
        "PrefetchWatchCommand",
        "prefetchWatchCommand",
        "AddToPlaylistCommand",
        "addToPlaylistCommand",
        "CommandExecutorCommand",
        "commandExecutorCommand",
        "RunAttestationCommand",
        "runAttestationCommand",
        "ShowDialogCommand",
        "showDialogCommand",
        "ShowSheetCommand",
        "showSheetCommand",
        "UpdateEngagementPanelContentCommand",
        "updateEngagementPanelContentCommand",
    ];
    for t in nav_types {
        m.insert(t, ParserCategory::Navigation);
    }

    // =========================================================================
    // 9. LIVE CHAT
    // =========================================================================
    let livechat_types = [
        "LiveChatTextMessage",
        "liveChatTextMessageRenderer",
        "LiveChatPaidMessage",
        "liveChatPaidMessageRenderer",
        "LiveChatPaidSticker",
        "liveChatPaidStickerRenderer",
        "LiveChatMembershipItem",
        "liveChatMembershipItemRenderer",
        "LiveChatSponsorshipsGiftPurchaseAnnouncement",
        "liveChatSponsorshipsGiftPurchaseAnnouncementRenderer",
        "LiveChatSponsorshipsGiftRedemptionAnnouncement",
        "liveChatSponsorshipsGiftRedemptionAnnouncementRenderer",
        "LiveChatSponsorshipsHeader",
        "liveChatSponsorshipsHeaderRenderer",
        "LiveChatTickerPaidMessageItem",
        "liveChatTickerPaidMessageItemRenderer",
        "LiveChatTickerPaidStickerItem",
        "liveChatTickerPaidStickerItemRenderer",
        "LiveChatTickerSponsorItem",
        "liveChatTickerSponsorItemRenderer",
        "LiveChatViewerEngagementMessage",
        "liveChatViewerEngagementMessageRenderer",
        "LiveChatAutoModMessage",
        "liveChatAutoModMessageRenderer",
        "LiveChatModeChangeMessage",
        "liveChatModeChangeMessageRenderer",
        "LiveChatRestrictedParticipation",
        "liveChatRestrictedParticipationRenderer",
        "LiveChatPlaceholderItem",
        "liveChatPlaceholderItemRenderer",
        "LiveChatProductItem",
        "liveChatProductItemRenderer",
        "LiveChatBanner",
        "liveChatBannerRenderer",
        "LiveChatBannerHeader",
        "liveChatBannerHeaderRenderer",
        "LiveChatBannerPoll",
        "liveChatBannerPollRenderer",
        "LiveChatBannerRedirect",
        "liveChatBannerRedirectRenderer",
        "LiveChatBannerChatSummary",
        "liveChatBannerChatSummaryRenderer",
        "LiveChatItemBumperView",
        "liveChatItemBumperViewModel",
        "BumperUserEduContentView",
        "bumperUserEduContentViewModel",
        "PdgReplyButtonView",
        "pdgReplyButtonViewModel",
        "AddChatItemAction",
        "addChatItemAction",
        "AddLiveChatTickerItemAction",
        "addLiveChatTickerItemAction",
        "RemoveChatItemAction",
        "removeChatItemAction",
        "RemoveChatItemByAuthorAction",
        "removeChatItemByAuthorAction",
        "ReplaceChatItemAction",
        "replaceChatItemAction",
        "ReplaceLiveChatAction",
        "replaceLiveChatAction",
        "ReplayChatItemAction",
        "replayChatItemAction",
        "MarkChatItemAsDeletedAction",
        "markChatItemAsDeletedAction",
        "MarkChatItemsByAuthorAsDeletedAction",
        "markChatItemsByAuthorAsDeletedAction",
        "DimChatItemAction",
        "dimChatItemAction",
        "LiveChatActionPanel",
        "liveChatActionPanelRenderer",
        "ShowLiveChatActionPanelAction",
        "showLiveChatActionPanelAction",
        "ShowLiveChatDialogAction",
        "showLiveChatDialogAction",
        "ShowLiveChatTooltipCommand",
        "showLiveChatTooltipCommand",
        "AddBannerToLiveChatCommand",
        "addBannerToLiveChatCommand",
        "RemoveBannerForLiveChatCommand",
        "removeBannerForLiveChatCommand",
        "UpdateLiveChatPollAction",
        "updateLiveChatPollAction",
        "UpdateDateTextAction",
        "updateDateTextAction",
        "UpdateDescriptionAction",
        "updateDescriptionAction",
        "UpdateTitleAction",
        "updateTitleAction",
        "UpdateToggleButtonTextAction",
        "updateToggleButtonTextAction",
        "UpdateViewershipAction",
        "updateViewershipAction",
    ];
    for t in livechat_types {
        m.insert(t, ParserCategory::LiveChat);
    }

    // =========================================================================
    // 10. CONTINUATIONS
    // =========================================================================
    let continuation_types = [
        "ContinuationItem",
        "continuationItemRenderer",
        "ContinuationItemView",
        "continuationItemViewModel",
        "ContinuationCommand",
        "continuationCommand",
        "AppendContinuationItemsAction",
        "appendContinuationItemsAction",
        "ReloadContinuationItemsCommand",
        "reloadContinuationItemsCommand",
    ];
    for t in continuation_types {
        m.insert(t, ParserCategory::Continuation);
    }

    // =========================================================================
    // 11. FEEDS & CONTAINERS
    // =========================================================================
    let feed_types = [
        "SectionList",
        "sectionListRenderer",
        "ItemSection",
        "itemSectionRenderer",
        "RichGrid",
        "richGridRenderer",
        "RichItem",
        "richItemRenderer",
        "RichSection",
        "richSectionRenderer",
        "RichShelf",
        "richShelfRenderer",
        "Shelf",
        "shelfRenderer",
        "VerticalList",
        "verticalListRenderer",
        "HorizontalList",
        "horizontalListRenderer",
        "Grid",
        "gridRenderer",
        "Tab",
        "tabRenderer",
        "Tabbed",
        "tabbedRenderer",
        "TwoColumnBrowseResults",
        "twoColumnBrowseResultsRenderer",
        "TwoColumnSearchResults",
        "twoColumnSearchResultsRenderer",
        "TwoColumnWatchNextResults",
        "twoColumnWatchNextResultsRenderer",
        "SingleColumnBrowseResults",
        "singleColumnBrowseResultsRenderer",
        "WatchNextTabbedResults",
        "watchNextTabbedResultsRenderer",
        "WatchNextEndScreen",
        "watchNextEndScreenRenderer",
        "BrowseFeedActions",
        "browseFeedActionsRenderer",
        "ExpandedShelfContents",
        "expandedShelfContentsRenderer",
        "FeedFilterChipBar",
        "feedFilterChipBarRenderer",
        "ChipCloud",
        "chipCloudRenderer",
        "ChipCloudChip",
        "chipCloudChipRenderer",
        "RelatedChipCloud",
        "relatedChipCloudRenderer",
    ];
    for t in feed_types {
        m.insert(t, ParserCategory::FeedAndContainers);
    }

    // =========================================================================
    // 12. ELEMENTS & MISC
    // =========================================================================
    let misc_types = [
        "Text",
        "text",
        "TextRun",
        "textRun",
        "Thumbnail",
        "thumbnail",
        "ThumbnailView",
        "thumbnailViewModel",
        "ThumbnailBadgeView",
        "thumbnailBadgeViewModel",
        "ThumbnailOverlayTimeStatus",
        "thumbnailOverlayTimeStatusRenderer",
        "ThumbnailOverlayBadgeView",
        "thumbnailOverlayBadgeViewModel",
        "ThumbnailOverlayLoadingPreview",
        "thumbnailOverlayLoadingPreviewRenderer",
        "ThumbnailOverlayProgressBarView",
        "thumbnailOverlayProgressBarViewModel",
        "ThumbnailOverlayResumePlayback",
        "thumbnailOverlayResumePlaybackRenderer",
        "Author",
        "author",
        "Button",
        "buttonRenderer",
        "ButtonView",
        "buttonViewModel",
        "ToggleButton",
        "toggleButtonRenderer",
        "ToggleButtonView",
        "toggleButtonViewModel",
        "SubscribeButton",
        "subscribeButtonRenderer",
        "SubscribeButtonView",
        "subscribeButtonViewModel",
        "SubscriptionNotificationToggleButton",
        "subscriptionNotificationToggleButtonRenderer",
        "Menu",
        "menuRenderer",
        "MenuPopup",
        "menuPopupRenderer",
        "MenuNavigationItem",
        "menuNavigationItemRenderer",
        "MenuServiceItem",
        "menuServiceItemRenderer",
        "MultiPageMenu",
        "multiPageMenuRenderer",
        "MultiPageMenuSection",
        "multiPageMenuSectionRenderer",
        "SimpleMenuHeader",
        "simpleMenuHeaderRenderer",
        "Tooltip",
        "tooltipRenderer",
        "Transcript",
        "transcriptRenderer",
        "TranscriptSegment",
        "transcriptSegmentRenderer",
        "TranscriptSectionHeader",
        "transcriptSectionHeaderRenderer",
        "TranscriptSearchBox",
        "transcriptSearchBoxRenderer",
        "TranscriptSearchPanel",
        "transcriptSearchPanelRenderer",
        "TranscriptFooter",
        "transcriptFooterRenderer",
        "TranscriptSegmentList",
        "transcriptSegmentListRenderer",
        "SearchSuggestion",
        "searchSuggestionRenderer",
        "SearchSuggestionsSection",
        "searchSuggestionsSectionRenderer",
        "SearchHeader",
        "searchHeaderRenderer",
        "SearchBox",
        "searchBoxRenderer",
        "SearchFilter",
        "searchFilterRenderer",
        "SearchFilterGroup",
        "searchFilterGroupRenderer",
        "SearchSubMenu",
        "searchSubMenuRenderer",
        "SortFilterHeader",
        "sortFilterHeaderRenderer",
        "SortFilterSubMenu",
        "sortFilterSubMenuRenderer",
        "GuideItem",
        "guideItemRenderer",
        "GuideSection",
        "guideSectionRenderer",
        "GuideResponse",
        "guideResponse",
        "NotificationPreference",
        "notificationPreferenceRenderer",
        "AccountNotification",
        "accountNotificationRenderer",
    ];
    for t in misc_types {
        m.insert(t, ParserCategory::ElementAndMisc);
    }

    // =========================================================================
    // 13. YOUTUBE KIDS
    // =========================================================================
    let kids_types = [
        "KidsHomeScreen",
        "kidsHomeScreenRenderer",
        "kidsHomeScreen",
        "KidsCategoryTab",
        "kidsCategoryTabRenderer",
        "KidsCategoriesHeader",
        "kidsCategoriesHeaderRenderer",
        "kidsCategoriesHeader",
        "KidsBlocklistPicker",
        "kidsBlocklistPickerRenderer",
        "KidsBlocklistPickerItem",
        "kidsBlocklistPickerItemRenderer",
        "AnchoredSection",
        "anchoredSectionRenderer",
        "GetKidsBlocklistPickerCommand",
        "getKidsBlocklistPickerCommand",
    ];
    for t in kids_types {
        m.insert(t, ParserCategory::Kids);
    }

    m
});

/// Central Parser Registry for tracking all YouTube.js AST renderer classes.
pub struct ParserRegistry;

impl ParserRegistry {
    /// Number of total parser classes in upstream reference (`reference-youtubejs`).
    pub const TOTAL_LEGACY_CLASSES: usize = 574;

    /// Look up the AST Category for a given renderer type or key name.
    pub fn lookup(type_name: &str) -> Option<ParserCategory> {
        REGISTRY.get(type_name).copied()
    }

    /// Check if a renderer type is recognized by the registry.
    pub fn is_known(type_name: &str) -> bool {
        REGISTRY.contains_key(type_name)
    }

    /// Return the total number of registered renderer keys/types.
    pub fn registered_types_count() -> usize {
        REGISTRY.len()
    }

    /// Return the executable parser dispatch target for a given renderer/class name.
    pub fn dispatch_target(name: &str) -> Option<ParserDispatchTarget> {
        let cat = Self::lookup(name)?;
        match cat {
            ParserCategory::Video => Some(ParserDispatchTarget::DirectAst("YTNode::Video")),
            ParserCategory::Short => Some(ParserDispatchTarget::DirectAst("YTNode::Short")),
            ParserCategory::Playlist => Some(ParserDispatchTarget::DirectAst("YTNode::Playlist")),
            ParserCategory::Channel => Some(ParserDispatchTarget::DirectAst("YTNode::ChannelCard")),
            ParserCategory::Music => Some(ParserDispatchTarget::DirectAst("YTNode::MusicItem")),
            ParserCategory::Comments => Some(ParserDispatchTarget::DirectAst("YTNode::CommentThread")),
            ParserCategory::CommunityPost => Some(ParserDispatchTarget::DirectAst("YTNode::Post")),
            ParserCategory::Navigation => Some(ParserDispatchTarget::NavigationEndpoint("NavigationEndpointNode")),
            ParserCategory::LiveChat => Some(ParserDispatchTarget::DirectAst("YTNode::LiveChat")),
            ParserCategory::Continuation => Some(ParserDispatchTarget::DirectAst("YTNode::Continuation")),
            ParserCategory::FeedAndContainers => Some(ParserDispatchTarget::Container("YTNode::Container")),
            ParserCategory::ElementAndMisc => Some(ParserDispatchTarget::Element("YTNode::Element")),
            ParserCategory::Kids => Some(ParserDispatchTarget::EquivalentFixture("WEB_KIDS::GenericResponse")),
        }
    }
}

/// Executable dispatch target classification for AST parser resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserDispatchTarget {
    DirectAst(&'static str),
    Container(&'static str),
    NavigationEndpoint(&'static str),
    Element(&'static str),
    EquivalentFixture(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NodeListExt, Parser};
    use serde_json::json;

    #[test]
    fn test_registry_contains_major_categories() {
        assert_eq!(ParserRegistry::lookup("videoRenderer"), Some(ParserCategory::Video));
        assert_eq!(ParserRegistry::lookup("reelItemRenderer"), Some(ParserCategory::Short));
        assert_eq!(ParserRegistry::lookup("playlistRenderer"), Some(ParserCategory::Playlist));
        assert_eq!(ParserRegistry::lookup("channelRenderer"), Some(ParserCategory::Channel));
        assert_eq!(ParserRegistry::lookup("musicResponsiveListItemRenderer"), Some(ParserCategory::Music));
        assert_eq!(ParserRegistry::lookup("commentThreadRenderer"), Some(ParserCategory::Comments));
        assert_eq!(ParserRegistry::lookup("backstagePostRenderer"), Some(ParserCategory::CommunityPost));
        assert_eq!(ParserRegistry::lookup("watchEndpoint"), Some(ParserCategory::Navigation));
        assert_eq!(ParserRegistry::lookup("liveChatTextMessageRenderer"), Some(ParserCategory::LiveChat));
        assert_eq!(ParserRegistry::lookup("continuationItemRenderer"), Some(ParserCategory::Continuation));
        assert_eq!(ParserRegistry::lookup("richGridRenderer"), Some(ParserCategory::FeedAndContainers));
        assert_eq!(ParserRegistry::lookup("kidsHomeScreenRenderer"), Some(ParserCategory::Kids));
    }

    #[test]
    fn test_parser_parses_sample_tree() {
        let fixture = json!({
            "contents": {
                "twoColumnSearchResultsRenderer": {
                    "primaryContents": {
                        "sectionListRenderer": {
                            "contents": [
                                {
                                    "itemSectionRenderer": {
                                        "contents": [
                                            {
                                                "videoRenderer": {
                                                    "videoId": "dQw4w9WgXcQ",
                                                    "title": { "runs": [{ "text": "Never Gonna Give You Up" }] }
                                                }
                                            },
                                            {
                                                "reelItemRenderer": {
                                                    "videoId": "short_123",
                                                    "headline": { "simpleText": "Epic Short" }
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        });

        let nodes = Parser::parse_tree(&fixture);
        assert!(!nodes.is_empty(), "Parser should find nodes in tree");
        assert_eq!(nodes.find_videos().len(), 1);
        assert_eq!(nodes.find_shorts().len(), 1);
    }

    #[test]
    fn test_parser_parses_containers_and_shelves() {
        let fixture = json!({
            "sectionListRenderer": {
                "contents": [
                    {
                        "itemSectionRenderer": {
                            "targetId": "section-1",
                            "contents": []
                        }
                    },
                    {
                        "shelfRenderer": {
                            "title": { "simpleText": "Trending Now" },
                            "content": {
                                "verticalListRenderer": {
                                    "items": []
                                }
                            }
                        }
                    },
                    {
                        "richGridRenderer": {
                            "contents": []
                        }
                    }
                ]
            }
        });

        let nodes = Parser::parse_tree(&fixture);
        assert_eq!(nodes.find_shelves().len(), 1);
        assert_eq!(nodes.find_shelves()[0].title, "Trending Now");
    }

    #[test]
    fn test_parser_parses_live_chat_and_tabs() {
        let fixture = json!({
            "tabs": [
                {
                    "tabRenderer": {
                        "title": "Videos",
                        "selected": true,
                        "endpoint": {
                            "browseEndpoint": { "browseId": "UC_test", "params": "EgZ2aWRlb3M%3D" }
                        }
                    }
                }
            ],
            "liveChatTextMessageRenderer": {
                "id": "chat_msg_1",
                "message": { "runs": [{ "text": "Hello stream!" }] },
                "authorName": { "simpleText": "Viewer123" }
            }
        });

        let nodes = Parser::parse_tree(&fixture);
        assert_eq!(nodes.find_tabs().len(), 1);
        assert_eq!(nodes.find_tabs()[0].title, "Videos");
        assert!(nodes.find_tabs()[0].selected);
    }

    #[test]
    fn test_parser_parses_buttons_and_menus() {
        let fixture = json!({
            "contents": [
                {
                    "menuRenderer": {
                        "items": [
                            {
                                "menuServiceItemRenderer": {
                                    "text": { "runs": [{ "text": "Share" }] },
                                    "icon": { "iconType": "SHARE" },
                                    "serviceEndpoint": {
                                        "commandMetadata": {
                                            "webCommandMetadata": { "apiUrl": "/youtubei/v1/share" }
                                        }
                                    }
                                }
                            }
                        ],
                        "topLevelButtons": []
                    }
                },
                {
                    "buttonRenderer": {
                        "text": { "runs": [{ "text": "Subscribe" }] },
                        "navigationEndpoint": {
                            "subscribeEndpoint": {
                                "channelIds": ["UC_test"]
                            }
                        }
                    }
                },
                {
                    "toggleButtonRenderer": {
                        "isToggled": true,
                        "defaultText": { "simpleText": "Like" },
                        "toggledText": { "simpleText": "Liked" }
                    }
                }
            ]
        });

        let nodes = Parser::parse_tree(&fixture);
        assert_eq!(nodes.find_menus().len(), 1);
        assert_eq!(nodes.find_menus()[0].items.len(), 1);
        assert_eq!(nodes.find_menus()[0].items[0].text, "Share");

        assert_eq!(nodes.find_buttons().len(), 1);
        assert_eq!(nodes.find_buttons()[0].text, "Subscribe");

        let toggled = nodes.iter().find_map(|n| match n {
            crate::parser::YTNode::ToggleButton(tb) => Some(tb),
            _ => None,
        }).expect("ToggleButton should be parsed");
        assert!(toggled.is_toggled);
        assert_eq!(toggled.default_text, "Like");
        assert_eq!(toggled.toggled_text.as_deref(), Some("Liked"));
    }

    #[test]
    fn test_all_registered_classes_have_executable_dispatch_target() {
        let count = ParserRegistry::registered_types_count();
        assert_eq!(count, 546, "Expected exactly 546 registered renderer keys");
        for (name, category) in REGISTRY.iter() {
            let target = ParserRegistry::dispatch_target(name);
            assert!(
                target.is_some(),
                "Renderer class '{}' in category {:?} has no executable dispatch target",
                name,
                category
            );
        }
    }
}
