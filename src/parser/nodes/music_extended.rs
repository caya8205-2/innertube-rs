use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::misc::text::TextNode;
use super::misc::thumbnail::ThumbnailListNode;

/// Strongly typed MusicCarouselShelf AST node (`musicCarouselShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicCarouselShelfNode {
    pub header: Option<Value>,
    #[serde(default)]
    pub contents: Vec<Value>,
    pub num_items_per_column: Option<u64>,
}

impl MusicCarouselShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicCarouselShelfRenderer").unwrap_or(val);
        
        let num_items_per_column = node.get("numItemsPerColumn").and_then(|v| v.as_u64()).or_else(|| {
            node.get("numItemsPerColumn").and_then(|v| v.as_str()).and_then(|v| v.parse().ok())
        });
        
        Some(Self {
            header: node.get("header").cloned(),
            contents: node.get("contents").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            num_items_per_column,
        })
    }
}

/// Strongly typed MusicShelf AST node (`musicShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicShelfNode {
    pub title: Option<String>,
    #[serde(default)]
    pub contents: Vec<Value>,
    pub endpoint: Option<Value>,
    pub continuation: Option<String>,
    pub bottom_text: Option<String>,
    pub bottom_button: Option<Value>,
    pub subheaders: Option<Vec<Value>>,
}

impl MusicShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicShelfRenderer").unwrap_or(val);
        
        let continuations = node.get("continuations").and_then(|v| v.as_array());
        let continuation = continuations.and_then(|c| c.first()).and_then(|c| {
            c.get("nextContinuationData").or_else(|| c.get("reloadContinuationData"))
        }).and_then(|c| c.get("continuation")).and_then(|c| c.as_str()).map(|s| s.to_owned());
        
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.to_string()),
            contents: node.get("contents").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            endpoint: node.get("bottomEndpoint").cloned(),
            continuation,
            bottom_text: node.get("bottomText").and_then(TextNode::from_value).map(|t| t.to_string()),
            bottom_button: node.get("bottomButton").cloned(),
            subheaders: node.get("subheaders").and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed MusicSideAlignedItem AST node (`musicSideAlignedItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicSideAlignedItemNode {
    pub start_items: Option<Vec<Value>>,
    pub end_items: Option<Vec<Value>>,
}

impl MusicSideAlignedItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicSideAlignedItemRenderer").unwrap_or(val);
        Some(Self {
            start_items: node.get("startItems").and_then(|v| v.as_array()).cloned(),
            end_items: node.get("endItems").and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed MusicVisualHeader AST node (`musicVisualHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicVisualHeaderNode {
    pub title: Option<String>,
    pub thumbnail: Option<ThumbnailListNode>,
    pub menu: Option<Value>,
    pub foreground_thumbnail: Option<ThumbnailListNode>,
}

impl MusicVisualHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicVisualHeaderRenderer").unwrap_or(val);
        
        let thumbnail = node.get("thumbnail")
            .and_then(|v| v.get("musicThumbnailRenderer"))
            .and_then(|v| v.get("thumbnail"))
            .map(ThumbnailListNode::from_value);
            
        let foreground_thumbnail = node.get("foregroundThumbnail")
            .and_then(|v| v.get("musicThumbnailRenderer"))
            .and_then(|v| v.get("thumbnail"))
            .map(ThumbnailListNode::from_value);
            
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.to_string()),
            thumbnail,
            menu: node.get("menu").cloned(),
            foreground_thumbnail,
        })
    }
}

/// Strongly typed MusicItemThumbnailOverlay AST node (`musicItemThumbnailOverlayRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicItemThumbnailOverlayNode {
    pub content: Option<Value>,
    pub content_position: Option<String>,
    pub display_style: Option<String>,
}

impl MusicItemThumbnailOverlayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicItemThumbnailOverlayRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").cloned(),
            content_position: node.get("contentPosition").and_then(|v| v.as_str()).map(|s| s.to_owned()),
            display_style: node.get("displayStyle").and_then(|v| v.as_str()).map(|s| s.to_owned()),
        })
    }
}

/// Strongly typed MusicPlaylistShelf AST node (`musicPlaylistShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlaylistShelfNode {
    pub playlist_id: Option<String>,
    #[serde(default)]
    pub contents: Vec<Value>,
    pub collapsed_item_count: Option<u64>,
    pub continuation: Option<String>,
}

impl MusicPlaylistShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicPlaylistShelfRenderer").unwrap_or(val);
        
        let continuations = node.get("continuations").and_then(|v| v.as_array());
        let continuation = continuations.and_then(|c| c.first())
            .and_then(|c| c.get("nextContinuationData"))
            .and_then(|c| c.get("continuation"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_owned());
            
        Some(Self {
            playlist_id: node.get("playlistId").and_then(|v| v.as_str()).map(|s| s.to_owned()),
            contents: node.get("contents").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            collapsed_item_count: node.get("collapsedItemCount").and_then(|v| v.as_u64()),
            continuation,
        })
    }
}

/// Strongly typed MusicCardShelf AST node (`musicCardShelfRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicCardShelfNode {
    pub thumbnail: Option<Value>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    #[serde(default)]
    pub buttons: Vec<Value>,
    pub menu: Option<Value>,
    pub on_tap: Option<Value>,
    pub header: Option<Value>,
    pub end_icon_type: Option<String>,
    pub subtitle_badges: Option<Vec<Value>>,
    pub thumbnail_overlay: Option<Value>,
    pub contents: Option<Vec<Value>>,
}

