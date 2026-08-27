use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed SharePanelHeader AST node (`SharePanelHeader`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharePanelHeaderNode {
    pub title: Option<Value>,
}

impl SharePanelHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sharePanelHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").cloned(),
        })
    }
}

/// Strongly typed SharePanelTitleV15 AST node (`SharePanelTitleV15`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharePanelTitleV15Node {
    pub title: Option<TextNode>,
}

impl SharePanelTitleV15Node {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sharePanelTitleV15Renderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed ShareTarget AST node (`ShareTarget`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareTargetNode {
    pub endpoint: Option<Value>,
    pub service_name: Option<String>,
    pub target_id: Option<String>,
    pub title: Option<TextNode>,
}

impl ShareTargetNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("shareTargetRenderer").unwrap_or(val);

        let endpoint = if node.get("serviceEndpoint").is_some() {
            node.get("serviceEndpoint").cloned()
        } else {
            node.get("navigationEndpoint").cloned()
        };

        Some(Self {
            endpoint,
            service_name: node.get("serviceName").and_then(|v| v.as_str().map(String::from)),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed SheetView AST node (`SheetView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetViewNode {
    pub content: Option<Value>,
    pub footer: Option<Value>,
    pub header: Option<Value>,
    pub renderer_context: Option<Value>,
}

impl SheetViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sheetViewRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").cloned(),
            footer: node.get("footer").cloned(),
            header: node.get("header").cloned(),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed ShowCustomThumbnail AST node (`ShowCustomThumbnail`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowCustomThumbnailNode {
    pub thumbnail: ThumbnailListNode,
}

impl ShowCustomThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showCustomThumbnailRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value).unwrap_or_default(),
        })
    }
}

/// Strongly typed SimpleCardContent AST node (`SimpleCardContent`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleCardContentNode {
    pub image: ThumbnailListNode,
    pub title: Option<TextNode>,
    pub display_domain: Option<TextNode>,
    pub show_link_icon: bool,
    pub call_to_action: Option<TextNode>,
    pub endpoint: Option<Value>,
}

impl SimpleCardContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("simpleCardContentRenderer").unwrap_or(val);
        Some(Self {
            image: node.get("image").map(ThumbnailListNode::from_value).unwrap_or_default(),
            title: node.get("title").and_then(TextNode::from_value),
            display_domain: node.get("displayDomain").and_then(TextNode::from_value),
            show_link_icon: node.get("showLinkIcon").and_then(|v| v.as_bool()).unwrap_or(false),
            call_to_action: node.get("callToAction").and_then(TextNode::from_value),
            endpoint: node.get("command").cloned(),
        })
    }
}

/// Strongly typed SimpleCardTeaser AST node (`SimpleCardTeaser`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleCardTeaserNode {
    pub message: Option<TextNode>,
    pub prominent: bool,
}

impl SimpleCardTeaserNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("simpleCardTeaserRenderer").unwrap_or(val);
        Some(Self {
            message: node.get("message").and_then(TextNode::from_value),
            prominent: node.get("prominent").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed SimpleTextSection AST node (`SimpleTextSection`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleTextSectionNode {
    pub lines: Vec<TextNode>,
    pub style: Option<String>,
}

impl SimpleTextSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("simpleTextSectionRenderer").unwrap_or(val);
        
        let lines = node
            .get("lines")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(TextNode::from_value).collect())
            .unwrap_or_default();

        Some(Self {
            lines,
            style: node.get("layoutStyle").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed SingleColumnBrowseResults AST node (`SingleColumnBrowseResults`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleColumnBrowseResultsNode {
    pub tabs: Vec<Value>,
}

impl SingleColumnBrowseResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("singleColumnBrowseResultsRenderer").unwrap_or(val);
        Some(Self {
            tabs: node
                .get("tabs")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed SingleColumnMusicWatchNextResults AST node (`SingleColumnMusicWatchNextResults`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleColumnMusicWatchNextResultsNode {
    pub contents: Option<Value>,
}

impl SingleColumnMusicWatchNextResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("singleColumnMusicWatchNextResultsRenderer").unwrap_or(val);
        // Fallback to the whole data if contents aren't separately nested in legacy YT.js sometimes parser parses the raw data.
        let contents = if node.get("contents").is_some() {
            node.get("contents").cloned()
        } else {
            Some(node.clone())
        };

        Some(Self { contents })
    }
}

/// Strongly typed SingleHeroImage AST node (`SingleHeroImage`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleHeroImageNode {
    pub thumbnails: ThumbnailListNode,
    pub style: Option<String>,
}

impl SingleHeroImageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("singleHeroImageRenderer").unwrap_or(val);
        Some(Self {
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value).unwrap_or_default(),
            style: node.get("style").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed SlimOwner AST node (`SlimOwner`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlimOwnerNode {
    pub thumbnail: ThumbnailListNode,
    pub title: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub subscribe_button: Option<Value>,
}

impl SlimOwnerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("slimOwnerRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value).unwrap_or_default(),
            title: node.get("title").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            subscribe_button: node.get("subscribeButton").cloned(),
        })
    }
}
