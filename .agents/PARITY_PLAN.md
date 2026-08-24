# Full Parity Execution Plan

## Authority and baseline

The target is `reference-youtubejs` at commit
`85473772ce9a9238091636fc2cb7ea3c331ea88d`. `PARITY_MANIFEST.md` is the
source of truth for whether parity has been proven. Historical completion
matrices and archived plans are not evidence of parity.

Full parity requires equivalent public behavior, request construction,
authentication requirements, errors, and parsed data. A similarly named Rust
method is not sufficient.

## Working order

1. Preserve the current audit checkpoint and validate the dirty worktree
   before adding new changes. Do not discard or overwrite user changes.
2. Build fixture-based request and response contract tests for each public
   `Innertube` method. Port missing methods before marking a row as `Partial`.
   Start with `getBasicInfo`, `getShortsVideoInfo`, search filters, comments,
   and streaming/download options.
3. Audit every manager action against its legacy endpoint class. Match the
   exact endpoint, payload, protobuf, authentication check, and response
   contract. Continue with playlist library, move, title, and description
   operations after the already fixed rating and playlist-removal payloads.
4. Build a renderer inventory from all legacy parser classes. Add a registry
   test that fails for an unmapped renderer, then port or document a tested
   equivalent path for each one.
5. Harden transport and authentication. Verify every direct request path has
   contextual non-2xx handling and that authenticated browse/action behavior
   matches legacy.
6. Add opt-in live tests for anonymous playback, authenticated feeds, and
   mutations. Keep credentials and network tests out of default test runs.
7. Update the manifest only with evidence. Completion requires no `Missing`
   or `Partial` rows and all mandatory evidence listed in the manifest.

## Current checkpoint — 2026-08-25

The latest local commit is `fafedb2 feat(parity): add browse, streaming, and endpoint compatibility`. The following subsequent changes are intentionally
uncommitted and must be preserved:

- `get_basic_info` and `get_shorts_video_info` with `VideoInfo`, `ShortFormVideoInfo`, and `ReelSequence` protobuf encoding.
- Typed `SearchFilters` with `SearchFilter` protobuf serialization for `/search`.
- `get_search_suggestions_with_options` with `previous_query` and session cookies.
- `get_comments_with_options` with `GetCommentsSectionParams` protobuf continuation tokens, sort options, and comment ID filtering.
- `get_streaming_data_with_options` and `download_with_options` supporting rich `FormatOptions` and HTTP byte ranges (`DownloadRange`).
- Fixture-based contract tests for all added protobuf encoders and format selections.
- Manifest updates reflecting these additions.

Touched files: `.agents/PARITY_MANIFEST.md`, `.agents/PARITY_PLAN.md`, `src/endpoints/comments.rs`,
`src/endpoints/player.rs`, `src/endpoints/search.rs`, `src/endpoints/suggestions.rs`, `src/lib.rs`,
`src/models/comments.rs`, `src/models/format.rs`, `src/models/search.rs`, `src/models/video.rs`,
and `src/utils/proto.rs`.

Last local validation: `cargo test --all-targets` passed 34 tests;
`cargo clippy --all-targets -- -D warnings` and `cargo check --examples` passed.
The current worktree is not yet a full-parity implementation.

## Handoff instructions

Read `.agents/AGENTS.md`, this plan, and `PARITY_MANIFEST.md` before making
changes. Inspect `git status --short` and `git diff` before editing. Continue
from the first applicable incomplete plan item; do not restore, reset, or
commit the existing changes unless the user explicitly asks.
