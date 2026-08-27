use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortFilterHeaderNode {
    pub filter_menu: Option<Value>,
}

impl SortFilterHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sortFilterHeaderRenderer").unwrap_or(val);
        Some(Self {
            filter_menu: node.get("filterMenu").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubMenuItemNode {
    pub title: Option<String>,
    pub selected: bool,
    pub continuation: Option<String>,
    pub endpoint: Option<Value>,
    pub subtitle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortFilterSubMenuNode {
    pub title: Option<String>,
    pub icon_type: Option<String>,
    pub tooltip: Option<String>,
    pub sub_menu_items: Option<Vec<SubMenuItemNode>>,
    pub accessibility: Option<Value>,
}

impl SortFilterSubMenuNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("sortFilterSubMenuRenderer").unwrap_or(val);
        
        let sub_menu_items = node.get("subMenuItems").and_then(|i| i.as_array()).map(|items| {
            items.iter().map(|item| SubMenuItemNode {
                title: item.get("title").and_then(|t| t.as_str()).map(String::from),
                selected: item.get("selected").and_then(|s| s.as_bool()).unwrap_or(false),
                continuation: item.get("continuation").and_then(|c| c.get("reloadContinuationData")).and_then(|r| r.get("continuation")).and_then(|c| c.as_str()).map(String::from),
                endpoint: item.get("serviceEndpoint").or_else(|| item.get("navigationEndpoint")).cloned(),
                subtitle: item.get("subtitle").and_then(|s| s.as_str()).map(String::from),
            }).collect()
        });

        Some(Self {
            title: node.get("title").and_then(|t| t.as_str()).map(String::from),
            icon_type: node.get("icon").and_then(|i| i.get("iconType")).and_then(|t| t.as_str()).map(String::from),
            tooltip: node.get("tooltip").and_then(|t| t.as_str()).map(String::from),
            sub_menu_items,
            accessibility: node.get("accessibility").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartAtNode {
    pub start_at_option_label: Option<TextNode>,
}

impl StartAtNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("startAtRenderer").unwrap_or(val);
        Some(Self {
            start_at_option_label: node.get("startAtOptionLabel").and_then(TextNode::from_value),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDescriptionContentNode {
    pub items: Option<Vec<Value>>,
}

impl StructuredDescriptionContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("structuredDescriptionContentRenderer").unwrap_or(val);
        Some(Self {
            items: node.get("items").and_then(|i| i.as_array()).cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDescriptionPlaylistLockupNode {
    pub thumbnail: Option<ThumbnailListNode>,
    pub title: Option<TextNode>,
    pub short_byline_text: Option<TextNode>,
    pub video_count_short_text: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub thumbnail_width: Option<f64>,
    pub aspect_ratio: Option<f64>,
    pub max_lines_title: Option<f64>,
    pub max_lines_short_byline_text: Option<f64>,
    pub overlay_position: Option<String>,
}

impl StructuredDescriptionPlaylistLockupNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("structuredDescriptionPlaylistLockupRenderer").unwrap_or(val);
        Some(Self {
            thumbnail: node.get("thumbnail").map(ThumbnailListNode::from_value),
            title: node.get("title").and_then(TextNode::from_value),
            short_byline_text: node.get("shortBylineText").and_then(TextNode::from_value),
            video_count_short_text: node.get("videoCountShortText").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            thumbnail_width: node.get("thumbnailWidth").and_then(|w| w.as_f64()),
            aspect_ratio: node.get("aspectRatio").and_then(|a| a.as_f64()),
            max_lines_title: node.get("maxLinesTitle").and_then(|m| m.as_f64()),
            max_lines_short_byline_text: node.get("maxLinesShortBylineText").and_then(|m| m.as_f64()),
            overlay_position: node.get("overlayPosition").and_then(|o| o.as_str()).map(String::from),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubFeedOptionNode {
    pub name: Option<TextNode>,
    pub is_selected: bool,
    pub endpoint: Option<Value>,
}

impl SubFeedOptionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("subFeedOptionRenderer").unwrap_or(val);
        Some(Self {
            name: node.get("name").and_then(TextNode::from_value),
            is_selected: node.get("isSelected").and_then(|b| b.as_bool()).unwrap_or(false),
            endpoint: node.get("navigationEndpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubFeedSelectorNode {
    pub title: Option<TextNode>,
    pub options: Option<Vec<Value>>,
}

impl SubFeedSelectorNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("subFeedSelectorRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            options: node.get("options").and_then(|o| o.as_array()).cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeButtonNode {
    pub button_text: Option<TextNode>,
    pub subscribed: bool,
    pub enabled: bool,
    pub item_type: Option<String>,
    pub channel_id: Option<String>,
    pub show_preferences: bool,
    pub subscribed_text: Option<TextNode>,
    pub unsubscribed_text: Option<TextNode>,
    pub unsubscribe_text: Option<TextNode>,
    pub notification_preference_button: Option<Value>,
    pub service_endpoints: Option<Vec<Value>>,
    pub on_subscribe_endpoints: Option<Vec<Value>>,
    pub on_unsubscribe_endpoints: Option<Vec<Value>>,
    pub subscribed_entity_key: Option<String>,
    pub target_id: Option<String>,
    pub subscribe_accessibility_label: Option<String>,
    pub unsubscribe_accessibility_label: Option<String>,
}

impl SubscribeButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("subscribeButtonRenderer").unwrap_or(val);
        Some(Self {
            button_text: node.get("buttonText").and_then(TextNode::from_value),
            subscribed: node.get("subscribed").and_then(|b| b.as_bool()).unwrap_or(false),
            enabled: node.get("enabled").and_then(|b| b.as_bool()).unwrap_or(false),
            item_type: node.get("type").and_then(|t| t.as_str()).map(String::from),
            channel_id: node.get("channelId").and_then(|i| i.as_str()).map(String::from),
            show_preferences: node.get("showPreferences").and_then(|b| b.as_bool()).unwrap_or(false),
            subscribed_text: node.get("subscribedButtonText").and_then(TextNode::from_value),
            unsubscribed_text: node.get("unsubscribedButtonText").and_then(TextNode::from_value),
            unsubscribe_text: node.get("unsubscribeButtonText").and_then(TextNode::from_value),
            notification_preference_button: node.get("notificationPreferenceButton").cloned(),
            service_endpoints: node.get("serviceEndpoints").and_then(|a| a.as_array()).cloned(),
            on_subscribe_endpoints: node.get("onSubscribeEndpoints").and_then(|a| a.as_array()).cloned(),
            on_unsubscribe_endpoints: node.get("onUnsubscribeEndpoints").and_then(|a| a.as_array()).cloned(),
            subscribed_entity_key: node.get("subscribedEntityKey").and_then(|k| k.as_str()).map(String::from),
            target_id: node.get("targetId").and_then(|t| t.as_str()).map(String::from),
            subscribe_accessibility_label: node.get("subscribeAccessibility").and_then(|a| a.get("accessibilityData")).and_then(|a| a.get("label")).and_then(|l| l.as_str()).map(String::from),
            unsubscribe_accessibility_label: node.get("unsubscribeAccessibility").and_then(|a| a.get("accessibilityData")).and_then(|a| a.get("label")).and_then(|l| l.as_str()).map(String::from),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonContentNode {
    pub button_text: Option<String>,
    pub accessibility_text: Option<String>,
    pub image_name: Option<String>,
    pub subscribe_state_subscribed: bool,
    pub endpoint: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonStyleNode {
    pub unsubscribed_state_style: Option<String>,
    pub subscribed_state_style: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BellAccessibilityDataNode {
    pub off_label: Option<String>,
    pub all_label: Option<String>,
    pub occasional_label: Option<String>,
    pub disabled_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeButtonViewNode {
    pub subscribe_button_content: Option<ButtonContentNode>,
    pub unsubscribe_button_content: Option<ButtonContentNode>,
    pub disable_notification_bell: bool,
    pub button_style: Option<ButtonStyleNode>,
    pub is_signed_out: bool,
    pub background_style: Option<String>,
    pub disable_subscribe_button: bool,
    pub on_show_subscription_options: Option<Value>,
    pub channel_id: Option<String>,
    pub enable_subscribe_button_post_click_animation: bool,
    pub bell_accessibility_data: Option<BellAccessibilityDataNode>,
}

impl SubscribeButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("subscribeButtonView").unwrap_or(val);

        let parse_button = |v: Option<&Value>| -> Option<ButtonContentNode> {
            let v = v?;
            Some(ButtonContentNode {
                button_text: v.get("buttonText").and_then(|t| t.as_str()).map(String::from),
                accessibility_text: v.get("accessibilityText").and_then(|t| t.as_str()).map(String::from),
                image_name: v.get("imageName").and_then(|t| t.as_str()).map(String::from),
                subscribe_state_subscribed: v.get("subscribeState").and_then(|s| s.get("subscribed")).and_then(|b| b.as_bool()).unwrap_or(false),
                endpoint: v.get("onTapCommand").cloned(),
            })
        };

        Some(Self {
            subscribe_button_content: parse_button(node.get("subscribeButtonContent")),
            unsubscribe_button_content: parse_button(node.get("unsubscribeButtonContent")),
            disable_notification_bell: node.get("disableNotificationBell").and_then(|b| b.as_bool()).unwrap_or(false),
            button_style: node.get("buttonStyle").map(|b| ButtonStyleNode {
                unsubscribed_state_style: b.get("unsubscribedStateStyle").and_then(|s| s.as_str()).map(String::from),
                subscribed_state_style: b.get("subscribedStateStyle").and_then(|s| s.as_str()).map(String::from),
            }),
            is_signed_out: node.get("isSignedOut").and_then(|b| b.as_bool()).unwrap_or(false),
            background_style: node.get("backgroundStyle").and_then(|s| s.as_str()).map(String::from),
            disable_subscribe_button: node.get("disableSubscribeButton").and_then(|b| b.as_bool()).unwrap_or(false),
            on_show_subscription_options: node.get("onShowSubscriptionOptions").cloned(),
            channel_id: node.get("channelId").and_then(|c| c.as_str()).map(String::from),
            enable_subscribe_button_post_click_animation: node.get("enableSubscribeButtonPostClickAnimation").and_then(|b| b.as_bool()).unwrap_or(false),
            bell_accessibility_data: node.get("bellAccessibilityData").map(|b| BellAccessibilityDataNode {
                off_label: b.get("offLabel").and_then(|l| l.as_str()).map(String::from),
                all_label: b.get("allLabel").and_then(|l| l.as_str()).map(String::from),
                occasional_label: b.get("occasionalLabel").and_then(|l| l.as_str()).map(String::from),
                disabled_label: b.get("disabledLabel").and_then(|l| l.as_str()).map(String::from),
            }),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabbedNode {
    pub contents: Option<Value>,
}

impl TabbedNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("tabbedRenderer").unwrap_or(val);
        Some(Self {
            contents: Some(node.clone()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabbedSearchResultsNode {
    pub tabs: Option<Vec<Value>>,
}

impl TabbedSearchResultsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("tabbedSearchResultsRenderer").unwrap_or(val);
        Some(Self {
            tabs: node.get("tabs").and_then(|t| t.as_array()).cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCarouselItemViewNode {
    pub icon_name: Option<String>,
    pub text: Option<TextNode>,
    pub on_tap_endpoint: Option<Value>,
    pub button: Option<Value>,
}

impl TextCarouselItemViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("textCarouselItemView").unwrap_or(val);
        Some(Self {
            icon_name: node.get("iconName").and_then(|i| i.as_str()).map(String::from),
            text: node.get("text").and_then(TextNode::from_value),
            on_tap_endpoint: node.get("onTap").cloned(),
            button: node.get("button").cloned(),
        })
    }
}
