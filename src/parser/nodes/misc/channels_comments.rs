use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed AboutChannel AST node (`aboutChannelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutChannelNode {
    pub metadata: Option<AboutChannelViewNode>,
    pub share_channel: Option<Value>,
}

impl AboutChannelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("aboutChannelRenderer").or_else(|| val.get("aboutChannel")).unwrap_or(val);
        let metadata = node.get("metadata").and_then(AboutChannelViewNode::from_value);
        let share_channel = node.get("shareChannel").cloned();

        Some(Self {
            metadata,
            share_channel,
        })
    }
}

/// Strongly typed AboutChannelView AST node (`aboutChannelViewModel`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutChannelViewNode {
    pub description: Option<String>,
    pub description_label: Option<String>,
    pub country: Option<String>,
    pub custom_links_label: Option<String>,
    pub subscriber_count: Option<String>,
    pub view_count: Option<String>,
    pub joined_date: Option<String>,
    pub canonical_channel_url: Option<String>,
    pub channel_id: Option<String>,
    pub additional_info_label: Option<String>,
    pub custom_url_on_tap: Option<Value>,
    pub video_count: Option<String>,
    pub sign_in_for_business_email: Option<String>,
    pub links: Vec<ChannelExternalLinkViewNode>,
}

impl AboutChannelViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("aboutChannelViewModel").or_else(|| val.get("aboutChannelView")).unwrap_or(val);

        let description = node
            .get("description")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| node.get("description").and_then(TextNode::from_value).map(|t| t.text));

        let description_label = node
            .get("descriptionLabel")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let country = node
            .get("country")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let custom_links_label = node
            .get("customLinksLabel")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let subscriber_count = node
            .get("subscriberCountText")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| node.get("subscriberCountText").and_then(TextNode::from_value).map(|t| t.text));

        let view_count = node
            .get("viewCountText")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| node.get("viewCountText").and_then(TextNode::from_value).map(|t| t.text));

        let joined_date = node
            .get("joinedDateText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let canonical_channel_url = node
            .get("canonicalChannelUrl")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let channel_id = node
            .get("channelId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let additional_info_label = node
            .get("additionalInfoLabel")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let custom_url_on_tap = node
            .get("customUrlOnTap")
            .or_else(|| node.get("navigationEndpoint"))
            .cloned();

        let video_count = node
            .get("videoCountText")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| node.get("videoCountText").and_then(TextNode::from_value).map(|t| t.text));

        let sign_in_for_business_email = node
            .get("signInForBusinessEmail")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let links = node
            .get("links")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(ChannelExternalLinkViewNode::from_value).collect())
            .unwrap_or_default();

        Some(Self {
            description,
            description_label,
            country,
            custom_links_label,
            subscriber_count,
            view_count,
            joined_date,
            canonical_channel_url,
            channel_id,
            additional_info_label,
            custom_url_on_tap,
            video_count,
            sign_in_for_business_email,
            links,
        })
    }
}

/// Strongly typed AccountChannel AST node (`accountChannelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountChannelNode {
    pub title: String,
    pub endpoint: Option<Value>,
}

impl AccountChannelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accountChannelRenderer").or_else(|| val.get("accountChannel")).unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let endpoint = node
            .get("navigationEndpoint")
            .or_else(|| node.get("endpoint"))
            .cloned();

        Some(Self { title, endpoint })
    }
}

/// Strongly typed Channel AST node (`channelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelNode {
    pub id: Option<String>,
    pub title: String,
    pub thumbnail: ThumbnailListNode,
    pub subscriber_count: Option<String>,
    pub video_count: Option<String>,
    pub long_byline: Option<String>,
    pub short_byline: Option<String>,
    pub endpoint: Option<Value>,
    pub subscribe_button: Option<Value>,
    pub description_snippet: Option<String>,
    pub owner_badges: Vec<Value>,
}

