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

## Current checkpoint — 2026-08-27

The checkpoint below was last synchronized at `c176637`. Always obtain the actual starting revision with `git log -1 --oneline` and inspect `git status --short` before acting.

Recent implementation batches:

- `77f3405`: Batch 8 — added 48 concrete renderer AST nodes via parallel subagents across Grid & Compact, Music Extended, Overlays & Dialogs, and Engagement & Comments.
- `c176637`: Batch 9 — added 48 concrete renderer AST nodes via parallel subagents across Carousels & Views, Cards & Interactive Items, LiveChat Extras, and Commands & Actions.
- Current parser status: `src/parser/registry.rs` has **267 Direct** concrete `YTNode` AST variants with executable parser branches (out of 574 legacy classes), with 129 Container, 134 Element, 40 Endpoint, 4 Kids. All 574 legacy classes are strongly typed with 0 generic fallbacks.

Current evidence:

- `cargo test --all-targets` passes 97 non-network unit/contract tests (40 in `src/lib.rs` + 24 in `tests/contract_fixtures.rs` + 25 in `tests/api_contracts.rs` + 8 in `tests/authenticated_integration.rs`).
- `cargo clippy --all-targets -- -D warnings` passes with 0 warnings.
- All 267 `YTNode` variants have executable deterministic parser fixture tests in `src/parser/registry.rs`.

## Handoff instructions

Read `.agents/AGENTS.md`, this plan, and `PARITY_MANIFEST.md` before making
changes. Inspect `git status --short`, `git log --oneline -6`, and `git diff`
before editing. Continue from the first manifest gap that can be evidenced;
do not restore, reset, or commit changes unless the user explicitly asks.
