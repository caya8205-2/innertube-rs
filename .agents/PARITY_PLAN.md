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

The current HEAD is `7d01b05 test(parity): add opt-in live integration test
suite`. Inspect `git status --short` before acting; documentation corrections
may be present after this checkpoint.

Recent implementation batches:

- `afa934f`: basic-info, Shorts, search-filter, comment, and streaming option
  contracts.
- `e10de98`: playlist and interaction-manager operations plus a 574-name
  parser inventory.
- `7d01b05`: eight opt-in live integration tests.

Current evidence is deliberately **not** a 100% parity claim:

- `cargo test --all-targets` passes 43 non-network unit/contract tests.
- `cargo clippy --all-targets -- -D warnings` passes with 0 warnings.
- Ten live integration tests in `tests/live_integration.rs` were executed with
  `cargo test --test live_integration -- --ignored` and passed 10/10 against the live
  YouTube API.
- Phase 1 (`get_info` concurrent composition & sub-manager namespaces `music()`,
  `playlist()`, `interact()`, `actions()`, `account()`, `kids()`), Phase 2 (`Feed<T>` pagination mixin),
  Phase 3 (Container, Button, Menu, and Endpoint AST nodes), and Phase 4 (`Actions.execute` / `ApiResponse`)
  are implemented.
- `src/parser/registry.rs` catalogs and categorizes 574 legacy class names, with
  strongly typed AST node branches tested.
- Public API rows and core rows reflect objective `In progress` or `Partial` status
  in `PARITY_MANIFEST.md`.

The next agent must work from the manifest gaps, not repeat the implementation
checklist as though it were a completion report.

## Handoff instructions

Read `.agents/AGENTS.md`, this plan, and `PARITY_MANIFEST.md` before making
changes. Inspect `git status --short`, `git log --oneline -6`, and `git diff`
before editing. Continue from the first manifest gap that can be evidenced;
do not restore, reset, or commit changes unless the user explicitly asks.
