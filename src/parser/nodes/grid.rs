use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::misc::text::TextNode;
use super::misc::thumbnail::ThumbnailListNode;

/// Strongly typed GridVideo AST node (`gridVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridVideoNode {
    pub video_id: Option<String>,
    pub title: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub thumbnail_overlays: Option<Vec<Value>>,
    pub rich_thumbnail: Option<Value>,
    pub published: Option<String>,
    pub duration: Option<String>,
    pub author: Option<Value>,
    pub views: Option<String>,
    pub short_view_count: Option<String>,
    pub endpoint: Option<Value>,
    pub menu: Option<Value>,
    pub buttons: Option<Vec<Value>>,
    pub upcoming: Option<Value>,
    pub upcoming_text: Option<String>,
    pub is_reminder_set: bool,
}

impl GridVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridVideoRenderer").unwrap_or(val);
        
        let length_alt = node.get("thumbnailOverlays")
            .and_then(|overlays| overlays.as_array())
            .and_then(|arr| arr.iter().find(|o| o.get("thumbnailOverlayTimeStatusRenderer").is_some()))
            .and_then(|o| o.get("thumbnailOverlayTimeStatusRenderer"));
            
        let duration = if let Some(length_text) = node.get("lengthText") {
            TextNode::from_value(length_text).map(|t| t.text)
        } else if let Some(alt) = length_alt {
            TextNode::from_value(alt.get("text").unwrap_or(&Value::Null)).map(|t| t.text)
        } else {
            None
        };

        let is_reminder_set = node.get("upcomingEventData")
            .and_then(|data| data.get("isReminderSet"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Some(Self {
            video_id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
            rich_thumbnail: node.get("richThumbnail").cloned(),
            published: node.get("publishedTimeText").and_then(TextNode::from_value).map(|t| t.text),
            duration,
            author: node.get("shortBylineText").cloned(),
            views: node.get("viewCountText").and_then(TextNode::from_value).map(|t| t.text),
            short_view_count: node.get("shortViewCountText").and_then(TextNode::from_value).map(|t| t.text),
            endpoint: node.get("navigationEndpoint").cloned(),
            menu: node.get("menu").cloned(),
            buttons: node.get("buttons").and_then(|v| v.as_array()).cloned(),
            upcoming: node.get("upcomingEventData").and_then(|d| d.get("startTime")).cloned(),
            upcoming_text: node.get("upcomingEventData").and_then(|d| d.get("upcomingEventText")).and_then(TextNode::from_value).map(|t| t.text),
            is_reminder_set,
        })
    }
}

/// Strongly typed GridChannel AST node (`gridChannelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridChannelNode {
    pub id: Option<String>,
    pub author: Option<Value>,
    pub subscribers: Option<String>,
    pub video_count: Option<String>,
    pub endpoint: Option<Value>,
    pub subscribe_button: Option<Value>,
}

impl GridChannelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridChannelRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("channelId").and_then(|v| v.as_str()).map(String::from),
            author: node.get("title").cloned(),
            subscribers: node.get("subscriberCountText").and_then(TextNode::from_value).map(|t| t.text),
            video_count: node.get("videoCountText").and_then(TextNode::from_value).map(|t| t.text),
            endpoint: node.get("navigationEndpoint").cloned(),
            subscribe_button: node.get("subscribeButton").cloned(),
        })
    }
}

/// Strongly typed GridPlaylist AST node (`gridPlaylistRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridPlaylistNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Value>,
    pub badges: Option<Vec<Value>>,
    pub endpoint: Option<Value>,
    pub view_playlist: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub thumbnail_renderer: Option<Value>,
    pub sidebar_thumbnails: Option<Vec<Value>>,
    pub video_count: Option<String>,
    pub video_count_short: Option<String>,
}

impl GridPlaylistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridPlaylistRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            author: node.get("shortBylineText").cloned(),
            badges: node.get("ownerBadges").and_then(|v| v.as_array()).cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            view_playlist: node.get("viewPlaylistText").and_then(TextNode::from_value).map(|t| t.text),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            thumbnail_renderer: node.get("thumbnailRenderer").cloned(),
            sidebar_thumbnails: node.get("sidebarThumbnails").and_then(|v| v.as_array()).cloned(),
            video_count: node.get("thumbnailText").and_then(TextNode::from_value).map(|t| t.text),
            video_count_short: node.get("videoCountShortText").and_then(TextNode::from_value).map(|t| t.text),
        })
    }
}

/// Strongly typed GridMix AST node (`gridRadioRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridMixNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub video_count: Option<String>,
    pub video_count_short: Option<String>,
    pub endpoint: Option<Value>,
    pub secondary_endpoint: Option<Value>,
    pub thumbnail_overlays: Option<Vec<Value>>,
}

