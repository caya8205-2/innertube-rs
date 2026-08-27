use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed `PlaylistAddToOption` AST node (`playlistAddToOptionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistAddToOptionNode {
    pub title: Option<TextNode>,
    pub contains_selected_videos: Option<String>,
    pub playlist_id: Option<String>,
    pub privacy: Option<String>,
}

impl PlaylistAddToOptionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistAddToOptionRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            contains_selected_videos: node.get("containsSelectedVideos").and_then(|v| v.as_str().map(String::from)),
            playlist_id: node.get("addToPlaylistId").and_then(|v| v.as_str().map(String::from)),
            privacy: node.get("privacy").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed `PlaylistCollaborationView` AST node (`playlistCollaborationViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollaborationViewNode {
    pub playlist_collaborators: Option<Vec<Value>>,
    pub turn_off_collaboration_dialog: Option<Value>,
    pub copy_link_button: Option<Value>,
    pub collaborate_playlist_collaboration_setting: Option<Value>,
    pub playlist_collaboration_entity_key: Option<String>,
    pub playlist_collaborators_data: Option<Vec<Value>>,
    pub leave_collaborative_playlist_confirmation_dialog: Option<Value>,
    pub collaboration_type: Option<String>,
    pub allow_new_collaborators_playlist_collaboration_setting: Option<Value>,
    pub playlist_collaboration_form_schema: Option<Value>,
    pub turn_off_allow_new_collaborators_dialog: Option<Value>,
    pub invite_collaborators_button: Option<Value>,
}

impl PlaylistCollaborationViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistCollaborationViewRenderer").unwrap_or(val);
        Some(Self {
            playlist_collaborators: node.get("playlistCollaborators").and_then(|v| v.as_array().cloned()),
            turn_off_collaboration_dialog: node.get("turnOffCollaborationDialog").cloned(),
            copy_link_button: node.get("copyLinkButton").cloned(),
            collaborate_playlist_collaboration_setting: node.get("collaboratePlaylistCollaborationSetting").cloned(),
            playlist_collaboration_entity_key: node.get("playlistCollaborationEntityKey").and_then(|v| v.as_str().map(String::from)),
            playlist_collaborators_data: node.get("playlistCollaboratorsData").and_then(|v| v.as_array().cloned()),
            leave_collaborative_playlist_confirmation_dialog: node.get("leaveCollaborativePlaylistConfirmationDialog").cloned(),
            collaboration_type: node.get("collaborationType").and_then(|v| v.as_str().map(String::from)),
            allow_new_collaborators_playlist_collaboration_setting: node.get("allowNewCollaboratorsPlaylistCollaborationSetting").cloned(),
            playlist_collaboration_form_schema: node.get("playlistCollaborationFormSchema").cloned(),
            turn_off_allow_new_collaborators_dialog: node.get("turnOffAllowNewCollaboratorsDialog").cloned(),
            invite_collaborators_button: node.get("inviteCollaboratorsButton").cloned(),
        })
    }
}

/// Strongly typed `PlaylistCustomThumbnail` AST node (`playlistCustomThumbnailRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCustomThumbnailNode {
    pub thumbnail: Option<ThumbnailListNode>,
}

impl PlaylistCustomThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistCustomThumbnailRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value),
        })
    }
}

/// Strongly typed `PlaylistHeader` AST node (`playlistHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistHeaderNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub stats: Option<Vec<TextNode>>,
    pub brief_stats: Option<Vec<TextNode>>,
    pub author: Option<Value>,
    pub description: Option<TextNode>,
    pub num_videos: Option<TextNode>,
    pub view_count: Option<TextNode>,
    pub can_share: bool,
    pub can_delete: bool,
    pub is_editable: bool,
    pub privacy: Option<String>,
    pub save_button: Option<Value>,
    pub shuffle_play_button: Option<Value>,
    pub menu: Option<Value>,
    pub banner: Option<Value>,
}

