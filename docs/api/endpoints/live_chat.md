# Endpoint: Live Chat (`src/endpoints/live_chat.rs`)

Extracts tokens and polls real-time live chat streams.

```rust
use innertube_rs::endpoints::live_chat::{extract_live_chat_continuation_token, get_live_chat};
```

---

## Functions

### `extract_live_chat_continuation_token(watch_next_raw: &Value) -> Option<String>`
Extracts the initial live chat continuation token from the `/next` response payload.

### `get_live_chat(session: &Session, token: &str) -> Result<LiveChatResponse>`
Polls active live stream chat messages (`Text`, `SuperChat`, `Membership`, `System`), returning parsed messages, polling timeouts, and the next continuation token.
