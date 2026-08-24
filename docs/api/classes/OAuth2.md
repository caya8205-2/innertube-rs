# Struct: `OAuth2`

`innertube_rs::core::OAuth2` implements Google TV / YouTube device code authorization flow.

```rust
use innertube_rs::core::oauth::OAuth2;
```

---

## 1. Flow Overview

```
Client                             Google OAuth2 Server
  │                                         │
  ├─── 1. Request User & Device Code ──────>│
  │<── Returns user_code & verification_url ┘
  │
  ├─── (User visits verification_url)
  │
  ├─── 2. Poll for Access & Refresh Tokens ─>│
  │<── Returns access_token, refresh_token ─┘
```

---

## 2. Key Methods

### `request_device_code(session: &Session)`
```rust
pub async fn request_device_code(session: &Session) -> Result<DeviceAndUserCode>
```
Requests a device code from Google OAuth TV endpoint. Returns:
- `user_code` (e.g. `ABCD-EFGH`)
- `verification_url` (`https://www.google.com/device`)
- `expires_in` (seconds)
- `interval` (polling interval in seconds)

### `poll_for_tokens(session: &Session, device_code: &str, interval: u64)`
```rust
pub async fn poll_for_tokens(session: &Session, device_code: &str, interval: u64) -> Result<OAuth2Tokens>
```
Polls Google OAuth servers until the user authorizes the login prompt on their device.

### `refresh_token(session: &Session, refresh_token: &str)`
```rust
pub async fn refresh_token(session: &Session, refresh_token: &str) -> Result<OAuth2Tokens>
```
Refreshes an expired access token using the stored refresh token.
