use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed MetadataRowContainer AST node (`metadataRowContainerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowContainerNode {
    pub rows: Vec<Value>,
    pub collapsed_item_count: Option<u64>,
}

impl MetadataRowContainerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("metadataRowContainerRenderer").unwrap_or(val);
        Some(Self {
            rows: node
                .get("rows")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            collapsed_item_count: node
                .get("collapsedItemCount")
                .and_then(|v| v.as_u64()),
        })
    }
}

/// Strongly typed MetadataRowHeader AST node (`metadataRowHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowHeaderNode {
    pub content: Option<TextNode>,
    pub has_divider_line: bool,
}

impl MetadataRowHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("metadataRowHeaderRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").and_then(TextNode::from_value),
            has_divider_line: node
                .get("hasDividerLine")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        })
    }
}

/// Strongly typed MetadataScreen AST node (`metadataScreenRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataScreenNode {
    pub section_list: Option<Value>,
}

impl MetadataScreenNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("metadataScreenRenderer").unwrap_or(val);
        Some(Self {
            section_list: Some(node.clone()),
        })
    }
}

/// Strongly typed Mix AST node (`mixRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixNode {
    pub raw: Option<Value>,
}

impl MixNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("mixRenderer").unwrap_or(val);
        Some(Self {
            raw: Some(node.clone()),
        })
    }
}

/// Strongly typed Movie AST node (`movieRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub description_snippet: Option<TextNode>,
    pub top_metadata_items: Option<TextNode>,
    pub thumbnails: ThumbnailListNode,
    pub thumbnail_overlays: Vec<Value>,
    pub author: Option<Value>,
    pub endpoint: Option<Value>,
    pub badges: Vec<Value>,
    pub use_vertical_poster: bool,
    pub show_action_menu: bool,
    pub menu: Option<Value>,
}

