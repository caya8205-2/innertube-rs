use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::author::AuthorNode;
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// Represents a full playlist or playlist metadata header
/// (1:1 port consolidating `Playlist.ts`, `PlaylistHeader.ts`, `PageHeader.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaylistNode {
    pub id: String,
    pub title: String,
    pub author: Option<AuthorNode>,
    pub description: Option<String>,
    pub video_count: Option<u32>,
    pub view_count: Option<String>,
    pub last_updated: Option<String>,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
}

/// Represents an individual video item within a playlist (`PlaylistVideo.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaylistVideoNode {
    pub id: String,
    pub title: String,
    pub author: Option<AuthorNode>,
    pub duration: Option<String>,
    pub duration_ms: Option<u64>,
    pub thumbnails: ThumbnailListNode,
    pub index: Option<u32>,
    pub is_playable: bool,
}

impl PlaylistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("playlistHeaderRenderer")
            .or_else(|| val.get("pageHeaderRenderer"))
            .or_else(|| val.get("playlistRenderer"))
            .or_else(|| val.get("gridPlaylistRenderer"))
            .or_else(|| val.pointer("/header/playlistHeaderRenderer"))
            .or_else(|| val.pointer("/header/pageHeaderRenderer"))
            .unwrap_or(val);

        let id = target.get("playlistId")
            .or_else(|| target.get("id"))
            .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/playlistId"))
            .and_then(|i| i.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let title = target.get("title")
            .or_else(|| target.get("pageTitle"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/title"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_else(|| "Untitled Playlist".to_string());

        let author = target.get("ownerText")
            .or_else(|| target.get("author"))
            .or_else(|| target.get("shortBylineText"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/avatarStack/avatarStackViewModel/text"))
            .and_then(AuthorNode::from_value);

        let description = target.get("descriptionText")
            .or_else(|| target.get("description"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/description/descriptionViewModel/description"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let mut video_count = None;
        let mut view_count = None;

        if let Some(num_str) = target.get("numVideosText")
            .or_else(|| target.get("videoCountText"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/1/text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
        {
            let digits: String = num_str.chars().filter(|c| c.is_ascii_digit()).collect();
            video_count = digits.parse().ok();
        }

        if let Some(views_str) = target.get("viewCountText")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/2/text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
        {
            view_count = Some(views_str);
        }

        let last_updated = target.get("lastUpdated")
            .or_else(|| target.get("lastUpdatedText"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let thumbnails = ThumbnailListNode::from_value(
            target.pointer("/playlistHeaderBanner/heroPlaylistThumbnailRenderer/thumbnail")
                .or_else(|| target.pointer("/content/pageHeaderViewModel/heroContent/previewImage/contentPreviewImageViewModel/image"))
                .or_else(|| target.get("thumbnail"))
                .unwrap_or(target)
        );

        let endpoint = target.get("navigationEndpoint").and_then(NavigationEndpointNode::from_value);

        Some(Self {
            id,
            title,
            author,
            description,
            video_count,
            view_count,
            last_updated,
            thumbnails,
            endpoint,
        })
    }
}

impl PlaylistVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("playlistVideoRenderer")
            .or_else(|| val.get("lockupViewModel"))
            .unwrap_or(val);

        // 1. Check lockupViewModel
        if target.get("contentType").is_some() || target.pointer("/metadata/lockupMetadataViewModel").is_some() {
            let id = target.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId")
                .or_else(|| target.get("contentId"))
                .and_then(|v| v.as_str())?
                .to_string();

            let title = target.pointer("/metadata/lockupMetadataViewModel/title/content")
                .and_then(|t| t.as_str())
                .unwrap_or("Untitled")
                .to_string();

            let author = target.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text")
                .and_then(AuthorNode::from_value);

            let duration = target.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/overlays/0/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text")
                .or_else(|| target.pointer("/contentImage/thumbnailViewModel/overlays/0/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text"))
                .and_then(|d| d.as_str())
                .map(|s| s.to_string());

            let duration_ms = duration.as_deref().and_then(parse_duration_string_to_ms);

            let thumbnails = ThumbnailListNode::from_value(
                target.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/image")
                    .or_else(|| target.pointer("/contentImage/thumbnailViewModel/image"))
                    .unwrap_or(target)
            );

            return Some(Self {
                id,
                title,
                author,
                duration,
                duration_ms,
                thumbnails,
                index: None,
                is_playable: true,
            });
        }

        // 2. Standard playlistVideoRenderer
        let id = target.get("videoId")
            .and_then(|v| v.as_str())
            .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/videoId").and_then(|v| v.as_str()))
            .map(|s| s.to_string())?;

        let title = target.get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_else(|| "Untitled".to_string());

        let author = target.get("shortBylineText")
            .or_else(|| target.get("ownerText"))
            .and_then(AuthorNode::from_value);

        let duration = target.get("lengthText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let duration_ms = target.get("lengthSeconds")
            .and_then(|s| s.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .map(|sec| sec * 1000)
            .or_else(|| duration.as_deref().and_then(parse_duration_string_to_ms));

        let thumbnails = ThumbnailListNode::from_value(target.get("thumbnail").unwrap_or(target));

        let index = target.get("index")
            .and_then(TextNode::from_value)
            .and_then(|t| t.text.parse::<u32>().ok());

        let is_playable = target.get("isPlayable").and_then(|p| p.as_bool()).unwrap_or(true);

        Some(Self {
            id,
            title,
            author,
            duration,
            duration_ms,
            thumbnails,
            index,
            is_playable,
        })
    }
}

fn parse_duration_string_to_ms(d: &str) -> Option<u64> {
    let parts: Vec<&str> = d.split(':').collect();
    if parts.len() == 2 {
        let m: u64 = parts[0].parse().ok()?;
        let s: u64 = parts[1].parse().ok()?;
        Some((m * 60 + s) * 1000)
    } else if parts.len() == 3 {
        let h: u64 = parts[0].parse().ok()?;
        let m: u64 = parts[1].parse().ok()?;
        let s: u64 = parts[2].parse().ok()?;
        Some((h * 3600 + m * 60 + s) * 1000)
    } else {
        None
    }
}

/// An individual video row in a watch next playlist panel (`PlaylistPanelVideoRenderer.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaylistPanelVideoNode {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub duration: Option<String>,
    pub selected: bool,
}

impl PlaylistPanelVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("playlistPanelVideoRenderer").unwrap_or(val);
        let id = target
            .get("videoId")
            .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/videoId"))
            .and_then(Value::as_str)?
            .to_string();

        let title = target
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let author = target
            .get("longBylineText")
            .or_else(|| target.get("shortBylineText"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let duration = target
            .get("lengthText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let selected = target
            .get("selected")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        Some(Self {
            id,
            title,
            author,
            duration,
            selected,
        })
    }
}

/// Watch next playlist panel (`PlaylistPanel.ts` / `playlistPanelRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaylistPanelNode {
    pub title: String,
    pub playlist_id: Option<String>,
    pub num_videos_text: Option<String>,
    pub items: Vec<PlaylistPanelVideoNode>,
}

impl PlaylistPanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("playlistPanelRenderer").unwrap_or(val);
        if target.get("contents").is_none() && target.get("title").is_none() {
            return None;
        }

        let title = target
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let playlist_id = target
            .get("playlistId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let num_videos_text = target
            .get("numVideosText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let mut items = Vec::new();
        if let Some(arr) = target.get("contents").and_then(|c| c.as_array()) {
            for item in arr {
                if let Some(pv) = PlaylistPanelVideoNode::from_value(item) {
                    items.push(pv);
                }
            }
        }

        Some(Self {
            title,
            playlist_id,
            num_videos_text,
            items,
        })
    }
}

/// Strongly typed PlaylistMetadata AST node (`playlistMetadataRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistMetadataNode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub privacy: Option<String>,
}

impl PlaylistMetadataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistMetadataRenderer").unwrap_or(val);
        let title = node.get("title").and_then(Value::as_str).map(ToString::to_string);
        let description = node.get("description").and_then(Value::as_str).map(ToString::to_string);
        let privacy = node.get("privacy").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            title,
            description,
            privacy,
        })
    }
}

/// Strongly typed PlaylistSidebarPrimaryInfo AST node (`playlistSidebarPrimaryInfoRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarPrimaryInfoNode {
    pub title: Option<String>,
    pub stats: Vec<String>,
    pub thumbnails: ThumbnailListNode,
}

impl PlaylistSidebarPrimaryInfoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistSidebarPrimaryInfoRenderer").unwrap_or(val);
        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);

        let mut stats = Vec::new();
        if let Some(arr) = node.get("stats").and_then(Value::as_array) {
            for item in arr {
                if let Some(txt) = TextNode::from_value(item) {
                    stats.push(txt.text);
                }
            }
        }

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnailRenderer").unwrap_or(node));

        Some(Self {
            title,
            stats,
            thumbnails,
        })
    }
}

/// Strongly typed PlaylistSidebarSecondaryInfo AST node (`playlistSidebarSecondaryInfoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarSecondaryInfoNode {
    pub owner: Option<Value>,
    pub button: Option<Value>,
}

impl PlaylistSidebarSecondaryInfoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistSidebarSecondaryInfoRenderer").unwrap_or(val);
        let owner = node.get("videoOwner").cloned();
        let button = node.get("button").cloned();

        Some(Self { owner, button })
    }
}

