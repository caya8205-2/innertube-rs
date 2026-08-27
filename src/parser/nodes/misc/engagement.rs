use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentifierNode {
    pub surface: Option<String>,
    pub tag: Option<String>,
}

/// Strongly typed EngagementPanelSectionList AST node (`engagementPanelSectionListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngagementPanelSectionListNode {
    pub header: Option<Value>,
    pub content: Option<Value>,
    pub target_id: Option<String>,
    pub panel_identifier: Option<String>,
    pub identifier: Option<IdentifierNode>,
    pub visibility: Option<String>,
}

impl EngagementPanelSectionListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("engagementPanelSectionListRenderer").unwrap_or(val);
        
        let identifier = node.get("identifier").map(|id_val| IdentifierNode {
                surface: id_val.get("surface").and_then(|v| v.as_str()).map(String::from),
                tag: id_val.get("tag").and_then(|v| v.as_str()).map(String::from),
            });

        Some(Self {
            header: node.get("header").cloned(),
            content: node.get("content").cloned(),
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
            panel_identifier: node.get("panelIdentifier").and_then(|v| v.as_str()).map(String::from),
            identifier,
            visibility: node.get("visibility").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed EngagementPanelTitleHeader AST node (`engagementPanelTitleHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngagementPanelTitleHeaderNode {
    pub title: Option<TextNode>,
    pub visibility_button: Option<Value>,
    pub contextual_info: Option<TextNode>,
    pub menu: Option<Value>,
}

impl EngagementPanelTitleHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("engagementPanelTitleHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            visibility_button: node.get("visibilityButton").cloned(),
            contextual_info: node.get("contextualInfo").and_then(TextNode::from_value),
            menu: node.get("menu").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomEmojiNode {
    pub emoji_id: Option<String>,
    pub shortcuts: Option<Vec<String>>,
    pub search_terms: Option<Vec<String>>,
    pub image: Option<ThumbnailListNode>,
    pub is_custom_emoji: Option<bool>,
}

/// Strongly typed CommentsHeader AST node (`commentsHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsHeaderNode {
    pub title: Option<TextNode>,
    pub count: Option<TextNode>,
    pub comments_count: Option<TextNode>,
    pub create_renderer: Option<Value>,
    pub sort_menu: Option<Value>,
    pub custom_emojis: Option<Vec<CustomEmojiNode>>,
}

impl CommentsHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentsHeaderRenderer").unwrap_or(val);
        
        let custom_emojis = node.get("customEmojis").and_then(|v| v.as_array()).map(|arr| {
            arr.iter().map(|emoji| CustomEmojiNode {
                emoji_id: emoji.get("emojiId").and_then(|v| v.as_str()).map(String::from),
                shortcuts: emoji.get("shortcuts").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|s| s.as_str().map(String::from)).collect()),
                search_terms: emoji.get("searchTerms").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|s| s.as_str().map(String::from)).collect()),
                image: emoji.get("image").map(ThumbnailListNode::from_value),
                is_custom_emoji: emoji.get("isCustomEmoji").and_then(|v| v.as_bool()),
            }).collect()
        });

        Some(Self {
            title: node.get("titleText").and_then(TextNode::from_value),
            count: node.get("countText").and_then(TextNode::from_value),
            comments_count: node.get("commentsCount").and_then(TextNode::from_value),
            create_renderer: node.get("createRenderer").cloned(),
            sort_menu: node.get("sortMenu").cloned(),
            custom_emojis,
        })
    }
}

/// Strongly typed CommentsEntryPointHeader AST node (`commentsEntryPointHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsEntryPointHeaderNode {
    pub header: Option<TextNode>,
    pub comment_count: Option<TextNode>,
    pub teaser_avatar: Option<ThumbnailListNode>,
    pub teaser_content: Option<TextNode>,
    pub content_renderer: Option<Value>,
    pub simplebox_placeholder: Option<TextNode>,
}