impl PlaylistHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistHeaderRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str().map(String::from)),
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            stats: node.get("stats").and_then(|v| {
                v.as_array().map(|arr| arr.iter().filter_map(TextNode::from_value).collect())
            }),
            brief_stats: node.get("briefStats").and_then(|v| {
                v.as_array().map(|arr| arr.iter().filter_map(TextNode::from_value).collect())
            }),
            author: node.get("ownerText").or(node.get("ownerEndpoint")).cloned(),
            description: node.get("descriptionText").and_then(TextNode::from_value),
            num_videos: node.get("numVideosText").and_then(TextNode::from_value),
            view_count: node.get("viewCountText").and_then(TextNode::from_value),
            can_share: node.get("shareData").and_then(|v| v.get("canShare")).and_then(|v| v.as_bool()).unwrap_or(false),
            can_delete: node.get("editableDetails").and_then(|v| v.get("canDelete")).and_then(|v| v.as_bool()).unwrap_or(false),
            is_editable: node.get("isEditable").and_then(|v| v.as_bool()).unwrap_or(false),
            privacy: node.get("privacy").and_then(|v| v.as_str().map(String::from)),
            save_button: node.get("saveButton").cloned(),
            shuffle_play_button: node.get("shufflePlayButton").cloned(),
            menu: node.get("moreActionsMenu").cloned(),
            banner: node.get("playlistHeaderBanner").cloned(),
        })
    }
}

/// Strongly typed `PlaylistInfoCardContent` AST node (`playlistInfoCardContentRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfoCardContentNode {
    pub title: Option<TextNode>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub video_count: Option<TextNode>,
    pub channel_name: Option<TextNode>,
    pub endpoint: Option<Value>,
}

impl PlaylistInfoCardContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistInfoCardContentRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("playlistTitle").and_then(TextNode::from_value),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            video_count: node.get("playlistVideoCount").and_then(TextNode::from_value),
            channel_name: node.get("channelName").and_then(TextNode::from_value),
            endpoint: node.get("action").cloned(),
        })
    }
}

/// Strongly typed `PlaylistPanelVideoWrapper` AST node (`playlistPanelVideoWrapperRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistPanelVideoWrapperNode {
    pub primary: Option<Value>,
    pub counterpart: Option<Vec<Value>>,
}

impl PlaylistPanelVideoWrapperNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistPanelVideoWrapperRenderer").unwrap_or(val);
        Some(Self {
            primary: node.get("primaryRenderer").cloned(),
            counterpart: node.get("counterpart").and_then(|v| v.as_array().cloned()),
        })
    }
}

/// Strongly typed `PlaylistSidebar` AST node (`playlistSidebarRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSidebarNode {
    pub items: Option<Vec<Value>>,
}

impl PlaylistSidebarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistSidebarRenderer").unwrap_or(val);
        Some(Self {
            items: node.get("items").and_then(|v| v.as_array().cloned()),
        })
    }
}

/// Strongly typed `PlaylistThumbnailOverlay` AST node (`playlistThumbnailOverlayRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistThumbnailOverlayNode {
    pub icon_type: Option<String>,
    pub text: Option<TextNode>,
}

impl PlaylistThumbnailOverlayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistThumbnailOverlayRenderer").unwrap_or(val);
        Some(Self {
            icon_type: node.get("icon").and_then(|v| v.get("iconType")).and_then(|v| v.as_str().map(String::from)),
            text: node.get("text").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `PlaylistVideoList` AST node (`playlistVideoListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoListNode {
    pub id: Option<String>,
    pub is_editable: bool,
    pub can_reorder: bool,
    pub videos: Option<Vec<Value>>,
}

impl PlaylistVideoListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistVideoListRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("playlistId").and_then(|v| v.as_str().map(String::from)),
            is_editable: node.get("isEditable").and_then(|v| v.as_bool()).unwrap_or(false),
            can_reorder: node.get("canReorder").and_then(|v| v.as_bool()).unwrap_or(false),
            videos: node.get("contents").and_then(|v| v.as_array().cloned()),
        })
    }
}

/// Strongly typed `PlaylistVideoThumbnail` AST node (`playlistVideoThumbnailRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoThumbnailNode {
    pub thumbnail: Option<ThumbnailListNode>,
}

impl PlaylistVideoThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("playlistVideoThumbnailRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value),
        })
    }
}

/// Strongly typed `PremiereTrailerBadge` AST node (`premiereTrailerBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiereTrailerBadgeNode {
    pub label: Option<TextNode>,
}

impl PremiereTrailerBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("premiereTrailerBadgeRenderer").unwrap_or(val);
        Some(Self {
            label: node.get("label").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `ProductList` AST node (`productListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductListNode {
    pub contents: Option<Vec<Value>>,
}

impl ProductListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("productListRenderer").unwrap_or(val);
        Some(Self {
            contents: node.get("contents").and_then(|v| v.as_array().cloned()),
        })
    }
}
