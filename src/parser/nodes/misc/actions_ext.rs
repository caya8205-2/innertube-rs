use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed WatchNextTabbedResults AST node (`watchNextTabbedResults`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextTabbedResultsNode {
    pub tabs: Option<Vec<Value>>,
    pub secondary_results: Option<Value>,
}

impl WatchNextTabbedResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("watchNextTabbedResults").unwrap_or(val);
        Some(Self {
            tabs: node.get("tabs").and_then(|v| v.as_array()).cloned(),
            secondary_results: node.get("secondaryResults").cloned(),
        })
    }
}

/// Strongly typed YpcTrailer AST node (`ypcTrailerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YpcTrailerNode {
    pub video_message: Option<String>,
    pub player_response: Option<Value>,
}

impl YpcTrailerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("ypcTrailerRenderer").unwrap_or(val);
        Some(Self {
            video_message: node.get("fullVideoMessage").and_then(|v| v.as_str()).map(String::from),
            player_response: node.get("unserializedPlayerResponse").cloned(),
        })
    }
}

/// Strongly typed CommandExecutorCommand AST node (`commandExecutorCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandExecutorCommandNode {
    pub commands: Option<Vec<Value>>,
}

impl CommandExecutorCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commandExecutorCommand").unwrap_or(val);
        Some(Self {
            commands: node.get("commands").and_then(|v| v.as_array()).cloned(),
        })
    }
}

/// Strongly typed GetKidsBlocklistPickerCommand AST node (`getKidsBlocklistPickerCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKidsBlocklistPickerCommandNode {
    pub blocked_for_kids_content: Option<String>,
}

impl GetKidsBlocklistPickerCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("getKidsBlocklistPickerCommand").unwrap_or(val);
        Some(Self {
            blocked_for_kids_content: node.get("blockedForKidsContent").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed ShowDialogCommand AST node (`showDialogCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowDialogCommandNode {
    pub inline_content: Option<Value>,
    pub remove_default_padding: bool,
}

impl ShowDialogCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showDialogCommand").unwrap_or(val);
        Some(Self {
            inline_content: node.get("panelLoadingStrategy").and_then(|v| v.get("inlineContent")).cloned(),
            remove_default_padding: node.get("removeDefaultPadding").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed CommentDialog AST node (`commentDialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDialogNode {
    pub editable_text: Option<TextNode>,
    pub author_thumbnail: ThumbnailListNode,
    pub submit_button: Option<Value>,
    pub cancel_button: Option<Value>,
    pub placeholder: Option<TextNode>,
    pub emoji_button: Option<Value>,
    pub emoji_picker: Option<Value>,
}

impl CommentDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentDialogRenderer").unwrap_or(val);
        Some(Self {
            editable_text: node.get("editableText").and_then(TextNode::from_value),
            author_thumbnail: node.get("authorThumbnail").map(ThumbnailListNode::from_value).unwrap_or_default(),
            submit_button: node.get("submitButton").cloned(),
            cancel_button: node.get("cancelButton").cloned(),
            placeholder: node.get("placeholderText").and_then(TextNode::from_value),
            emoji_button: node.get("emojiButton").cloned(),
            emoji_picker: node.get("emojiPicker").cloned(),
        })
    }
}

/// Strongly typed CommentReplyDialog AST node (`commentReplyDialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentReplyDialogNode {
    pub reply_button: Option<Value>,
    pub cancel_button: Option<Value>,
    pub author_thumbnail: ThumbnailListNode,
    pub placeholder: Option<TextNode>,
    pub error_message: Option<TextNode>,
}