impl GridMixNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridRadioRenderer").unwrap_or(val);
        
        let author = if let Some(short) = node.get("shortBylineText") {
            TextNode::from_value(short).map(|t| t.text)
        } else if let Some(long) = node.get("longBylineText") {
            TextNode::from_value(long).map(|t| t.text)
        } else {
            None
        };

        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            author,
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            video_count: node.get("videoCountText").and_then(TextNode::from_value).map(|t| t.text),
            video_count_short: node.get("videoCountShortText").and_then(TextNode::from_value).map(|t| t.text),
            endpoint: node.get("navigationEndpoint").cloned(),
            secondary_endpoint: node.get("secondaryNavigationEndpoint").cloned(),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed GridMovie AST node (`gridMovieRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridMovieNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub duration: Option<String>,
    pub endpoint: Option<Value>,
    pub badges: Option<Vec<Value>>,
    pub metadata: Option<String>,
    pub thumbnail_overlays: Option<Vec<Value>>,
}

impl GridMovieNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridMovieRenderer").unwrap_or(val);
        
        let length_alt = node.get("thumbnailOverlays")
            .and_then(|overlays| overlays.as_array())
            .and_then(|arr| arr.iter().find(|o| o.get("thumbnailOverlayTimeStatusRenderer").is_some()))
            .and_then(|o| o.get("thumbnailOverlayTimeStatusRenderer"));
            
        let duration = if let Some(length_text) = node.get("lengthText") {
            TextNode::from_value(length_text).map(|t| t.text)
        } else if let Some(alt) = length_alt {
            TextNode::from_value(alt.get("text").unwrap_or(&Value::Null)).map(|t| t.text)
        } else {
            None
        };

        Some(Self {
            id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            duration,
            endpoint: node.get("navigationEndpoint").cloned(),
            badges: node.get("badges").and_then(|v| v.as_array()).cloned(),
            metadata: node.get("metadata").and_then(TextNode::from_value).map(|t| t.text),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed GridShow AST node (`gridShowRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridShowNode {
    pub title: Option<String>,
    pub thumbnail_renderer: Option<Value>,
    pub endpoint: Option<Value>,
    pub long_byline_text: Option<String>,
    pub thumbnail_overlays: Option<Vec<Value>>,
    pub author: Option<Value>,
}

impl GridShowNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridShowRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            thumbnail_renderer: node.get("thumbnailRenderer").cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            long_byline_text: node.get("longBylineText").and_then(TextNode::from_value).map(|t| t.text),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
            author: node.get("shortBylineText").cloned(),
        })
    }
}

/// Strongly typed CompactVideo AST node (`compactVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactVideoNode {
    pub video_id: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub rich_thumbnail: Option<Value>,
    pub title: Option<String>,
    pub author: Option<Value>,
    pub view_count: Option<String>,
    pub short_view_count: Option<String>,
    pub short_byline_text: Option<String>,
    pub long_byline_text: Option<String>,
    pub published: Option<String>,
    pub badges: Option<Vec<Value>>,
    pub thumbnail_overlays: Option<Vec<Value>>,
    pub endpoint: Option<Value>,
    pub menu: Option<Value>,
    pub length_text: Option<String>,
    pub is_watched: bool,
    pub service_endpoints: Option<Vec<Value>>,
    pub service_endpoint: Option<Value>,
    pub style: Option<String>,
}

