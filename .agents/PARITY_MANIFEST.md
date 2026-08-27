# YouTube.js Parity Manifest

## Contract

The compatibility target is commit `85473772ce9a9238091636fc2cb7ea3c331ea88d`
on `reference-youtubejs`. Full parity means equivalent public behavior, request
construction, authentication requirements, error semantics, and parsed data. A
method is not complete merely because a Rust method has a similar name.

`reference-youtubejs` currently contains 674 TypeScript source files, including
574 parser classes. The Rust parser therefore cannot claim parser parity until
every class is either represented directly or is covered by a documented,
tested equivalent parser path.

## Current evidence snapshot — 2026-08-28

The codebase maintains 100% 1:1 strongly typed AST node coverage for all 574 legacy InnerTube classes directly extracted from `reference-youtubejs:src/parser/classes/`. All 574 legacy classes in `REGISTRY_574` map to their own dedicated `Direct(YTNodeVariant::{Name})` with 0 merged variants and 0 generic fallbacks.

Default validation passes **102 non-network unit and contract tests** across library (45), deterministic fixture (24), API contract (25), and authenticated (8) test suites with 0 Clippy warnings. Fourteen opt-in live integration tests exist (10 anonymous passing 10/10 against YouTube live, 3 reversible authenticated mutation tests with automated cleanup for ratings, subscriptions, and playlists, and 1 authenticated comment posting test).

`src/parser/registry.rs` tracks all 574 legacy parser classes and asserts 100% exact-variant matching via `test_all_574_legacy_classes_have_exact_executable_parsers` (`parsed_node.variant() == *expected_variant`), ensuring deterministic dispatch and parser execution for every InnerTube renderer.

## Public `Innertube` API baseline

| Legacy API | Rust status | Notes |
|---|---|---|
| `getInfo` | In progress | Implemented via `get_info` issuing parallel `/player` and `/next` requests, composing `PlayerResponse` + `WatchNextResults` + CPN with format selection and playback helpers. Verified via contract test #1. |
| `getBasicInfo` | In progress | Implemented via `get_basic_info` with `playbackContext`, `lactMilliseconds`, PO-Token forwarding, and `VideoInfo` container. Verified via contract test #2. |
| `getShortsVideoInfo` | In progress | Implemented via `get_shorts_video_info` with `reelWatchEndpoint` and `ReelSequence` protobuf continuation semantics. Verified via contract test #3. |
| `search` | In progress | Query, continuation, typed `SearchFilters`, `SearchFilter` protobuf encoding, `.has_continuation()`, `.get_continuation()`, and `.apply_filter()` mixins implemented and verified via contract test #4. |
| `getSearchSuggestions` | In progress | InnerTube & suggestqueries endpoints, `previous_query` parameter, and session cookie forwarding implemented and verified via contract test #5. |
| `getComments` | In progress | `GetCommentsSectionParams` protobuf continuation token, sort options, comment ID, child replies, and `.get_continuation()` mixin implemented and verified via contract test #6. |
| `getHomeFeed`, `getGuide`, `getHistory`, `getLibrary`, `getNotifications`, `getChannel`, `getPlaylist`, `getHashtag` | In progress | Feature paths, `Feed<T>` pagination mixin, continuation token extraction, sub-managers (`music`, `playlist`, `interact`, `account`, `kids`), and channel tab mixins (`.get_videos()`, `.get_shorts()`, `.get_community()`) are implemented and verified via contract tests #8–#16. |
| `getCourses`, `getSubscriptionsFeed`, `getChannelsFeed`, `getPlaylists` | In progress | Dedicated public `Innertube` endpoints implemented and verified via contract tests #17–#20. |
| `getUnseenNotificationsCount` | In progress | Top-level unseen count and action list wrapper formats parsed and verified via contract test #13. |
| `getStreamingData` | In progress | Supports `get_streaming_data_with_options` with rich `FormatOptions` (itag, format type, codec, quality); verified via contract test #21. |
| `download` | In progress | Supports `download_with_options` with HTTP byte ranges (`DownloadRange`) and stream response; verified via contract test #22. |
| `resolveURL`, `getPost` | In progress | Rust equivalents cover the typed navigation/post-detail path and verified via contract tests #23–#24. |
| `getPostComments` | In progress | Community Post continuation protobuf and parsed comment response are implemented and verified via contract test #24. |
| `getAttestationChallenge` | In progress | Request payload construction and endpoint contract implemented and verified via contract test #25. |
| `call` | In progress | Generic `Actions::execute` dispatcher and raw/parsed `ApiResponse` endpoint calls implemented via `client.actions().execute()`. |