impl ChannelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelRenderer").or_else(|| val.get("channel")).unwrap_or(val);

        let id = node
            .get("channelId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let thumbnail = ThumbnailListNode::from_value(
            node.get("thumbnail")
                .or_else(|| node.get("thumbnails"))
                .unwrap_or(node),
        );

        let subscriber_count = node
            .get("subscriberCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("subscriberCountText").and_then(Value::as_str).map(ToString::to_string));

        let video_count = node
            .get("videoCountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("videoCountText").and_then(Value::as_str).map(ToString::to_string));

        let long_byline = node
            .get("longBylineText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("longBylineText").and_then(Value::as_str).map(ToString::to_string));

        let short_byline = node
            .get("shortBylineText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("shortBylineText").and_then(Value::as_str).map(ToString::to_string));

        let endpoint = node
            .get("navigationEndpoint")
            .or_else(|| node.get("endpoint"))
            .cloned();

        let subscribe_button = node.get("subscribeButton").cloned();

        let description_snippet = node
            .get("descriptionSnippet")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("descriptionSnippet").and_then(Value::as_str).map(ToString::to_string));

        let owner_badges = node
            .get("ownerBadges")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self {
            id,
            title,
            thumbnail,
            subscriber_count,
            video_count,
            long_byline,
            short_byline,
            endpoint,
            subscribe_button,
            description_snippet,
            owner_badges,
        })
    }
}

/// Strongly typed ChannelAgeGate AST node (`channelAgeGateRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelAgeGateNode {
    pub channel_title: String,
    pub avatar: ThumbnailListNode,
    pub header: String,
    pub main_text: String,
    pub sign_in_button: Option<Value>,
    pub secondary_text: Option<String>,
}

impl ChannelAgeGateNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelAgeGateRenderer").or_else(|| val.get("channelAgeGate")).unwrap_or(val);

        let channel_title = node
            .get("channelTitle")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        let avatar = ThumbnailListNode::from_value(node.get("avatar").unwrap_or(node));

        let header = node
            .get("header")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("header").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let main_text = node
            .get("mainText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("mainText").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let sign_in_button = node.get("signInButton").cloned();

        let secondary_text = node
            .get("secondaryText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("secondaryText").and_then(Value::as_str).map(ToString::to_string));

        Some(Self {
            channel_title,
            avatar,
            header,
            main_text,
            sign_in_button,
            secondary_text,
        })
    }
}

/// Strongly typed ChannelExternalLinkView AST node (`channelExternalLinkViewModel`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelExternalLinkViewNode {
    pub title: String,
    pub link: String,
    pub favicon: ThumbnailListNode,
}

impl ChannelExternalLinkViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelExternalLinkViewModel").or_else(|| val.get("channelExternalLinkView")).unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let link = node
            .get("link")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("link").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let favicon = ThumbnailListNode::from_value(node.get("favicon").unwrap_or(node));

        Some(Self {
            title,
            link,
            favicon,
        })
    }
}

/// Strongly typed ChannelFeaturedContent AST node (`channelFeaturedContentRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelFeaturedContentNode {
    pub title: String,
    pub items: Vec<Value>,
}

impl ChannelFeaturedContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelFeaturedContentRenderer").or_else(|| val.get("channelFeaturedContent")).unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let items = node
            .get("items")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self { title, items })
    }
}

/// Strongly typed ChannelOptions AST node (`channelOptionsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelOptionsNode {
    pub avatar: ThumbnailListNode,
    pub endpoint: Option<Value>,
    pub name: String,
    pub links: Vec<String>,
}

impl ChannelOptionsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelOptionsRenderer").or_else(|| val.get("channelOptions")).unwrap_or(val);

        let avatar = ThumbnailListNode::from_value(node.get("avatar").unwrap_or(node));
        let endpoint = node
            .get("avatarEndpoint")
            .or_else(|| node.get("navigationEndpoint"))
            .or_else(|| node.get("endpoint"))
            .cloned();

        let name = node
            .get("name")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        let links = node
            .get("links")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|link| {
                        TextNode::from_value(link)
                            .map(|t| t.text)
                            .or_else(|| link.as_str().map(ToString::to_string))
                    })
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            avatar,
            endpoint,
            name,
            links,
        })
    }
}

/// Strongly typed ChannelTagline AST node (`channelTaglineRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTaglineNode {
    pub content: String,
    pub max_lines: Option<u64>,
    pub more_endpoint: Option<Value>,
    pub more_icon_type: Option<String>,
    pub more_label: Option<String>,
    pub target_id: Option<String>,
}

