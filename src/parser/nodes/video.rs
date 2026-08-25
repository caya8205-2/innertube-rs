use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::author::AuthorNode;
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// Represents a video item across search, browse, playlists, and recommendations.
/// (1:1 port consolidating `Video.ts`, `CompactVideo.ts`, `GridVideo.ts`, `LockupView.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VideoNode {
    pub id: String,
    pub title: String,
    pub description_snippet: Option<String>,
    pub author: Option<AuthorNode>,
    pub duration: Option<String>,
    pub duration_ms: Option<u64>,
    pub view_count: Option<String>,
    pub published_time: Option<String>,
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
    pub is_live: bool,
    pub is_upcoming: bool,
    pub badges: Vec<String>,
}

impl VideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // Unwrap container wrapper if present
        let target = val.get("videoRenderer")
            .or_else(|| val.get("compactVideoRenderer"))
            .or_else(|| val.get("gridVideoRenderer"))
            .or_else(|| val.get("lockupViewModel"))
            .or_else(|| val.pointer("/richItemRenderer/content/videoRenderer"))
            .or_else(|| val.pointer("/richItemRenderer/content/lockupViewModel"))
            .unwrap_or(val);

        // 1. Check lockupViewModel (modern ViewModel)
        if target.get("contentType").is_some() || target.pointer("/metadata/lockupMetadataViewModel").is_some() {
            return parse_lockup_view_model(target);
        }

        // 2. Standard videoRenderer / compactVideoRenderer / gridVideoRenderer
        let id = target.get("videoId")
            .and_then(|v| v.as_str())
            .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/videoId").and_then(|v| v.as_str()))
            .map(|s| s.to_string())?;

        let title = TextNode::from_value(target.get("title").unwrap_or(&Value::Null))
            .map(|t| t.text)
            .unwrap_or_else(|| "Untitled".to_string());

        let description_snippet = target.get("descriptionSnippet")
            .or_else(|| target.get("detailedMetadataSnippets"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let author = target.get("ownerText")
            .or_else(|| target.get("shortBylineText"))
            .or_else(|| target.get("longBylineText"))
            .and_then(AuthorNode::from_value);

        let duration = target.pointer("/lengthText/simpleText")
            .or_else(|| target.pointer("/lengthText/runs/0/text"))
            .and_then(|d| d.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                // Check thumbnail overlays for duration
                target.get("thumbnailOverlays").and_then(|o| o.as_array()).and_then(|arr| {
                    arr.iter().find_map(|item| {
                        item.pointer("/thumbnailOverlayTimeStatusRenderer/text/simpleText")
                            .or_else(|| item.pointer("/thumbnailOverlayTimeStatusRenderer/text/runs/0/text"))
                            .and_then(|t| t.as_str())
                            .map(|s| s.to_string())
                    })
                })
            });

        let duration_ms = target.get("lengthSeconds")
            .and_then(|s| s.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .map(|sec| sec * 1000)
            .or_else(|| duration.as_deref().and_then(parse_duration_string_to_ms));

        let view_count = target.pointer("/viewCountText/simpleText")
            .or_else(|| target.pointer("/viewCountText/runs/0/text"))
            .or_else(|| target.pointer("/shortViewCountText/simpleText"))
            .or_else(|| target.pointer("/shortViewCountText/runs/0/text"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let published_time = target.pointer("/publishedTimeText/simpleText")
            .or_else(|| target.pointer("/publishedTimeText/runs/0/text"))
            .and_then(|p| p.as_str())
            .map(|s| s.to_string());

        let thumbnails = ThumbnailListNode::from_value(target.get("thumbnail").unwrap_or(target));
        let endpoint = target.get("navigationEndpoint").and_then(NavigationEndpointNode::from_value);

        let mut badges = Vec::new();
        if let Some(badge_arr) = target.get("badges").and_then(|b| b.as_array()) {
            for b in badge_arr {
                if let Some(label) = b.pointer("/metadataBadgeRenderer/label").and_then(|l| l.as_str()) {
                    badges.push(label.to_string());
                }
            }
        }

        let is_live = badges.iter().any(|b| b.eq_ignore_ascii_case("LIVE") || b.eq_ignore_ascii_case("LIVE NOW"))
            || target.pointer("/thumbnailOverlays/0/thumbnailOverlayTimeStatusRenderer/style").and_then(|s| s.as_str()) == Some("LIVE");

        let is_upcoming = target.get("upcomingEventData").is_some()
            || badges.iter().any(|b| b.eq_ignore_ascii_case("PREMIERE") || b.eq_ignore_ascii_case("UPCOMING"));

        Some(Self {
            id,
            title,
            description_snippet,
            author,
            duration,
            duration_ms,
            view_count,
            published_time,
            thumbnails,
            endpoint,
            is_live,
            is_upcoming,
            badges,
        })
    }
}

fn parse_lockup_view_model(lvm: &Value) -> Option<VideoNode> {
    let id = lvm.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId")
        .or_else(|| lvm.get("contentId"))
        .and_then(|v| v.as_str())?
        .to_string();

    let title = lvm.pointer("/metadata/lockupMetadataViewModel/title/content")
        .or_else(|| lvm.pointer("/title/content"))
        .and_then(|t| t.as_str())
        .unwrap_or("Untitled")
        .to_string();

    let meta_rows = lvm.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows")
        .and_then(|r| r.as_array());

    let mut author = None;
    let mut view_count = None;
    let mut published_time = None;

    if let Some(rows) = meta_rows {
        // Row 0 usually author
        if let Some(row0) = rows.first() {
            if let Some(part0) = row0.pointer("/metadataParts/0/text") {
                author = AuthorNode::from_value(part0);
            }
        }
        // Row 1 usually views & published time
        if let Some(row1) = rows.get(1) {
            if let Some(parts) = row1.get("metadataParts").and_then(|p| p.as_array()) {
                if let Some(p0) = parts.first().and_then(|p| p.pointer("/text/content")).and_then(|t| t.as_str()) {
                    view_count = Some(p0.to_string());
                }
                if let Some(p1) = parts.get(1).and_then(|p| p.pointer("/text/content")).and_then(|t| t.as_str()) {
                    published_time = Some(p1.to_string());
                }
            }
        }
    }

    let duration = lvm.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/overlays/0/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text")
        .or_else(|| lvm.pointer("/contentImage/thumbnailViewModel/overlays/0/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text"))
        .and_then(|d| d.as_str())
        .map(|s| s.to_string());

    let duration_ms = duration.as_deref().and_then(parse_duration_string_to_ms);

    let thumbnails = ThumbnailListNode::from_value(
        lvm.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/image")
            .or_else(|| lvm.pointer("/contentImage/thumbnailViewModel/image"))
            .unwrap_or(lvm)
    );

    let endpoint = lvm.pointer("/rendererContext/commandContext/onTap").and_then(NavigationEndpointNode::from_value);

    Some(VideoNode {
        id,
        title,
        description_snippet: None,
        author,
        duration,
        duration_ms,
        view_count,
        published_time,
        thumbnails,
        endpoint,
        is_live: false,
        is_upcoming: false,
        badges: Vec::new(),
    })
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

/// Primary video metadata (`VideoPrimaryInfo.ts` / `videoPrimaryInfoRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VideoPrimaryInfoNode {
    pub title: String,
    pub view_count: Option<String>,
    pub published_date: Option<String>,
    pub relative_date: Option<String>,
}

impl VideoPrimaryInfoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("videoPrimaryInfoRenderer").unwrap_or(val);
        if target.get("title").is_none() && target.get("viewCount").is_none() {
            return None;
        }

        let title = target
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let view_count = target
            .pointer("/viewCount/videoViewCountRenderer/viewCount")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                target
                    .get("viewCount")
                    .and_then(TextNode::from_value)
                    .map(|t| t.text)
            });

        let published_date = target
            .get("dateText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let relative_date = target
            .get("relativeDateText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        Some(Self {
            title,
            view_count,
            published_date,
            relative_date,
        })
    }
}

/// Secondary video metadata and channel owner (`VideoSecondaryInfo.ts` / `videoSecondaryInfoRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct VideoSecondaryInfoNode {
    pub owner_name: Option<String>,
    pub owner_channel_id: Option<String>,
    pub subscriber_count: Option<String>,
    pub description: Option<String>,
}

impl VideoSecondaryInfoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("videoSecondaryInfoRenderer").unwrap_or(val);
        if target.get("owner").is_none() && target.get("description").is_none() {
            return None;
        }

        let owner_name = target
            .pointer("/owner/videoOwnerRenderer/title")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let owner_channel_id = target
            .pointer("/owner/videoOwnerRenderer/navigationEndpoint/browseEndpoint/browseId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let subscriber_count = target
            .pointer("/owner/videoOwnerRenderer/subscriberCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let description = target
            .get("description")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                target
                    .get("attributedDescription")
                    .and_then(|v| v.get("content"))
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            });

        Some(Self {
            owner_name,
            owner_channel_id,
            subscriber_count,
            description,
        })
    }
}
