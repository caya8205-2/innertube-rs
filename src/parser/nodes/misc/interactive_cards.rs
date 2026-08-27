use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed CardCollection AST node (`cardCollectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardCollectionNode {
    pub cards: Vec<Value>,
    pub header: Option<TextNode>,
    pub allow_teaser_dismiss: bool,
}

impl CardCollectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("cardCollectionRenderer").unwrap_or(val);
        Some(Self {
            cards: node
                .get("cards")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
            header: node.get("headerText").and_then(TextNode::from_value),
            allow_teaser_dismiss: node
                .get("allowTeaserDismiss")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        })
    }
}

/// Strongly typed CollaboratorInfoCardContent AST node (`collaboratorInfoCardContentRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaboratorInfoCardContentNode {
    pub channel_avatar: ThumbnailListNode,
    pub custom_text: Option<TextNode>,
    pub channel_name: Option<TextNode>,
    pub subscriber_count: Option<TextNode>,
    pub endpoint: Option<Value>,
}

impl CollaboratorInfoCardContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("collaboratorInfoCardContentRenderer").unwrap_or(val);
        Some(Self {
            channel_avatar: ThumbnailListNode::from_value(node.get("channelAvatar").unwrap_or(&serde_json::Value::Null)),
            custom_text: node.get("customText").and_then(TextNode::from_value),
            channel_name: node.get("channelName").and_then(TextNode::from_value),
            subscriber_count: node.get("subscriberCountText").and_then(TextNode::from_value),
            endpoint: node.get("endpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackColorNode {
    pub light_theme: Option<f64>,
    pub dark_theme: Option<f64>,
}

/// Strongly typed CollectionThumbnailView AST node (`collectionThumbnailView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionThumbnailViewNode {
    pub primary_thumbnail: Option<Value>,
    pub stack_color: Option<StackColorNode>,
}

impl CollectionThumbnailViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("collectionThumbnailView").unwrap_or(val);
        
        let stack_color = node.get("stackColor").map(|sc| StackColorNode {
            light_theme: sc.get("lightTheme").and_then(|v| v.as_f64()),
            dark_theme: sc.get("darkTheme").and_then(|v| v.as_f64()),
        });

        Some(Self {
            primary_thumbnail: node.get("primaryThumbnail").cloned(),
            stack_color,
        })
    }
}

/// Strongly typed ClipAdState AST node (`clipAdStateRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipAdStateNode {
    pub title: Option<TextNode>,
    pub body: Option<TextNode>,
}

impl ClipAdStateNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("clipAdStateRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            body: node.get("body").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed ClipCreationTextInput AST node (`clipCreationTextInputRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipCreationTextInputNode {
    pub placeholder_text: Option<TextNode>,
    pub max_character_limit: Option<u64>,
}

impl ClipCreationTextInputNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("clipCreationTextInputRenderer").unwrap_or(val);
        Some(Self {
            placeholder_text: node.get("placeholderText").and_then(TextNode::from_value),
            max_character_limit: node.get("maxCharacterLimit").and_then(|v| v.as_u64()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingDirectivesVisibilityNode {
    pub types: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingDirectivesNode {
    pub visibility: Option<LoggingDirectivesVisibilityNode>,
    pub enable_displaylogger_experiment: bool,
}

/// Strongly typed ClientSideToggleMenuItem AST node (`clientSideToggleMenuItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSideToggleMenuItemNode {
    pub text: Option<TextNode>,
    pub icon_type: Option<String>,
    pub toggled_text: Option<TextNode>,
    pub toggled_icon_type: Option<String>,
    pub is_toggled: Option<bool>,
    pub menu_item_identifier: Option<String>,
    pub endpoint: Option<Value>,
    pub logging_directives: Option<LoggingDirectivesNode>,
}

impl ClientSideToggleMenuItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("clientSideToggleMenuItemRenderer").unwrap_or(val);
        
        let logging_directives = node.get("loggingDirectives").map(|ld| LoggingDirectivesNode {
            visibility: ld.get("visibility").map(|v| LoggingDirectivesVisibilityNode {
                types: v.get("types").and_then(|t| t.as_str()).map(|t| t.to_string()),
            }),
            enable_displaylogger_experiment: ld.get("enableDisplayloggerExperiment").and_then(|v| v.as_bool()).unwrap_or(false),
        });

        Some(Self {
            text: node.get("defaultText").and_then(TextNode::from_value),
            icon_type: node.get("defaultIcon").and_then(|v| v.get("iconType")).and_then(|v| v.as_str()).map(|s| s.to_string()),
            toggled_text: node.get("toggledText").and_then(TextNode::from_value),
            toggled_icon_type: node.get("toggledIcon").and_then(|v| v.get("iconType")).and_then(|v| v.as_str()).map(|s| s.to_string()),
            is_toggled: node.get("isToggled").and_then(|v| v.as_bool()),
            menu_item_identifier: node.get("menuItemIdentifier").and_then(|v| v.as_str()).map(|s| s.to_string()),
            endpoint: node.get("command").cloned(),
            logging_directives,
        })
    }
}

/// Strongly typed AudioOnlyPlayability AST node (`audioOnlyPlayabilityRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOnlyPlayabilityNode {
    pub audio_only_availability: Option<String>,
}

impl AudioOnlyPlayabilityNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("audioOnlyPlayabilityRenderer").unwrap_or(val);
        Some(Self {
            audio_only_availability: node.get("audioOnlyAvailability").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DurationNode {
    pub text: Option<String>,
    pub seconds: Option<u64>,
}

/// Strongly typed CompactMovie AST node (`compactMovieRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactMovieNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub top_metadata_items: Option<TextNode>,
    pub thumbnails: ThumbnailListNode,
    pub thumbnail_overlays: Vec<Value>,
    pub author: Option<Value>,
    pub duration: Option<DurationNode>,
    pub endpoint: Option<Value>,
    pub badges: Vec<Value>,
    pub use_vertical_poster: bool,
    pub menu: Option<Value>,
}

impl CompactMovieNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactMovieRenderer").unwrap_or(val);
        
        let overlay_time_status = node.get("thumbnailOverlays")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.iter().find(|o| o.get("thumbnailOverlayTimeStatusRenderer").is_some()))
            .and_then(|o| o.get("thumbnailOverlayTimeStatusRenderer"))
            .and_then(|o| o.get("text"))
            .and_then(TextNode::from_value);

        let length_text = node.get("lengthText").and_then(TextNode::from_value).or(overlay_time_status);
        let duration = length_text.as_ref().map(|t| DurationNode {
            text: Some(t.to_string()),
            seconds: None, 
        });

        Some(Self {
            id: node.get("videoId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            title: node.get("title").and_then(TextNode::from_value),
            top_metadata_items: node.get("topMetadataItems").and_then(TextNode::from_value),
            thumbnails: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&serde_json::Value::Null)),
            thumbnail_overlays: node.get("thumbnailOverlays").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            author: node.get("shortBylineText").cloned(),
            duration,
            endpoint: node.get("navigationEndpoint").cloned(),
            badges: node.get("badges").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            use_vertical_poster: node.get("useVerticalPoster").and_then(|v| v.as_bool()).unwrap_or(false),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed CompactStation AST node (`compactStationRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompactStationNode {
    pub title: Option<TextNode>,
    pub description: Option<TextNode>,
    pub video_count: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub thumbnail: ThumbnailListNode,
}

impl CompactStationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("compactStationRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            description: node.get("description").and_then(TextNode::from_value),
            video_count: node.get("videoCountText").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&serde_json::Value::Null)),
        })
    }
}

/// Strongly typed AddToPlaylist AST node (`addToPlaylistRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylistNode {
    pub actions: Vec<Value>,
    pub playlists: Vec<Value>,
}

impl AddToPlaylistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("addToPlaylistRenderer").unwrap_or(val);
        Some(Self {
            actions: node.get("actions").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            playlists: node.get("playlists").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
        })
    }
}

/// Strongly typed C4TabbedHeader AST node (`c4TabbedHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct C4TabbedHeaderNode {
    pub author: Option<Value>,
    pub banner: Option<ThumbnailListNode>,
    pub tv_banner: Option<ThumbnailListNode>,
    pub mobile_banner: Option<ThumbnailListNode>,
    pub subscribers: Option<TextNode>,
    pub videos_count: Option<TextNode>,
    pub sponsor_button: Option<Value>,
    pub subscribe_button: Option<Value>,
    pub header_links: Option<Value>,
    pub channel_handle: Option<TextNode>,
    pub channel_id: Option<String>,
    pub tagline: Option<Value>,
}

impl C4TabbedHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("c4TabbedHeaderRenderer").unwrap_or(val);
        
        let mut author_data = serde_json::Map::new();
        if let Some(title) = node.get("title") { author_data.insert("simpleText".to_string(), title.clone()); }
        if let Some(ne) = node.get("navigationEndpoint") { author_data.insert("navigationEndpoint".to_string(), ne.clone()); }
        if let Some(badges) = node.get("badges") { author_data.insert("badges".to_string(), badges.clone()); }
        if let Some(avatar) = node.get("avatar") { author_data.insert("avatar".to_string(), avatar.clone()); }
        let author = if author_data.is_empty() { None } else { Some(serde_json::Value::Object(author_data)) };

        Some(Self {
            author,
            banner: node.get("banner").map(ThumbnailListNode::from_value),
            tv_banner: node.get("tvBanner").map(ThumbnailListNode::from_value),
            mobile_banner: node.get("mobileBanner").map(ThumbnailListNode::from_value),
            subscribers: node.get("subscriberCountText").and_then(TextNode::from_value),
            videos_count: node.get("videosCountText").and_then(TextNode::from_value),
            sponsor_button: node.get("sponsorButton").cloned(),
            subscribe_button: node.get("subscribeButton").cloned(),
            header_links: node.get("headerLinks").cloned(),
            channel_handle: node.get("channelHandleText").and_then(TextNode::from_value),
            channel_id: node.get("channelId").and_then(|v| v.as_str()).map(|s| s.to_string()),
            tagline: node.get("tagline").cloned(),
        })
    }
}

/// Strongly typed ChannelSwitcherPage AST node (`channelSwitcherPageRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSwitcherPageNode {
    pub header: Option<Value>,
    pub contents: Option<Vec<Value>>,
}

impl ChannelSwitcherPageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelSwitcherPageRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("header").cloned(),
            contents: node.get("contents").and_then(|v| v.as_array()).cloned(),
        })
    }
}