impl ChannelTaglineNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelTaglineRenderer").or_else(|| val.get("channelTagline")).unwrap_or(val);

        let content = node
            .get("content")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| node.get("content").and_then(TextNode::from_value).map(|t| t.text))
            .unwrap_or_default();

        let max_lines = node.get("maxLines").and_then(Value::as_u64);
        let more_endpoint = node.get("moreEndpoint").cloned();
        let more_icon_type = node.pointer("/moreIcon/iconType").and_then(Value::as_str).map(ToString::to_string);
        let more_label = node.get("moreLabel").and_then(Value::as_str).map(ToString::to_string);
        let target_id = node.get("targetId").and_then(Value::as_str).map(ToString::to_string);

        Some(Self {
            content,
            max_lines,
            more_endpoint,
            more_icon_type,
            more_label,
            target_id,
        })
    }
}

/// Strongly typed ChannelThumbnailWithLink AST node (`channelThumbnailWithLinkRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnailWithLinkNode {
    pub thumbnails: ThumbnailListNode,
    pub endpoint: Option<Value>,
    pub label: Option<String>,
}

impl ChannelThumbnailWithLinkNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelThumbnailWithLinkRenderer").or_else(|| val.get("channelThumbnailWithLink")).unwrap_or(val);

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(node));
        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();
        let label = node
            .pointer("/accessibility/accessibilityData/label")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            thumbnails,
            endpoint,
            label,
        })
    }
}

/// Strongly typed TopicChannelDetails AST node (`topicChannelDetailsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicChannelDetailsNode {
    pub title: String,
    pub avatar: ThumbnailListNode,
    pub subtitle: Option<String>,
    pub subscribe_button: Option<Value>,
    pub endpoint: Option<Value>,
}

impl TopicChannelDetailsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("topicChannelDetailsRenderer").or_else(|| val.get("topicChannelDetails")).unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let avatar = ThumbnailListNode::from_value(
            node.get("thumbnail")
                .or_else(|| node.get("avatar"))
                .unwrap_or(node),
        );

        let subtitle = node
            .get("subtitle")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("subtitle").and_then(Value::as_str).map(ToString::to_string));

        let subscribe_button = node.get("subscribeButton").cloned();
        let endpoint = node.get("navigationEndpoint").or_else(|| node.get("endpoint")).cloned();

        Some(Self {
            title,
            avatar,
            subtitle,
            subscribe_button,
            endpoint,
        })
    }
}

/// Strongly typed ActiveAccountHeader AST node (`activeAccountHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAccountHeaderNode {
    pub account_name: String,
    pub account_photo: ThumbnailListNode,
    pub endpoint: Option<Value>,
    pub manage_account_title: Option<String>,
    pub channel_handle: Option<String>,
}

impl ActiveAccountHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("activeAccountHeaderRenderer").or_else(|| val.get("activeAccountHeader")).unwrap_or(val);

        let account_name = node
            .get("accountName")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("accountName").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let account_photo = ThumbnailListNode::from_value(node.get("accountPhoto").unwrap_or(node));
        let endpoint = node
            .get("serviceEndpoint")
            .or_else(|| node.get("navigationEndpoint"))
            .or_else(|| node.get("endpoint"))
            .cloned();

        let manage_account_title = node
            .get("manageAccountTitle")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("manageAccountTitle").and_then(Value::as_str).map(ToString::to_string));

        let channel_handle = node
            .get("channelHandle")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("channelHandle").and_then(Value::as_str).map(ToString::to_string));

        Some(Self {
            account_name,
            account_photo,
            endpoint,
            manage_account_title,
            channel_handle,
        })
    }
}

/// Strongly typed HeaderLink AST subnode for ChannelHeaderLinks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderLinkNode {
    pub endpoint: Option<Value>,
    pub icon: ThumbnailListNode,
    pub title: String,
}

impl HeaderLinkNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let endpoint = val.get("navigationEndpoint").or_else(|| val.get("endpoint")).cloned();
        let icon = ThumbnailListNode::from_value(val.get("icon").unwrap_or(val));
        let title = val
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| val.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        Some(Self {
            endpoint,
            icon,
            title,
        })
    }
}

/// Strongly typed ChannelHeaderLinks AST node (`channelHeaderLinksRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelHeaderLinksNode {
    pub primary: Vec<HeaderLinkNode>,
    pub secondary: Vec<HeaderLinkNode>,
}

