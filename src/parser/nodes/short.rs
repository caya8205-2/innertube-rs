use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// Represents a YouTube Short item (1:1 port consolidating `ReelItem.ts` and `ShortsLockupView.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ShortNode {
    pub id: String,
    pub title: String,
    pub view_count: Option<String>,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
    pub sequence_params: Option<String>,
}

impl ShortNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // Unwrap container if present
        let target = val.get("reelItemRenderer")
            .or_else(|| val.get("shortsLockupViewModel"))
            .or_else(|| val.pointer("/richItemRenderer/content/reelItemRenderer"))
            .or_else(|| val.pointer("/richItemRenderer/content/shortsLockupViewModel"))
            .unwrap_or(val);

        // 1. Check shortsLockupViewModel (modern ViewModel)
        if target.get("overlayMetadata").is_some() || target.pointer("/onTap/innertubeCommand/reelWatchEndpoint").is_some() {
            let id = target.pointer("/onTap/innertubeCommand/reelWatchEndpoint/videoId")
                .or_else(|| target.pointer("/onTap/innertubeCommand/watchEndpoint/videoId"))
                .or_else(|| target.get("videoId"))
                .and_then(|v| v.as_str())?
                .to_string();

            let title = target.pointer("/overlayMetadata/primaryText/content")
                .and_then(|t| t.as_str())
                .unwrap_or("Untitled Short")
                .to_string();

            let view_count = target.pointer("/overlayMetadata/secondaryText/content")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let thumbnails = ThumbnailListNode::from_value(
                target.pointer("/thumbnailViewModel/thumbnailViewModel/image")
                    .or_else(|| target.pointer("/thumbnailViewModel/image"))
                    .unwrap_or(target)
            );

            let endpoint = target.get("onTap").and_then(NavigationEndpointNode::from_value);

            let sequence_params = target.pointer("/onTap/innertubeCommand/reelWatchEndpoint/sequenceParams")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());

            return Some(Self {
                id,
                title,
                view_count,
                thumbnails,
                endpoint,
                sequence_params,
            });
        }

        // 2. Standard reelItemRenderer
        let id = target.get("videoId")
            .and_then(|v| v.as_str())
            .or_else(|| target.pointer("/navigationEndpoint/reelWatchEndpoint/videoId").and_then(|v| v.as_str()))
            .map(|s| s.to_string())?;

        let title = target.get("headline")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_else(|| "Untitled Short".to_string());

        let view_count = target.get("viewCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let thumbnails = ThumbnailListNode::from_value(target.get("thumbnail").unwrap_or(target));
        let endpoint = target.get("navigationEndpoint").and_then(NavigationEndpointNode::from_value);

        let sequence_params = target.pointer("/navigationEndpoint/reelWatchEndpoint/sequenceParams")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        Some(Self {
            id,
            title,
            view_count,
            thumbnails,
            endpoint,
            sequence_params,
        })
    }
}

/// Shelf containing Shorts items (`ReelShelf.ts` / `reelShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReelShelfNode {
    pub title: Option<String>,
    pub items: Vec<ShortNode>,
}

impl ReelShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("reelShelfRenderer").unwrap_or(val);
        if target.get("items").is_none() && target.get("title").is_none() {
            return None;
        }

        let title = target
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let mut items = Vec::new();
        if let Some(arr) = target.get("items").and_then(|i| i.as_array()) {
            for item in arr {
                if let Some(short) = ShortNode::from_value(item) {
                    items.push(short);
                }
            }
        }

        Some(Self { title, items })
    }
}
