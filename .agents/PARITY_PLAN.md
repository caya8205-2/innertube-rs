# Full Parity Execution Plan

## Authority and baseline

The target is `reference-youtubejs` at commit
`85473772ce9a9238091636fc2cb7ea3c331ea88d`. `PARITY_MANIFEST.md` is the
source of truth for whether parity has been proven. Historical completion
matrices and archived plans are not evidence of parity.

Full parity requires equivalent public behavior, request construction,
authentication requirements, errors, and parsed data. A similarly named Rust
method is not sufficient.

## Gap audit — 2026-09-04 (HEAD `cde65a5`, worktree clean, 102 tests + 0 clippy)

Full source-level audit of `reference-youtubejs` (all non-parser-class files)
against `src/` produced the gap list below. Parser *class* coverage is done
(574/574 dispatch targets); the remaining work is **behavioral** parity in
transport, session, auth, player, managers, parser infrastructure, protobufs,
and streaming utilities.

## Batch plan

Each batch ends with: implementation + contract/unit tests, `cargo test
--all-targets` green, `cargo clippy --all-targets -- -D warnings` clean,
checkpoint updated below with HEAD commit and validation output. QA verifies
before the next batch starts.

### Batch 1 — HTTP transport & client context parity
Legacy: `utils/HTTPClient.ts` (`#adjustContext`, per-client headers/UA),
`Session.ts` context building.
- `Session::post_innertube_client`: full client adjustment — per-client
  version/UA/os/sdk (MWEB, IOS, ANDROID, ANDROID_VR, VISIONOS, ANDROID_MUSIC,
  ANDROID_CREATOR, TVHTML5, TVHTML5_SIMPLY, TV_EMBEDDED, WEB_EMBEDDED,
  WEB_CREATOR, WEB_KIDS); delete `configInfo` for non-WEB; `clientScreen:
  EMBED` + `thirdParty.embedUrl` for embedded clients; `X-GOOG-API-FORMAT-
  VERSION: 2` for Android-family; reject unknown clients with the supported
  list (legacy error semantics).
- `prettyPrint=false&alt=json` on every API call; visitor/client headers on
  all requests.
- Remove or repurpose dead `src/core/http_client.rs`.
- Context completeness: `utcOffsetMinutes`, `memoryTotalKbytes`,
  `mainAppWebInfo`, `configInfo.appInstallData`, `user.onBehalfOfUser`,
  screen fields — match legacy `#buildContext`.
- Tests: header/context contract tests per client family.

### Batch 2 — `Actions::execute` request-munging parity
Legacy: `core/Actions.ts` `execute()`.
- Arg munging: `action`→`actions:[...]`, `boolValue`→`newValue.boolValue`,
  `token`→`continuation`; strip control keys (`skip_auth_check`,
  `override_endpoint`, `parse`, `request`, `clientActions`,
  `settingItemIdForClient`); `override_endpoint` swaps path.
- `isAudioOnly: true` for YTMUSIC client; `protobuf: true` sends raw bytes
  with `application/x-protobuf`.
- Login-gated browseId list (FElibrary, FEhistory, FEsubscriptions,
  FEchannels, FEplaylist_aggregation, FEmusic_listening_review,
  FEmusic_library_landing, SPaccount_*, SPtime_watched) →
  `AuthenticationRequired` unless `skip_auth_check`.
- `parse: true` → parsed response; `navigateAction` redirect follow
  (recursion, same as `Innertube.getChannel`).
- Tests: payload-munging contract tests per rule; redirect fixture.

### Batch 3 — Auth layering & OAuth2 parity
Legacy: `core/OAuth2.ts`, HTTPClient auth block, `Utils.generateSidAuth`.
- SAPISIDHASH header generation (`{ts}_{sha1(...)}`), `X-Goog-Authuser`
  (account_index), `X-Goog-PageId` (onBehalfOfUser), Cookie forwarding.
- OAuth bearer on authenticated requests with auto-refresh when expired;
  WEB_KIDS excluded from auth headers.
- OAuth2: `poll_for_access_token` interval/backoff semantics
  (`authorization_pending`/`slow_down` continue; `access_denied`/
  `expired_token`/unknown fatal), expiry tracking + `should_refresh`,
  `refresh_access_token`, `revoke`, credential persistence.
- `Session::sign_in`/`sign_out` with auth state transitions; mutations
  rejected anonymously (already partly done — extend to all gated paths).
- Tests: sid-auth known-vector test; OAuth state machine contract tests
  (mocked token endpoint); auth-header presence/absence contract tests.

