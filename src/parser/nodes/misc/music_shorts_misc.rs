use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed MusicDownloadStateBadge AST node (`musicDownloadStateBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicDownloadStateBadgeNode {
    pub playlist_id: Option<String>,
    pub supported_download_states: Vec<String>,
}

impl MusicDownloadStateBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicDownloadStateBadgeRenderer")
            .or_else(|| val.get("musicDownloadStateBadge"))
            .unwrap_or(val);
        Some(Self {
            playlist_id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            supported_download_states: node
                .get("supportedDownloadStates")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MusicElementHeader AST node (`musicElementHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicElementHeaderNode {
    pub element: Option<Value>,
}

impl MusicElementHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicElementHeaderRenderer")
            .or_else(|| val.get("musicElementHeader"))
            .unwrap_or(val);
        Some(Self {
            element: node.get("elementRenderer").or_else(|| node.get("element")).cloned(),
        })
    }
}

/// Strongly typed MusicSortFilterButton AST node (`musicSortFilterButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicSortFilterButtonNode {
    pub title: Option<TextNode>,
    pub icon_type: Option<String>,
    pub menu: Option<Value>,
}

impl MusicSortFilterButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicSortFilterButtonRenderer")
            .or_else(|| val.get("musicSortFilterButton"))
            .unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            icon_type: node
                .get("icon")
                .and_then(|v| v.get("iconType"))
                .and_then(|v| v.as_str())
                .map(String::from),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed MusicThumbnail AST node (`musicThumbnailRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicThumbnailNode {
    pub contents: ThumbnailListNode,
}

impl MusicThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicThumbnailRenderer")
            .or_else(|| val.get("musicThumbnail"))
            .unwrap_or(val);
        Some(Self {
            contents: node
                .get("thumbnail")
                .map(ThumbnailListNode::from_value)
                .unwrap_or_else(|| ThumbnailListNode::from_value(node)),
        })
    }
}

/// Strongly typed MusicMenuItemDivider AST node (`musicMenuItemDividerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicMenuItemDividerNode {}

impl MusicMenuItemDividerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let _node = val
            .get("musicMenuItemDividerRenderer")
            .or_else(|| val.get("musicMenuItemDivider"))
            .unwrap_or(val);
        Some(Self {})
    }
}

/// Strongly typed MusicMultiSelectMenu AST node (`musicMultiSelectMenuRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicMultiSelectMenuNode {
    pub title: Option<TextNode>,
    pub options: Vec<Value>,
}

impl MusicMultiSelectMenuNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicMultiSelectMenuRenderer")
            .or_else(|| val.get("musicMultiSelectMenu"))
            .unwrap_or(val);
        let title = node
            .pointer("/title/musicMenuTitleRenderer/primaryText")
            .or_else(|| node.get("title"))
            .and_then(TextNode::from_value);
        let options = node
            .get("options")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        Some(Self { title, options })
    }
}

/// Strongly typed MusicMultiSelectMenuItem AST node (`musicMultiSelectMenuItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicMultiSelectMenuItemNode {
    pub title: Option<TextNode>,
    pub form_item_entity_key: Option<String>,
    pub selected_icon_type: Option<String>,
    pub endpoint: Option<Value>,
    pub selected: bool,
}

impl MusicMultiSelectMenuItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("musicMultiSelectMenuItemRenderer")
            .or_else(|| val.get("musicMultiSelectMenuItem"))
            .unwrap_or(val);
        let endpoint = node.get("selectedCommand").cloned();
        let selected = endpoint.is_some()
            || node
                .get("selected")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            form_item_entity_key: node
                .get("formItemEntityKey")
                .and_then(|v| v.as_str())
                .map(String::from),
            selected_icon_type: node
                .pointer("/selectedIcon/iconType")
                .and_then(|v| v.as_str())
                .or_else(|| node.get("selectedIcon").and_then(|v| v.as_str()))
                .map(String::from),
            endpoint,
            selected,
        })
    }
}

