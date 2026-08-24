# Struct: `Session`

`innertube_rs::core::Session` manages HTTP connections, cookies, visitor data protobuf generation, and request contextual headers for InnerTube API calls.

```rust
use innertube_rs::core::Session;
```

---

## 1. Fields & Configuration

* **`context`**: Contextual JSON payload containing client information (`hl`, `gl`, `visitorData`, `clientName`, `clientVersion`).
* **`visitor_data`**: Base64 protobuf string representing unique visitor token.
* **`api_key`**: YouTube API key (e.g. `INNERTUBE_API_KEY`).
* **`po_token`**: Proof-of-Origin token for passing BotGuard integrity checks.
* **`http_client`**: Underlying `reqwest::Client` configured with HTTP/2 and Rustls TLS.

---

## 2. Key Methods

### `Session::new()`
```rust
pub async fn new() -> Result<Self>
```
Initializes a new session, fetches initial visitor data from YouTube, and builds the default `WEB` InnerTube context.

### `Session::with_options(options: SessionOptions)`
```rust
pub async fn with_options(options: SessionOptions) -> Result<Self>
```
Initializes a session with custom locale, proxy, cookie storage, or tokens.

### `post_innertube(endpoint: &str, payload: Value)`
```rust
pub async fn post_innertube(&self, endpoint: &str, payload: Value) -> Result<reqwest::Response>
```
Sends a standard POST request to an InnerTube endpoint (e.g. `/browse`, `/search`, `/player`) with automatic header injection (`X-YouTube-Client-Name`, `X-YouTube-Client-Version`, `X-Goog-Visitor-Id`).