### Batch 4 — Player & decipher parity
Legacy: `core/Player.ts`, `Utils.getNsigProcessorFn`.
- `signatureTimestamp`: extraction failure must error, not default 0.
- Per-response n-token cache (dedup within one player response).
- `pot=` appended unless `sabr=1`; `cver` rewrite per `c` param for all
  clients in legacy switch.
- Player script fetch: version-locked player-id cache (skip legacy
  BinarySerializer gzip format; use own cache format, document deviation).
- Robustness: keep QuickJS whole-script eval but add fallback nsig matcher
  for player variants; keep `enhanced_except_` sentinel handling.
- Tests: fixed base.js fixture with known n/sig vectors; URL rewrite
  contract tests (pot, cver, sabr skip, cache dedup).

### Batch 5 — Protobuf surface completion
Legacy: `protos/misc/params.proto` + `protos/youtube/api/pfiinnertube/*`.
- Compile the 16 `pfiinnertube` protos in `build.rs` (needed by Batch 8
  Studio + protobuf-typed requests).
- Add encoders for compiled-but-unused messages where legacy uses them:
  `PeformCommentActionParams` (comment translate), `NextParams`.
- Verify wire-format equality against legacy golden vectors (encode →
  base64url → URI-encode chain) for every existing encoder.
- Tests: golden-vector roundtrips per message.

### Batch 6 — Manager surface completion
Legacy: `core/managers/*`, `core/clients/{Music,Kids,Studio}.ts`.
- `InteractionManager`: add `translate` (comment action params type 22).
- `PlaylistManager`: `remove_videos`/`move_video` recursive continuation
  pagination until all setVideoIds resolved; `is_editable` precondition;
  legacy payload quirks (`playlist_id` snake key in setName vs `playlistId`
  in setDescription).
- `AccountManager`: `get_info` (accounts list endpoint, `all` variant with
  TV client) and `get_settings` (`SPaccount_overview`).
- `MusicManager`: add `get_playlist`, `get_library`, `get_search_suggestions`,
  `get_up_next` (automix endpoint follow), `get_related`, `get_recap`;
  `get_lyrics` Message-throw semantics; YTMUSIC client on all calls.
- `KidsManager`: `get_info`, `get_channel`, `block_channel`
  (getKidsBlocklistPickerCommand + per-item toggle); parse responses into
  typed Kids nodes instead of raw `Value`.
- Tests: request-contract tests per method; continuation-pagination fixture
  for playlist setVideoId resolution.

### Batch 7 — Parser response-assembly parity
Legacy: `parser/parser.ts` `parseResponse`, mutations, continuations.
- Top-level response struct: `playability_status` (status/reason/
  embeddable/audio_only/error_screen), `streaming_data` (expires,
  formats/adaptive_formats), `captions`, `storyboards`, `endscreen`,
  `cards`, `engagement_panels`, `player_overlays`, `annotations`,
  `metadata`, `microformat`, `bg_challenge` (BotGuard fields), `alerts`,
  `refinements`, `estimated_results`, `playback_tracking`, `endpoint`,
  `current_video_endpoint`.
- `frameworkUpdates.entityBatchUpdate` mutation application: comments
  enrichment, multi-select `selected`, heatmap append.
- Continuation wrappers: timed/invalidation/replay variants;
  `PlaylistPanelContinuation` nextRadio fallback.
- `IGNORED_LIST` silent skip + unknown-node reporting hook (legacy
  `setParserErrorHandler` equivalent).
- Tests: fixture tests per response section + mutation fixtures.

### Batch 8 — Studio client (upload + metadata)
Legacy: `core/clients/Studio.ts`.
- `update_video_metadata` — `MetadataUpdateRequest` protobuf POST
  (`/video_manager/metadata_update`, hardcoded Android context fields).
- `upload` — 3-phase resumable upload (start headers, chunk upload,
  `/upload/createvideo`) with exact header names.
- All methods login-gated.
- Tests: protobuf golden vector; upload-flow header contract tests with
  mocked transport.

### Batch 9 — Streaming & format utilities parity
Legacy: `utils/FormatUtils.ts`, `utils/StreamingInfo.ts`,
`utils/DashManifest.tsx`, `MediaInfo.ts`.
- `chooseFormat` full semantics: itag shortcut, audio `is_original`/
  language preference, bestefficiency/best bitrate sort, error message.
