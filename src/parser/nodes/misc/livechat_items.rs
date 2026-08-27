use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed `LiveChatAuthorBadge` AST node (`liveChatAuthorBadgeRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatAuthorBadgeNode {
    pub custom_thumbnail: ThumbnailListNode,
    pub icon_type: Option<String>,
    pub style: Option<String>,
    pub label: Option<String>,
    pub tooltip: Option<String>,
    pub accessibility_label: Option<String>,
}

impl LiveChatAuthorBadgeNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatAuthorBadgeRenderer").unwrap_or(val);
        let custom_thumbnail = ThumbnailListNode::from_value(
            node.get("customThumbnail").unwrap_or(&Value::Null),
        );
        let icon_type = node
            .pointer("/icon/iconType")
            .or_else(|| node.get("iconType"))
            .and_then(Value::as_str)
            .map(String::from);
        let style = node.get("style").and_then(Value::as_str).map(String::from);
        let label = node.get("label").and_then(Value::as_str).map(String::from);
        let tooltip = node
            .get("tooltip")
            .or_else(|| node.get("iconTooltip"))
            .and_then(Value::as_str)
            .map(String::from);
        let accessibility_label = node
            .pointer("/accessibility/accessibilityData/label")
            .or_else(|| node.pointer("/customThumbnail/accessibility/accessibilityData/label"))
            .and_then(Value::as_str)
            .map(String::from);

        Some(Self {
            custom_thumbnail,
            icon_type,
            style,
            label,
            tooltip,
            accessibility_label,
        })
    }
}

/// Strongly typed `LiveChatHeader` AST node (`liveChatHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatHeaderNode {
    pub overflow_menu: Option<Value>,
    pub collapse_button: Option<Value>,
    pub view_selector: Option<Value>,
}

impl LiveChatHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatHeaderRenderer")
            .or_else(|| val.get("liveChatHeader"))
            .unwrap_or(val);

        Some(Self {
            overflow_menu: node.get("overflowMenu").cloned(),
            collapse_button: node.get("collapseButton").cloned(),
            view_selector: node.get("viewSelector").cloned(),
        })
    }
}

/// Strongly typed `LiveChatMessageInput` AST node (`liveChatMessageInputRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatMessageInputNode {
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub send_button: Option<Value>,
    pub target_id: Option<String>,
}

impl LiveChatMessageInputNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatMessageInputRenderer").unwrap_or(val);

        Some(Self {
            author_name: node.get("authorName").and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            send_button: node.get("sendButton").cloned(),
            target_id: node
                .get("targetId")
                .and_then(Value::as_str)
                .map(String::from),
        })
    }
}

/// Strongly typed `LiveChatParticipant` AST node (`liveChatParticipantRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatParticipantNode {
    pub name: Option<TextNode>,
    pub photo: ThumbnailListNode,
    pub badges: Vec<Value>,
}

impl LiveChatParticipantNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatParticipantRenderer").unwrap_or(val);

        Some(Self {
            name: node.get("authorName").and_then(TextNode::from_value),
            photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed `LiveChatBannerChatSummary` AST node (`liveChatBannerChatSummaryRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatBannerChatSummaryNode {
    pub id: Option<String>,
    pub chat_summary: Option<TextNode>,
    pub icon_type: Option<String>,
    pub like_feedback_button: Option<Value>,
    pub dislike_feedback_button: Option<Value>,
}

impl LiveChatBannerChatSummaryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatBannerChatSummaryRenderer").unwrap_or(val);

        Some(Self {
            id: node
                .get("liveChatSummaryId")
                .or_else(|| node.get("id"))
                .and_then(Value::as_str)
                .map(String::from),
            chat_summary: node.get("chatSummary").and_then(TextNode::from_value),
            icon_type: node
                .pointer("/icon/iconType")
                .or_else(|| node.get("iconType"))
                .and_then(Value::as_str)
                .map(String::from),
            like_feedback_button: node.get("likeFeedbackButton").cloned(),
            dislike_feedback_button: node.get("dislikeFeedbackButton").cloned(),
        })
    }
}

/// Strongly typed `LiveChatBannerHeader` AST node (`liveChatBannerHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatBannerHeaderNode {
    pub text: Option<TextNode>,
    pub icon_type: Option<String>,
    pub context_menu_button: Option<Value>,
}

impl LiveChatBannerHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatBannerHeaderRenderer").unwrap_or(val);

        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            icon_type: node
                .pointer("/icon/iconType")
                .or_else(|| node.get("iconType"))
                .and_then(Value::as_str)
                .map(String::from),
            context_menu_button: node.get("contextMenuButton").cloned(),
        })
    }
}

