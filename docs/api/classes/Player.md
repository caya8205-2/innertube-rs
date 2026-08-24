# Struct: `Player`

`innertube_rs::core::Player` manages player deciphering algorithms, signature unscrambling (`sig` decipher), and `n-token` parameter transformation using an embedded QuickJS runtime.

```rust
use innertube_rs::core::Player;
```

---

## 1. Overview & Embedded Engine

YouTube protects playable streaming URLs using signature transformations (`s` parameter) and throttling parameter transformations (`n` parameter). 

`innertube-rs` implements an embedded QuickJS sandbox (`rquickjs` with parallel runtime support) to execute YouTube's official player decipher functions in **<5ms** with zero external dependencies.

---

## 2. Key Methods

### `Player::from_session(session: &Session)`
```rust
pub async fn from_session(session: &Session) -> Result<Self>
```
Fetches the active YouTube base player JavaScript file (e.g. `base.js`), extracts the decipher and n-token algorithm functions via regex AST introspection, and evaluates them inside the QuickJS runtime context.

### `decipher(signature: &str)`
```rust
pub fn decipher(&self, signature: &str) -> Result<String>
```
Deciphers a video format signature.

### `transform_n_token(n_token: &str)`
```rust
pub fn transform_n_token(&self, n_token: &str) -> Result<String>
```
Transforms an n-token to bypass YouTube stream bandwidth throttling.

### `decipher_url(url: &str)`
```rust
pub fn decipher_url(&self, url: &str) -> Result<String>
```
Takes a raw stream URL, deciphers signatures (`s`), applies `sp`, and transforms `n`, returning a direct playable stream URL.
