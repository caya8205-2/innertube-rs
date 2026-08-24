# Endpoint: Account (`src/endpoints/account.rs`)

Fetches authenticated user feeds and notifications.

```rust
use innertube_rs::endpoints::account::{get_history, get_library, get_notifications};
```

---

## Functions

### `get_history(session: &Session) -> Result<HistoryFeed>`
Fetches the logged-in user's watch history (`FEhistory`).

### `get_library(session: &Session) -> Result<LibraryFeed>`
Fetches the user's library sections, saved playlists, and watch later items (`FElibrary`).

### `get_notifications(session: &Session) -> Result<AccountNotificationsResponse>`
Fetches real-time channel and comment notifications.
