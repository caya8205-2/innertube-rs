use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
#[allow(unused_imports)]
use super::thumbnail::ThumbnailListNode;

/// Strongly typed SearchHeader AST node (`searchHeader`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHeaderNode {
    pub chip_bar: Option<Value>,
    pub search_filter_button: Option<Value>,
}

impl SearchHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchHeader").unwrap_or(val);
        Some(Self {
            chip_bar: node.get("chipBar").cloned(),
            search_filter_button: node.get("searchFilterButton").cloned(),
        })
    }
}

/// Strongly typed SearchSuggestion AST node (`searchSuggestion`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionNode {
    pub suggestion: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub icon_type: Option<String>,
    pub service_endpoint: Option<Value>,
}

impl SearchSuggestionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchSuggestion").unwrap_or(val);
        Some(Self {
            suggestion: node.get("suggestion").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            icon_type: node
                .get("icon")
                .and_then(|v| v.get("iconType"))
                .and_then(|v| v.as_str())
                .map(String::from),
            service_endpoint: node.get("serviceEndpoint").cloned(),
        })
    }
}

/// Strongly typed SearchSuggestionsSection AST node (`searchSuggestionsSection`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestionsSectionNode {
    pub contents: Option<Vec<Value>>,
}

impl SearchSuggestionsSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("searchSuggestionsSection").unwrap_or(val);
        Some(Self {
            contents: node
                .get("contents")
                .and_then(|v| v.as_array())
                .map(|v| v.to_vec()),
        })
    }
}

/// Strongly typed SecondarySearchContainer AST node (`secondarySearchContainer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondarySearchContainerNode {
    pub target_id: Option<String>,
    pub contents: Option<Vec<Value>>,
}

impl SecondarySearchContainerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("secondarySearchContainer").unwrap_or(val);
        Some(Self {
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
            contents: node
                .get("contents")
                .and_then(|v| v.as_array())
                .map(|v| v.to_vec()),
        })
    }
}

/// Strongly typed SectionHeaderView AST node (`sectionHeaderView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionHeaderViewNode {
    pub headline: Option<TextNode>,
}

impl SectionHeaderViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sectionHeaderView").unwrap_or(val);
        Some(Self {
            headline: node.get("headline").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed SegmentedLikeDislikeButton AST node (`segmentedLikeDislikeButton`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentedLikeDislikeButtonNode {
    pub like_button: Option<Value>,
    pub dislike_button: Option<Value>,
}

impl SegmentedLikeDislikeButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("segmentedLikeDislikeButton").unwrap_or(val);
        Some(Self {
            like_button: node.get("likeButton").cloned(),
            dislike_button: node.get("dislikeButton").cloned(),
        })
    }
}

/// Strongly typed SegmentedLikeDislikeButtonView AST node (`segmentedLikeDislikeButtonView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentedLikeDislikeButtonViewNode {
    pub like_button: Option<Value>,
    pub dislike_button: Option<Value>,
    pub icon_type: Option<String>,
    pub like_count_entity_key: Option<String>,
    pub update_status_key: Option<String>,
    pub placeholder_like_count_values_key: Option<String>,
    pub update_delay_loop_id: Option<String>,
    pub update_delay_sec: Option<f64>,
}

impl SegmentedLikeDislikeButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("segmentedLikeDislikeButtonViewModel").or_else(|| val.get("segmentedLikeDislikeButtonView")).unwrap_or(val);
        
        let like_count_entity = node.get("likeCountEntity");
        let like_count_entity_key = like_count_entity.and_then(|v| v.get("key")).and_then(|v| v.as_str()).map(String::from);
        
        let dynamic_like_count = node.get("dynamicLikeCountUpdateData");
        
        Some(Self {
            like_button: node.get("likeButtonViewModel").cloned(),
            dislike_button: node.get("dislikeButtonViewModel").cloned(),
            icon_type: node.get("iconType").and_then(|v| v.as_str()).map(String::from),
            like_count_entity_key,
            update_status_key: dynamic_like_count.and_then(|v| v.get("updateStatusKey")).and_then(|v| v.as_str()).map(String::from),
            placeholder_like_count_values_key: dynamic_like_count.and_then(|v| v.get("placeholderLikeCountValuesKey")).and_then(|v| v.as_str()).map(String::from),
            update_delay_loop_id: dynamic_like_count.and_then(|v| v.get("updateDelayLoopId")).and_then(|v| v.as_str()).map(String::from),
            update_delay_sec: dynamic_like_count.and_then(|v| v.get("updateDelaySec")).and_then(|v| v.as_f64()),
        })
    }
}

/// Strongly typed SettingBoolean AST node (`settingBoolean`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingBooleanNode {
    pub title: Option<TextNode>,
    pub summary: Option<TextNode>,
    pub enable_endpoint: Option<Value>,
    pub disable_endpoint: Option<Value>,
    pub item_id: Option<String>,
}

impl SettingBooleanNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("settingBoolean").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            summary: node.get("summary").and_then(TextNode::from_value),
            enable_endpoint: node.get("enableServiceEndpoint").cloned(),
            disable_endpoint: node.get("disableServiceEndpoint").cloned(),
            item_id: node.get("itemId").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed SettingsCheckbox AST node (`settingsCheckbox`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsCheckboxNode {
    pub title: Option<TextNode>,
    pub help_text: Option<TextNode>,
    pub enabled: bool,
    pub disabled: bool,
    pub id: Option<String>,
}

impl SettingsCheckboxNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("settingsCheckbox").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            help_text: node.get("helpText").and_then(TextNode::from_value),
            enabled: node.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false),
            disabled: node.get("disabled").and_then(|v| v.as_bool()).unwrap_or(false),
            id: node.get("id").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed SettingsOptions AST node (`settingsOptions`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsOptionsNode {
    pub title: Option<TextNode>,
    pub text: Option<String>,
    pub options: Option<Vec<Value>>,
}

impl SettingsOptionsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("settingsOptions").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            text: node.get("text").and_then(|v| {
                if v.is_string() {
                    v.as_str().map(String::from)
                } else if v.is_object() {
                    // For backwards compatibility, maybe it's just raw text, let's treat object as text if possible
                    TextNode::from_value(v).map(|t| t.to_string())
                } else {
                    None
                }
            }),
            options: node
                .get("options")
                .and_then(|v| v.as_array())
                .map(|v| v.to_vec()),
        })
    }
}

/// Strongly typed SettingsSidebar AST node (`settingsSidebar`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSidebarNode {
    pub title: Option<TextNode>,
    pub items: Option<Vec<Value>>,
}

impl SettingsSidebarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("settingsSidebar").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            items: node
                .get("items")
                .and_then(|v| v.as_array())
                .map(|v| v.to_vec()),
        })
    }
}

/// Strongly typed SettingsSwitch AST node (`settingsSwitch`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsSwitchNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub enabled: bool,
    pub enable_endpoint: Option<Value>,
    pub disable_endpoint: Option<Value>,
}

impl SettingsSwitchNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("settingsSwitch").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            enabled: node.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false),
            enable_endpoint: node.get("enableServiceEndpoint").cloned(),
            disable_endpoint: node.get("disableServiceEndpoint").cloned(),
        })
    }
}
