use serde::{Deserialize, Serialize};

/// A single search suggestion item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestion {
    /// The suggested query text.
    pub text: String,
    /// Whether this was from user search history (if authenticated).
    pub is_history: bool,
}

/// Search suggestions query response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionsResult {
    /// The original input query.
    pub query: String,
    /// Whether this query was for YouTube Music.
    pub is_music: bool,
    /// List of suggestion strings.
    pub suggestions: Vec<String>,
}