/// Strongly typed BackstagePost AST node (`backstagePostRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstagePostNode {
    pub id: Option<String>,
    pub author_text: Option<TextNode>,
    pub author_thumbnail: Option<ThumbnailListNode>,
    pub author_endpoint: Option<Value>,
    pub content: Option<TextNode>,
    pub published: Option<TextNode>,
    pub poll_status: Option<String>,
    pub vote_status: Option<String>,
    pub vote_count: Option<TextNode>,
    pub menu: Option<Value>,
    pub action_buttons: Option<Value>,
    pub vote_button: Option<Value>,
    pub surface: Option<String>,
    pub endpoint: Option<Value>,
    pub attachment: Option<Value>,
}

impl BackstagePostNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("backstagePostRenderer")
            .or_else(|| val.get("backstagePost"))
            .unwrap_or(val);
        Some(Self {
            id: node.get("postId").and_then(|v| v.as_str()).map(String::from),
            author_text: node.get("authorText").and_then(TextNode::from_value),
            author_thumbnail: node.get("authorThumbnail").map(ThumbnailListNode::from_value),
            author_endpoint: node.get("authorEndpoint").cloned(),
            content: node.get("contentText").and_then(TextNode::from_value),
            published: node.get("publishedTimeText").and_then(TextNode::from_value),
            poll_status: node.get("pollStatus").and_then(|v| v.as_str()).map(String::from),
            vote_status: node.get("voteStatus").and_then(|v| v.as_str()).map(String::from),
            vote_count: node.get("voteCount").and_then(TextNode::from_value),
            menu: node.get("actionMenu").cloned(),
            action_buttons: node.get("actionButtons").cloned(),
            vote_button: node.get("voteButton").cloned(),
            surface: node.get("surface").and_then(|v| v.as_str()).map(String::from),
            endpoint: node.get("navigationEndpoint").cloned(),
            attachment: node.get("backstageAttachment").cloned(),
        })
    }
}

/// Strongly typed BackstagePostThread AST node (`backstagePostThreadRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackstagePostThreadNode {
    pub post: Option<Value>,
}

impl BackstagePostThreadNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("backstagePostThreadRenderer")
            .or_else(|| val.get("backstagePostThread"))
            .unwrap_or(val);
        Some(Self {
            post: node.get("post").cloned(),
        })
    }
}

/// Strongly typed SharedPost AST node (`sharedPostRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPostNode {
    pub id: Option<String>,
    pub thumbnail: ThumbnailListNode,
    pub content: Option<TextNode>,
    pub published: Option<TextNode>,
    pub menu: Option<Value>,
    pub original_post: Option<Value>,
    pub endpoint: Option<Value>,
    pub expand_button: Option<Value>,
    pub author_display_name: Option<TextNode>,
}

impl SharedPostNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("sharedPostRenderer")
            .or_else(|| val.get("sharedPost"))
            .unwrap_or(val);
        Some(Self {
            id: node.get("postId").and_then(|v| v.as_str()).map(String::from),
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            content: node.get("content").and_then(TextNode::from_value),
            published: node.get("publishedTimeText").and_then(TextNode::from_value),
            menu: node.get("actionMenu").cloned(),
            original_post: node.get("originalPost").cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            expand_button: node.get("expandButton").cloned(),
            author_display_name: node.get("displayName").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed ReelItem AST node (`reelItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelItemNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub thumbnails: ThumbnailListNode,
    pub views: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub accessibility_label: Option<String>,
}

impl ReelItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("reelItemRenderer")
            .or_else(|| val.get("reelItem"))
            .unwrap_or(val);
        Some(Self {
            id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            title: node.get("headline").and_then(TextNode::from_value),
            thumbnails: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            views: node.get("viewCountText").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            accessibility_label: node
                .pointer("/accessibility/accessibilityData/label")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed ReelPlayerHeader AST node (`reelPlayerHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelPlayerHeaderNode {
    pub reel_title_text: Option<TextNode>,
    pub timestamp_text: Option<TextNode>,
    pub channel_title_text: Option<TextNode>,
    pub channel_thumbnail: ThumbnailListNode,
    pub channel_navigation_endpoint: Option<Value>,
}

impl ReelPlayerHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("reelPlayerHeaderRenderer")
            .or_else(|| val.get("reelPlayerHeader"))
            .unwrap_or(val);
        Some(Self {
            reel_title_text: node.get("reelTitleText").and_then(TextNode::from_value),
            timestamp_text: node.get("timestampText").and_then(TextNode::from_value),
            channel_title_text: node.get("channelTitleText").and_then(TextNode::from_value),
            channel_thumbnail: ThumbnailListNode::from_value(
                node.get("channelThumbnail").unwrap_or(&Value::Null),
            ),
            channel_navigation_endpoint: node.get("channelNavigationEndpoint").cloned(),
        })
    }
}

/// Strongly typed ReelPlayerOverlay AST node (`reelPlayerOverlayRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReelPlayerOverlayNode {
    pub like_button: Option<Value>,
    pub reel_player_header_supported_renderers: Option<Value>,
    pub menu: Option<Value>,
    pub next_item_button: Option<Value>,
    pub prev_item_button: Option<Value>,
    pub subscribe_button_renderer: Option<Value>,
    pub style: Option<String>,
    pub view_comments_button: Option<Value>,
    pub share_button: Option<Value>,
    pub pivot_button: Option<Value>,
    pub info_panel: Option<Value>,
}

