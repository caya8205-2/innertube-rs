use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// Represents a channel header or profile overview (1:1 port of `C4TabbedHeader.ts` and `PageHeader.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChannelHeaderNode {
    pub id: String,
    pub title: String,
    pub handle: Option<String>,
    pub subscriber_count: Option<String>,
    pub video_count: Option<String>,
    pub description: Option<String>,
    pub avatar: ThumbnailListNode,
    pub banner: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
    pub badges: Vec<String>,
}

/// Represents a channel card item in search or recommendations (`Channel.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChannelCardNode {
    pub id: String,
    pub title: String,
    pub handle: Option<String>,
    pub subscriber_count: Option<String>,
    pub video_count: Option<String>,
    pub description_snippet: Option<String>,
    pub avatar: ThumbnailListNode,
    pub endpoint: Option<NavigationEndpointNode>,
}

impl ChannelHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("c4TabbedHeaderRenderer")
            .or_else(|| val.get("pageHeaderRenderer"))
            .or_else(|| val.pointer("/header/c4TabbedHeaderRenderer"))
            .or_else(|| val.pointer("/header/pageHeaderRenderer"))
            .unwrap_or(val);

        let id = target.get("channelId")
            .or_else(|| target.pointer("/navigationEndpoint/browseEndpoint/browseId"))
            .and_then(|i| i.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let title = target.get("title")
            .or_else(|| target.get("pageTitle"))
            .or_else(|| target.pointer("/content/pageHeaderViewModel/title"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_else(|| "Unknown Channel".to_string());

        let handle = target.get("channelHandleText")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/0/metadataParts/0/text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let subscriber_count = target.get("subscriberCountText")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/0/text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let video_count = target.get("videosCountText")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/metadata/contentMetadataViewModel/metadataRows/1/metadataParts/1/text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let description = target.get("description")
            .or_else(|| target.pointer("/content/pageHeaderViewModel/description/descriptionViewModel/description"))
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let avatar = ThumbnailListNode::from_value(
            target.get("avatar")
                .or_else(|| target.pointer("/content/pageHeaderViewModel/image/decoratedAvatarViewModel/avatar/avatarViewModel/image"))
                .unwrap_or(target)
        );

        let banner = ThumbnailListNode::from_value(
            target.get("banner")
                .or_else(|| target.pointer("/content/pageHeaderViewModel/banner/bannerViewModel/image"))
                .unwrap_or(target)
        );

        let endpoint = target.get("navigationEndpoint").and_then(NavigationEndpointNode::from_value);

        let mut badges = Vec::new();
        if let Some(badge_arr) = target.get("badges").and_then(|b| b.as_array()) {
            for b in badge_arr {
                if let Some(label) = b.pointer("/metadataBadgeRenderer/label").and_then(|l| l.as_str()) {
                    badges.push(label.to_string());
                }
            }
        }

        Some(Self {
            id,
            title,
            handle,
            subscriber_count,
            video_count,
            description,
            avatar,
            banner,
            endpoint,
            badges,
        })
    }
}

impl ChannelCardNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("channelRenderer")
            .or_else(|| val.get("gridChannelRenderer"))
            .unwrap_or(val);

        let id = target.get("channelId")
            .or_else(|| target.pointer("/navigationEndpoint/browseEndpoint/browseId"))
            .and_then(|i| i.as_str())?
            .to_string();

        let title = target.get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_else(|| "Unknown Channel".to_string());

        let handle = target.get("subscriberCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let video_count = target.get("videoCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let description_snippet = target.get("descriptionSnippet")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let avatar = ThumbnailListNode::from_value(target.get("thumbnail").unwrap_or(target));
        let endpoint = target.get("navigationEndpoint").and_then(NavigationEndpointNode::from_value);

        Some(Self {
            id,
            title,
            handle,
            subscriber_count: None,
            video_count,
            description_snippet,
            avatar,
            endpoint,
        })
    }
}

/// Strongly typed ChannelAboutFullMetadata AST node (`channelAboutFullMetadataRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChannelAboutFullMetadataNode {
    pub description: Option<String>,
    pub view_count: Option<String>,
    pub joined_date: Option<String>,
    pub country: Option<String>,
    pub canonical_channel_url: Option<String>,
}

impl ChannelAboutFullMetadataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelAboutFullMetadataRenderer").unwrap_or(val);
        let description = node
            .get("description")
            .and_then(TextNode::from_value)
            .map(|t| t.text);
        let view_count = node
            .get("viewCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);
        let joined_date = node
            .get("joinedDateText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);
        let country = node
            .get("country")
            .and_then(TextNode::from_value)
            .map(|t| t.text);
        let canonical_channel_url = node
            .get("canonicalChannelUrl")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            description,
            view_count,
            joined_date,
            country,
            canonical_channel_url,
        })
    }
}

/// Strongly typed ChannelMetadata AST node (`channelMetadataRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChannelMetadataNode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub external_id: Option<String>,
    pub vanity_channel_url: Option<String>,
    pub avatar: ThumbnailListNode,
}

impl ChannelMetadataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelMetadataRenderer").unwrap_or(val);
        let title = node.get("title").and_then(Value::as_str).map(ToString::to_string);
        let description = node.get("description").and_then(Value::as_str).map(ToString::to_string);
        let external_id = node.get("externalId").and_then(Value::as_str).map(ToString::to_string);
        let vanity_channel_url = node.get("vanityChannelUrl").and_then(Value::as_str).map(ToString::to_string);
        let avatar = ThumbnailListNode::from_value(node.get("avatar").unwrap_or(node));

        Some(Self {
            title,
            description,
            external_id,
            vanity_channel_url,
            avatar,
        })
    }
}

