use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
// use super::thumbnail::ThumbnailListNode; // commented out to avoid unused import since none of the classes use ThumbnailListNode here

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSearchBoxNode {
    pub formatted_placeholder: Option<TextNode>,
    pub clear_button: Option<Value>,
    pub endpoint: Option<Value>,
    pub search_button: Option<Value>,
}

impl TranscriptSearchBoxNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptSearchBoxRenderer").unwrap_or(val);
        Some(Self {
            formatted_placeholder: node.get("formattedPlaceholder").and_then(TextNode::from_value),
            clear_button: node.get("clearButton").cloned(),
            endpoint: node.get("onTextChangeCommand").cloned(),
            search_button: node.get("searchButton").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSearchPanelNode {
    pub header: Option<Value>,
    pub body: Option<Value>,
    pub footer: Option<Value>,
    pub target_id: Option<String>,
}

impl TranscriptSearchPanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptSearchPanelRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("header").cloned(),
            body: node.get("body").cloned(),
            footer: node.get("footer").cloned(),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSectionHeaderNode {
    pub start_ms: Option<String>,
    pub end_ms: Option<String>,
    pub snippet: Option<TextNode>,
}

impl TranscriptSectionHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptSectionHeaderRenderer").unwrap_or(val);
        Some(Self {
            start_ms: node.get("startMs").and_then(|v| v.as_str().map(String::from)),
            end_ms: node.get("endMs").and_then(|v| v.as_str().map(String::from)),
            snippet: node.get("snippet").and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegmentNode {
    pub start_ms: Option<String>,
    pub end_ms: Option<String>,
    pub snippet: Option<TextNode>,
    pub start_time_text: Option<TextNode>,
    pub target_id: Option<String>,
}

impl TranscriptSegmentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptSegmentRenderer").unwrap_or(val);
        Some(Self {
            start_ms: node.get("startMs").and_then(|v| v.as_str().map(String::from)),
            end_ms: node.get("endMs").and_then(|v| v.as_str().map(String::from)),
            snippet: node.get("snippet").and_then(TextNode::from_value),
            start_time_text: node.get("startTimeText").and_then(TextNode::from_value),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegmentListNode {
    pub initial_segments: Option<Vec<Value>>,
    pub no_result_label: Option<TextNode>,
    pub retry_label: Option<TextNode>,
    pub touch_captions_enabled: bool,
}

impl TranscriptSegmentListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptSegmentListRenderer").unwrap_or(val);
        Some(Self {
            initial_segments: node.get("initialSegments").and_then(|v| v.as_array().cloned()),
            no_result_label: node.get("noResultLabel").and_then(TextNode::from_value),
            retry_label: node.get("retryLabel").and_then(TextNode::from_value),
            touch_captions_enabled: node.get("touchCaptionsEnabled").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnBrowseResultsNode {
    pub tabs: Option<Vec<Value>>,
    pub secondary_contents: Option<Value>,
}

impl TwoColumnBrowseResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("twoColumnBrowseResultsRenderer").unwrap_or(val);
        Some(Self {
            tabs: node.get("tabs").and_then(|v| v.as_array().cloned()),
            secondary_contents: node.get("secondaryContents").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnSearchResultsNode {
    pub header: Option<Value>,
    pub primary_contents: Option<Value>,
    pub secondary_contents: Option<Value>,
    pub target_id: Option<String>,
}

impl TwoColumnSearchResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("twoColumnSearchResultsRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("header").cloned(),
            primary_contents: node.get("primaryContents").cloned(),
            secondary_contents: node.get("secondaryContents").cloned(),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoplaySetNode {
    pub autoplay_video: Option<Value>,
    pub next_button_video: Option<Value>,
}

impl AutoplaySetNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            autoplay_video: val.get("autoplayVideo").cloned(),
            next_button_video: val.get("nextButtonVideo").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoplayNode {
    pub sets: Option<Vec<AutoplaySetNode>>,
    pub modified_sets: Option<Vec<AutoplaySetNode>>,
    pub count_down_secs: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Value>,
    pub contents: Option<Vec<Value>>,
    pub current_index: Option<u64>,
    pub is_infinite: bool,
    pub menu: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoColumnWatchNextResultsNode {
    pub results: Option<Vec<Value>>,
    pub secondary_results: Option<Vec<Value>>,
    pub conversation_bar: Option<Value>,
    pub playlist: Option<PlaylistNode>,
    pub autoplay: Option<AutoplayNode>,
}

impl TwoColumnWatchNextResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("twoColumnWatchNextResultsRenderer").unwrap_or(val);
        
        let playlist = node.get("playlist").and_then(|p| p.get("playlist")).map(|p| {
            PlaylistNode {
                id: p.get("playlistId").and_then(|v| v.as_str().map(String::from)),
                title: p.get("title").and_then(|v| v.as_str().map(String::from)),
                author: p.get("shortBylineText").or_else(|| p.get("longBylineText")).cloned(),
                contents: p.get("contents").and_then(|v| v.as_array().cloned()),
                current_index: p.get("currentIndex").and_then(|v| v.as_u64()),
                is_infinite: p.get("isInfinite").and_then(|v| v.as_bool()).unwrap_or(false),
                menu: p.get("menu").cloned(),
            }
        });

        let autoplay = node.get("autoplay").and_then(|a| a.get("autoplay")).map(|a| {
            AutoplayNode {
                sets: a.get("sets").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(AutoplaySetNode::from_value).collect()),
                modified_sets: a.get("modifiedSets").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(AutoplaySetNode::from_value).collect()),
                count_down_secs: a.get("countDownSecs").and_then(|v| v.as_u64()),
            }
        });

        Some(Self {
            results: node.get("results").and_then(|r| r.get("results")).and_then(|r| r.get("contents")).and_then(|v| v.as_array().cloned()),
            secondary_results: node.get("secondaryResults").and_then(|s| s.get("secondaryResults")).and_then(|s| s.get("results")).and_then(|v| v.as_array().cloned()),
            conversation_bar: node.get("conversationBar").cloned(),
            playlist,
            autoplay,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyNetworkSectionNode {
    pub share_target_container: Option<Value>,
    pub copy_link_container: Option<Value>,
    pub start_at_container: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedSharePanelNode {
    pub third_party_network_section: Option<ThirdPartyNetworkSectionNode>,
    pub header: Option<Value>,
    pub share_panel_version: Option<u64>,
    pub show_loading_spinner: Option<bool>,
}

impl UnifiedSharePanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("unifiedSharePanelRenderer").unwrap_or(val);
        
        let third_party_network_section = node.get("contents").and_then(|c| c.as_array()).and_then(|arr| {
            arr.iter().find(|c| c.get("thirdPartyNetworkSection").is_some()).and_then(|c| c.get("thirdPartyNetworkSection"))
        }).map(|section| ThirdPartyNetworkSectionNode {
            share_target_container: section.get("shareTargetContainer").cloned(),
            copy_link_container: section.get("copyLinkContainer").cloned(),
            start_at_container: section.get("startAtContainer").cloned(),
        });

        Some(Self {
            third_party_network_section,
            header: node.get("header").cloned(),
            share_panel_version: node.get("sharePanelVersion").and_then(|v| v.as_u64()),
            show_loading_spinner: node.get("showLoadingSpinner").and_then(|v| v.as_bool()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalWatchCardNode {
    pub header: Option<Value>,
    pub call_to_action: Option<Value>,
    pub sections: Option<Vec<Value>>,
    pub collapsed_label: Option<TextNode>,
}

impl UniversalWatchCardNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("universalWatchCardRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("header").cloned(),
            call_to_action: node.get("callToAction").cloned(),
            sections: node.get("sections").and_then(|v| v.as_array().cloned()),
            collapsed_label: node.get("collapsedLabel").and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadTimeFactoidNode {
    pub factoid: Option<Value>,
}

impl UploadTimeFactoidNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("uploadTimeFactoidRenderer").unwrap_or(val);
        Some(Self {
            factoid: node.get("factoid").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsellDialogNode {
    pub message_title: Option<TextNode>,
    pub message_text: Option<TextNode>,
    pub action_button: Option<Value>,
    pub dismiss_button: Option<Value>,
    pub is_visible: bool,
}

impl UpsellDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("upsellDialogRenderer").unwrap_or(val);
        Some(Self {
            message_title: node.get("dialogMessageTitle").and_then(TextNode::from_value),
            message_text: node.get("dialogMessageText").and_then(TextNode::from_value),
            action_button: node.get("actionButton").cloned(),
            dismiss_button: node.get("dismissButton").cloned(),
            is_visible: node.get("isVisible").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}
