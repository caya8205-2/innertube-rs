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
    /// Whether this playlist is a YouTube Mix (radio) playlist rather than a
    /// user-created playlist.
    ///
    /// Detection covers all the shapes YouTube uses for Mix cards: an
    /// `RD`-prefixed playlist ID, legacy `radioRenderer` families, and the
    /// `MIX` badge overlay used by modern `lockupViewModel` cards.
    #[serde(default)]
    pub is_mix: bool,
}

/// Returns `true` when `id` names a YouTube radio (Mix) playlist.
///
/// Radio playlists always use `RD`-prefixed IDs (`RDEM...`, `RDCLAK...`,
/// `RDAMVM...`), and `/browse` wraps every playlist ID in `VL` (`VLRD...`),
/// so that prefix is stripped before the check.
pub fn is_mix_playlist_id(id: &str) -> bool {
    id.strip_prefix("VL").unwrap_or(id).starts_with("RD")
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
    /// Set-video ID used by playlist edit mutations (`setVideoId`).
    pub set_video_id: Option<String>,
}

impl PlaylistNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val
            .get("playlistHeaderRenderer")
            .or_else(|| val.get("pageHeaderRenderer"))
            .or_else(|| val.get("playlistRenderer"))
            .or_else(|| val.get("gridPlaylistRenderer"))
            // Legacy radio (Mix) renderers.
            .or_else(|| val.get("radioRenderer"))
            .or_else(|| val.get("compactRadioRenderer"))
            .or_else(|| val.get("gridRadioRenderer"))
            .or_else(|| val.get("lockupViewModel"))
            .or_else(|| val.pointer("/richItemRenderer/content/lockupViewModel"))
            .or_else(|| val.pointer("/header/playlistHeaderRenderer"))
            .or_else(|| val.pointer("/header/pageHeaderRenderer"))
            .unwrap_or(val);

        let id = target.get("playlistId")
            .or_else(|| target.get("id"))
            .or_else(|| target.get("contentId"))
            .or_else(|| target.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/playlistId"))
            .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/playlistId"))
            .and_then(|i| i.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let is_mix = detect_is_mix(val, target, &id);

        let title = target.get("title")
            .or_else(|| target.get("pageTitle"))
            .or_else(|| target.pointer("/metadata/lockupMetadataViewModel/title"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/title"))
            .or_else(|| val.pointer("/sidebar/playlistSidebarRenderer/items/0/playlistSidebarPrimaryInfoRenderer/title"))
            .or_else(|| val.pointer("/microformat/microformatDataRenderer/title"))
            .or_else(|| val.pointer("/metadata/playlistMetadataRenderer/title"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                target.pointer("/metadata/lockupMetadataViewModel/title/content")
                    .or_else(|| target.pointer("/title/content"))
                    .or_else(|| val.pointer("/header/pageHeaderRenderer/pageTitle"))
                    .or_else(|| val.pointer("/microformat/microformatDataRenderer/title"))
                    .or_else(|| val.pointer("/metadata/playlistMetadataRenderer/title"))
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "Untitled Playlist".to_string());

        let mut author = target.get("ownerText")
            .or_else(|| target.get("author"))
            .or_else(|| target.get("shortBylineText"))
            .or_else(|| target.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/avatarStack/avatarStackViewModel/text"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text"))
            .or_else(|| val.pointer("/sidebar/playlistSidebarRenderer/items/1/playlistSidebarSecondaryInfoRenderer/videoOwner/videoOwnerRenderer/title"))
            .and_then(AuthorNode::from_value);

        if let Some(ref mut auth) = author {
            if auth.name.starts_with("by ") {
                auth.name = auth
                    .name
                    .strip_prefix("by ")
                    .unwrap_or(&auth.name)
                    .trim()
                    .to_string();
            } else if auth.name.starts_with("โดย ") {
                auth.name = auth
                    .name
                    .strip_prefix("โดย ")
                    .unwrap_or(&auth.name)
                    .trim()
                    .to_string();
            }

            if auth.id.is_none() {
                auth.id = target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/avatarStack/avatarStackViewModel/rendererContext/commandContext/onTap/innertubeCommand/browseEndpoint/browseId")
                    .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/avatarStack/avatarStackViewModel/onTap/innertubeCommand/browseEndpoint/browseId"))
                    .or_else(|| target.pointer("/ownerEndpoint/browseEndpoint/browseId"))
                    .or_else(|| target.pointer("/shortBylineText/runs/0/navigationEndpoint/browseEndpoint/browseId"))
                    .or_else(|| target.pointer("/ownerText/runs/0/navigationEndpoint/browseEndpoint/browseId"))
                    .or_else(|| target.pointer("/videoOwner/videoOwnerRenderer/navigationEndpoint/browseEndpoint/browseId"))
                    .or_else(|| val.pointer("/sidebar/playlistSidebarRenderer/items/1/playlistSidebarSecondaryInfoRenderer/videoOwner/videoOwnerRenderer/navigationEndpoint/browseEndpoint/browseId"))
                    .and_then(|id| id.as_str())
                    .map(|s| s.to_string());
            }
        }

        let description = target.get("descriptionText")
            .or_else(|| target.get("description"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/description/descriptionViewModel/description"))
            .or_else(|| val.pointer("/sidebar/playlistSidebarRenderer/items/0/playlistSidebarPrimaryInfoRenderer/description"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                val.pointer("/microformat/microformatDataRenderer/description")
                    .and_then(|d| d.as_str())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
            });

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

        if video_count.is_none() {
            if let Some(rows) = target.pointer("/metadata/lockupMetadataViewModel/metadata/contentMetadataViewModel/metadataRows").and_then(|r| r.as_array()) {
                for r in rows {
                    if let Some(parts) = r.get("metadataParts").and_then(|p| p.as_array()) {
                        for p in parts {
                            if let Some(txt) = p.pointer("/text/content").or_else(|| p.pointer("/text/runs/0/text")).and_then(|t| t.as_str()) {
                                if txt.contains("video") || txt.contains("วิดีโอ") {
                                    let digits: String = txt.chars().filter(|c| c.is_ascii_digit()).collect();
                                    if let Ok(num) = digits.parse() {
                                        video_count = Some(num);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(views_str) = target.get("viewCountText")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/2/text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
        {
            view_count = Some(views_str);
        }

        let mut last_updated = target
            .get("lastUpdated")
            .or_else(|| target.get("lastUpdatedText"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        // Sidebar stats fallback for video_count, view_count, and last_updated
        if video_count.is_none() || view_count.is_none() || last_updated.is_none() {
            if let Some(stats) = val.pointer("/sidebar/playlistSidebarRenderer/items/0/playlistSidebarPrimaryInfoRenderer/stats").and_then(|s| s.as_array()) {
                for stat in stats {
                    if let Some(txt_node) = TextNode::from_value(stat) {
                        let txt = txt_node.text.trim();
                        if video_count.is_none() && (txt.contains("video") || txt.contains("วิดีโอ")) {
                            let digits: String = txt.chars().filter(|c| c.is_ascii_digit()).collect();
                            video_count = digits.parse().ok();
                        } else if view_count.is_none() && (txt.contains("view") || txt.contains("การดู")) {
                            view_count = Some(txt.to_string());
                        } else if last_updated.is_none() && (txt.contains("update") || txt.contains("Update") || txt.contains("อัปเดต")) {
                            last_updated = Some(txt.to_string());
                        }
                    }
                }
            }
        }

        let thumbnails = ThumbnailListNode::from_value(
            target.pointer("/playlistHeaderBanner/heroPlaylistThumbnailRenderer/thumbnail")
                .or_else(|| target.pointer("/content/pageHeaderViewModel/heroContent/previewImage/contentPreviewImageViewModel/image"))
                .or_else(|| target.pointer("/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/image"))
                .or_else(|| target.pointer("/contentImage/thumbnailViewModel/image"))
                .or_else(|| val.pointer("/sidebar/playlistSidebarRenderer/items/0/playlistSidebarPrimaryInfoRenderer/thumbnailRenderer/playlistVideoThumbnailRenderer/thumbnail"))
                .or_else(|| val.pointer("/sidebar/playlistSidebarRenderer/items/0/playlistSidebarPrimaryInfoRenderer/thumbnailRenderer/playlistCustomThumbnailRenderer/thumbnail"))
                .or_else(|| val.pointer("/microformat/microformatDataRenderer/thumbnail"))
                .or_else(|| target.get("thumbnail"))
                .unwrap_or(target)
        );

        let endpoint = target.get("navigationEndpoint")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/actions/actionsViewModel/primaryButton/buttonViewModel/onTap/innertubeCommand"))
            .or_else(|| target.pointer("/rendererContext/commandContext/onTap/innertubeCommand"))
            .and_then(NavigationEndpointNode::from_value);

        if id.is_empty() && title == "Untitled Playlist" && author.is_none() {
            return None;
        }

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
            is_mix,
        })
    }
}

/// Detects a YouTube Mix (radio) playlist from its parsed node.
///
/// Checked in order of reliability:
/// 1. `RD`-prefixed playlist ID (the canonical signal).
/// 2. `watchEndpoint.playlistId` of the card's tap action.
/// 3. Legacy `radioRenderer` / `compactRadioRenderer` / `gridRadioRenderer`.
/// 4. The `MIX` badge overlay (`imageName: "MIX"`) of modern lockup cards.
fn detect_is_mix(val: &Value, target: &Value, id: &str) -> bool {
    if is_mix_playlist_id(id) {
        return true;
    }

    let endpoint_playlist_id = target
        .pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/playlistId")
        .or_else(|| target.pointer("/navigationEndpoint/watchEndpoint/playlistId"))
        .and_then(Value::as_str);
    if endpoint_playlist_id.is_some_and(is_mix_playlist_id) {
        return true;
    }

    if has_radio_renderer(target) || has_radio_renderer(val) {
        return true;
    }

    // Badge overlays are only inspected on the node itself (never on a whole
    // page fallback), so an unrelated Mix card elsewhere in the response
    // cannot mark this playlist as a Mix.
    const BADGE_PATHS: [&str; 5] = [
        "/contentImage/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel/overlays",
        "/contentImage/thumbnailViewModel/overlays",
        "/thumbnailViewModel/overlays",
        "/thumbnail/thumbnailViewModel/overlays",
        "/thumbnailOverlays",
    ];

    BADGE_PATHS
        .iter()
        .filter_map(|path| target.pointer(path))
        .any(has_mix_badge)
}

fn has_radio_renderer(value: &Value) -> bool {
    value.get("radioRenderer").is_some()
        || value.get("compactRadioRenderer").is_some()
        || value.get("gridRadioRenderer").is_some()
}

/// Recursively looks for a `MIX` badge inside thumbnail overlays.
///
/// The badge icon is locale-independent (`clientResource.imageName` is
/// `"MIX"`), while the visible text is localized (`"Mix"`, `"มิกซ์"`, ...).
fn has_mix_badge(value: &Value) -> bool {
    match value {
        Value::Object(map) => {
            if map.get("imageName").and_then(Value::as_str) == Some("MIX") {
                return true;
            }
            if map
                .get("text")
                .and_then(Value::as_str)
                .is_some_and(is_mix_badge_text)
            {
                return true;
            }
            map.values().any(has_mix_badge)
        }
        Value::Array(items) => items.iter().any(has_mix_badge),
        _ => false,
    }
}

fn is_mix_badge_text(text: &str) -> bool {
    let text = text.trim();
    text.eq_ignore_ascii_case("mix") || text == "มิกซ์"
}

impl PlaylistVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val
            .get("playlistVideoRenderer")
            .or_else(|| val.get("lockupViewModel"))
            .unwrap_or(val);

        // 1. Check lockupViewModel
        if target.get("contentType").is_some()
            || target
                .pointer("/metadata/lockupMetadataViewModel")
                .is_some()
        {
            let id = target
                .pointer(
                    "/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId",
                )
                .or_else(|| target.get("contentId"))
                .and_then(|v| v.as_str())?
                .to_string();

            let title = target
                .pointer("/metadata/lockupMetadataViewModel/title/content")
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
                set_video_id: None,
            });
        }

        // 2. Standard playlistVideoRenderer
        let id = target
            .get("videoId")
            .and_then(|v| v.as_str())
            .or_else(|| {
                target
                    .pointer("/navigationEndpoint/watchEndpoint/videoId")
                    .and_then(|v| v.as_str())
            })
            .map(|s| s.to_string())?;

        let title = target
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_else(|| "Untitled".to_string());

        let author = target
            .get("shortBylineText")
            .or_else(|| target.get("ownerText"))
            .and_then(AuthorNode::from_value);

        let duration = target
            .get("lengthText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let duration_ms = target
            .get("lengthSeconds")
            .and_then(|s| s.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .map(|sec| sec * 1000)
            .or_else(|| duration.as_deref().and_then(parse_duration_string_to_ms));

        let thumbnails = ThumbnailListNode::from_value(target.get("thumbnail").unwrap_or(target));

        let index = target
            .get("index")
            .and_then(TextNode::from_value)
            .and_then(|t| t.text.parse::<u32>().ok());

        let is_playable = target
            .get("isPlayable")
            .and_then(|p| p.as_bool())
            .unwrap_or(true);

        let set_video_id = target
            .get("setVideoId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            id,
            title,
            author,
            duration,
            duration_ms,
            thumbnails,
            index,
            is_playable,
            set_video_id,
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
    /// Current rating state, inverted from the toggle menu's default action.
    pub like_status: Option<String>,
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

        // The toggle menu's default action is the inverse of the current state:
        // "LIKE" (click to like) means INDIFFERENT now, and vice versa.
        let like_status = target
            .pointer("/menu/menuRenderer/items")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find_map(|item| {
                let action = item
                    .pointer(
                        "/toggleMenuServiceItemRenderer/defaultServiceEndpoint/likeEndpoint/status",
                    )
                    .and_then(Value::as_str)?;
                match action {
                    "LIKE" => Some("INDIFFERENT".to_string()),
                    "INDIFFERENT" => Some("LIKE".to_string()),
                    _ => None,
                }
            });

        Some(Self {
            id,
            title,
            author,
            duration,
            selected,
            like_status,
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
        let title = node
            .get("title")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let description = node
            .get("description")
            .and_then(Value::as_str)
            .map(ToString::to_string);
        let privacy = node
            .get("privacy")
            .and_then(Value::as_str)
            .map(ToString::to_string);

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
        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let mut stats = Vec::new();
        if let Some(arr) = node.get("stats").and_then(Value::as_array) {
            for item in arr {
                if let Some(txt) = TextNode::from_value(item) {
                    stats.push(txt.text);
                }
            }
        }

        let thumbnails =
            ThumbnailListNode::from_value(node.get("thumbnailRenderer").unwrap_or(node));

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
        let node = val
            .get("playlistSidebarSecondaryInfoRenderer")
            .unwrap_or(val);
        let owner = node.get("videoOwner").cloned();
        let button = node.get("button").cloned();

        Some(Self { owner, button })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_is_mix_playlist_id() {
        assert!(is_mix_playlist_id("RDEM5KXlGSfIinpcCL9eVl4XVw"));
        assert!(is_mix_playlist_id("VLRDEM5KXlGSfIinpcCL9eVl4XVw"));
        assert!(is_mix_playlist_id("RDCLAK5uy_kExample"));
        assert!(is_mix_playlist_id("RDAMVM8XXPxLLgwt8"));
        assert!(!is_mix_playlist_id("PL8XXPxLLgwt8"));
        assert!(!is_mix_playlist_id("VLPL8XXPxLLgwt8"));
        assert!(!is_mix_playlist_id(""));
    }

    #[test]
    fn test_lockup_mix_card_uses_mix_badge() {
        // Shape returned by modern Mix cards: the badge is the only signal
        // when the node itself does not expose an RD-prefixed playlist ID.
        let val = json!({
            "lockupViewModel": {
                "contentId": "not-a-radio-id",
                "contentType": "LOCKUP_CONTENT_TYPE_PLAYLIST",
                "contentImage": {
                    "collectionThumbnailViewModel": {
                        "primaryThumbnail": {
                            "thumbnailViewModel": {
                                "overlays": [{
                                    "thumbnailOverlayBadgeViewModel": {
                                        "thumbnailBadges": [{
                                            "thumbnailBadgeViewModel": {
                                                "icon": {
                                                    "sources": [{ "clientResource": { "imageName": "MIX" } }]
                                                },
                                                "text": "มิกซ์"
                                            }
                                        }]
                                    }
                                }]
                            }
                        }
                    }
                },
                "metadata": {
                    "lockupMetadataViewModel": {
                        "title": { "content": "Mix - Test Artist" }
                    }
                }
            }
        });

        let node = PlaylistNode::from_value(&val).expect("lockup mix card should parse");
        assert!(node.is_mix);
        assert_eq!(node.id, "not-a-radio-id");
        assert_eq!(node.title, "Mix - Test Artist");
    }

    #[test]
    fn test_lockup_mix_card_uses_localized_badge_text() {
        let val = json!({
            "lockupViewModel": {
                "contentId": "not-a-radio-id",
                "contentType": "LOCKUP_CONTENT_TYPE_PLAYLIST",
                "contentImage": {
                    "thumbnailViewModel": {
                        "overlays": [{
                            "thumbnailOverlayBadgeViewModel": {
                                "thumbnailBadges": [{
                                    "thumbnailBadgeViewModel": { "text": "มิกซ์" }
                                }]
                            }
                        }]
                    }
                },
                "metadata": {
                    "lockupMetadataViewModel": { "title": { "content": "Mix - Test" } }
                }
            }
        });

        let node = PlaylistNode::from_value(&val).expect("playlist lockup should parse");
        assert!(node.is_mix);
    }

    #[test]
    fn test_legacy_radio_renderer_is_mix() {
        let val = json!({
            "radioRenderer": {
                "playlistId": "RDAMVM8XXPxLLgwt8",
                "title": { "runs": [{ "text": "Radio Mix" }] }
            }
        });

        let node = PlaylistNode::from_value(&val).expect("radio renderer should parse");
        assert!(node.id.starts_with("RD"));
        assert!(node.is_mix);
    }

    #[test]
    fn test_regular_playlist_is_not_mix() {
        let val = json!({
            "lockupViewModel": {
                "contentId": "PL8XXPxLLgwt8",
                "contentType": "LOCKUP_CONTENT_TYPE_PLAYLIST",
                "metadata": {
                    "lockupMetadataViewModel": {
                        "title": { "content": "My Playlist" }
                    }
                }
            }
        });

        let node = PlaylistNode::from_value(&val).expect("playlist lockup should parse");
        assert!(!node.is_mix);
    }

    #[test]
    fn test_mix_card_in_page_body_does_not_mark_header_as_mix() {
        let val = json!({
            "header": {
                "playlistHeaderRenderer": {
                    "playlistId": "PL8XXPxLLgwt8",
                    "title": { "runs": [{ "text": "My Playlist" }] }
                }
            },
            "contents": {
                "richItemRenderer": {
                    "content": {
                        "lockupViewModel": {
                            "contentId": "RDEM5KXlGSfIinpcCL9eVl4XVw",
                            "contentType": "LOCKUP_CONTENT_TYPE_PLAYLIST",
                            "contentImage": {
                                "thumbnailViewModel": {
                                    "overlays": [{
                                        "thumbnailOverlayBadgeViewModel": {
                                            "thumbnailBadges": [{
                                                "thumbnailBadgeViewModel": { "text": "Mix" }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }
                }
            }
        });

        let node = PlaylistNode::from_value(&val).expect("header should parse");
        assert_eq!(node.id, "PL8XXPxLLgwt8");
        assert!(!node.is_mix);
    }
}
