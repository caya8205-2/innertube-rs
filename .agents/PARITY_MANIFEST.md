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

## Current evidence snapshot — 2026-08-25

Implementation batches have completed Phase 1 (VideoInfo concurrent composition & sub-manager namespaces), Phase 2 (Feed<T> mixins & continuation paths), Phase 3 (Container, Button, Menu, and Endpoint AST expansions), Phase 4 (Generic `Actions.execute` dispatcher and `ApiResponse`), Phase 5 (Public endpoints for Courses, Subscriptions, Channels, Playlists, Unseen Notifications, and Attestation Challenge with rich feed mixins), Phase 6 (100% Strongly Typed Domain-Specific Parser Target Mapping for all 574 Legacy Classes, 25 Legacy Public API contracts harness, and Reversible Authenticated Mutation Lifecycle Tests), and Phase 7 (Concrete Semantic AST Node Ports for Search Modifiers, Endscreens/Overlays, Metadata/Badges, and Channel Metadata with deterministic fixture contracts).
Default validation currently passes 85 non-network unit/contract tests across library (40), deterministic fixture (12), API contract (25), and authenticated (8) test suites with 0 Clippy warnings. Fourteen opt-in live integration tests exist (10 anonymous passing 10/10 against YouTube live, 3 reversible authenticated mutation tests with automated cleanup for ratings, subscriptions, and playlists, and 1 authenticated comment posting test).

`src/parser/registry.rs` tracks all 574 legacy parser classes directly extracted from `reference-youtubejs:src/parser/classes/` and maps each into 100% domain-specific typed enum targets (138 `Direct(YTNodeVariant)`, 173 `Container(ContainerKind)`, 182 `Element(ElementKind)`, 74 `NavigationEndpoint(EndpointKind)`, 7 `Kids(KidsKind)`, with **0 `Generic*` fallbacks and 0 `DocumentedEquivalent` strings**), verified by `test_all_574_legacy_classes_are_registered_with_strongly_typed_targets` and `test_all_ytnode_variants_have_executable_parsers`.

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
| Parser | In progress | All 574 legacy classes mapped with domain-specific dispatch targets. Expanded concrete AST nodes for `VideoPrimaryInfo`, `VideoSecondaryInfo`, `ReelShelf`, `PlaylistPanel`, `PlaylistPanelVideo`, `CreatorHeart`, `ChipCloud`, `ChipCloudChip`, `DidYouMean`, `ShowingResultsFor`, `SearchSubMenu`, `Endscreen`, `EndscreenElement`, `MetadataBadge`, `ViewCount`, `VideoOwner`, `MicroformatData`, `ChannelAboutFullMetadata`, `ChannelMetadata`, `LiveChatPaidSticker`, `LiveChatMembershipItem`, `LiveChatViewerEngagementMessage`, `LiveChatBanner`, `AddChatItemAction`, `MarkChatItemAsDeletedAction`, `LiveChatAutoModMessage`, `LiveChatModeChangeMessage`, `MusicHeader`, `MusicInlineBadge`, `MusicNavigationButton`, `Alert`, `Card`, `Clarification`, `Poll`, `PlayerOverlay`, `PlayerStoryboardSpec`, `TimedMarkerDecoration`, `ProfileColumn`, `ProfileColumnUserInfo`, `VerticalList`, `Chapter`, `Heatmap`, `MacroMarkersList`, `MacroMarkersListItem`, `SearchRefinementCard`, `HorizontalCardList`, `ExpandableTab`, `BackstageImage`, `PostMultiImage`, and `ChannelSubMenu`. |

## Mandatory evidence for completion

1. A fixture-based contract test covers every public API and every protobuf-producing action (verified via `tests/api_contracts.rs`).
2. A renderer registry test fails whenever a legacy renderer has no Rust mapping (verified via `test_all_574_legacy_classes_are_registered_with_concrete_targets`).
3. Opt-in live tests cover anonymous playback, authenticated account access, and mutations (`tests/live_integration.rs` and `tests/authenticated_integration.rs`).
4. The manifest has no `Missing` or `Partial` entries.