impl MusicCardShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicCardShelfRenderer").unwrap_or(val);
        
        let end_icon_type = node.get("endIcon").and_then(|i| i.get("iconType")).and_then(|i| i.as_str()).map(|s| s.to_owned());
        
        Some(Self {
            thumbnail: node.get("thumbnail").cloned(),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.to_string()),
            subtitle: node.get("subtitle").and_then(TextNode::from_value).map(|t| t.to_string()),
            buttons: node.get("buttons").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            menu: node.get("menu").cloned(),
            on_tap: node.get("onTap").cloned(),
            header: node.get("header").cloned(),
            end_icon_type,
            subtitle_badges: node.get("subtitleBadges").and_then(|v| v.as_array()).cloned(),
            thumbnail_overlay: node.get("thumbnailOverlay").cloned(),
            contents: node.get("contents").and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed MusicImmersiveHeader AST node (`musicImmersiveHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicImmersiveHeaderNode {
    pub title: Option<String>,
    pub menu: Option<Value>,
    pub more_button: Option<Value>,
    pub play_button: Option<Value>,
    pub share_endpoint: Option<Value>,
    pub start_radio_button: Option<Value>,
    pub subscription_button: Option<Value>,
    pub description: Option<String>,
    pub thumbnail: Option<Value>,
}

impl MusicImmersiveHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicImmersiveHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.to_string()),
            menu: node.get("menu").cloned(),
            more_button: node.get("moreButton").cloned(),
            play_button: node.get("playButton").cloned(),
            share_endpoint: node.get("shareEndpoint").cloned(),
            start_radio_button: node.get("startRadioButton").cloned(),
            subscription_button: node.get("subscriptionButton").cloned(),
            description: node.get("description").and_then(TextNode::from_value).map(|t| t.to_string()),
            thumbnail: node.get("thumbnail").cloned(),
        })
    }
}

/// Strongly typed MusicDetailHeader AST node (`musicDetailHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicDetailHeaderNode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub subtitle: Option<String>,
    pub second_subtitle: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub badges: Option<Vec<Value>>,
    pub menu: Option<Value>,
}

impl MusicDetailHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicDetailHeaderRenderer").unwrap_or(val);
        
        let thumbnails = node.get("thumbnail")
            .and_then(|v| v.get("croppedSquareThumbnailRenderer"))
            .and_then(|v| v.get("thumbnail"))
            .map(ThumbnailListNode::from_value);
            
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.to_string()),
            description: node.get("description").and_then(TextNode::from_value).map(|t| t.to_string()),
            subtitle: node.get("subtitle").and_then(TextNode::from_value).map(|t| t.to_string()),
            second_subtitle: node.get("secondSubtitle").and_then(TextNode::from_value).map(|t| t.to_string()),
            thumbnails,
            badges: node.get("subtitleBadges").and_then(|v| v.as_array()).cloned(),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed MusicEditablePlaylistDetailHeader AST node (`musicEditablePlaylistDetailHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicEditablePlaylistDetailHeaderNode {
    pub header: Option<Value>,
    pub edit_header: Option<Value>,
    pub playlist_id: Option<String>,
}

impl MusicEditablePlaylistDetailHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicEditablePlaylistDetailHeaderRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("header").cloned(),
            edit_header: node.get("editHeader").cloned(),
            playlist_id: node.get("playlistId").and_then(|v| v.as_str()).map(|s| s.to_owned()),
        })
    }
}

/// Strongly typed MusicResponsiveHeader AST node (`musicResponsiveHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicResponsiveHeaderNode {
    pub thumbnail: Option<Value>,
    #[serde(default)]
    pub buttons: Vec<Value>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub strapline_text_one: Option<String>,
    pub strapline_thumbnail: Option<Value>,
    pub second_subtitle: Option<String>,
    pub subtitle_badge: Option<Vec<Value>>,
    pub description: Option<Value>,
}

impl MusicResponsiveHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicResponsiveHeaderRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").cloned(),
            buttons: node.get("buttons").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.to_string()),
            subtitle: node.get("subtitle").and_then(TextNode::from_value).map(|t| t.to_string()),
            strapline_text_one: node.get("straplineTextOne").and_then(TextNode::from_value).map(|t| t.to_string()),
            strapline_thumbnail: node.get("straplineThumbnail").cloned(),
            second_subtitle: node.get("secondSubtitle").and_then(TextNode::from_value).map(|t| t.to_string()),
            subtitle_badge: node.get("subtitleBadge").and_then(|v| v.as_array()).cloned(),
            description: node.get("description").cloned(),
        })
    }
}

/// Strongly typed MusicAutoplay AST node (`automixPreviewVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicAutoplayNode {
    pub playlist_video_endpoint: Option<Value>,
}

impl MusicAutoplayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("automixPreviewVideoRenderer").unwrap_or(val);
        
        let playlist_video_endpoint = node.get("content")
            .and_then(|v| v.get("automixPlaylistVideoRenderer"))
            .and_then(|v| v.get("navigationEndpoint"))
            .cloned();
            
        Some(Self {
            playlist_video_endpoint,
        })
    }
}
