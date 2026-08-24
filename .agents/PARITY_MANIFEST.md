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

Implementation batches have completed Phase 1 (VideoInfo concurrent composition & sub-manager namespaces), Phase 2 (Feed<T> mixins & continuation paths), Phase 3 (Container, Button, Menu, and Endpoint AST expansions), and Phase 4 (Generic `Actions.execute` dispatcher and `ApiResponse`).
Default validation currently passes 43 unit/contract tests and Clippy with 0 warnings. Ten live integration tests exist in `tests/live_integration.rs` and were verified live against YouTube (10/10 passing).

## Public `Innertube` API baseline

| Legacy API | Rust status | Notes |
|---|---|---|
| `getInfo` | In progress | Implemented via `get_info` issuing parallel `/player` and `/next` requests, composing `PlayerResponse` + `WatchNextResults` + CPN with format selection and playback helpers. |
| `getBasicInfo` | In progress | Implemented via `get_basic_info` with `playbackContext`, `lactMilliseconds`, PO-Token forwarding, and `VideoInfo` container. |
| `getShortsVideoInfo` | In progress | Implemented via `get_shorts_video_info` with `reelWatchEndpoint` and `ReelSequence` protobuf continuation semantics. |
| `search` | Partial | Query, continuation, typed `SearchFilters`, and `SearchFilter` protobuf encoding are implemented; legacy parsed feed behavior remains. |
| `getSearchSuggestions` | Partial | InnerTube & suggestqueries endpoints, `previous_query` parameter, and session cookie forwarding are implemented. |
| `getComments` | Partial | `GetCommentsSectionParams` protobuf continuation token, sort options, comment ID, and child replies are implemented. |
| `getHomeFeed`, `getGuide`, `getHistory`, `getLibrary`, `getNotifications`, `getChannel`, `getPlaylist`, `getHashtag` | In progress | Feature paths, `Feed<T>` pagination mixin, continuation token extraction, and sub-managers (`music`, `playlist`, `interact`, `account`, `kids`) are implemented. |
| `getCourses`, `getSubscriptionsFeed`, `getChannelsFeed`, `getPlaylists` | Partial | Generic typed browse-feed wrappers exist; legacy feed behavior and authenticated live verification remain. |
| `getUnseenNotificationsCount` | Partial | Legacy response layouts and zero fallback are covered; authenticated live verification remains. |
| `getStreamingData` | Partial | Supports `get_streaming_data_with_options` with rich `FormatOptions` (itag, format type, codec, quality); full legacy parsed wrappers remain. |
| `download` | Partial | Supports `download_with_options` with HTTP byte ranges (`DownloadRange`) and stream response. |
| `resolveURL`, `getPost` | Partial | Rust equivalents cover the typed navigation/post-detail path; generic legacy endpoint metadata and feed behavior remain. |
| `getPostComments` | Partial | Community Post continuation protobuf and parsed comment response are implemented; generic legacy `Comments` feed behavior remains. |
| `getAttestationChallenge` | Partial | Request contract is implemented; typed challenge parsing and BotGuard integration remain. |
| `call` | In progress | Generic `Actions::execute` dispatcher and raw/parsed `ApiResponse` endpoint calls implemented via `client.actions().execute()`. |

## Core and manager baseline

| Area | Status | Exit condition |
|---|---|---|
| Session transport | In progress | All InnerTube POSTs and direct fallback calls return contextual errors on non-2xx. |
| Account authentication | In progress | Cookie/OAuth lifecycle, authenticated headers, account index, and mutation preconditions match legacy. |
| Actions and playlist manager | In progress | Playlist mutations (title, description, video move, library actions), channel notification preferences, and rating/subscription contracts are implemented and tested. |
| Player and decipher | Partial | Player selection, cache lifecycle, client fallback, and current-player fixtures are equivalent. |
| Parser | In progress | A registry inventories 574 legacy class names by category; typed-node or tested-equivalent coverage remains unproven. |

## Mandatory evidence for completion

1. A fixture-based contract test covers every public API and every protobuf-producing action.
2. A renderer registry test fails whenever a legacy renderer has no Rust mapping.
3. Opt-in live tests cover anonymous playback, authenticated account access, and mutations.
4. The manifest has no `Missing` or `Partial` entries.