impl ChannelHeaderLinksNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelHeaderLinksRenderer").or_else(|| val.get("channelHeaderLinks")).unwrap_or(val);

        let primary = node
            .get("primaryLinks")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(HeaderLinkNode::from_value).collect())
            .unwrap_or_default();

        let secondary = node
            .get("secondaryLinks")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(HeaderLinkNode::from_value).collect())
            .unwrap_or_default();

        Some(Self { primary, secondary })
    }
}

/// Strongly typed ChannelHeaderLinksView AST node (`channelHeaderLinksViewModel`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelHeaderLinksViewNode {
    pub first_link: Option<String>,
    pub more: Option<String>,
}

impl ChannelHeaderLinksViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelHeaderLinksViewModel").or_else(|| val.get("channelHeaderLinksView")).unwrap_or(val);

        let first_link = node
            .get("firstLink")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("firstLink").and_then(Value::as_str).map(ToString::to_string));

        let more = node
            .get("more")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("more").and_then(Value::as_str).map(ToString::to_string));

        Some(Self { first_link, more })
    }
}

/// Strongly typed ChannelMobileHeader AST node (`channelMobileHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMobileHeaderNode {
    pub title: String,
}

impl ChannelMobileHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelMobileHeaderRenderer").or_else(|| val.get("channelMobileHeader")).unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        Some(Self { title })
    }
}

/// Strongly typed ChannelSwitcherHeader AST node (`channelSwitcherHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSwitcherHeaderNode {
    pub title: String,
    pub button: Option<Value>,
}

impl ChannelSwitcherHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("channelSwitcherHeaderRenderer").or_else(|| val.get("channelSwitcherHeader")).unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let button = node.get("button").cloned();

        Some(Self { title, button })
    }
}

/// Strongly typed AuthorCommentBadge AST node (`authorCommentBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorCommentBadgeNode {
    pub icon_type: Option<String>,
    pub tooltip: String,
    pub style: Option<String>,
}

impl AuthorCommentBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("authorCommentBadgeRenderer").or_else(|| val.get("authorCommentBadge")).unwrap_or(val);

        let icon_type = node.pointer("/icon/iconType").and_then(Value::as_str).map(ToString::to_string);
        let tooltip = node
            .get("iconTooltip")
            .or_else(|| node.get("tooltip"))
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        let mut style = node.get("style").and_then(Value::as_str).map(ToString::to_string);
        if tooltip == "Verified" && style.is_none() {
            style = Some("BADGE_STYLE_TYPE_VERIFIED".to_string());
        }

        Some(Self {
            icon_type,
            tooltip,
            style,
        })
    }
}

/// Strongly typed CommentReplies AST node (`commentRepliesRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRepliesNode {
    pub contents: Vec<Value>,
    pub sub_threads: Vec<Value>,
    pub view_replies: Option<Value>,
    pub hide_replies: Option<Value>,
    pub view_replies_creator_thumbnail: ThumbnailListNode,
    pub has_channel_owner_replied: bool,
}

impl CommentRepliesNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentRepliesRenderer").or_else(|| val.get("commentReplies")).unwrap_or(val);

        let contents = node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default();
        let sub_threads = node.get("subThreads").and_then(Value::as_array).cloned().unwrap_or_default();
        let view_replies = node.get("viewReplies").cloned();
        let hide_replies = node.get("hideReplies").cloned();
        let has_channel_owner_replied = node.get("viewRepliesCreatorThumbnail").is_some();
        let view_replies_creator_thumbnail = ThumbnailListNode::from_value(
            node.get("viewRepliesCreatorThumbnail").unwrap_or(node),
        );

        Some(Self {
            contents,
            sub_threads,
            view_replies,
            hide_replies,
            view_replies_creator_thumbnail,
            has_channel_owner_replied,
        })
    }
}

/// Keys associated with a CommentView for mutations and actions.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentKeysNode {
    pub comment: Option<String>,
    pub comment_surface: Option<String>,
    pub toolbar_state: Option<String>,
    pub toolbar_surface: Option<String>,
    pub shared: Option<String>,
}

/// Member badge metadata for a comment author.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberBadgeNode {
    pub url: String,
    pub a11y: Option<String>,
}