impl MovieNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("movieRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("title").and_then(TextNode::from_value),
            description_snippet: node.get("descriptionSnippet").and_then(TextNode::from_value),
            top_metadata_items: node.get("topMetadataItems").and_then(TextNode::from_value),
            thumbnails: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&serde_json::Value::Null)),
            thumbnail_overlays: node
                .get("thumbnailOverlays")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            author: node.get("longBylineText").cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            badges: node
                .get("badges")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            use_vertical_poster: node.get("useVerticalPoster").and_then(|v| v.as_bool()).unwrap_or(false),
            show_action_menu: node.get("showActionMenu").and_then(|v| v.as_bool()).unwrap_or(false),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed MovingThumbnail AST node (`movingThumbnailRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovingThumbnailNode {
    pub thumbnails: ThumbnailListNode,
}

impl MovingThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("movingThumbnailRenderer").unwrap_or(val);
        Some(Self {
            thumbnails: ThumbnailListNode::from_value(
                node.get("movingThumbnailDetails")
                    .and_then(|v| v.get("thumbnails"))
                    .unwrap_or(&serde_json::Value::Null)
            ),
        })
    }
}

/// Helper struct for Marker
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkerNode {
    pub marker_key: Option<String>,
    pub heatmap: Option<Value>,
    pub chapters: Vec<Value>,
}

impl MarkerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            marker_key: val.get("key").and_then(|v| v.as_str()).map(String::from),
            heatmap: val.get("value").and_then(|v| v.get("heatmap")).cloned(),
            chapters: val
                .get("value")
                .and_then(|v| v.get("chapters"))
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MultiMarkersPlayerBar AST node (`multiMarkersPlayerBarRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiMarkersPlayerBarNode {
    pub markers_map: Vec<MarkerNode>,
}

impl MultiMarkersPlayerBarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("multiMarkersPlayerBarRenderer").unwrap_or(val);
        Some(Self {
            markers_map: node
                .get("markersMap")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(MarkerNode::from_value).collect())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MusicCardShelfHeaderBasic AST node (`musicCardShelfHeaderBasicRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicCardShelfHeaderBasicNode {
    pub title: Option<TextNode>,
}

impl MusicCardShelfHeaderBasicNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicCardShelfHeaderBasicRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed MusicCarouselShelfBasicHeader AST node (`musicCarouselShelfBasicHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicCarouselShelfBasicHeaderNode {
    pub title: Option<TextNode>,
    pub strapline: Option<TextNode>,
    pub thumbnail: Option<Value>,
    pub more_content: Option<Value>,
    pub end_icons: Vec<Value>,
}

impl MusicCarouselShelfBasicHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicCarouselShelfBasicHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            strapline: node.get("strapline").and_then(TextNode::from_value),
            thumbnail: node.get("thumbnail").cloned(),
            more_content: node.get("moreContentButton").cloned(),
            end_icons: node
                .get("endIcons")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Helper struct for ActionButton
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionButtonNode {
    pub icon_name: Option<String>,
    pub endpoint: Option<Value>,
    pub a11y_text: Option<String>,
    pub style: Option<String>,
}

impl ActionButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            icon_name: val.get("iconName").and_then(|v| v.as_str()).map(String::from),
            endpoint: val.get("onTap").cloned(),
            a11y_text: val.get("a11yText").and_then(|v| v.as_str()).map(String::from),
            style: val.get("style").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Helper struct for Panel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelNode {
    pub image: ThumbnailListNode,
    pub content_mode: Option<String>,
    pub crop_options: Option<String>,
    pub image_aspect_ratio: Option<String>,
    pub caption: Option<String>,
    pub action_buttons: Vec<ActionButtonNode>,
}

impl PanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            image: ThumbnailListNode::from_value(
                val.get("image").and_then(|v| v.get("image")).unwrap_or(&serde_json::Value::Null)
            ),
            content_mode: val.get("image").and_then(|v| v.get("contentMode")).and_then(|v| v.as_str()).map(String::from),
            crop_options: val.get("image").and_then(|v| v.get("cropOptions")).and_then(|v| v.as_str()).map(String::from),
            image_aspect_ratio: val.get("imageAspectRatio").and_then(|v| v.as_str()).map(String::from),
            caption: val.get("caption").and_then(|v| v.as_str()).map(String::from),
            action_buttons: val
                .get("actionButtons")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(ActionButtonNode::from_value).collect())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MusicLargeCardItemCarousel AST node (`musicLargeCardItemCarouselRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicLargeCardItemCarouselNode {
    pub panels: Vec<PanelNode>,
    pub header: Option<Value>,
}

impl MusicLargeCardItemCarouselNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicLargeCardItemCarouselRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("shelf").and_then(|v| v.get("header")).cloned(),
            panels: node
                .get("shelf")
                .and_then(|v| v.get("panels"))
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(PanelNode::from_value).collect())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MusicMultiRowListItem AST node (`musicMultiRowListItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicMultiRowListItemNode {
    pub thumbnail: Option<Value>,
    pub overlay: Option<Value>,
    pub on_tap: Option<Value>,
    pub menu: Option<Value>,
    pub subtitle: Option<TextNode>,
    pub title: Option<TextNode>,
    pub second_title: Option<TextNode>,
    pub description: Option<TextNode>,
    pub display_style: Option<String>,
}

impl MusicMultiRowListItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicMultiRowListItemRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").cloned(),
            overlay: node.get("overlay").cloned(),
            on_tap: node.get("onTap").cloned(),
            menu: node.get("menu").cloned(),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            title: node.get("title").and_then(TextNode::from_value),
            second_title: node.get("secondTitle").and_then(TextNode::from_value),
            description: node.get("description").and_then(TextNode::from_value),
            display_style: node.get("displayStyle").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed MusicPlaylistEditHeader AST node (`musicPlaylistEditHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlaylistEditHeaderNode {
    pub title: Option<TextNode>,
    pub edit_title: Option<TextNode>,
    pub edit_description: Option<TextNode>,
    pub privacy: Option<String>,
    pub playlist_id: Option<String>,
    pub endpoint: Option<Value>,
    pub privacy_dropdown: Option<Value>,
}

impl MusicPlaylistEditHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicPlaylistEditHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            edit_title: node.get("editTitle").and_then(TextNode::from_value),
            edit_description: node.get("editDescription").and_then(TextNode::from_value),
            privacy: node.get("privacy").and_then(|v| v.as_str()).map(String::from),
            playlist_id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            endpoint: node.get("collaborationSettingsCommand").cloned(),
            privacy_dropdown: node.get("privacyDropdown").cloned(),
        })
    }
}
