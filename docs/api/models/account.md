# Model: Account (`src/models/account.rs`)

```rust
use innertube_rs::models::account::{
    HistoryFeed,
    LibraryFeed,
    AccountNotificationsResponse,
    AccountNotificationItem,
};
```

---

## Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFeed {
    pub sections: Vec<HistorySection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryFeed {
    pub stats: Vec<LibraryStat>,
    pub playlists: Vec<PlaylistNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountNotificationsResponse {
    pub notifications: Vec<AccountNotificationItem>,
}
```
