# Struct `Session`

`innertube_rs::core::Session` manages HTTP connections, visitor data protobuf generation, and request contextual headers.

```rust
use innertube_rs::core::{Session, SessionOptions};
```

---

## 1. Options & Initialization

```rust
#[derive(Debug, Clone, Default)]
pub struct SessionOptions {
    pub po_token: Option<String>,
    pub visitor_data: Option<String>,
    pub cookie: Option<String>,
    pub language: Option<String>,
    pub region: Option<String>,
    pub proxy: Option<String>,
}
```

```rust
let options = SessionOptions {
    language: Some("en".to_string()),
    region: Some("US".to_string()),
    ..Default::default()
};
let session = Session::with_options(options).await?;
```

---

## 2. InnerTube API Requests

### `post_innertube(endpoint: &str, payload: Value)`
```rust
pub async fn post_innertube(&self, endpoint: &str, payload: Value) -> Result<reqwest::Response>
```

Sends an authorized InnerTube request to `https://www.youtube.com/youtubei/v1{endpoint}` with automatically populated client context and headers (`X-YouTube-Client-Name`, `X-YouTube-Client-Version`, `X-Goog-Visitor-Id`).
