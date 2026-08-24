use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents navigation endpoints (1:1 port of `src/parser/classes/NavigationEndpoint.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NavigationEndpointNode {
    /// InnerTube API path supplied by `commandMetadata`, or inferred for core endpoints.
    pub api_path: Option<String>,
    /// The endpoint name and unmodified request payload needed to replay it.
    pub endpoint_name: Option<String>,
    pub payload: Value,
    pub watch: Option<WatchEndpointNode>,
    pub browse: Option<BrowseEndpointNode>,
    pub reel_watch: Option<ReelWatchEndpointNode>,
    pub search: Option<SearchEndpointNode>,
    pub like: Option<LikeEndpointNode>,
    pub subscribe: Option<SubscribeEndpointNode>,
    pub continuation: Option<ContinuationEndpointNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WatchEndpointNode {
    pub video_id: String,
    pub playlist_id: Option<String>,
    pub index: Option<u32>,
    pub params: Option<String>,
    pub player_params: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BrowseEndpointNode {
    pub browse_id: String,
    pub params: Option<String>,
    pub canonical_base_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReelWatchEndpointNode {
    pub video_id: String,
    pub params: Option<String>,
    pub sequence_params: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchEndpointNode {
    pub query: String,
    pub params: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LikeEndpointNode {
    pub target: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SubscribeEndpointNode {
    pub channel_ids: Vec<String>,
    pub params: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContinuationEndpointNode {
    pub token: String,
    pub request: Option<String>,
}

impl NavigationEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // Unwrap CommandContext while retaining the outer command metadata.
        let target = val.get("innertubeCommand")
            .or_else(|| val.pointer("/commandContext/onTap/innertubeCommand"))
            .or_else(|| val.pointer("/onTap/innertubeCommand"))
            .unwrap_or(val);

        let endpoint_name = target
            .as_object()
            .and_then(|fields| {
                fields
                    .keys()
                    .find(|name| name.ends_with("Endpoint") || name.ends_with("Command"))
            })
            .cloned();
        let payload = endpoint_name
            .as_ref()
            .and_then(|name| target.get(name))
            .cloned()
            .unwrap_or(Value::Null);
        let api_path = val
            .pointer("/commandMetadata/webCommandMetadata/apiUrl")
            .or_else(|| target.pointer("/commandMetadata/webCommandMetadata/apiUrl"))
            .and_then(Value::as_str)
            .map(normalize_api_path)
            .or_else(|| endpoint_name.as_deref().and_then(infer_api_path));

        let mut node = Self {
            api_path,
            endpoint_name,
            payload,
            ..Self::default()
        };
        let mut matched = false;

        if let Some(w) = target.get("watchEndpoint") {
            if let Some(vid) = w.get("videoId").and_then(|v| v.as_str()) {
                node.watch = Some(WatchEndpointNode {
                    video_id: vid.to_string(),
                    playlist_id: w.get("playlistId").and_then(|p| p.as_str()).map(|s| s.to_string()),
                    index: w.get("index").and_then(|i| i.as_u64()).map(|i| i as u32),
                    params: w.get("params").and_then(|p| p.as_str()).map(|s| s.to_string()),
                    player_params: w.get("playerParams").and_then(|p| p.as_str()).map(|s| s.to_string()),
                });
                matched = true;
            }
        }

        if let Some(b) = target.get("browseEndpoint") {
            if let Some(bid) = b.get("browseId").and_then(|id| id.as_str()) {
                node.browse = Some(BrowseEndpointNode {
                    browse_id: bid.to_string(),
                    params: b.get("params").and_then(|p| p.as_str()).map(|s| s.to_string()),
                    canonical_base_url: b.get("canonicalBaseUrl").and_then(|u| u.as_str()).map(|s| s.to_string()),
                });
                matched = true;
            }
        }

        if let Some(r) = target.get("reelWatchEndpoint") {
            if let Some(vid) = r.get("videoId").and_then(|v| v.as_str()) {
                node.reel_watch = Some(ReelWatchEndpointNode {
                    video_id: vid.to_string(),
                    params: r.get("params").and_then(|p| p.as_str()).map(|s| s.to_string()),
                    sequence_params: r.get("sequenceParams").and_then(|s| s.as_str()).map(|s| s.to_string()),
                });
                matched = true;
            }
        }

        if let Some(s) = target.get("searchEndpoint") {
            if let Some(q) = s.get("query").and_then(|q| q.as_str()) {
                node.search = Some(SearchEndpointNode {
                    query: q.to_string(),
                    params: s.get("params").and_then(|p| p.as_str()).map(|p| p.to_string()),
                });
                matched = true;
            }
        }

        if let Some(l) = target.get("likeEndpoint") {
            if let Some(target_id) = l.pointer("/target/videoId").or_else(|| l.pointer("/target/playlistId")).and_then(|t| t.as_str()) {
                node.like = Some(LikeEndpointNode {
                    target: target_id.to_string(),
                    status: l.get("status").and_then(|s| s.as_str()).map(|s| s.to_string()),
                });
                matched = true;
            }
        }

        if let Some(s) = target.get("subscribeEndpoint") {
            let channel_ids = s.get("channelIds")
                .and_then(|c| c.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            node.subscribe = Some(SubscribeEndpointNode {
                channel_ids,
                params: s.get("params").and_then(|p| p.as_str()).map(|s| s.to_string()),
            });
            matched = true;
        }

        if let Some(c) = target.get("continuationCommand").or_else(|| target.get("continuationEndpoint")) {
            if let Some(token) = c.get("token").and_then(|t| t.as_str()) {
                node.continuation = Some(ContinuationEndpointNode {
                    token: token.to_string(),
                    request: c.get("request").and_then(|r| r.as_str()).map(|s| s.to_string()),
                });
                matched = true;
            }
        }

        if matched || node.endpoint_name.is_some() {
            Some(node)
        } else {
            None
        }
    }
}

fn normalize_api_path(path: &str) -> String {
    path.strip_prefix("/youtubei/v1").unwrap_or(path).to_string()
}

fn infer_api_path(endpoint_name: &str) -> Option<String> {
    match endpoint_name {
        "browseEndpoint" => Some("/browse".to_string()),
        "watchEndpoint" | "reelWatchEndpoint" => Some("/player".to_string()),
        "searchEndpoint" => Some("/search".to_string()),
        "watchPlaylistEndpoint" => Some("/next".to_string()),
        "likeEndpoint" => Some("/like/like".to_string()),
        "subscribeEndpoint" => Some("/subscription/subscribe".to_string()),
        "unsubscribeEndpoint" => Some("/subscription/unsubscribe".to_string()),
        "liveChatItemContextMenuEndpoint" => Some("/live_chat/get_item_context_menu".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn preserves_endpoint_payload_and_api_metadata() {
        let node = NavigationEndpointNode::from_value(&json!({
            "commandMetadata": {
                "webCommandMetadata": { "apiUrl": "/youtubei/v1/browse" }
            },
            "browseEndpoint": { "browseId": "UC_test" }
        }))
        .expect("fixture should parse");

        assert_eq!(node.api_path.as_deref(), Some("/browse"));
        assert_eq!(node.endpoint_name.as_deref(), Some("browseEndpoint"));
        assert_eq!(node.payload["browseId"], "UC_test");
    }

    #[test]
    fn parses_like_and_continuation_endpoints() {
        let like_node = NavigationEndpointNode::from_value(&json!({
            "likeEndpoint": {
                "status": "LIKE",
                "target": { "videoId": "dQw4w9WgXcQ" }
            }
        }))
        .expect("like fixture should parse");

        assert_eq!(like_node.like.as_ref().map(|l| l.target.as_str()), Some("dQw4w9WgXcQ"));
        assert_eq!(like_node.api_path.as_deref(), Some("/like/like"));

        let cont_node = NavigationEndpointNode::from_value(&json!({
            "continuationCommand": {
                "token": "4qmFsgI0EhhGQ2..."
            }
        }))
        .expect("continuation command fixture should parse");

        assert_eq!(cont_node.continuation.as_ref().map(|c| c.token.as_str()), Some("4qmFsgI0EhhGQ2..."));
    }
}
