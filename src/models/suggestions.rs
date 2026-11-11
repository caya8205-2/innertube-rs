use serde::{Deserialize, Serialize};

/// A single search suggestion item with rich metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestion {
    /// The suggested query text (e.g. "taylor swift", "mrbeast").
    pub text: String,
    /// Subtitle, entity type, or channel handle (e.g. "American singer-songwriter", "@MrBeast", "2022 film").
    pub subtitle: Option<String>,
    /// Thumbnail image URL (artist avatar, channel logo, or movie poster).
    pub thumbnail: Option<String>,
    /// YouTube Channel ID if this suggestion represents a channel (e.g. "UCX6OQ3DkcsbYNE6H8uQQuVA").
    pub channel_id: Option<String>,
    /// Official or canonical display name (e.g. "Avatar: The Way of Water").
    pub canonical_name: Option<String>,
    /// Google Knowledge Graph Entity ID (e.g. "/m/0dl567").
    pub entity_id: Option<String>,
    /// External reference URL (e.g. Wikipedia or IMDb link).
    pub info_url: Option<String>,
    /// Whether this suggestion is from user search history (when authenticated).
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
    /// List of suggestion strings (for backwards compatibility).
    pub suggestions: Vec<String>,
    /// Detailed suggestion items containing rich metadata.
    #[serde(default)]
    pub items: Vec<SearchSuggestion>,
}