impl CommentReplyDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentReplyDialogRenderer").unwrap_or(val);
        Some(Self {
            reply_button: node.get("replyButton").cloned(),
            cancel_button: node.get("cancelButton").cloned(),
            author_thumbnail: node.get("authorThumbnail").map(ThumbnailListNode::from_value).unwrap_or_default(),
            placeholder: node.get("placeholderText").and_then(TextNode::from_value),
            error_message: node.get("errorMessage").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed EmojiPicker AST node (`emojiPickerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiPickerNode {
    pub id: Option<String>,
    pub categories: Option<Vec<Value>>,
    pub category_buttons: Option<Vec<Value>>,
    pub search_placeholder: Option<TextNode>,
    pub search_no_results: Option<TextNode>,
    pub pick_skin_tone: Option<TextNode>,
    pub clear_search_label: Option<String>,
    pub skin_tone_generic_label: Option<String>,
    pub skin_tone_light_label: Option<String>,
    pub skin_tone_medium_light_label: Option<String>,
    pub skin_tone_medium_label: Option<String>,
    pub skin_tone_medium_dark_label: Option<String>,
    pub skin_tone_dark_label: Option<String>,
}

impl EmojiPickerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("emojiPickerRenderer").unwrap_or(val);
        Some(Self {
            id: node.get("id").and_then(|v| v.as_str()).map(String::from),
            categories: node.get("categories").and_then(|v| v.as_array()).cloned(),
            category_buttons: node.get("categoryButtons").and_then(|v| v.as_array()).cloned(),
            search_placeholder: node.get("searchPlaceholderText").and_then(TextNode::from_value),
            search_no_results: node.get("searchNoResultsText").and_then(TextNode::from_value),
            pick_skin_tone: node.get("pickSkinToneText").and_then(TextNode::from_value),
            clear_search_label: node.get("clearSearchLabel").and_then(|v| v.as_str()).map(String::from),
            skin_tone_generic_label: node.get("skinToneGenericLabel").and_then(|v| v.as_str()).map(String::from),
            skin_tone_light_label: node.get("skinToneLightLabel").and_then(|v| v.as_str()).map(String::from),
            skin_tone_medium_light_label: node.get("skinToneMediumLightLabel").and_then(|v| v.as_str()).map(String::from),
            skin_tone_medium_label: node.get("skinToneMediumLabel").and_then(|v| v.as_str()).map(String::from),
            skin_tone_medium_dark_label: node.get("skinToneMediumDarkLabel").and_then(|v| v.as_str()).map(String::from),
            skin_tone_dark_label: node.get("skinToneDarkLabel").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed VoiceReplyContainerView AST node (`voiceReplyContainerView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoiceReplyContainerViewNode {
    pub voice_reply_unavailable_text: Option<TextNode>,
    pub transcript_text: Option<TextNode>,
}

impl VoiceReplyContainerViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("voiceReplyContainerView").unwrap_or(val);
        Some(Self {
            voice_reply_unavailable_text: node.get("voiceReplyUnavailableText").and_then(TextNode::from_value),
            transcript_text: node.get("transcriptText").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed AddToPlaylistEndpoint AST node (`addToPlaylistEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylistEndpointNode {
    pub video_id: Option<String>,
    pub video_ids: Option<Vec<String>>,
    pub playlist_id: Option<String>,
    pub params: Option<String>,
    pub exclude_watch_later: bool,
}

impl AddToPlaylistEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("addToPlaylistEndpoint").unwrap_or(val);
        Some(Self {
            video_id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            video_ids: node.get("videoIds").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()),
            playlist_id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            params: node.get("params").and_then(|v| v.as_str()).map(String::from),
            exclude_watch_later: node.get("excludeWatchLater").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed AddToPlaylistServiceEndpoint AST node (`addToPlaylistServiceEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylistServiceEndpointNode {
    pub video_id: Option<String>,
    pub video_ids: Option<Vec<String>>,
    pub playlist_id: Option<String>,
    pub params: Option<String>,
    pub exclude_watch_later: bool,
}

impl AddToPlaylistServiceEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("addToPlaylistServiceEndpoint").unwrap_or(val);
        Some(Self {
            video_id: node.get("videoId").and_then(|v| v.as_str()).map(String::from),
            video_ids: node.get("videoIds").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()),
            playlist_id: node.get("playlistId").and_then(|v| v.as_str()).map(String::from),
            params: node.get("params").and_then(|v| v.as_str()).map(String::from),
            exclude_watch_later: node.get("excludeWatchLater").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed CreateCommentEndpoint AST node (`createCommentEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentEndpointNode {
    pub create_comment_params: Option<String>,
    pub comment_text: Option<String>,
    pub attached_video_id: Option<String>,
    pub poll_options: Option<Vec<String>>,
    pub image_blob_id: Option<String>,
    pub shared_post_id: Option<String>,
    pub access_restrictions: Option<u64>,
    pub botguard_response: Option<String>,
}

impl CreateCommentEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("createCommentEndpoint").unwrap_or(val);
        Some(Self {
            create_comment_params: node.get("createCommentParams").and_then(|v| v.as_str()).map(String::from),
            comment_text: node.get("commentText").and_then(|v| v.as_str()).map(String::from),
            attached_video_id: node.get("attachedVideoId").and_then(|v| v.as_str()).map(String::from),
            poll_options: node.get("pollOptions").and_then(|v| v.as_array()).map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()),
            image_blob_id: node.get("imageBlobId").and_then(|v| v.as_str()).map(String::from),
            shared_post_id: node.get("sharedPostId").and_then(|v| v.as_str()).map(String::from),
            access_restrictions: node.get("accessRestrictions").and_then(|v| v.as_u64()),
            botguard_response: node.get("botguardResponse").and_then(|v| v.as_str()).map(String::from),
        })
    }
}
