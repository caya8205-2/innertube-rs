# Model: Live Chat (`src/models/live_chat.rs`)

```rust
use innertube_rs::models::live_chat::{
    LiveChatResponse,
    LiveChatMessage,
    LiveChatTextMessage,
    LiveChatSuperChat,
    LiveChatMembership,
    LiveChatSystemMessage,
};
```

---

## Enums & Structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChatResponse {
    pub messages: Vec<LiveChatMessage>,
    pub continuation_token: Option<String>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LiveChatMessage {
    Text(LiveChatTextMessage),
    SuperChat(LiveChatSuperChat),
    Membership(LiveChatMembership),
    System(LiveChatSystemMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChatTextMessage {
    pub id: String,
    pub timestamp_usec: u64,
    pub author_name: String,
    pub author_photo: Option<String>,
    pub author_channel_id: Option<String>,
    pub message: String,
    pub is_owner: bool,
    pub is_moderator: bool,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChatSuperChat {
    pub id: String,
    pub timestamp_usec: u64,
    pub author_name: String,
    pub author_photo: Option<String>,
    pub purchase_amount_text: String,
    pub message: Option<String>,
}
```