/// Strongly typed `LiveChatBannerRedirect` AST node (`liveChatBannerRedirectRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatBannerRedirectNode {
    pub banner_message: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub inline_action_button: Option<Value>,
    pub context_menu_button: Option<Value>,
}

impl LiveChatBannerRedirectNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatBannerRedirectRenderer").unwrap_or(val);

        Some(Self {
            banner_message: node.get("bannerMessage").and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            inline_action_button: node.get("inlineActionButton").cloned(),
            context_menu_button: node.get("contextMenuButton").cloned(),
        })
    }
}

/// Strongly typed `LiveChatItemBumperView` AST node (`liveChatItemBumperView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatItemBumperViewNode {
    pub content: Option<Value>,
}

impl LiveChatItemBumperViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatItemBumperView")
            .or_else(|| val.get("liveChatItemBumperViewRenderer"))
            .unwrap_or(val);

        Some(Self {
            content: node.get("content").cloned(),
        })
    }
}

/// Strongly typed `LiveChatPaidMessage` AST node (`liveChatPaidMessageRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPaidMessageNode {
    pub id: Option<String>,
    pub message: Option<TextNode>,
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub author_badges: Vec<Value>,
    pub author_external_channel_id: Option<String>,
    pub author_name_text_color: Option<u64>,
    pub header_background_color: Option<u64>,
    pub header_text_color: Option<u64>,
    pub body_background_color: Option<u64>,
    pub body_text_color: Option<u64>,
    pub purchase_amount: Option<String>,
    pub menu_endpoint: Option<Value>,
    pub context_menu_accessibility_label: Option<String>,
    pub timestamp_usec: Option<String>,
    pub timestamp: Option<u64>,
    pub timestamp_text: Option<String>,
    pub timestamp_color: Option<u64>,
    pub header_overlay_image: ThumbnailListNode,
    pub text_input_background_color: Option<u64>,
    pub lower_bumper: Option<Value>,
    pub creator_heart_button: Option<Value>,
    pub is_v2_style: bool,
    pub reply_button: Option<Value>,
}

impl LiveChatPaidMessageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatPaidMessageRenderer").unwrap_or(val);

        let timestamp_usec = node
            .get("timestampUsec")
            .and_then(Value::as_str)
            .map(String::from);

        let timestamp = timestamp_usec
            .as_deref()
            .and_then(|s| s.parse::<u64>().ok())
            .map(|usec| usec / 1000);

        let purchase_amount = node
            .get("purchaseAmountText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("purchaseAmountText")
                    .and_then(Value::as_str)
                    .map(String::from)
            });

        let timestamp_text = node
            .get("timestampText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("timestampText")
                    .and_then(Value::as_str)
                    .map(String::from)
            });

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            message: node.get("message").and_then(TextNode::from_value),
            author_name: node.get("authorName").and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            author_badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            author_name_text_color: node.get("authorNameTextColor").and_then(Value::as_u64),
            header_background_color: node.get("headerBackgroundColor").and_then(Value::as_u64),
            header_text_color: node.get("headerTextColor").and_then(Value::as_u64),
            body_background_color: node.get("bodyBackgroundColor").and_then(Value::as_u64),
            body_text_color: node.get("bodyTextColor").and_then(Value::as_u64),
            purchase_amount,
            menu_endpoint: node.get("contextMenuEndpoint").cloned(),
            context_menu_accessibility_label: node
                .pointer("/contextMenuAccessibility/accessibilityData/label")
                .and_then(Value::as_str)
                .map(String::from),
            timestamp_usec,
            timestamp,
            timestamp_text,
            timestamp_color: node.get("timestampColor").and_then(Value::as_u64),
            header_overlay_image: ThumbnailListNode::from_value(
                node.get("headerOverlayImage").unwrap_or(&Value::Null),
            ),
            text_input_background_color: node
                .get("textInputBackgroundColor")
                .and_then(Value::as_u64),
            lower_bumper: node.get("lowerBumper").cloned(),
            creator_heart_button: node.get("creatorHeartButton").cloned(),
            is_v2_style: node
                .get("isV2Style")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            reply_button: node.get("replyButton").cloned(),
        })
    }
}

