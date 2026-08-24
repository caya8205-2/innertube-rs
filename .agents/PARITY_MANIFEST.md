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

Implementation work has completed a first pass through plan items 2–6, but
that is **not** completion evidence. Default validation currently passes 38
unit/contract tests and Clippy. Eight live integration tests exist in
`tests/live_integration.rs` and were executed in an explicit run with 8/8 passing
(video info & stream URL, search filters, suggestions, comments, channel, YT Music,
guide & hashtag feed, transcript). The parser registry is a 574-name inventory,
not proof of 574 typed parser implementations.

## Public `Innertube` API baseline

| Legacy API | Rust status | Notes |
|---|---|---|
| `getInfo` | Partial | `get_video_info` returns raw player data; `get_basic_info` covers player-only compatibility with `VideoInfo`. |
| `getBasicInfo` | Partial | Implemented via `get_basic_info` with `playbackContext`, `lactMilliseconds`, PO-Token forwarding, and `VideoInfo` container. |
| `getShortsVideoInfo` | Partial | Implemented via `get_shorts_video_info` with `reelWatchEndpoint` and `ReelSequence` protobuf continuation semantics. |
| `search` | Partial | Query, continuation, typed `SearchFilters`, and `SearchFilter` protobuf encoding are implemented; legacy parsed feed behavior remains. |
| `getSearchSuggestions` | Partial | InnerTube & suggestqueries endpoints, `previous_query` parameter, and session cookie forwarding are implemented. |
| `getComments` | Partial | `GetCommentsSectionParams` protobuf continuation token, sort options, comment ID, and child replies are implemented. |
| `getHomeFeed`, `getGuide`, `getHistory`, `getLibrary`, `getNotifications`, `getChannel`, `getPlaylist`, `getHashtag` | Partial | Feature paths exist but do not expose the legacy feed/parser contracts. |
| `getCourses`, `getSubscriptionsFeed`, `getChannelsFeed`, `getPlaylists` | Partial | Generic typed browse-feed wrappers exist; legacy feed behavior and authenticated live verification remain. |
| `getUnseenNotificationsCount` | Partial | Legacy response layouts and zero fallback are covered; authenticated live verification remains. |
| `getStreamingData` | Partial | Supports `get_streaming_data_with_options` with rich `FormatOptions` (itag, format type, codec, quality); full legacy parsed wrappers remain. |
| `download` | Partial | Supports `download_with_options` with HTTP byte ranges (`DownloadRange`) and stream response. |
| `resolveURL`, `getPost` | Partial | Rust equivalents cover the typed navigation/post-detail path; generic legacy endpoint metadata and feed behavior remain. |
| `getPostComments` | Partial | Community Post continuation protobuf and parsed comment response are implemented; generic legacy `Comments` feed behavior remains. |
| `getAttestationChallenge` | Partial | Request contract is implemented; typed challenge parsing and BotGuard integration remain. |
| `call` | Partial | Raw and parsed `NavigationEndpoint` calls exist; command parsing and every legacy endpoint path remain incomplete. |

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
