# Struct `Player`

`innertube_rs::core::Player` manages player deciphering algorithms, signature unscrambling (`sig`), and `n-token` parameter transformation using embedded QuickJS.

```rust
use innertube_rs::core::Player;
```

---

## 1. Embedded Sandbox Engine

`innertube-rs` embeds QuickJS (`rquickjs`) to run official YouTube player JavaScript transformations in isolated environments with sub-millisecond execution times.

---

## 2. Key Methods

### `Player::from_session(session: &Session)`
```rust
pub async fn from_session(session: &Session) -> Result<Self>
```
Fetches the active YouTube player (`base.js`), parses decipher regex routines, and evaluates them inside the QuickJS runtime context.

### `decipher(signature: &str)`
```rust
pub fn decipher(&self, signature: &str) -> Result<String>
```
Deciphers an obfuscated video signature.

### `transform_n_token(n_token: &str)`
```rust
pub fn transform_n_token(&self, n_token: &str) -> Result<String>
```
Transforms an n-token to bypass bandwidth throttling.

### `decipher_url(url: &str)`
```rust
pub fn decipher_url(&self, url: &str) -> Result<String>
```
Transforms a raw stream URL into a direct playable stream URL.
