use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed AST node (`ThumbnailOverlayResumePlayback`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayResumePlaybackNode {
    pub percent_duration_watched: Option<f64>,
}

impl ThumbnailOverlayResumePlaybackNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayResumePlaybackRenderer").unwrap_or(val);
        Some(Self {
            percent_duration_watched: node
                .get("percentDurationWatched")
                .and_then(|v| v.as_f64()),
        })
    }
}

/// Strongly typed AST node (`ThumbnailOverlayTitleView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailOverlayTitleViewNode {
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

impl ThumbnailOverlayTitleViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailOverlayTitleViewModel").unwrap_or(val);
        Some(Self {
            title: node
                .get("title")
                .and_then(|v| v.get("content"))
                .and_then(|v| v.as_str())
                .map(String::from),
            subtitle: node
                .get("subtitle")
                .and_then(|v| v.get("content"))
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailBackgroundColor {
    pub light_theme: Option<i64>,
    pub dark_theme: Option<i64>,
}

/// Strongly typed AST node (`ThumbnailView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailViewNode {
    pub image: ThumbnailListNode,
    pub overlays: Vec<Value>,
    pub background_color: Option<ThumbnailBackgroundColor>,
}

impl ThumbnailViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("thumbnailViewModel").unwrap_or(val);
        
        let image = node
            .get("image")
            .map(ThumbnailListNode::from_value)
            .unwrap_or_else(|| ThumbnailListNode::from_value(&serde_json::json!(null)));

        let overlays = node
            .get("overlays")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();

        let background_color = node.get("backgroundColor").map(|bg| ThumbnailBackgroundColor {
            light_theme: bg.get("lightTheme").and_then(|v| v.as_i64()),
            dark_theme: bg.get("darkTheme").and_then(|v| v.as_i64()),
        });

        Some(Self {
            image,
            overlays,
            background_color,
        })
    }
}

/// Strongly typed AST node (`TicketEvent`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketEventNode {
    pub title: Option<String>,
    pub time_month: Option<String>,
    pub time_day: Option<String>,
    pub link_text: Option<String>,
    pub button_text: Option<String>,
    pub endpoint: Option<Value>,
    pub subtitle1: Option<String>,
    pub subtitle2: Option<String>,
    pub time_date: Option<String>,
    pub time_time: Option<String>,
    pub time_weekday: Option<String>,
    pub button_accessibility_text: Option<String>,
    pub has_multiple_offers: bool,
}

impl TicketEventNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("ticketEventRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(|v| v.as_str()).map(String::from),
            time_month: node.get("timeMonth").and_then(|v| v.as_str()).map(String::from),
            time_day: node.get("timeDay").and_then(|v| v.as_str()).map(String::from),
            link_text: node.get("linkText").and_then(|v| v.as_str()).map(String::from),
            button_text: node.get("buttonText").and_then(|v| v.as_str()).map(String::from),
            endpoint: node.get("buttonCommand").cloned(),
            subtitle1: node.get("subtitle1").and_then(|v| v.as_str()).map(String::from),
            subtitle2: node.get("subtitle2").and_then(|v| v.as_str()).map(String::from),
            time_date: node.get("timeDate").and_then(|v| v.as_str()).map(String::from),
            time_time: node.get("timeTime").and_then(|v| v.as_str()).map(String::from),
            time_weekday: node.get("timeWeekday").and_then(|v| v.as_str()).map(String::from),
            button_accessibility_text: node.get("buttonAccessibilityText").and_then(|v| v.as_str()).map(String::from),
            has_multiple_offers: node.get("hasMultipleOffers").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed AST node (`TicketShelf`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketShelfNode {
    pub title: Option<String>,
    pub events: Vec<Value>,
    pub information_text: Option<String>,
    pub use_calendar_avatar: bool,
}

impl TicketShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("ticketShelfRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(|v| v.as_str()).map(String::from),
            events: node
                .get("events")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            information_text: node.get("informationText").and_then(|v| v.as_str()).map(String::from),
            use_calendar_avatar: node.get("useCalendarAvatar").and_then(|v| v.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed AST node (`TitleAndButtonListHeader`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleAndButtonListHeaderNode {
    pub title: Option<TextNode>,
}

impl TitleAndButtonListHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("titleAndButtonListHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed AST node (`ToggleButtonView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleButtonViewNode {
    pub default_button: Option<Value>,
    pub toggled_button: Option<Value>,
    pub is_toggling_disabled: bool,
    pub identifier: Option<String>,
    pub is_toggled: Option<bool>,
}

impl ToggleButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("toggleButtonViewModel").unwrap_or(val);
        Some(Self {
            default_button: node.get("defaultButtonViewModel").cloned(),
            toggled_button: node.get("toggledButtonViewModel").cloned(),
            is_toggling_disabled: node.get("isTogglingDisabled").and_then(|v| v.as_bool()).unwrap_or(false),
            identifier: node.get("identifier").and_then(|v| v.as_str()).map(String::from),
            is_toggled: node.get("isToggled").and_then(|v| v.as_bool()),
        })
    }
}

/// Strongly typed AST node (`ToggleFormField`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFormFieldNode {
    pub label: Option<TextNode>,
    pub toggled: bool,
    pub toggle_on_action: Option<Value>,
    pub toggle_off_action: Option<Value>,
}

impl ToggleFormFieldNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("toggleFormFieldRenderer").unwrap_or(val);
        Some(Self {
            label: node.get("label").and_then(TextNode::from_value),
            toggled: node.get("toggled").and_then(|v| v.as_bool()).unwrap_or(false),
            toggle_on_action: node.get("toggleOnAction").cloned(),
            toggle_off_action: node.get("toggleOffAction").cloned(),
        })
    }
}