- `download`: chunked `range=` **query param** streaming (10MB), STREAM_
  HEADERS, `cpn` append, playability guards (UNPLAYABLE/LOGIN_REQUIRED),
  live/post-live-DVR rejection.
- `getStreamingInfo`: audio/video/image/text sets, OTF segment template
  (`&rn=0&sq=0` scrape of `Segment-Durations-Ms`), post-live-DVR HEAD
  headers, SABR `sabr://` URLs, DRC/VB labels, CICP color mapping.
- `toDash` MPD generation (SegmentBase vs SegmentTemplate, DRM
  ContentProtection UUIDs, XML escaping rules).
- `MediaInfo` behaviors: `getTranscript` via engagement-panel endpoint
  (replace timedtext-only path), `addToWatchHistory`, `updateWatchTime`.
- Tests: format-selection vectors; chunk-range contract; DASH golden XML;
  OTF/DVR header fixtures.

### Batch 10 — Feed/mixin behavior parity
Legacy: `core/mixins/{Feed,FilterableFeed,TabbedFeed}.ts`.
- `Feed` memo concatenation from 7 sources; header-continuation exclusion;
  `videos`/`playlists`/`posts`/`channels`/`shelves` typed getters incl.
  LockupView content_type filtering.
- `FilterableFeed`: chip extraction (FeedFilterChipBar/ChipCloudChip,
  ChipView dropdown path), `getFilteredFeed` no-op-when-selected short-
  circuit, error listing available filters.
- `TabbedFeed`: case-insensitive tab lookup, selected-tab short-circuit,
  URL-path tab matching.
- `Innertube::get_channel`: `navigateAction` redirect follow.
- Replace `get_library` positional 8/8/rest heuristic with typed parsing;
  fix hardcoded `continuation_token: None` in notifications.
- Tests: memo-concat fixtures; filter resolution + short-circuit tests;
  tab lookup tests.

### Batch 11 — Final evidence & manifest closure
- Every manifest row: named fixture/contract test evidence recorded.
- Opt-in live tests: anonymous playback, authenticated feeds, mutations —
  rerun and record pass counts.
- Manifest rows move to `Complete` only with named evidence; delete any
  row that cannot be evidenced instead of marking it.
- Final: `cargo test --all-targets`, clippy clean, live suite recorded.

## Current checkpoint — 2026-09-04 (Batch 1 done, uncommitted)

HEAD `cde65a5` + uncommitted Batch 1 changes. `cargo test --all-targets`:
118 non-network tests pass (45 lib + 25 api_contracts + 24 contract_fixtures
+ 16 client_contexts NEW + 8 authenticated; 4 auth mutations + 10 live
ignored opt-in). `cargo clippy --all-targets -- -D warnings`: 0 warnings.

Batch 1 delivered:
- Full per-client `Session::adjust_context` port (14 aliases from
  `SUPPORTED_CLIENTS`, exact legacy versions/UA/os/sdk, configInfo removal,
  EMBED screens + thirdParty embedUrls, kidsAppInfo).
- `post_innertube_client` reworked: context adjustment, per-client header
  overrides (Android-family UA + `X-GOOG-API-FORMAT-VERSION: 2`), payload
  `client` key stripping; stale hardcoded WEB_REMIX version removed.
- Complete context fields: screen metrics, UI theme, originalUrl,
  browserName/Version (from sw.js_data), memoryTotalKbytes, rolloutToken,
  deviceExperimentId, mainAppWebInfo, configInfo (incl. appInstallData +
  cold/hot hashes via `/v1/config` fetch, `retrieve_innertube_config`
  default true), onBehalfOfUser option, internalExperimentFlags.
- Dead `src/core/http_client.rs` deleted; player fallback helpers
  (ANDROID/iOS/ANDROID_VR/MWEB) unified through `post_innertube_client`;
  callers switched to legacy aliases (`YTMUSIC`, `YTKIDS`).
- New evidence: `tests/client_contexts.rs` (16 contract tests).

Known deviations (documented): `utcOffsetMinutes` pinned to 0/UTC (no tz
dep); config-fetch failure swallowed silently (no log facade).

State: Batch 1 complete pending QA. Next action: Batch 2
(`Actions::execute` request munging).

## Handoff instructions

Read `.agents/AGENTS.md`, this plan, and `PARITY_MANIFEST.md` before making
changes. Inspect `git status --short`, `git log --oneline -6`, and `git diff`
before editing. Execute batches in order; do not restore, reset, or commit
changes unless the user explicitly asks. Update the checkpoint after every
batch with actual HEAD, validation results, and remaining gap.
