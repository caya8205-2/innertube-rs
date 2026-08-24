# Architecture & Internal Design

`innertube-rs` is designed from the ground up as a native, asynchronous Rust client for YouTube's internal InnerTube API (`/youtubei/v1`).

---

## 1. System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                       Innertube                             │
│                  (High-Level Client)                        │
└───────────────┬─────────────────────────────┬───────────────┘
                │                             │
                ▼                             ▼
┌───────────────────────────────┐ ┌───────────────────────────┐
│            Session            │ │          Player           │
│  - Protobuf Visitor Data      │ │  - QuickJS Sandbox        │
│  - PO-Token & Context Headers │ │  - Signature Decipher     │
│  - Multi-Client Dispatch      │ │  - n-Token Transformation │
└───────────────┬───────────────┘ └───────────────────────────┘
                │
                ▼
┌───────────────────────────────┐
│        Endpoints Layer        │
│  - Player (/player)           │
│  - Browse (/browse)           │
│  - Search (/search)           │
│  - Next (/next)               │
│  - Music, Comments, Feeds...  │
└───────────────┬───────────────┘
                │
                ▼
┌───────────────────────────────┐
│     Modular AST Parser        │
│  - Polymorphic JSON Parser    │
│  - Strongly Typed YTNodes     │
└───────────────────────────────┘
```

---

## 2. Key Components

### A. Session & Contextual Headers (`src/core/session.rs`)
InnerTube API endpoints require specific contextual payloads (`context.client.clientName`, `clientVersion`, `visitorData`, `hl`, `gl`). `Session` handles constructing these payloads, computing dynamic protobuf visitor data, and attaching required headers (`X-YouTube-Client-Name`, `X-Goog-Visitor-Id`).

### B. Embedded Decipher Engine (`src/core/player.rs` & `src/utils/quickjs.rs`)
YouTube obfuscates video streaming URLs using:
1. **Signature Decipher (`s` / `sig`)**: Unscrambling arrays using slice, reverse, and swap operations.
2. **Bandwidth Throttling (`n` token)**: Parameter transformation functions executed at runtime.

`innertube-rs` uses `rquickjs` (an embedded QuickJS WebAssembly/C engine) to evaluate the JavaScript transformations in **<5ms** without spawning external Node.js or Python processes.

### C. Modular AST Parser (`src/parser/`)
Instead of manual string slicing or rigid schema bindings, the parser walks arbitrary InnerTube JSON responses recursively and converts known renderers into strongly typed `YTNode` enum variants.