impl CompactVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactVideoRenderer").unwrap_or(val);
        Some(Self {
            video_id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            rich_thumbnail: node.get("richThumbnail").cloned(),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            author: node.get("longBylineText").cloned(),
            view_count: node.get("viewCountText").and_then(TextNode::from_value).map(|t| t.text),
            short_view_count: node.get("shortViewCountText").and_then(TextNode::from_value).map(|t| t.text),
            short_byline_text: node.get("shortBylineText").and_then(TextNode::from_value).map(|t| t.text),
            long_byline_text: node.get("longBylineText").and_then(TextNode::from_value).map(|t| t.text),
            published: node.get("publishedTimeText").and_then(TextNode::from_value).map(|t| t.text),
            badges: node.get("badges").and_then(|v| v.as_array()).cloned(),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            menu: node.get("menu").cloned(),
            length_text: node.get("lengthText").and_then(TextNode::from_value).map(|t| t.text),
            is_watched: node.get("isWatched").and_then(|v| v.as_bool()).unwrap_or(false),
            service_endpoints: node.get("serviceEndpoints").and_then(|v| v.as_array()).cloned(),
            service_endpoint: node.get("serviceEndpoint").cloned(),
            style: node.get("style").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed CompactChannel AST node (`compactChannelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactChannelNode {
    pub title: Option<String>,
    pub channel_id: Option<String>,
    pub thumbnail: Option<ThumbnailListNode>,
    pub display_name: Option<String>,
    pub video_count: Option<String>,
    pub subscriber_count: Option<String>,
    pub endpoint: Option<Value>,
    pub tv_banner: Option<ThumbnailListNode>,
    pub menu: Option<Value>,
}

impl CompactChannelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactChannelRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            channel_id: node.get("channelId").and_then(|v| v.as_str()).map(String::from),
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value),
            display_name: node.get("displayName").and_then(TextNode::from_value).map(|t| t.text),
            video_count: node.get("videoCountText").and_then(TextNode::from_value).map(|t| t.text),
            subscriber_count: node.get("subscriberCountText").and_then(TextNode::from_value).map(|t| t.text),
            endpoint: node.get("navigationEndpoint").cloned(),
            tv_banner: node.get("tvBanner").map(ThumbnailListNode::from_value),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed CompactPlaylist AST node (`compactPlaylistRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactPlaylistNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Value>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub thumbnail_renderer: Option<Value>,
    pub video_count: Option<String>,
    pub video_count_short: Option<String>,
    pub first_videos: Option<Vec<Value>>,
    pub share_url: Option<String>,
    pub menu: Option<Value>,
    pub badges: Option<Vec<Value>>,
    pub endpoint: Option<Value>,
    pub thumbnail_overlays: Option<Vec<Value>>,
    pub view_playlist: Option<String>,
}

impl CompactPlaylistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactPlaylistRenderer").unwrap_or(val);
        
        let author = node.get("shortBylineText").or_else(|| node.get("longBylineText")).cloned();

        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            author,
            thumbnails: node.get("thumbnail").or_else(|| node.get("thumbnails")).map(ThumbnailListNode::from_value),
            thumbnail_renderer: node.get("thumbnailRenderer").cloned(),
            video_count: node.get("thumbnailText").and_then(TextNode::from_value).map(|t| t.text),
            video_count_short: node.get("videoCountShortText").and_then(TextNode::from_value).map(|t| t.text),
            first_videos: node.get("videos").and_then(|v| v.as_array()).cloned(),
            share_url: node.get("shareUrl").and_then(|v| v.as_str()).map(String::from),
            menu: node.get("menu").cloned(),
            badges: node.get("ownerBadges").and_then(|v| v.as_array()).cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
            view_playlist: node.get("viewPlaylistText").and_then(TextNode::from_value).map(|t| t.text),
        })
    }
}

/// Strongly typed CompactMix AST node (`compactRadioRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactMixNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Value>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub thumbnail_renderer: Option<Value>,
    pub video_count: Option<String>,
    pub video_count_short: Option<String>,
    pub first_videos: Option<Vec<Value>>,
    pub share_url: Option<String>,
    pub menu: Option<Value>,
    pub badges: Option<Vec<Value>>,
    pub endpoint: Option<Value>,
    pub thumbnail_overlays: Option<Vec<Value>>,
    pub view_playlist: Option<String>,
}

impl CompactMixNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactRadioRenderer").unwrap_or(val);
        
        let author = node.get("shortBylineText").or_else(|| node.get("longBylineText")).cloned();

        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value).map(|t| t.text),
            author,
            thumbnails: node.get("thumbnail").or_else(|| node.get("thumbnails")).map(ThumbnailListNode::from_value),
            thumbnail_renderer: node.get("thumbnailRenderer").cloned(),
            video_count: node.get("thumbnailText").and_then(TextNode::from_value).map(|t| t.text),
            video_count_short: node.get("videoCountShortText").and_then(TextNode::from_value).map(|t| t.text),
            first_videos: node.get("videos").and_then(|v| v.as_array()).cloned(),
            share_url: node.get("shareUrl").and_then(|v| v.as_str()).map(String::from),
            menu: node.get("menu").cloned(),
            badges: node.get("ownerBadges").and_then(|v| v.as_array()).cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned(),
            view_playlist: node.get("viewPlaylistText").and_then(TextNode::from_value).map(|t| t.text),
        })
    }
}

/// Strongly typed RichItem AST node (`richItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichItemNode {
    pub content: Option<Value>,
}

impl RichItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("richItemRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").cloned(),
        })
    }
}

/// Strongly typed RichSection AST node (`richSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichSectionNode {
    pub content: Option<Value>,
    pub full_bleed: bool,
    pub target_id: Option<String>,
}

impl RichSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("richSectionRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").cloned(),
            full_bleed: node.get("fullBleed").and_then(|v| v.as_bool()).unwrap_or(false),
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
        })
    }
}
