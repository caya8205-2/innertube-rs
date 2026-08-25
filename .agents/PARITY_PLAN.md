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
4. Build an executable renderer coverage map from all legacy parser classes.
   A category inventory alone is insufficient: the coverage test must link
   every class to a Rust dispatch branch or a named, tested equivalent fixture
   and fail for any unmapped class.
5. Harden transport and authentication. Verify every direct request path has
   contextual non-2xx handling and that authenticated browse/action behavior
   matches legacy.
6. Add opt-in live tests for anonymous playback, authenticated feeds, and
   mutations. Keep credentials and network tests out of default test runs.
7. Update the manifest only with evidence. Completion requires no `Missing`
   or `Partial` rows and all mandatory evidence listed in the manifest.

## Current checkpoint — 2026-08-25

The checkpoint below was last synchronized at `6cc9d02`. It may be stale;
always obtain the actual starting revision with `git log -1 --oneline` and
inspect `git status --short` before acting.

Recent implementation batches:

- `fb29a3f`: docs sync and opt-in live integration test suite.
- `ac155f2`: VideoInfo concurrent composition, sub-manager namespaces (`music`, `playlist`, `interact`, `account`, `kids`), Feed mixins, and container AST nodes.
- `58a0afa`: Button, Menu, Overlay AST node expansions and `Actions.execute` dispatcher.
- `a021912`: Live integration tests expanded to 10 endpoints (10/10 passing).
- `8a7fcd8`: Documentation sync and baseline consistency.
- `aa714d6`: Public endpoints for Courses, Subscriptions, Channels, Playlists, Unseen Notifications, and Attestation Challenge with rich feed mixins.
- Current batch: `src/parser/registry.rs` exhaustive 574/574 legacy parser class mapping with strongly typed `ParserDispatchTarget` enum targets (`YTNodeVariant`, `ContainerKind`, `EndpointKind`, `ElementKind`), `tests/api_contracts.rs` covering all 25 legacy public APIs, `tests/authenticated_integration.rs` testing mutation payload contracts and safe live mutation runner with cleanup, and expanded AST nodes (`VideoPrimaryInfo`, `VideoSecondaryInfo`, `ReelShelf`, `PlaylistPanel`, `PlaylistPanelVideo`, `CreatorHeart`, `ChipCloud`, `ChipCloudChip`).

Current evidence is deliberately **not** a 100% parity claim:

- `cargo test --all-targets` passes 83 non-network unit/contract tests (40 in `src/lib.rs` + 10 in `tests/contract_fixtures.rs` + 25 in `tests/api_contracts.rs` + 8 in `tests/authenticated_integration.rs`).
- `cargo clippy --all-targets -- -D warnings` passes with 0 warnings.
- Ten live integration tests in `tests/live_integration.rs` were executed with
  `cargo test --test live_integration -- --ignored` and passed 10/10 against the live
  YouTube API.
- Eleven opt-in live tests exist in total (10 anonymous + 1 reversible authenticated mutation with cleanup).
- `src/parser/registry.rs` provides an executable `ParserDispatchTarget` mapping for all 574 registered legacy parser classes
  to typed AST node variants, containers, endpoints, elements, or equivalent fallbacks, verified by `test_all_574_legacy_classes_are_registered_with_concrete_targets` and `test_all_ytnode_variants_have_executable_parsers`.
- Dedicated API contracts in `tests/api_contracts.rs` cover request construction, response handling, and parameters for all 25 public `Innertube` APIs.
- Public API rows and core rows reflect objective `In progress` or `Partial` status
  in `PARITY_MANIFEST.md`.

## Handoff instructions

Read `.agents/AGENTS.md`, this plan, and `PARITY_MANIFEST.md` before making
changes. Inspect `git status --short`, `git log --oneline -6`, and `git diff`
before editing. Continue from the first manifest gap that can be evidenced;
do not restore, reset, or commit changes unless the user explicitly asks.