/// Strongly typed CommentView AST node (`commentViewModel`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentViewNode {
    pub comment_id: String,
    pub is_pinned: bool,
    pub keys: CommentKeysNode,
    pub content: Option<String>,
    pub published_time: Option<String>,
    pub author_is_channel_owner: bool,
    pub author_button_a11y: Option<String>,
    pub creator_thumbnail_url: Option<String>,
    pub like_button_a11y: Option<String>,
    pub like_count: Option<String>,
    pub like_count_liked: Option<String>,
    pub like_count_a11y: Option<String>,
    pub like_active_tooltip: Option<String>,
    pub like_inactive_tooltip: Option<String>,
    pub dislike_active_tooltip: Option<String>,
    pub dislike_inactive_tooltip: Option<String>,
    pub heart_active_tooltip: Option<String>,
    pub reply_count: Option<String>,
    pub reply_count_a11y: Option<String>,
    pub reply_level: Option<u64>,
    pub is_member: bool,
    pub member_badge: Option<MemberBadgeNode>,
    pub author_name: Option<String>,
    pub author_avatar: Option<ThumbnailListNode>,
    pub author_channel_id: Option<String>,
    pub is_liked: bool,
    pub is_disliked: bool,
    pub is_hearted: bool,
    pub voice_reply_container: Option<Value>,
}