impl ReelPlayerOverlayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("reelPlayerOverlayRenderer")
            .or_else(|| val.get("reelPlayerOverlay"))
            .unwrap_or(val);
        Some(Self {
            like_button: node.get("likeButton").cloned(),
            reel_player_header_supported_renderers: node
                .get("reelPlayerHeaderSupportedRenderers")
                .cloned(),
            menu: node.get("menu").cloned(),
            next_item_button: node.get("nextItemButton").cloned(),
            prev_item_button: node.get("prevItemButton").cloned(),
            subscribe_button_renderer: node.get("subscribeButtonRenderer").cloned(),
            style: node.get("style").and_then(|v| v.as_str()).map(String::from),
            view_comments_button: node.get("viewCommentsButton").cloned(),
            share_button: node.get("shareButton").cloned(),
            pivot_button: node.get("pivotButton").cloned(),
            info_panel: node.get("infoPanel").cloned(),
        })
    }
}

/// Strongly typed ShortsLockupView AST node (`shortsLockupViewModel` / `shortsLockupView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortsLockupViewNode {
    pub entity_id: Option<String>,
    pub accessibility_text: Option<String>,
    pub thumbnail: ThumbnailListNode,
    pub on_tap_endpoint: Option<Value>,
    pub menu_on_tap: Option<Value>,
    pub index_in_collection: Option<u64>,
    pub menu_on_tap_a11y_label: Option<String>,
    pub overlay_primary_text: Option<TextNode>,
    pub overlay_secondary_text: Option<TextNode>,
    pub inline_player_data: Option<Value>,
    pub badge: Option<Value>,
}

impl ShortsLockupViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("shortsLockupViewModel")
            .or_else(|| val.get("shortsLockupView"))
            .unwrap_or(val);
        Some(Self {
            entity_id: node.get("entityId").and_then(|v| v.as_str()).map(String::from),
            accessibility_text: node
                .get("accessibilityText")
                .and_then(|v| v.as_str())
                .map(String::from),
            thumbnail: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            on_tap_endpoint: node.get("onTap").cloned(),
            menu_on_tap: node.get("menuOnTap").cloned(),
            index_in_collection: node.get("indexInCollection").and_then(|v| v.as_u64()),
            menu_on_tap_a11y_label: node
                .get("menuOnTapA11yLabel")
                .and_then(|v| v.as_str())
                .map(String::from),
            overlay_primary_text: node
                .pointer("/overlayMetadata/primaryText")
                .and_then(TextNode::from_value),
            overlay_secondary_text: node
                .pointer("/overlayMetadata/secondaryText")
                .and_then(TextNode::from_value),
            inline_player_data: node.pointer("/inlinePlayerData/onVisible").cloned(),
            badge: node.get("badge").cloned(),
        })
    }
}

/// Strongly typed AlertWithButton AST node (`alertWithButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertWithButtonNode {
    pub text: Option<TextNode>,
    pub alert_type: Option<String>,
    pub dismiss_button: Option<Value>,
}

impl AlertWithButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("alertWithButtonRenderer")
            .or_else(|| val.get("alertWithButton"))
            .unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            alert_type: node.get("type").and_then(|v| v.as_str()).map(String::from),
            dismiss_button: node.get("dismissButton").cloned(),
        })
    }
}

