use serde::{Deserialize, Serialize};

/// Result of an account mutation action (like, subscribe, comment, playlist edit).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub success: bool,
    pub status: Option<String>,
    pub action_id: Option<String>,
}

/// Result of a playlist creation action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaylistResult {
    pub success: bool,
    pub playlist_id: Option<String>,
}

/// Result of a comment creation action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentResult {
    pub success: bool,
    pub comment_id: Option<String>,
}

/// Result of a comment-translation action (legacy `InteractionManager.translate`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TranslateResult {
    pub success: bool,
    pub status_code: u16,
    pub translated_content: Option<String>,
    pub data: serde_json::Value,
}

/// Channel notification preference option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPreferenceType {
    Personalized,
    All,
    None,
}

impl NotificationPreferenceType {
    pub const fn index(self) -> i32 {
        match self {
            Self::Personalized => 1,
            Self::All => 2,
            Self::None => 3,
        }
    }
}
