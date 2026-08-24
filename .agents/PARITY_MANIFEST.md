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

## Public `Innertube` API baseline

| Legacy API | Rust status | Notes |
|---|---|---|
| `getInfo` | Partial | `get_video_info` returns raw player data, not the complete legacy `VideoInfo` composition. |
| `getBasicInfo` | Missing | Needs a player-only compatibility result. |
| `getShortsVideoInfo` | Missing | Needs Shorts client/request semantics. |
| `search` | Partial | Query and continuation exist; legacy filters and parsed feed behavior do not. |
| `getSearchSuggestions` | Partial | No `previous_query` contract. |
| `getComments` | Partial | Legacy sort/comment-id arguments and continuation semantics differ. |
| `getHomeFeed`, `getGuide`, `getHistory`, `getLibrary`, `getNotifications`, `getChannel`, `getPlaylist`, `getHashtag` | Partial | Feature paths exist but do not expose the legacy feed/parser contracts. |
| `getCourses`, `getSubscriptionsFeed`, `getChannelsFeed`, `getPlaylists` | Partial | Generic typed browse-feed wrappers exist; legacy feed behavior and authenticated live verification remain. |
| `getUnseenNotificationsCount` | Partial | Legacy response layouts and zero fallback are covered; authenticated live verification remains. |
| `getStreamingData` | Partial | Returns a selected format with deciphered URL; legacy format options remain incomplete. |
| `download` | Partial | Returns a streaming `reqwest::Response`; legacy range and option contracts remain incomplete. |
| `resolveURL`, `getPost` | Partial | Rust equivalents cover the typed navigation/post-detail path; generic legacy endpoint metadata and feed behavior remain. |
| `getPostComments` | Partial | Community Post continuation protobuf and parsed comment response are implemented; generic legacy `Comments` feed behavior remains. |
| `getAttestationChallenge` | Partial | Request contract is implemented; typed challenge parsing and BotGuard integration remain. |
| `call` | Partial | Raw and parsed `NavigationEndpoint` calls exist; command parsing and every legacy endpoint path remain incomplete. |

## Core and manager baseline

| Area | Status | Exit condition |
|---|---|---|
| Session transport | In progress | All InnerTube POSTs and direct fallback calls return contextual errors on non-2xx. |
| Account authentication | In progress | Cookie/OAuth lifecycle, authenticated headers, account index, and mutation preconditions match legacy. |
| Actions and playlist manager | Partial | Playlist-removal protocol now matches legacy; every remaining interaction and playlist operation still needs matching request/response contract tests. |
| Player and decipher | Partial | Player selection, cache lifecycle, client fallback, and current-player fixtures are equivalent. |
| Parser | Not started | Renderer registry maps all 574 legacy classes to Rust types or tested equivalent paths. |

## Mandatory evidence for completion

1. A fixture-based contract test covers every public API and every protobuf-producing action.
2. A renderer registry test fails whenever a legacy renderer has no Rust mapping.
3. Opt-in live tests cover anonymous playback, authenticated account access, and mutations.
4. The manifest has no `Missing` or `Partial` entries.