/// Strongly typed CompositeVideoPrimaryInfo AST node (`compositeVideoPrimaryInfoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeVideoPrimaryInfoNode {}

impl CompositeVideoPrimaryInfoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let _node = val
            .get("compositeVideoPrimaryInfoRenderer")
            .or_else(|| val.get("compositeVideoPrimaryInfo"))
            .unwrap_or(val);
        Some(Self {})
    }
}

/// Strongly typed EmergencyOnebox AST node (`emergencyOneboxRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyOneboxNode {
    pub title: Option<TextNode>,
    pub first_option: Option<Value>,
    pub menu: Option<Value>,
}

impl EmergencyOneboxNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("emergencyOneboxRenderer")
            .or_else(|| val.get("emergencyOnebox"))
            .unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            first_option: node.get("firstOption").cloned(),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed SingleActionEmergencySupport AST node (`singleActionEmergencySupportRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleActionEmergencySupportNode {
    pub action_text: Option<TextNode>,
    pub nav_text: Option<TextNode>,
    pub details: Option<TextNode>,
    pub icon_type: Option<String>,
    pub endpoint: Option<Value>,
}

impl SingleActionEmergencySupportNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("singleActionEmergencySupportRenderer")
            .or_else(|| val.get("singleActionEmergencySupport"))
            .unwrap_or(val);
        Some(Self {
            action_text: node.get("actionText").and_then(TextNode::from_value),
            nav_text: node.get("navigationText").and_then(TextNode::from_value),
            details: node.get("detailsText").and_then(TextNode::from_value),
            icon_type: node
                .pointer("/icon/iconType")
                .and_then(|v| v.as_str())
                .or_else(|| node.get("icon").and_then(|v| v.as_str()))
                .map(String::from),
            endpoint: node.get("navigationEndpoint").cloned(),
        })
    }
}

/// Strongly typed PlayerLiveStoryboardSpec AST node (`playerLiveStoryboardSpecRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLiveStoryboardSpecNode {
    pub spec: Option<String>,
    pub template_url: Option<String>,
    pub thumbnail_width: Option<u32>,
    pub thumbnail_height: Option<u32>,
    pub columns: Option<u32>,
    pub rows: Option<u32>,
}

impl PlayerLiveStoryboardSpecNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerLiveStoryboardSpecRenderer")
            .or_else(|| val.get("playerLiveStoryboardSpec"))
            .unwrap_or(val);
        let spec = node.get("spec").and_then(|v| v.as_str()).map(String::from);
        let mut template_url = None;
        let mut thumbnail_width = None;
        let mut thumbnail_height = None;
        let mut columns = None;
        let mut rows = None;

        if let Some(ref s) = spec {
            let parts: Vec<&str> = s.split('#').collect();
            if parts.len() >= 5 {
                template_url = Some(parts[0].to_string());
                thumbnail_width = parts[1].parse::<u32>().ok();
                thumbnail_height = parts[2].parse::<u32>().ok();
                columns = parts[3].parse::<u32>().ok();
                rows = parts[4].parse::<u32>().ok();
            }
        }

        Some(Self {
            spec,
            template_url,
            thumbnail_width,
            thumbnail_height,
            columns,
            rows,
        })
    }
}

/// Strongly typed PollHeader AST node (`pollHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollHeaderNode {
    pub poll_question: Option<TextNode>,
    pub thumbnails: ThumbnailListNode,
    pub metadata: Option<TextNode>,
    pub live_chat_poll_type: Option<String>,
    pub context_menu_button: Option<Value>,
}

impl PollHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("pollHeaderRenderer")
            .or_else(|| val.get("pollHeader"))
            .unwrap_or(val);
        Some(Self {
            poll_question: node.get("pollQuestion").and_then(TextNode::from_value),
            thumbnails: ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(&Value::Null)),
            metadata: node.get("metadataText").and_then(TextNode::from_value),
            live_chat_poll_type: node
                .get("liveChatPollType")
                .and_then(|v| v.as_str())
                .map(String::from),
            context_menu_button: node.get("contextMenuButton").cloned(),
        })
    }
}

/// Strongly typed ChangeEngagementPanelVisibilityAction AST node (`changeEngagementPanelVisibilityAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEngagementPanelVisibilityActionNode {
    pub target_id: Option<String>,
    pub visibility: Option<String>,
}

impl ChangeEngagementPanelVisibilityActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("changeEngagementPanelVisibilityAction")
            .unwrap_or(val);
        Some(Self {
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
            visibility: node.get("visibility").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed ShowEngagementPanelEndpoint AST node (`showEngagementPanelEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowEngagementPanelEndpointNode {
    pub panel_identifier: Option<String>,
    pub source_panel_identifier: Option<String>,
}

impl ShowEngagementPanelEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showEngagementPanelEndpoint").unwrap_or(val);
        Some(Self {
            panel_identifier: node
                .get("panelIdentifier")
                .or_else(|| node.pointer("/identifier/tag"))
                .and_then(|v| v.as_str())
                .map(String::from),
            source_panel_identifier: node
                .get("sourcePanelIdentifier")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed CreatorHeartView AST node (`creatorHeartViewModel` / `creatorHeartView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorHeartViewNode {
    pub creator_thumbnail: ThumbnailListNode,
    pub hearted_icon_name: Option<String>,
    pub unhearted_icon_name: Option<String>,
    pub unhearted_icon_color: Option<i64>,
    pub hearted_hover_text: Option<String>,
    pub hearted_accessibility_label: Option<String>,
    pub unhearted_accessibility_label: Option<String>,
    pub engagement_state_key: Option<String>,
}

impl CreatorHeartViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("creatorHeartViewModel")
            .or_else(|| val.get("creatorHeartViewRenderer"))
            .or_else(|| val.get("creatorHeartView"))
            .unwrap_or(val);
        Some(Self {
            creator_thumbnail: ThumbnailListNode::from_value(
                node.get("creatorThumbnail").unwrap_or(&Value::Null),
            ),
            hearted_icon_name: node
                .pointer("/heartedIcon/sources/0/clientResource/imageName")
                .or_else(|| node.pointer("/heartedIcon/iconType"))
                .and_then(|v| v.as_str())
                .map(String::from),
            unhearted_icon_name: node
                .pointer("/unheartedIcon/sources/0/clientResource/imageName")
                .or_else(|| node.pointer("/unheartedIcon/iconType"))
                .and_then(|v| v.as_str())
                .map(String::from),
            unhearted_icon_color: node
                .pointer("/unheartedIcon/processor/borderImageProcessor/imageTint/color")
                .and_then(|v| v.as_i64()),
            hearted_hover_text: node
                .get("heartedHoverText")
                .and_then(|v| v.as_str())
                .map(String::from),
            hearted_accessibility_label: node
                .get("heartedAccessibilityLabel")
                .and_then(|v| v.as_str())
                .map(String::from),
            unhearted_accessibility_label: node
                .get("unheartedAccessibilityLabel")
                .and_then(|v| v.as_str())
                .map(String::from),
            engagement_state_key: node
                .get("engagementStateKey")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Strongly typed KidsCategoryTab AST node (`kidsCategoryTabRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsCategoryTabNode {
    pub title: Option<TextNode>,
    pub category_asset_key: Option<String>,
    pub category_background_color: Option<String>,
    pub category_type: Option<String>,
    pub endpoint: Option<Value>,
}

impl KidsCategoryTabNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("kidsCategoryTabRenderer")
            .or_else(|| val.get("kidsCategoryTab"))
            .unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            category_asset_key: node
                .pointer("/categoryAssets/assetKey")
                .and_then(|v| v.as_str())
                .map(String::from),
            category_background_color: node
                .pointer("/categoryAssets/backgroundColor")
                .and_then(|v| v.as_str())
                .map(String::from),
            category_type: node
                .get("categoryType")
                .and_then(|v| v.as_str())
                .map(String::from),
            endpoint: node.get("endpoint").cloned(),
        })
    }
}

/// Strongly typed AutomixPreviewVideo AST node (`automixPreviewVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomixPreviewVideoNode {
    pub endpoint: Option<Value>,
}

impl AutomixPreviewVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("automixPreviewVideoRenderer")
            .or_else(|| val.get("automixPreviewVideo"))
            .unwrap_or(val);
        let endpoint = node
            .pointer("/content/automixPlaylistVideoRenderer/navigationEndpoint")
            .or_else(|| node.get("navigationEndpoint"))
            .cloned();
        Some(Self { endpoint })
    }
}
