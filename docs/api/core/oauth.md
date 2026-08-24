# Struct `OAuth2`

`innertube_rs::core::oauth::OAuth2` manages Google TV device authentication flow.

```rust
use innertube_rs::core::oauth::OAuth2;
```

---

## Key Methods

### `request_device_code(session: &Session)`
```rust
pub async fn request_device_code(session: &Session) -> Result<DeviceAndUserCode>
```
Requests a device authorization code from Google OAuth TV servers.

### `poll_for_tokens(session: &Session, device_code: &str, interval: u64)`
```rust
pub async fn poll_for_tokens(session: &Session, device_code: &str, interval: u64) -> Result<OAuth2Tokens>
```
Polls Google authorization servers until the user confirms the code on `https://www.google.com/device`.

### `refresh_token(session: &Session, refresh_token: &str)`
```rust
pub async fn refresh_token(session: &Session, refresh_token: &str) -> Result<OAuth2Tokens>
```
Refreshes an expired access token using the stored refresh token.
