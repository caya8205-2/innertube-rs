use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents navigation endpoints (1:1 port of `src/parser/classes/NavigationEndpoint.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NavigationEndpointNode {
    pub watch: Option<WatchEndpointNode>,
    pub browse: Option<BrowseEndpointNode>,
    pub reel_watch: Option<ReelWatchEndpointNode>,
    pub search: Option<SearchEndpointNode>,
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

impl NavigationEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // Unwrap CommandContext or commandMetadata if present
        let target = val.get("innertubeCommand")
            .or_else(|| val.pointer("/commandContext/onTap/innertubeCommand"))
            .or_else(|| val.pointer("/onTap/innertubeCommand"))
            .unwrap_or(val);

        let mut node = Self::default();
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

        if matched {
            Some(node)
        } else {
            None
        }
    }
}