/// Strongly typed `LiveChatPlaceholderItem` AST node (`liveChatPlaceholderItemRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPlaceholderItemNode {
    pub id: Option<String>,
    pub timestamp_usec: Option<String>,
    pub timestamp: Option<u64>,
}

impl LiveChatPlaceholderItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatPlaceholderItemRenderer").unwrap_or(val);

        let timestamp_usec = node
            .get("timestampUsec")
            .and_then(Value::as_str)
            .map(String::from);

        let timestamp = timestamp_usec
            .as_deref()
            .and_then(|s| s.parse::<u64>().ok())
            .map(|usec| usec / 1000);

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            timestamp_usec,
            timestamp,
        })
    }
}

/// Strongly typed `LiveChatProductItem` AST node (`liveChatProductItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatProductItemNode {
    pub title: Option<String>,
    pub accessibility_title: Option<String>,
    pub thumbnail: ThumbnailListNode,
    pub price: Option<String>,
    pub vendor_name: Option<String>,
    pub from_vendor_text: Option<String>,
    pub information_button: Option<Value>,
    pub endpoint: Option<Value>,
    pub creator_message: Option<String>,
    pub creator_name: Option<String>,
    pub author_photo: ThumbnailListNode,
    pub information_dialog: Option<Value>,
    pub is_verified: bool,
    pub creator_custom_message: Option<TextNode>,
}

impl LiveChatProductItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatProductItemRenderer").unwrap_or(val);

        Some(Self {
            title: node.get("title").and_then(Value::as_str).map(String::from),
            accessibility_title: node
                .get("accessibilityTitle")
                .and_then(Value::as_str)
                .map(String::from),
            thumbnail: ThumbnailListNode::from_value(
                node.get("thumbnail").unwrap_or(&Value::Null),
            ),
            price: node.get("price").and_then(Value::as_str).map(String::from),
            vendor_name: node
                .get("vendorName")
                .and_then(Value::as_str)
                .map(String::from),
            from_vendor_text: node
                .get("fromVendorText")
                .and_then(Value::as_str)
                .map(String::from),
            information_button: node.get("informationButton").cloned(),
            endpoint: node.get("onClickCommand").cloned(),
            creator_message: node
                .get("creatorMessage")
                .and_then(Value::as_str)
                .map(String::from),
            creator_name: node
                .get("creatorName")
                .and_then(Value::as_str)
                .map(String::from),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            information_dialog: node.get("informationDialog").cloned(),
            is_verified: node
                .get("isVerified")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            creator_custom_message: node
                .get("creatorCustomMessage")
                .and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `LiveChatRestrictedParticipation` AST node (`liveChatRestrictedParticipationRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatRestrictedParticipationNode {
    pub message: Option<TextNode>,
    pub icon_type: Option<String>,
    pub on_click_command: Option<Value>,
}

impl LiveChatRestrictedParticipationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatRestrictedParticipationRenderer")
            .unwrap_or(val);

        Some(Self {
            message: node.get("message").and_then(TextNode::from_value),
            icon_type: node
                .pointer("/icon/iconType")
                .or_else(|| node.get("iconType"))
                .and_then(Value::as_str)
                .map(String::from),
            on_click_command: node.get("onClickCommand").cloned(),
        })
    }
}

/// Strongly typed `LiveChatSponsorshipsGiftPurchaseAnnouncement` AST node (`liveChatSponsorshipsGiftPurchaseAnnouncementRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftPurchaseAnnouncementNode {
    pub id: Option<String>,
    pub timestamp_usec: Option<String>,
    pub author_external_channel_id: Option<String>,
    pub header: Option<LiveChatSponsorshipsHeaderNode>,
}

impl LiveChatSponsorshipsGiftPurchaseAnnouncementNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatSponsorshipsGiftPurchaseAnnouncementRenderer")
            .unwrap_or(val);

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            timestamp_usec: node
                .get("timestampUsec")
                .and_then(Value::as_str)
                .map(String::from),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            header: node
                .get("header")
                .and_then(LiveChatSponsorshipsHeaderNode::from_value),
        })
    }
}

