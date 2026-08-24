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
