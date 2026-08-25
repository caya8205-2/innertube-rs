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
- Current batch: `src/parser/registry.rs` executable `ParserDispatchTarget` mapping with 546 verified entries, `tests/contract_fixtures.rs` deterministic fixture suite with 10 comprehensive tests, and public endpoints for Courses, Subscriptions, Channels, Playlists, Unseen Notifications, and Attestation Challenge with rich feed mixins.

Current evidence is deliberately **not** a 100% parity claim:

- `cargo test --all-targets` passes 54 non-network unit/contract tests (44 in `src/lib.rs` + 10 in `tests/contract_fixtures.rs`).
- `cargo clippy --all-targets -- -D warnings` passes with 0 warnings.
- Ten live integration tests in `tests/live_integration.rs` were executed with
  `cargo test --test live_integration -- --ignored` and passed 10/10 against the live
  YouTube API.
- Phase 1 (`get_info` concurrent composition & sub-manager namespaces `music()`,
  `playlist()`, `interact()`, `actions()`, `account()`, `kids()`), Phase 2 (`Feed<T>` pagination mixin),
  Phase 3 (Container, Button, Menu, and Endpoint AST nodes), Phase 4 (`Actions.execute` / `ApiResponse`),
  and Phase 5 (Public endpoints for Courses, Subscriptions, Channels, Playlists, Unseen Notifications, Attestation Challenge, and rich feed mixins)
  are implemented.
- `src/parser/registry.rs` provides an executable `ParserDispatchTarget` mapping for all 546 registered keys
  to direct AST nodes, containers, elements, or equivalent fallbacks, verified by `test_all_registered_classes_have_executable_dispatch_target`.
- Deterministic fixture contracts in `tests/contract_fixtures.rs` cover Search, Channel tabs, Playlists, Music lists, Comments/Posts, and Actions.
- Public API rows and core rows reflect objective `In progress` or `Partial` status
  in `PARITY_MANIFEST.md`.

## Handoff instructions

Read `.agents/AGENTS.md`, this plan, and `PARITY_MANIFEST.md` before making
changes. Inspect `git status --short`, `git log --oneline -6`, and `git diff`
before editing. Continue from the first manifest gap that can be evidenced;
do not restore, reset, or commit changes unless the user explicitly asks.
