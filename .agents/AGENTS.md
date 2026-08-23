# Agent Instructions — innertube-rs

## 1. Project Overview & Mission

`innertube-rs` is a high-performance, asynchronous, zero-bloat **pure Rust port of [YouTube.js (LuanRT/YouTube.js)](https://github.com/LuanRT/YouTube.js)** — YouTube's internal API client (known as InnerTube).

### Why this project exists:
- **Pain Point:** Tooling in the local workspace:
  - **`noctune`** (`C:/Users/Caya/Desktop/Project/music-player`): Desktop music player that has working Rust scraping in `src-tauri/src/youtube_channel.rs` (~5ms), but still depends on a bloated Node.js sidecar for streaming.
  - **`avpull`** (`C:/Users/Caya/Desktop/Project/avpull`): CLI audio/video downloader needing a fast, lightweight native engine.
  - Both traditionally relied on `youtubei.js` (Node/Bun runtime overhead) or `yt-dlp` subprocesses (3+ seconds latency, high memory).
- **Ecosystem Gap:** Existing third-party Rust crates (`rusty_ytdl`, `rustube`) frequently break and suffer from maintenance lag whenever YouTube changes their internal APIs or obfuscation logic.
- **Solution:** Maintain our own native Rust InnerTube engine (`innertube-rs`). We have full control over API patches, zero runtime overhead, minimal binary footprint, and maximum performance across our workspace projects.

---

## 2. Core Operating Rules for Agents

### A. Scope & Focus
1. **Strict Request Scope:** Only touch, create, or refactor code explicitly requested. Do not perform unsolicited broad architectural refactors or "cleanups".
2. **Incremental & Pragmatic Porting:** We do **not** need 100% feature parity with `YouTube.js` on day one. Focus first on core needs:
   - Session & InnerTube context creation
   - Video metadata & stream URL extraction (deciphering signatures / n-tokens)
   - Channel & playlist scraping
   - Search functionality
3. **Preserve Reference Integrity:** The existing JS/TS files in this repository serve as the upstream reference implementation. Do not delete or mangle reference code unless explicitly instructed to clean up.
4. **Idiomatic Rust Over Direct Translation:** Do not blindly translate JavaScript's dynamic typing / loose objects into complex `serde_json::Value` hell. Design clean, strongly typed Rust structs and enums with `serde` wherever feasible, falling back to dynamic parsing only where YouTube schemas are highly polymorphic.

### B. Technical Standards
- **Language Standard:** Rust (Edition 2021+)
- **Async Runtime:** `tokio`
- **HTTP Client:** `reqwest` (with rustls / native-tls)
- **Serialization:** `serde`, `serde_json`
- **Documentation & Code Language:** All code, comments, docstrings, commit messages, and documentation must be written in **English**. (Communication with the user remains in **Bahasa Indonesia**).
- **Safety First:** Avoid `unwrap()` / `expect()` in library code; use proper error handling (`thiserror` / `anyhow` for CLI/examples).

---

## 3. Porting Strategy & Architecture Mapping

| YouTube.js (JS/TS) Module | Rust Equivalent / Target | Purpose | Priority |
|---|---|---|---|
| `src/core/Session.ts` | `src/core/session.rs` | Client context, visitor data, API keys, HTTP headers | **High** |
| `src/core/Player.ts` | `src/core/player.rs` | Decipher algorithms (n-token, signature timestamp) | **High** |
| `src/parser/` | `src/parser/` | Parsing InnerTube renderers / JSON responses | **Medium** |
| `src/core/endpoints/` | `src/endpoints/` | Player, Browse, Search, Next API calls | **High** |
| `src/actions/` | `src/actions/` | High-level API wrappers (Video, Channel, Playlist) | **Medium** |

---

## 4. Verification & Testing Rule
- Every implemented module must be accompanied by unit tests or integration examples using real/mocked InnerTube payload fixtures.
- When adding network-dependent functionality, provide runnable examples in `examples/` for quick sanity verification.