impl CommentViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentViewModel").or_else(|| val.get("commentView")).unwrap_or(val);

        let comment_id = node
            .get("commentId")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        let is_pinned = node.get("pinnedText").is_some();

        let keys = CommentKeysNode {
            comment: node.get("commentKey").and_then(Value::as_str).map(ToString::to_string),
            comment_surface: node.get("commentSurfaceKey").and_then(Value::as_str).map(ToString::to_string),
            toolbar_state: node.get("toolbarStateKey").and_then(Value::as_str).map(ToString::to_string),
            toolbar_surface: node.get("toolbarSurfaceKey").and_then(Value::as_str).map(ToString::to_string),
            shared: node.get("sharedKey").and_then(Value::as_str).map(ToString::to_string),
        };

        let content = node
            .pointer("/properties/content")
            .or_else(|| node.get("content"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.pointer("/properties/content").and_then(Value::as_str).map(ToString::to_string));

        let published_time = node
            .pointer("/properties/publishedTime")
            .or_else(|| node.get("publishedTime"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let author_is_channel_owner = node
            .pointer("/author/isCreator")
            .or_else(|| node.get("authorIsChannelOwner"))
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let author_button_a11y = node
            .pointer("/properties/authorButtonA11y")
            .or_else(|| node.get("authorButtonA11y"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let creator_thumbnail_url = node
            .pointer("/toolbar/creatorThumbnailUrl")
            .or_else(|| node.get("creatorThumbnailUrl"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let like_button_a11y = node
            .pointer("/toolbar/likeButtonA11y")
            .or_else(|| node.get("likeButtonA11y"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let like_count = node
            .pointer("/toolbar/likeCountNotliked")
            .or_else(|| node.get("likeCount"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let like_count_liked = node
            .pointer("/toolbar/likeCountLiked")
            .or_else(|| node.get("likeCountLiked"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let like_count_a11y = node
            .pointer("/toolbar/likeCountA11y")
            .or_else(|| node.get("likeCountA11y"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let like_active_tooltip = node
            .pointer("/toolbar/likeActiveTooltip")
            .or_else(|| node.get("likeActiveTooltip"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let like_inactive_tooltip = node
            .pointer("/toolbar/likeInactiveTooltip")
            .or_else(|| node.get("likeInactiveTooltip"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let dislike_active_tooltip = node
            .pointer("/toolbar/dislikeActiveTooltip")
            .or_else(|| node.get("dislikeActiveTooltip"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let dislike_inactive_tooltip = node
            .pointer("/toolbar/dislikeInactiveTooltip")
            .or_else(|| node.get("dislikeInactiveTooltip"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let heart_active_tooltip = node
            .pointer("/toolbar/heartActiveTooltip")
            .or_else(|| node.get("heartActiveTooltip"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let reply_count = node
            .pointer("/toolbar/replyCount")
            .or_else(|| node.get("replyCount"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let reply_count_a11y = node
            .pointer("/toolbar/replyCountA11y")
            .or_else(|| node.get("replyCountA11y"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let reply_level = node
            .pointer("/properties/replyLevel")
            .or_else(|| node.get("replyLevel"))
            .and_then(Value::as_u64);

        let is_member = node.pointer("/author/sponsorBadgeUrl").is_some() || node.get("isMember").and_then(Value::as_bool).unwrap_or(false);

        let member_badge = node
            .pointer("/author/sponsorBadgeUrl")
            .and_then(Value::as_str)
            .map(|url| MemberBadgeNode {
                url: url.to_string(),
                a11y: node.pointer("/author/sponsorBadgeA11y").and_then(Value::as_str).map(ToString::to_string),
            });

        let author_name = node
            .pointer("/author/displayName")
            .or_else(|| node.get("authorName"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let author_avatar = node
            .pointer("/avatar/image")
            .or_else(|| node.get("avatar"))
            .map(ThumbnailListNode::from_value);

        let author_channel_id = node
            .pointer("/author/channelId")
            .or_else(|| node.get("authorChannelId"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let is_liked = node.get("isLiked").and_then(Value::as_bool).unwrap_or(false);
        let is_disliked = node.get("isDisliked").and_then(Value::as_bool).unwrap_or(false);
        let is_hearted = node.get("isHearted").and_then(Value::as_bool).unwrap_or(false);

        let voice_reply_container = node.get("voiceReplyContainerViewModel").or_else(|| node.get("voiceReplyContainer")).cloned();

        Some(Self {
            comment_id,
            is_pinned,
            keys,
            content,
            published_time,
            author_is_channel_owner,
            author_button_a11y,
            creator_thumbnail_url,
            like_button_a11y,
            like_count,
            like_count_liked,
            like_count_a11y,
            like_active_tooltip,
            like_inactive_tooltip,
            dislike_active_tooltip,
            dislike_inactive_tooltip,
            heart_active_tooltip,
            reply_count,
            reply_count_a11y,
            reply_level,
            is_member,
            member_badge,
            author_name,
            author_avatar,
            author_channel_id,
            is_liked,
            is_disliked,
            is_hearted,
            voice_reply_container,
        })
    }
}

/// Strongly typed CommentsEntryPointTeaser AST node (`commentsEntryPointTeaserRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsEntryPointTeaserNode {
    pub teaser_avatar: Option<ThumbnailListNode>,
    pub teaser_content: Option<String>,
}

impl CommentsEntryPointTeaserNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentsEntryPointTeaserRenderer").or_else(|| val.get("commentsEntryPointTeaser")).unwrap_or(val);

        let teaser_avatar = node.get("teaserAvatar").map(ThumbnailListNode::from_value);

        let teaser_content = node
            .get("teaserContent")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("teaserContent").and_then(Value::as_str).map(ToString::to_string));

        Some(Self {
            teaser_avatar,
            teaser_content,
        })
    }
}

/// Strongly typed CommentsSimplebox AST node (`commentsSimpleboxRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsSimpleboxNode {
    pub simplebox_avatar: ThumbnailListNode,
    pub simplebox_placeholder: String,
}

impl CommentsSimpleboxNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentsSimpleboxRenderer").or_else(|| val.get("commentsSimplebox")).unwrap_or(val);

        let simplebox_avatar = ThumbnailListNode::from_value(node.get("simpleboxAvatar").unwrap_or(node));

        let simplebox_placeholder = node
            .get("simpleboxPlaceholder")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("simpleboxPlaceholder").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        Some(Self {
            simplebox_avatar,
            simplebox_placeholder,
        })
    }
}

/// Strongly typed PdgCommentChip AST node (`pdgCommentChipRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdgCommentChipNode {
    pub text: String,
    pub background_color: Option<String>,
    pub foreground_title_color: Option<String>,
    pub icon_type: Option<String>,
}

impl PdgCommentChipNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pdgCommentChipRenderer").or_else(|| val.get("pdgCommentChip")).unwrap_or(val);

        let text = node
            .get("chipText")
            .or_else(|| node.get("text"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("chipText").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let background_color = node
            .pointer("/chipColorPalette/backgroundColor")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let foreground_title_color = node
            .pointer("/chipColorPalette/foregroundTitleColor")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let icon_type = node
            .pointer("/chipIcon/iconType")
            .or_else(|| node.pointer("/icon/iconType"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            text,
            background_color,
            foreground_title_color,
            icon_type,
        })
    }
}

/// Strongly typed SponsorCommentBadge AST node (`sponsorCommentBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsorCommentBadgeNode {
    pub custom_badge: ThumbnailListNode,
    pub tooltip: String,
}

impl SponsorCommentBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sponsorCommentBadgeRenderer").or_else(|| val.get("sponsorCommentBadge")).unwrap_or(val);

        let custom_badge = ThumbnailListNode::from_value(node.get("customBadge").unwrap_or(node));
        let tooltip = node
            .get("tooltip")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or_default();

        Some(Self {
            custom_badge,
            tooltip,
        })
    }
}

/// Strongly typed CommentsContinuation AST node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsContinuationNode {
    pub contents: Vec<Value>,
    pub continuation_token: Option<String>,
    pub continuation_endpoint: Option<Value>,
    pub has_continuation: bool,
}

impl CommentsContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("commentsContinuation").unwrap_or(val);

        let contents = node
            .get("contents")
            .or_else(|| node.get("continuationItems"))
            .or_else(|| node.pointer("/appendContinuationItemsAction/continuationItems"))
            .or_else(|| node.pointer("/onResponseReceivedEndpoints/0/appendContinuationItemsAction/continuationItems"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let continuation_token = node
            .get("continuation")
            .or_else(|| node.get("token"))
            .or_else(|| node.pointer("/continuationItemRenderer/continuationEndpoint/continuationCommand/token"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let continuation_endpoint = node
            .get("continuationEndpoint")
            .or_else(|| node.pointer("/continuationItemRenderer/continuationEndpoint"))
            .cloned();

        let has_continuation = continuation_token.is_some() || continuation_endpoint.is_some();

        Some(Self {
            contents,
            continuation_token,
            continuation_endpoint,
            has_continuation,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_about_channel_node() {
        let val = json!({
            "aboutChannelRenderer": {
                "metadata": {
                    "aboutChannelViewModel": {
                        "description": "About test channel",
                        "country": "US"
                    }
                }
            }
        });
        let node = AboutChannelNode::from_value(&val).unwrap();
        assert!(node.metadata.is_some());
        let meta = node.metadata.unwrap();
        assert_eq!(meta.description.as_deref(), Some("About test channel"));
        assert_eq!(meta.country.as_deref(), Some("US"));
    }

    #[test]
    fn test_channel_node() {
        let val = json!({
            "channelRenderer": {
                "channelId": "UC123456",
                "title": { "simpleText": "Channel 1" },
                "subscriberCountText": { "simpleText": "100K subscribers" }
            }
        });
        let node = ChannelNode::from_value(&val).unwrap();
        assert_eq!(node.id.as_deref(), Some("UC123456"));
        assert_eq!(node.title, "Channel 1");
        assert_eq!(node.subscriber_count.as_deref(), Some("100K subscribers"));
    }

    #[test]
    fn test_comment_view_node() {
        let val = json!({
            "commentViewModel": {
                "commentId": "comment_123",
                "commentKey": "key_abc",
                "properties": {
                    "content": { "content": "Great video!" },
                    "publishedTime": "2 days ago"
                }
            }
        });
        let node = CommentViewNode::from_value(&val).unwrap();
        assert_eq!(node.comment_id, "comment_123");
        assert_eq!(node.keys.comment.as_deref(), Some("key_abc"));
        assert_eq!(node.content.as_deref(), Some("Great video!"));
        assert_eq!(node.published_time.as_deref(), Some("2 days ago"));
    }

    #[test]
    fn test_pdg_comment_chip_node() {
        let val = json!({
            "pdgCommentChipRenderer": {
                "chipText": { "simpleText": "$5.00 Super Thanks" },
                "chipColorPalette": {
                    "backgroundColor": "#00FF00",
                    "foregroundTitleColor": "#000000"
                }
            }
        });
        let node = PdgCommentChipNode::from_value(&val).unwrap();
        assert_eq!(node.text, "$5.00 Super Thanks");
        assert_eq!(node.background_color.as_deref(), Some("#00FF00"));
        assert_eq!(node.foreground_title_color.as_deref(), Some("#000000"));
    }

    #[test]
    fn test_author_comment_badge_node() {
        let val = json!({
            "authorCommentBadgeRenderer": {
                "iconTooltip": "Verified"
            }
        });
        let node = AuthorCommentBadgeNode::from_value(&val).unwrap();
        assert_eq!(node.tooltip, "Verified");
        assert_eq!(node.style.as_deref(), Some("BADGE_STYLE_TYPE_VERIFIED"));
    }
}
