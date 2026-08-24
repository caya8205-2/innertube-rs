use serde::{Deserialize, Serialize};
use crate::parser::nodes::video::VideoNode;

/// User watch history feed (`History.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HistoryFeed {
    pub videos: Vec<VideoNode>,
    pub continuation_token: Option<String>,
}

/// User library feed (`Library.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LibraryFeed {
    pub history_videos: Vec<VideoNode>,
    pub watch_later_videos: Vec<VideoNode>,
    pub liked_videos: Vec<VideoNode>,
    pub playlists_count: usize,
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