/// Strongly typed AST node (`ToggleMenuServiceItem`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleMenuServiceItemNode {
    pub text: Option<TextNode>,
    pub toggled_text: Option<TextNode>,
    pub icon_type: Option<String>,
    pub toggled_icon_type: Option<String>,
    pub default_endpoint: Option<Value>,
    pub toggled_endpoint: Option<Value>,
}

impl ToggleMenuServiceItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("toggleMenuServiceItemRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("defaultText").and_then(TextNode::from_value),
            toggled_text: node.get("toggledText").and_then(TextNode::from_value),
            icon_type: node.get("defaultIcon").and_then(|v| v.get("iconType")).and_then(|v| v.as_str()).map(String::from),
            toggled_icon_type: node.get("toggledIcon").and_then(|v| v.get("iconType")).and_then(|v| v.as_str()).map(String::from),
            default_endpoint: node.get("defaultServiceEndpoint").cloned(),
            toggled_endpoint: node.get("toggledServiceEndpoint").cloned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromoConfigNode {
    pub promo_id: Option<String>,
    pub impression_endpoints: Vec<Value>,
    pub accept: Option<Value>,
    pub dismiss: Option<Value>,
}

/// Strongly typed AST node (`Tooltip`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TooltipNode {
    pub promo_config: Option<PromoConfigNode>,
    pub target_id: Option<String>,
    pub details: Option<TextNode>,
    pub suggested_position: Option<String>,
    pub dismiss_strategy: Option<String>,
    pub dwell_time_ms: Option<i64>,
}

impl TooltipNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("tooltipRenderer").unwrap_or(val);
        
        let promo_config = node.get("promoConfig").map(|pc| PromoConfigNode {
            promo_id: pc.get("promoId").and_then(|v| v.as_str()).map(String::from),
            impression_endpoints: pc.get("impressionEndpoints").and_then(|v| v.as_array()).map(|a| a.to_vec()).unwrap_or_default(),
            accept: pc.get("acceptCommand").cloned(),
            dismiss: pc.get("dismissCommand").cloned(),
        });

        let dwell_time_ms = node.get("dwellTimeMs").and_then(|v| v.as_str()).and_then(|v| v.parse::<i64>().ok())
            .or_else(|| node.get("dwellTimeMs").and_then(|v| v.as_i64()));

        Some(Self {
            promo_config,
            target_id: node.get("targetId").and_then(|v| v.as_str()).map(String::from),
            details: node.get("detailsText").and_then(TextNode::from_value),
            suggested_position: node.get("suggestedPosition").and_then(|v| v.get("type")).and_then(|v| v.as_str()).map(String::from),
            dismiss_strategy: node.get("dismissStrategy").and_then(|v| v.get("type")).and_then(|v| v.as_str()).map(String::from),
            dwell_time_ms,
        })
    }
}

/// Strongly typed AST node (`Transcript`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptNode {
    pub content: Option<Value>,
}

impl TranscriptNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptRenderer").unwrap_or(val);
        Some(Self {
            content: node.get("content").cloned(),
        })
    }
}

/// Strongly typed AST node (`TranscriptFooter`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptFooterNode {
    pub language_menu: Option<Value>,
}

impl TranscriptFooterNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("transcriptFooterRenderer").unwrap_or(val);
        Some(Self {
            language_menu: node.get("languageMenu").cloned(),
        })
    }
}