impl CommentsEntryPointHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentsEntryPointHeaderRenderer").unwrap_or(val);
        
        let teaser_avatar = node.get("teaserAvatar")
            .or_else(|| node.get("simpleboxAvatar"))
            .map(ThumbnailListNode::from_value);

        Some(Self {
            header: node.get("headerText").and_then(TextNode::from_value),
            comment_count: node.get("commentCount").and_then(TextNode::from_value),
            teaser_avatar,
            teaser_content: node.get("teaserContent").and_then(TextNode::from_value),
            content_renderer: node.get("contentRenderer").cloned(),
            simplebox_placeholder: node.get("simpleboxPlaceholder").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed CommentActionButtons AST node (`commentActionButtonsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentActionButtonsNode {
    pub like_button: Option<Value>,
    pub dislike_button: Option<Value>,
    pub reply_button: Option<Value>,
    pub creator_heart: Option<Value>,
}

impl CommentActionButtonsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentActionButtonsRenderer").unwrap_or(val);
        Some(Self {
            like_button: node.get("likeButton").cloned(),
            dislike_button: node.get("dislikeButton").cloned(),
            reply_button: node.get("replyButton").cloned(),
            creator_heart: node.get("creatorHeart").cloned(),
        })
    }
}

/// Strongly typed CommentSimplebox AST node (`commentSimpleboxRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentSimpleboxNode {
    pub submit_button: Option<Value>,
    pub cancel_button: Option<Value>,
    pub author_thumbnail: Option<ThumbnailListNode>,
    pub placeholder: Option<TextNode>,
    pub avatar_size: Option<String>,
}

impl CommentSimpleboxNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentSimpleboxRenderer").unwrap_or(val);
        Some(Self {
            submit_button: node.get("submitButton").cloned(),
            cancel_button: node.get("cancelButton").cloned(),
            author_thumbnail: node.get("authorThumbnail").map(ThumbnailListNode::from_value),
            placeholder: node.get("placeholderText").and_then(TextNode::from_value),
            avatar_size: node.get("avatarSize").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionNotificationToggleButtonStateNode {
    pub id: Option<String>,
    pub next_id: Option<String>,
    pub state: Option<Value>,
}

/// Strongly typed SubscriptionNotificationToggleButton AST node (`subscriptionNotificationToggleButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionNotificationToggleButtonNode {
    pub states: Option<Vec<SubscriptionNotificationToggleButtonStateNode>>,
    pub current_state_id: Option<String>,
    pub target_id: Option<String>,
}

impl SubscriptionNotificationToggleButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("subscriptionNotificationToggleButtonRenderer").unwrap_or(val);
        
        let states = node.get("states").and_then(|v| v.as_array()).map(|arr| {
            arr.iter().map(|s| SubscriptionNotificationToggleButtonStateNode {
                id: s.get("stateId").and_then(|v| v.as_str()).map(String::from),
                next_id: s.get("nextStateId").and_then(|v| v.as_str()).map(String::from),
                state: s.get("state").cloned(),
            }).collect()
        });

        Some(Self {
            states,
            current_state_id: node.get("currentStateId").and_then(|v| v.as_str()).map(String::from),
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed InfoRow AST node (`infoRowRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRowNode {
    pub title: Option<TextNode>,
    pub default_metadata: Option<TextNode>,
    pub expanded_metadata: Option<TextNode>,
    pub info_row_expand_status_key: Option<String>,
}

impl InfoRowNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("infoRowRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            default_metadata: node.get("defaultMetadata").and_then(TextNode::from_value),
            expanded_metadata: node.get("expandedMetadata").and_then(TextNode::from_value),
            info_row_expand_status_key: node.get("infoRowExpandStatusKey").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed CollageHeroImage AST node (`collageHeroImageRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollageHeroImageNode {
    pub left: Option<ThumbnailListNode>,
    pub top_right: Option<ThumbnailListNode>,
    pub bottom_right: Option<ThumbnailListNode>,
    pub endpoint: Option<Value>,
}

impl CollageHeroImageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("collageHeroImageRenderer").unwrap_or(val);
        Some(Self {
            left: node.get("leftThumbnail").map(ThumbnailListNode::from_value),
            top_right: node.get("topRightThumbnail").map(ThumbnailListNode::from_value),
            bottom_right: node.get("bottomRightThumbnail").map(ThumbnailListNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
        })
    }
}

/// Strongly typed FeedNudge AST node (`feedNudgeRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedNudgeNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub apply_modernized_style: Option<bool>,
    pub trim_style: Option<String>,
    pub background_style: Option<String>,
}

impl FeedNudgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("feedNudgeRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            endpoint: node.get("impressionEndpoint").cloned(),
            apply_modernized_style: node.get("applyModernizedStyle").and_then(|v| v.as_bool()),
            trim_style: node.get("trimStyle").and_then(|v| v.as_str()).map(String::from),
            background_style: node.get("backgroundStyle").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed ChannelOwnerEmptyState AST node (`channelOwnerEmptyStateRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelOwnerEmptyStateNode {
    pub illustration: Option<ThumbnailListNode>,
    pub description: Option<TextNode>,
}

impl ChannelOwnerEmptyStateNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelOwnerEmptyStateRenderer").unwrap_or(val);
        Some(Self {
            illustration: node.get("illustration").map(ThumbnailListNode::from_value),
            description: node.get("description").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed TextHeader AST node (`textHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextHeaderNode {
    pub title: Option<TextNode>,
    pub style: Option<String>,
}

impl TextHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("textHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            style: node.get("style").and_then(|v| v.as_str()).map(String::from),
        })
    }
}