/// Strongly typed `LiveChatSponsorshipsGiftRedemptionAnnouncement` AST node (`liveChatSponsorshipsGiftRedemptionAnnouncementRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsGiftRedemptionAnnouncementNode {
    pub id: Option<String>,
    pub timestamp_usec: Option<String>,
    pub timestamp_text: Option<TextNode>,
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub author_badges: Vec<Value>,
    pub author_external_channel_id: Option<String>,
    pub message: Option<TextNode>,
    pub menu_endpoint: Option<Value>,
    pub context_menu_accessibility_label: Option<String>,
}

impl LiveChatSponsorshipsGiftRedemptionAnnouncementNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatSponsorshipsGiftRedemptionAnnouncementRenderer")
            .unwrap_or(val);

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            timestamp_usec: node
                .get("timestampUsec")
                .and_then(Value::as_str)
                .map(String::from),
            timestamp_text: node
                .get("timestampText")
                .and_then(TextNode::from_value),
            author_name: node
                .get("authorName")
                .and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            author_badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            message: node.get("message").and_then(TextNode::from_value),
            menu_endpoint: node.get("contextMenuEndpoint").cloned(),
            context_menu_accessibility_label: node
                .pointer("/contextMenuAccessibility/accessibilityData/label")
                .and_then(Value::as_str)
                .map(String::from),
        })
    }
}

/// Strongly typed `LiveChatSponsorshipsHeader` AST node (`liveChatSponsorshipsHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSponsorshipsHeaderNode {
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub author_badges: Vec<Value>,
    pub primary_text: Option<TextNode>,
    pub menu_endpoint: Option<Value>,
    pub context_menu_accessibility_label: Option<String>,
    pub image: ThumbnailListNode,
}

impl LiveChatSponsorshipsHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatSponsorshipsHeaderRenderer")
            .unwrap_or(val);

        Some(Self {
            author_name: node
                .get("authorName")
                .and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            author_badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            primary_text: node
                .get("primaryText")
                .and_then(TextNode::from_value),
            menu_endpoint: node.get("contextMenuEndpoint").cloned(),
            context_menu_accessibility_label: node
                .pointer("/contextMenuAccessibility/accessibilityData/label")
                .and_then(Value::as_str)
                .map(String::from),
            image: ThumbnailListNode::from_value(
                node.get("image").unwrap_or(&Value::Null),
            ),
        })
    }
}

/// Strongly typed `LiveChatTextMessage` AST node (`liveChatTextMessageRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTextMessageNode {
    pub id: Option<String>,
    pub message: Option<TextNode>,
    pub inline_action_buttons: Vec<Value>,
    pub timestamp_usec: Option<String>,
    pub timestamp: Option<u64>,
    pub timestamp_text: Option<String>,
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub author_badges: Vec<Value>,
    pub author_external_channel_id: Option<String>,
    pub menu_endpoint: Option<Value>,
    pub context_menu_accessibility_label: Option<String>,
    pub before_content_buttons: Vec<Value>,
}

