# Model: OAuth (`src/models/oauth.rs`)

```rust
use innertube_rs::models::oauth::{OAuth2Tokens, DeviceAndUserCode};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAndUserCode {
    pub device_code: String,
    pub user_code: String,
    pub expires_in: u64,
    pub interval: u64,
    pub verification_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
}
```
