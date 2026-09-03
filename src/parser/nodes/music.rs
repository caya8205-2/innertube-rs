use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::author::AuthorNode;
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// Represents a YouTube Music list item (`MusicResponsiveListItem.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicResponsiveListItemNode {
    pub id: Option<String>,
    pub title: String,
    pub artists: Vec<AuthorNode>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub duration: Option<String>,
    pub duration_ms: Option<u64>,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
    pub item_type: Option<String>,
    pub is_explicit: bool,
}

/// Represents a YouTube Music two-row grid card (`MusicTwoRowItem.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicTwoRowItemNode {
    pub id: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub track_count: Option<u32>,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
    pub item_type: Option<String>,
}

/// Represents a YouTube Music description/lyrics shelf (`MusicDescriptionShelf.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicDescriptionShelfNode {
    pub header: Option<String>,
    pub description: String,
    pub footer: Option<String>,
}

impl MusicResponsiveListItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("musicResponsiveListItemRenderer").unwrap_or(val);

        let id = target.pointer("/playlistItemData/videoId")
            .or_else(|| target.pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/navigationEndpoint/watchEndpoint/videoId"))
            .or_else(|| target.pointer("/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let title = target.pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let mut artists = Vec::new();
        let mut album = None;
        let mut album_id = None;
        let mut duration = None;
        let mut item_type = None;

        if let Some(col1) = target.pointer("/flexColumns/1/musicResponsiveListItemFlexColumnRenderer/text/runs").and_then(|r| r.as_array()) {
            for (i, run) in col1.iter().enumerate() {
                let text = run.get("text").and_then(|t| t.as_str()).unwrap_or("");
                if text == " • " || text.is_empty() {
                    continue;
                }

                if let Some(bid) = run.pointer("/navigationEndpoint/browseEndpoint/browseId").and_then(|b| b.as_str()) {
                    if bid.starts_with("UC") || bid.starts_with("FEmusic_library_privately_owned_artist") {
                        if let Some(author) = AuthorNode::from_value(run) {
                            artists.push(author);
                        }
                    } else if bid.starts_with("MPREb_") || bid.starts_with("FEmusic_library_privately_owned_release") {
                        album = Some(text.to_string());
                        album_id = Some(bid.to_string());
                    }
                } else if text.contains(':') && duration.is_none() {
                    duration = Some(text.to_string());
                } else if i == 0 && artists.is_empty() {
                    item_type = Some(text.to_string());
                }
            }
        }

        let duration_ms = duration.as_deref().and_then(parse_duration_string_to_ms);

        let thumbnails = ThumbnailListNode::from_value(
            target.pointer("/thumbnail/musicThumbnailRenderer")
                .or_else(|| target.get("thumbnail"))
                .unwrap_or(target)
        );

        let endpoint = target.pointer("/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/navigationEndpoint")
            .and_then(NavigationEndpointNode::from_value);

        let is_explicit = target.pointer("/badges/0/musicInlineBadgeRenderer/icon/iconType")
            .and_then(|i| i.as_str()) == Some("MUSIC_EXPLICIT_BADGE");

        Some(Self {
            id,
            title,
            artists,
            album,
            album_id,
            duration,
            duration_ms,
            thumbnails,
            endpoint,
            item_type,
            is_explicit,
        })
    }
}

impl MusicTwoRowItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("musicTwoRowItemRenderer").unwrap_or(val);

        let id = target.pointer("/navigationEndpoint/browseEndpoint/browseId")
            .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/videoId"))
            .or_else(|| target.pointer("/navigationEndpoint/watchPlaylistEndpoint/playlistId"))
            .and_then(|i| i.as_str())
            .map(|s| s.to_string());

        let title = target.get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let subtitle = target.get("subtitle")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let track_count = target
            .pointer("/subtitle/runs")
            .and_then(Value::as_array)
            .filter(|runs| runs.len() == 3)
            .and_then(|runs| runs.get(2))
            .and_then(|run| run.get("text"))
            .and_then(Value::as_str)
            .and_then(parse_music_card_track_count);

        let thumbnails = ThumbnailListNode::from_value(
            target.pointer("/thumbnailRenderer/musicThumbnailRenderer")
                .or_else(|| target.get("thumbnailRenderer"))
                .unwrap_or(target)
        );

        let endpoint = target.get("navigationEndpoint").and_then(NavigationEndpointNode::from_value);

        let item_type = target.pointer("/navigationEndpoint/browseEndpoint/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
            .and_then(|p| p.as_str())
            .map(|s| s.to_string());

        Some(Self {
            id,
            title,
            subtitle,
            track_count,
            thumbnails,
            endpoint,
            item_type,
        })
    }
}

