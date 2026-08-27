use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Strongly typed CreatePlaylistServiceEndpoint AST node (`createPlaylistServiceEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlaylistServiceEndpointNode {
    pub title: Option<String>,
    pub privacy_status: Option<String>,
    pub description: Option<String>,
    pub video_ids: Option<Vec<String>>,
    pub params: Option<String>,
    pub source_playlist_id: Option<String>,
}

impl CreatePlaylistServiceEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("createPlaylistServiceEndpoint").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(Value::as_str).map(ToString::to_string),
            privacy_status: node.get("privacyStatus").and_then(Value::as_str).map(ToString::to_string),
            description: node.get("description").and_then(Value::as_str).map(ToString::to_string),
            video_ids: node.get("videoIds").and_then(Value::as_array).map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .map(ToString::to_string)
                    .collect()
            }),
            params: node.get("params").and_then(Value::as_str).map(ToString::to_string),
            source_playlist_id: node.get("sourcePlaylistId").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed DeletePlaylistEndpoint AST node (`deletePlaylistEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePlaylistEndpointNode {
    pub playlist_id: Option<String>,
    pub source_playlist_id: Option<String>,
}

impl DeletePlaylistEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("deletePlaylistEndpoint").unwrap_or(val);
        Some(Self {
            playlist_id: node.get("playlistId").and_then(Value::as_str).map(ToString::to_string),
            source_playlist_id: node.get("sourcePlaylistId").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed FeedbackEndpoint AST node (`feedbackEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackEndpointNode {
    pub feedback_token: Option<String>,
    pub cpn: Option<String>,
    pub is_feedback_token_unencrypted: Option<bool>,
    pub should_merge: Option<bool>,
}

impl FeedbackEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("feedbackEndpoint").unwrap_or(val);
        Some(Self {
            feedback_token: node.get("feedbackToken").and_then(Value::as_str).map(ToString::to_string),
            cpn: node.get("cpn").and_then(Value::as_str).map(ToString::to_string),
            is_feedback_token_unencrypted: node.get("isFeedbackTokenUnencrypted").and_then(Value::as_bool),
            should_merge: node.get("shouldMerge").and_then(Value::as_bool),
        })
    }
}

/// Strongly typed GetAccountsListInnertubeEndpoint AST node (`getAccountsListInnertubeEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountsListInnertubeEndpointNode {
    pub request_type: Option<String>,
    pub next_url: Option<String>,
    pub channel_switcher_query: Option<Value>,
    pub trigger_channel_creation: Option<bool>,
    pub content_owner_config: Option<Value>,
    pub obfuscated_selected_gaia_id: Option<String>,
    pub selected_serialized_delegation_context: Option<String>,
    pub call_circumstance: Option<String>,
}

impl GetAccountsListInnertubeEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("getAccountsListInnertubeEndpoint").unwrap_or(val);
        Some(Self {
            request_type: node.get("requestType").and_then(Value::as_str).map(ToString::to_string),
            next_url: node.get("nextUrl").and_then(Value::as_str).map(ToString::to_string),
            channel_switcher_query: node.get("channelSwitcherQuery").cloned(),
            trigger_channel_creation: node.get("triggerChannelCreation").and_then(Value::as_bool),
            content_owner_config: node.get("contentOwnerConfig").cloned(),
            obfuscated_selected_gaia_id: node.get("obfuscatedSelectedGaiaId").and_then(Value::as_str).map(ToString::to_string),
            selected_serialized_delegation_context: node.get("selectedSerializedDelegationContext").and_then(Value::as_str).map(ToString::to_string),
            call_circumstance: node.get("callCircumstance").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed HideEngagementPanelEndpoint AST node (`hideEngagementPanelEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HideEngagementPanelEndpointNode {
    pub panel_identifier: Option<String>,
}

impl HideEngagementPanelEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("hideEngagementPanelEndpoint").unwrap_or(val);
        Some(Self {
            panel_identifier: node.get("panelIdentifier").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed LiveChatItemContextMenuEndpoint AST node (`liveChatItemContextMenuEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatItemContextMenuEndpointNode {
    pub params: Option<String>,
}

impl LiveChatItemContextMenuEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatItemContextMenuEndpoint").unwrap_or(val);
        Some(Self {
            params: node.get("params").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed ModifyChannelNotificationPreferenceEndpoint AST node (`modifyChannelNotificationPreferenceEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyChannelNotificationPreferenceEndpointNode {
    pub params: Option<String>,
    pub secondary_params: Option<String>,
}

impl ModifyChannelNotificationPreferenceEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("modifyChannelNotificationPreferenceEndpoint").unwrap_or(val);
        Some(Self {
            params: node.get("params").and_then(Value::as_str).map(ToString::to_string),
            secondary_params: node.get("secondaryParams").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed PerformCommentActionEndpoint AST node (`performCommentActionEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformCommentActionEndpointNode {
    pub actions: Option<Vec<Value>>,
    pub action: Option<Value>,
}

impl PerformCommentActionEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("performCommentActionEndpoint").unwrap_or(val);
        Some(Self {
            actions: node.get("actions").and_then(Value::as_array).cloned(),
            action: node.get("action").cloned(),
        })
    }
}

/// Strongly typed PlaylistEditEndpoint AST node (`playlistEditEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEditEndpointNode {
    pub actions: Option<Vec<Value>>,
    pub playlist_id: Option<String>,
    pub params: Option<String>,
}

impl PlaylistEditEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistEditEndpoint").unwrap_or(val);
        Some(Self {
            actions: node.get("actions").and_then(Value::as_array).cloned(),
            playlist_id: node.get("playlistId").and_then(Value::as_str).map(ToString::to_string),
            params: node.get("params").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed PrefetchWatchCommand AST node (`prefetchWatchCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefetchWatchCommandNode {
    pub video_id: Option<String>,
    pub playlist_id: Option<String>,
    pub index: Option<u64>,
    pub playlist_index: Option<u64>,
    pub player_params: Option<String>,
    pub params: Option<String>,
    pub start_time_seconds: Option<f64>,
    pub override_muted_at_start: Option<bool>,
    pub racy_check_ok: Option<bool>,
    pub content_check_ok: Option<bool>,
}

impl PrefetchWatchCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("prefetchWatchCommand").unwrap_or(val);
        Some(Self {
            video_id: node.get("videoId").and_then(Value::as_str).map(ToString::to_string),
            playlist_id: node.get("playlistId").and_then(Value::as_str).map(ToString::to_string),
            index: node.get("index").and_then(Value::as_u64),
            playlist_index: node.get("playlistIndex").and_then(Value::as_u64),
            player_params: node.get("playerParams").and_then(Value::as_str).map(ToString::to_string),
            params: node.get("params").and_then(Value::as_str).map(ToString::to_string),
            start_time_seconds: node.get("startTimeSeconds").and_then(Value::as_f64),
            override_muted_at_start: node.get("overrideMutedAtStart").and_then(Value::as_bool),
            racy_check_ok: node.get("racyCheckOk").and_then(Value::as_bool),
            content_check_ok: node.get("contentCheckOk").and_then(Value::as_bool),
        })
    }
}

/// Strongly typed ShareEndpoint AST node (`shareEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareEndpointNode {
    pub serialized_share_entity: Option<String>,
    pub client_params: Option<String>,
}

impl ShareEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("shareEndpoint").unwrap_or(val);
        Some(Self {
            serialized_share_entity: node.get("serializedShareEntity").and_then(Value::as_str).map(ToString::to_string),
            client_params: node.get("clientParams").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}

/// Strongly typed ShareEntityEndpoint AST node (`shareEntityEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareEntityEndpointNode {
    pub serialized_share_entity: Option<String>,
    pub client_params: Option<String>,
}

impl ShareEntityEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("shareEntityEndpoint").unwrap_or(val);
        Some(Self {
            serialized_share_entity: node.get("serializedShareEntity").and_then(Value::as_str).map(ToString::to_string),
            client_params: node.get("clientParams").and_then(Value::as_str).map(ToString::to_string),
        })
    }
}