impl LiveChatTextMessageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatTextMessageRenderer").unwrap_or(val);

        let timestamp_usec = node
            .get("timestampUsec")
            .and_then(Value::as_str)
            .map(String::from);

        let timestamp = timestamp_usec
            .as_deref()
            .and_then(|s| s.parse::<u64>().ok())
            .map(|usec| usec / 1000);

        let timestamp_text = node
            .get("timestampText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| {
                node.get("timestampText")
                    .and_then(Value::as_str)
                    .map(String::from)
            });

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            message: node.get("message").and_then(TextNode::from_value),
            inline_action_buttons: node
                .get("inlineActionButtons")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            timestamp_usec,
            timestamp,
            timestamp_text,
            author_name: node
                .get("authorName")
                .and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            author_badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            menu_endpoint: node.get("contextMenuEndpoint").cloned(),
            context_menu_accessibility_label: node
                .pointer("/contextMenuAccessibility/accessibilityData/label")
                .and_then(Value::as_str)
                .map(String::from),
            before_content_buttons: node
                .get("beforeContentButtons")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed `LiveChatTickerPaidMessageItem` AST node (`liveChatTickerPaidMessageItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTickerPaidMessageItemNode {
    pub id: Option<String>,
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub author_badges: Vec<Value>,
    pub author_external_channel_id: Option<String>,
    pub amount: Option<TextNode>,
    pub amount_text_color: Option<u64>,
    pub start_background_color: Option<u64>,
    pub end_background_color: Option<u64>,
    pub duration_sec: Option<u64>,
    pub full_duration_sec: Option<u64>,
    pub show_item: Option<Value>,
    pub show_item_endpoint: Option<Value>,
    pub animation_origin: Option<String>,
    pub open_engagement_panel_command: Option<Value>,
}

impl LiveChatTickerPaidMessageItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatTickerPaidMessageItemRenderer")
            .unwrap_or(val);

        let show_item = node
            .pointer("/showItemEndpoint/showLiveChatItemEndpoint/renderer")
            .cloned();

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            author_name: node
                .get("authorName")
                .or_else(|| node.get("authorUsername"))
                .and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            author_badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            amount: node.get("amount").and_then(TextNode::from_value),
            amount_text_color: node.get("amountTextColor").and_then(Value::as_u64),
            start_background_color: node.get("startBackgroundColor").and_then(Value::as_u64),
            end_background_color: node.get("endBackgroundColor").and_then(Value::as_u64),
            duration_sec: node.get("durationSec").and_then(Value::as_u64),
            full_duration_sec: node.get("fullDurationSec").and_then(Value::as_u64),
            show_item,
            show_item_endpoint: node.get("showItemEndpoint").cloned(),
            animation_origin: node
                .get("animationOrigin")
                .and_then(Value::as_str)
                .map(String::from),
            open_engagement_panel_command: node.get("openEngagementPanelCommand").cloned(),
        })
    }
}

/// Represents a thumbnail entry in `LiveChatTickerPaidStickerItem`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTickerThumbnailNode {
    pub thumbnails: ThumbnailListNode,
    pub label: Option<String>,
}

impl LiveChatTickerThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let thumbnails = ThumbnailListNode::from_value(val);
        let label = val
            .pointer("/accessibility/accessibilityData/label")
            .and_then(Value::as_str)
            .map(String::from);
        Some(Self { thumbnails, label })
    }
}

/// Strongly typed `LiveChatTickerPaidStickerItem` AST node (`liveChatTickerPaidStickerItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTickerPaidStickerItemNode {
    pub id: Option<String>,
    pub author_external_channel_id: Option<String>,
    pub author_photo: ThumbnailListNode,
    pub start_background_color: Option<u64>,
    pub end_background_color: Option<u64>,
    pub duration_sec: Option<u64>,
    pub full_duration_sec: Option<u64>,
    pub show_item: Option<Value>,
    pub show_item_endpoint: Option<Value>,
    pub ticker_thumbnails: Vec<LiveChatTickerThumbnailNode>,
}

impl LiveChatTickerPaidStickerItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatTickerPaidStickerItemRenderer")
            .unwrap_or(val);

        let show_item = node
            .pointer("/showItemEndpoint/showLiveChatItemEndpoint/renderer")
            .cloned();

        let ticker_thumbnails = node
            .get("tickerThumbnails")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(LiveChatTickerThumbnailNode::from_value)
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            start_background_color: node.get("startBackgroundColor").and_then(Value::as_u64),
            end_background_color: node.get("endBackgroundColor").and_then(Value::as_u64),
            duration_sec: node.get("durationSec").and_then(Value::as_u64),
            full_duration_sec: node.get("fullDurationSec").and_then(Value::as_u64),
            show_item,
            show_item_endpoint: node.get("showItemEndpoint").cloned(),
            ticker_thumbnails,
        })
    }
}

/// Strongly typed `LiveChatTickerSponsorItem` AST node (`liveChatTickerSponsorItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatTickerSponsorItemNode {
    pub id: Option<String>,
    pub detail: Option<TextNode>,
    pub author_name: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub author_badges: Vec<Value>,
    pub author_external_channel_id: Option<String>,
    pub duration_sec: Option<u64>,
    pub show_item_endpoint: Option<Value>,
}

impl LiveChatTickerSponsorItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("liveChatTickerSponsorItemRenderer")
            .unwrap_or(val);

        let duration_sec = node
            .get("durationSec")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("durationSec")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse::<u64>().ok())
            });

        Some(Self {
            id: node.get("id").and_then(Value::as_str).map(String::from),
            detail: node.get("detailText").and_then(TextNode::from_value),
            author_name: node.get("authorName").and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("sponsorPhoto")
                    .or_else(|| node.get("authorPhoto"))
                    .unwrap_or(&Value::Null),
            ),
            author_badges: node
                .get("authorBadges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            author_external_channel_id: node
                .get("authorExternalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
            duration_sec,
            show_item_endpoint: node.get("showItemEndpoint").cloned(),
        })
    }
}

/// Strongly typed `ShowLiveChatActionPanelAction` AST node (`showLiveChatActionPanelAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowLiveChatActionPanelActionNode {
    pub panel_to_show: Option<Value>,
}

impl ShowLiveChatActionPanelActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("showLiveChatActionPanelAction")
            .unwrap_or(val);

        Some(Self {
            panel_to_show: node.get("panelToShow").cloned(),
        })
    }
}

/// Strongly typed `ShowLiveChatDialogAction` AST node (`showLiveChatDialogAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowLiveChatDialogActionNode {
    pub dialog: Option<Value>,
}

impl ShowLiveChatDialogActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showLiveChatDialogAction").unwrap_or(val);

        Some(Self {
            dialog: node.get("dialog").cloned(),
        })
    }
}

/// Strongly typed `ShowLiveChatTooltipCommand` AST node (`showLiveChatTooltipCommand`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowLiveChatTooltipCommandNode {
    pub tooltip: Option<Value>,
}

impl ShowLiveChatTooltipCommandNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("showLiveChatTooltipCommand").unwrap_or(val);

        Some(Self {
            tooltip: node.get("tooltip").cloned(),
        })
    }
}

/// Strongly typed `MarkChatItemsByAuthorAsDeletedAction` AST node (`markChatItemsByAuthorAsDeletedAction`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkChatItemsByAuthorAsDeletedActionNode {
    pub deleted_state_message: Option<TextNode>,
    pub external_channel_id: Option<String>,
}

impl MarkChatItemsByAuthorAsDeletedActionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("markChatItemsByAuthorAsDeletedAction")
            .unwrap_or(val);

        Some(Self {
            deleted_state_message: node
                .get("deletedStateMessage")
                .and_then(TextNode::from_value),
            external_channel_id: node
                .get("externalChannelId")
                .and_then(Value::as_str)
                .map(String::from),
        })
    }
}

/// Represents an option/choice in `LiveChatBannerPoll`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatPollChoiceNode {
    pub option_id: Option<String>,
    pub text: Option<TextNode>,
}

impl LiveChatPollChoiceNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        Some(Self {
            option_id: val
                .get("pollOptionId")
                .or_else(|| val.get("optionId"))
                .and_then(Value::as_str)
                .map(String::from),
            text: val.get("text").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `LiveChatBannerPoll` AST node (`liveChatBannerPollRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatBannerPollNode {
    pub poll_question: Option<TextNode>,
    pub author_photo: ThumbnailListNode,
    pub choices: Vec<LiveChatPollChoiceNode>,
    pub collapsed_state_entity_key: Option<String>,
    pub live_chat_poll_state_entity_key: Option<String>,
    pub context_menu_button: Option<Value>,
}

impl LiveChatBannerPollNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatBannerPollRenderer").unwrap_or(val);

        let choices = node
            .get("pollChoices")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(LiveChatPollChoiceNode::from_value)
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            poll_question: node.get("pollQuestion").and_then(TextNode::from_value),
            author_photo: ThumbnailListNode::from_value(
                node.get("authorPhoto").unwrap_or(&Value::Null),
            ),
            choices,
            collapsed_state_entity_key: node
                .get("collapsedStateEntityKey")
                .and_then(Value::as_str)
                .map(String::from),
            live_chat_poll_state_entity_key: node
                .get("liveChatPollStateEntityKey")
                .and_then(Value::as_str)
                .map(String::from),
            context_menu_button: node.get("contextMenuButton").cloned(),
        })
    }
}