fn parse_music_card_track_count(text: &str) -> Option<u32> {
    let lower = text.to_ascii_lowercase();
    if !lower.contains("track") && !lower.contains("song") {
        return None;
    }
    let digits: String = text.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

impl MusicDescriptionShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("musicDescriptionShelfRenderer").unwrap_or(val);

        let header = target.get("header")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let description = target.get("description")
            .and_then(TextNode::from_value)
            .map(|t| t.text)?;

        let footer = target.get("footer")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        Some(Self {
            header,
            description,
            footer,
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

/// Strongly typed MusicHeader AST node (`musicHeaderRenderer` / `musicVisualHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicHeaderNode {
    pub title: String,
    pub subtitle: Option<String>,
    pub thumbnails: ThumbnailListNode,
    pub buttons: Vec<Value>,
}

impl MusicHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicHeaderRenderer")
            .or_else(|| val.get("musicVisualHeaderRenderer"))
            .unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_else(|| "Music Header".to_string());

        let subtitle = node
            .get("subtitle")
            .or_else(|| node.get("description"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let thumbnails = ThumbnailListNode::from_value(
            node.get("thumbnail")
                .or_else(|| node.pointer("/foregroundThumbnail/musicThumbnailRenderer/thumbnail"))
                .unwrap_or(node),
        );

        let buttons = node
            .get("buttons")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self {
            title,
            subtitle,
            thumbnails,
            buttons,
        })
    }
}

/// Strongly typed MusicInlineBadge AST node (`musicInlineBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MusicInlineBadgeNode {
    pub icon_type: Option<String>,
    pub label: Option<String>,
    pub tooltip: Option<String>,
}

impl MusicInlineBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicInlineBadgeRenderer").unwrap_or(val);
        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);
        let label = node
            .pointer("/accessibilityData/accessibilityData/label")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let tooltip = node.get("tooltip").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            icon_type,
            label,
            tooltip,
        })
    }
}

/// Strongly typed MusicNavigationButton AST node (`musicNavigationButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicNavigationButtonNode {
    pub text: String,
    pub icon_type: Option<String>,
    pub endpoint: Option<Value>,
}

impl MusicNavigationButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicNavigationButtonRenderer").unwrap_or(val);
        let text = node
            .get("buttonText")
            .or_else(|| node.get("text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();
        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);
        let endpoint = node
            .get("clickCommand")
            .or_else(|| node.get("navigationEndpoint"))
            .cloned();

        Some(Self {
            text,
            icon_type,
            endpoint,
        })
    }
}

/// Strongly typed MusicQueue AST node (`musicQueueRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicQueueNode {
    pub content: Option<Value>,
}

impl MusicQueueNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicQueueRenderer").unwrap_or(val);
        let content = node.get("content").cloned();

        Some(Self { content })
    }
}

/// Strongly typed MusicPlayButton AST node (`musicPlayButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicPlayButtonNode {
    pub play_navigation_endpoint: Option<Value>,
    pub icon_type: Option<String>,
}

impl MusicPlayButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("musicPlayButtonRenderer").unwrap_or(val);
        let play_navigation_endpoint = node
            .get("playNavigationEndpoint")
            .or_else(|| node.get("navigationEndpoint"))
            .cloned();
        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            play_navigation_endpoint,
            icon_type,
        })
    }
}


