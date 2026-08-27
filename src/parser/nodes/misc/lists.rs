use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HypePointsFactoidNode {
    pub factoid: Option<Value>,
}

impl HypePointsFactoidNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("hypePointsFactoidRenderer").unwrap_or(val);
        Some(Self {
            factoid: node.get("factoid").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconLinkNode {
    pub icon_type: Option<String>,
    pub tooltip: Option<TextNode>,
    pub endpoint: Option<Value>,
}

impl IconLinkNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("iconLinkRenderer").unwrap_or(val);
        let icon_type = node
            .get("icon")
            .and_then(|i| i.get("iconType"))
            .and_then(|t| t.as_str().map(String::from));
        Some(Self {
            icon_type,
            tooltip: node.get("tooltip").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageBannerViewNode {
    pub image: Option<ThumbnailListNode>,
    pub style: Option<String>,
}

impl ImageBannerViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("imageBannerViewModel").unwrap_or(val);
        Some(Self {
            image: node.get("image").map(ThumbnailListNode::from_value),
            style: node.get("style").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncludingResultsForNode {
    pub including_results_for: Option<TextNode>,
    pub corrected_query: Option<TextNode>,
    pub corrected_query_endpoint: Option<Value>,
    pub search_only_for: Option<TextNode>,
    pub original_query: Option<TextNode>,
    pub original_query_endpoint: Option<Value>,
}

impl IncludingResultsForNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("includingResultsForRenderer").unwrap_or(val);
        Some(Self {
            including_results_for: node.get("includingResultsFor").and_then(TextNode::from_value),
            corrected_query: node.get("correctedQuery").and_then(TextNode::from_value),
            corrected_query_endpoint: node.get("correctedQueryEndpoint").cloned(),
            search_only_for: node.get("searchOnlyFor").and_then(TextNode::from_value),
            original_query: node.get("originalQuery").and_then(TextNode::from_value),
            original_query_endpoint: node.get("originalQueryEndpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoPanelContainerNode {
    pub title: Option<TextNode>,
    pub menu: Option<Value>,
    pub content: Option<Value>,
    pub header_endpoint: Option<Value>,
    pub background: Option<String>,
    pub title_style: Option<String>,
    pub icon_type: Option<String>,
}

impl InfoPanelContainerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("infoPanelContainerRenderer").unwrap_or(val);
        let icon_type = node
            .get("icon")
            .and_then(|i| i.get("iconType"))
            .and_then(|t| t.as_str().map(String::from));
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            menu: node.get("menu").cloned(),
            content: node.get("content").cloned(),
            header_endpoint: node.get("headerEndpoint").cloned(),
            background: node.get("background").and_then(|v| v.as_str().map(String::from)),
            title_style: node.get("titleStyle").and_then(|v| v.as_str().map(String::from)),
            icon_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoPanelContentNode {
    pub title: Option<TextNode>,
    pub source: Option<TextNode>,
    pub paragraphs: Option<Vec<TextNode>>,
    pub attributed_paragraphs: Option<Vec<TextNode>>,
    pub thumbnail: Option<ThumbnailListNode>,
    pub source_endpoint: Option<Value>,
    pub truncate_paragraphs: bool,
    pub background: Option<String>,
    pub inline_link_icon_type: Option<String>,
}

impl InfoPanelContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("infoPanelContentRenderer").unwrap_or(val);

        let paragraphs = node.get("paragraphs").and_then(|p| p.as_array()).map(|arr| {
            arr.iter().filter_map(TextNode::from_value).collect::<Vec<_>>()
        });

        let attributed_paragraphs = node
            .get("attributedParagraphs")
            .and_then(|p| p.as_array())
            .map(|arr| {
                arr.iter().filter_map(TextNode::from_value).collect::<Vec<_>>()
            });

        let inline_link_icon_type = node
            .get("inlineLinkIcon")
            .and_then(|i| i.get("iconType"))
            .and_then(|t| t.as_str().map(String::from));

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            source: node.get("source").and_then(TextNode::from_value),
            paragraphs,
            attributed_paragraphs,
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value),
            source_endpoint: node.get("sourceEndpoint").cloned(),
            truncate_paragraphs: node
                .get("truncateParagraphs")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            background: node.get("background").and_then(|v| v.as_str().map(String::from)),
            inline_link_icon_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveTabbedHeaderNode {
    pub header_type: Option<String>,
    pub title: Option<TextNode>,
    pub description: Option<TextNode>,
    pub metadata: Option<TextNode>,
    pub badges: Option<Vec<Value>>,
    pub box_art: Option<ThumbnailListNode>,
    pub banner: Option<ThumbnailListNode>,
    pub buttons: Option<Vec<Value>>,
    pub auto_generated: Option<TextNode>,
}

impl InteractiveTabbedHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("interactiveTabbedHeaderRenderer").unwrap_or(val);
        Some(Self {
            header_type: node.get("type").and_then(|v| v.as_str().map(String::from)),
            title: node.get("title").and_then(TextNode::from_value),
            description: node.get("description").and_then(TextNode::from_value),
            metadata: node.get("metadata").and_then(TextNode::from_value),
            badges: node.get("badges").and_then(|v| v.as_array().cloned()),
            box_art: node.get("boxArt").map(ThumbnailListNode::from_value),
            banner: node.get("banner").map(ThumbnailListNode::from_value),
            buttons: node.get("buttons").and_then(|v| v.as_array().cloned()),
            auto_generated: node.get("autoGenerated").and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionHeaderNode {
    pub title: Option<TextNode>,
}

impl ItemSectionHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("itemSectionHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionTabNode {
    pub title: Option<TextNode>,
    pub selected: bool,
    pub endpoint: Option<Value>,
}

impl ItemSectionTabNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("tabRenderer")
            .or_else(|| val.get("itemSectionTabRenderer"))
            .unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            selected: node.get("selected").and_then(|v| v.as_bool()).unwrap_or(false),
            endpoint: node.get("endpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSectionTabbedHeaderNode {
    pub title: Option<TextNode>,
    pub tabs: Option<Vec<Value>>,
    pub end_items: Option<Vec<Value>>,
}

impl ItemSectionTabbedHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("itemSectionTabbedHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            tabs: node.get("tabs").and_then(|v| v.as_array().cloned()),
            end_items: node.get("endItems").and_then(|v| v.as_array().cloned()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeButtonTarget {
    pub video_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeButtonNode {
    pub target: Option<LikeButtonTarget>,
    pub like_status: Option<String>,
    pub likes_allowed: Option<String>,
    pub endpoints: Option<Vec<Value>>,
}

impl LikeButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("likeButtonRenderer").unwrap_or(val);

        let target = node.get("target").map(|t| LikeButtonTarget {
            video_id: t.get("videoId").and_then(|v| v.as_str().map(String::from)),
        });

        Some(Self {
            target,
            like_status: node.get("likeStatus").and_then(|v| v.as_str().map(String::from)),
            likes_allowed: node.get("likesAllowed").and_then(|v| {
                if let Some(b) = v.as_bool() {
                    Some(b.to_string())
                } else {
                    v.as_str().map(String::from)
                }
            }),
            endpoints: node.get("serviceEndpoints").and_then(|v| v.as_array().cloned()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeStatusEntity {
    pub key: Option<String>,
    pub like_status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeButtonViewNode {
    pub toggle_button: Option<Value>,
    pub like_status_entity_key: Option<String>,
    pub like_status_entity: Option<LikeStatusEntity>,
}

impl LikeButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("likeButtonViewModel").unwrap_or(val);

        let like_status_entity = node.get("likeStatusEntity").map(|e| LikeStatusEntity {
            key: e.get("key").and_then(|k| k.as_str().map(String::from)),
            like_status: e.get("likeStatus").and_then(|s| s.as_str().map(String::from)),
        });

        Some(Self {
            toggle_button: node.get("toggleButtonViewModel").cloned(),
            like_status_entity_key: node
                .get("likeStatusEntityKey")
                .and_then(|k| k.as_str().map(String::from)),
            like_status_entity,
        })
    }
}