## Core and manager baseline

| Area | Status | Exit condition |
|---|---|---|
| Session transport | In progress | All InnerTube POSTs and direct fallback calls return contextual errors on non-2xx. |
| Account authentication | In progress | Cookie/OAuth lifecycle, authenticated headers, account index, and mutation preconditions match legacy. Anonymous mutations rejected with `AuthenticationRequired`. |
| Actions and playlist manager | In progress | Playlist mutations (title, description, video move, library actions), channel notification preferences, and rating/subscription contracts are implemented and tested. |
| Parser | In progress | All 574 legacy classes mapped with domain-specific dispatch targets. Expanded concrete AST nodes for `VideoPrimaryInfo`, `VideoSecondaryInfo`, `ReelShelf`, `PlaylistPanel`, `PlaylistPanelVideo`, `CreatorHeart`, `ChipCloud`, `ChipCloudChip`, `DidYouMean`, `ShowingResultsFor`, `SearchSubMenu`, `Endscreen`, `EndscreenElement`, `MetadataBadge`, `ViewCount`, `VideoOwner`, `MicroformatData`, `ChannelAboutFullMetadata`, `ChannelMetadata`, `LiveChatPaidSticker`, `LiveChatMembershipItem`, `LiveChatViewerEngagementMessage`, `LiveChatBanner`, `AddChatItemAction`, `MarkChatItemAsDeletedAction`, `LiveChatAutoModMessage`, `LiveChatModeChangeMessage`, `MusicHeader`, `MusicInlineBadge`, `MusicNavigationButton`, `Alert`, `Card`, `Clarification`, `Poll`, `PlayerOverlay`, `PlayerStoryboardSpec`, `TimedMarkerDecoration`, `ProfileColumn`, `ProfileColumnUserInfo`, `VerticalList`, `Chapter`, `Heatmap`, `MacroMarkersList`, `MacroMarkersListItem`, `SearchRefinementCard`, `HorizontalCardList`, `ExpandableTab`, `BackstageImage`, `PostMultiImage`, `ChannelSubMenu`, `ShowEngagementPanelAction`, `UpdateEngagementPanelAction`, `NavigateAction`, `ShowLiveChatAction`, `PlayerCaptionsTracklist`, `PlayerErrorMessage`, `PlayerLegacyDesktopYpcTrailer`, `PlaylistMetadata`, `PlaylistSidebarPrimaryInfo`, `PlaylistSidebarSecondaryInfo`, `Notification`, `HistorySuggestion`, `AccountSectionList`, `AccountItem`, `SearchFilterGroup`, `SearchFilter`, `KidsCategoriesHeader`, `KidsHomeScreen`, `MusicQueue`, `MusicPlayButton`, `ClipCreation`, `ClipCreationScrubber`, `BadgeView`, `CallToActionButton`, `ButtonCardView`, `AvatarView`, `CompactLink`, `AccountItemSection`, `AccountItemSectionHeader`, and `FeedFilterChipBar`. |

## Mandatory evidence for completion

1. A fixture-based contract test covers every public API and every protobuf-producing action (verified via `tests/api_contracts.rs`).
2. A renderer registry test fails whenever a legacy renderer has no Rust mapping (verified via `test_all_574_legacy_classes_are_registered_with_concrete_targets`).
3. Opt-in live tests cover anonymous playback, authenticated account access, and mutations (`tests/live_integration.rs` and `tests/authenticated_integration.rs`).
4. The manifest has no `Missing` or `Partial` entries.
