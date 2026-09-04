use serde::{Deserialize, Serialize};

/// A channel belonging to the signed-in account (legacy `AccountItem`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountChannelItem {
    pub account_name: String,
    pub account_photo: Option<String>,
    pub is_selected: bool,
}
use crate::parser::nodes::video::VideoNode;

/// User watch history feed (`History.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HistoryFeed {
    pub videos: Vec<VideoNode>,
    pub continuation_token: Option<String>,
}

/// A shelf section of the library page (legacy `Library.sections`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LibrarySection {
    pub title: String,
    pub icon_type: Option<String>,
    pub videos: Vec<VideoNode>,
}

/// User library feed (`Library.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LibraryFeed {
    pub history_videos: Vec<VideoNode>,
    pub watch_later_videos: Vec<VideoNode>,
    pub liked_videos: Vec<VideoNode>,
    pub playlists_count: usize,
    /// Shelf-grouped sections (legacy `Library.sections`).
    pub sections: Vec<LibrarySection>,
}

/// Account notification item (`Notification.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountNotification {
    pub id: String,
    pub title: String,
    pub sent_time: Option<String>,
    pub thumbnail: Option<String>,
    pub video_id: Option<String>,
    pub is_read: bool,
}

/// List of account notifications (`Notifications.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountNotificationsResponse {
    pub notifications: Vec<AccountNotification>,
    pub continuation_token: Option<String>,
}
