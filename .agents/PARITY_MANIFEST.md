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

Implementation batches have completed Phase 1 (VideoInfo concurrent composition & sub-manager namespaces), Phase 2 (Feed<T> mixins & continuation paths), Phase 3 (Container, Button, Menu, and Endpoint AST expansions), Phase 4 (Generic `Actions.execute` dispatcher and `ApiResponse`), and Phase 5 (Public endpoints for Courses, Subscriptions, Channels, Playlists, Unseen Notifications, and Attestation Challenge with rich feed mixins).
Default validation currently passes 54 unit/contract tests across library and deterministic fixture suites with 0 Clippy warnings. Ten live integration tests exist and pass 10/10 against YouTube.

`src/parser/registry.rs` provides an executable `ParserDispatchTarget` mapping for 546 registered keys to direct AST nodes, containers, elements, or equivalent fallbacks, verified by `test_all_registered_classes_have_executable_dispatch_target`.

## Public `Innertube` API baseline

| Legacy API | Rust status | Notes |
|---|---|---|
| `getInfo` | In progress | Implemented via `get_info` issuing parallel `/player` and `/next` requests, composing `PlayerResponse` + `WatchNextResults` + CPN with format selection and playback helpers. |
| `getBasicInfo` | In progress | Implemented via `get_basic_info` with `playbackContext`, `lactMilliseconds`, PO-Token forwarding, and `VideoInfo` container. |
| `getShortsVideoInfo` | In progress | Implemented via `get_shorts_video_info` with `reelWatchEndpoint` and `ReelSequence` protobuf continuation semantics. |
| `search` | In progress | Query, continuation, typed `SearchFilters`, `SearchFilter` protobuf encoding, `.has_continuation()`, `.get_continuation()`, and `.apply_filter()` mixins implemented and fixture-tested. |
| `getSearchSuggestions` | Partial | InnerTube & suggestqueries endpoints, `previous_query` parameter, and session cookie forwarding are implemented. |
| `getComments` | In progress | `GetCommentsSectionParams` protobuf continuation token, sort options, comment ID, child replies, and `.get_continuation()` mixin implemented and fixture-tested. |
| `getHomeFeed`, `getGuide`, `getHistory`, `getLibrary`, `getNotifications`, `getChannel`, `getPlaylist`, `getHashtag` | In progress | Feature paths, `Feed<T>` pagination mixin, continuation token extraction, sub-managers (`music`, `playlist`, `interact`, `account`, `kids`), and channel tab mixins (`.get_videos()`, `.get_shorts()`, `.get_community()`) are implemented. |
| `getCourses`, `getSubscriptionsFeed`, `getChannelsFeed`, `getPlaylists` | In progress | Dedicated public `Innertube` endpoints implemented and verified via deterministic browse fixture tests. |
| `getUnseenNotificationsCount` | In progress | Top-level unseen count and action list wrapper formats parsed and verified via fixture tests. |
| `getStreamingData` | Partial | Supports `get_streaming_data_with_options` with rich `FormatOptions` (itag, format type, codec, quality); full legacy parsed wrappers remain. |
| `download` | Partial | Supports `download_with_options` with HTTP byte ranges (`DownloadRange`) and stream response. |
| `resolveURL`, `getPost` | In progress | Rust equivalents cover the typed navigation/post-detail path and fixture contract tests. |
| `getPostComments` | In progress | Community Post continuation protobuf and parsed comment response are implemented and fixture-tested. |
| `getAttestationChallenge` | In progress | Request payload construction and endpoint contract implemented and fixture-tested. |
| `call` | In progress | Generic `Actions::execute` dispatcher and raw/parsed `ApiResponse` endpoint calls implemented via `client.actions().execute()`. |

## Core and manager baseline

| Area | Status | Exit condition |
|---|---|---|
| Session transport | In progress | All InnerTube POSTs and direct fallback calls return contextual errors on non-2xx. |
| Account authentication | In progress | Cookie/OAuth lifecycle, authenticated headers, account index, and mutation preconditions match legacy. |
| Actions and playlist manager | In progress | Playlist mutations (title, description, video move, library actions), channel notification preferences, and rating/subscription contracts are implemented and tested. |
| Player and decipher | Partial | Player selection, cache lifecycle, client fallback, and current-player fixtures are equivalent. |
| Parser | In progress | 574 names are inventoried, but only 23 YTNode variants and about 35 dispatcher checks exist. Replace the inventory with executable class-to-dispatch or class-to-fixture coverage before claiming parity. |

## Mandatory evidence for completion

1. A fixture-based contract test covers every public API and every protobuf-producing action.
2. A renderer registry test fails whenever a legacy renderer has no Rust mapping.
3. Opt-in live tests cover anonymous playback, authenticated account access, and mutations.
4. The manifest has no `Missing` or `Partial` entries.
