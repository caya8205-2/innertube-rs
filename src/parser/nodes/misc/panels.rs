use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed `ListItemView` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItemViewNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub selection_text: Option<TextNode>,
    pub selection_style: Option<String>,
    pub background_color: Option<u64>,
    pub leading_accessory: Option<Value>,
    pub trailing_button: Option<Value>,
    pub trailing_buttons: Vec<Value>,
    pub is_disabled: bool,
    pub is_selected: bool,
    pub has_divider_below: bool,
    pub renderer_context: Option<Value>,
}

impl ListItemViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("listItemView").or_else(|| val.get("ListItemView")).unwrap_or(val);

        let background_color = node.get("backgroundColor").and_then(|v| {
            if let Some(s) = v.as_str() {
                u64::from_str_radix(s, 16).ok()
            } else {
                v.as_u64()
            }
        });

        let trailing_buttons = node.get("trailingButtons")
            .and_then(|t| t.get("buttons"))
            .and_then(|b| b.as_array())
            .cloned()
            .unwrap_or_default();

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            selection_text: node.get("selectionText").and_then(TextNode::from_value),
            selection_style: node.get("selectionStyle").and_then(|v| v.as_str().map(String::from)),
            background_color,
            leading_accessory: node.get("leadingAccessory").cloned(),
            trailing_button: node.get("trailingButton").cloned(),
            trailing_buttons,
            is_disabled: node.get("isDisabled").and_then(|v| v.as_bool()).unwrap_or(false),
            is_selected: node.get("isSelected").and_then(|v| v.as_bool()).unwrap_or(false),
            has_divider_below: node.get("hasDividerBelow").and_then(|v| v.as_bool()).unwrap_or(false),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed `ListView` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListViewNode {
    pub items: Vec<Value>,
    pub renderer_context: Option<Value>,
}

impl ListViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("listView").or_else(|| val.get("ListView")).unwrap_or(val);
        Some(Self {
            items: node.get("listItems").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed `LiveChatDialog` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatDialogNode {
    pub confirm_button: Option<Value>,
    pub dialog_messages: Vec<TextNode>,
}

impl LiveChatDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("liveChatDialogRenderer").or_else(|| val.get("liveChatDialog")).unwrap_or(val);
        
        let dialog_messages = node.get("dialogMessages")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(TextNode::from_value).collect())
            .unwrap_or_default();
            
        Some(Self {
            confirm_button: node.get("confirmButton").cloned(),
            dialog_messages,
        })
    }
}

/// Strongly typed `LockupMetadataView` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockupMetadataViewNode {
    pub title: Option<TextNode>,
    pub metadata: Option<Value>,
    pub image: Option<Value>,
    pub menu_button: Option<Value>,
}

impl LockupMetadataViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("lockupMetadataView").or_else(|| val.get("LockupMetadataView")).unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            metadata: node.get("metadata").cloned(),
            image: node.get("image").cloned(),
            menu_button: node.get("menuButton").cloned(),
        })
    }
}

/// Strongly typed `LockupView` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockupViewNode {
    pub content_image: Option<Value>,
    pub metadata: Option<Value>,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub renderer_context: Option<Value>,
}

impl LockupViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("lockupView").or_else(|| val.get("LockupView")).unwrap_or(val);
        Some(Self {
            content_image: node.get("contentImage").cloned(),
            metadata: node.get("metadata").cloned(),
            content_id: node.get("contentId").and_then(|v| v.as_str().map(String::from)),
            content_type: node.get("contentType")
                .and_then(|v| v.as_str().map(|s| s.replace("LOCKUP_CONTENT_TYPE_", ""))),
            renderer_context: node.get("rendererContext").cloned(),
        })
    }
}

/// Strongly typed `MacroMarkersInfoItem` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroMarkersInfoItemNode {
    pub info_text: Option<TextNode>,
    pub menu: Option<Value>,
}

impl MacroMarkersInfoItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("macroMarkersInfoItemRenderer").or_else(|| val.get("macroMarkersInfoItem")).unwrap_or(val);
        Some(Self {
            info_text: node.get("infoText").and_then(TextNode::from_value),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed `MacroMarkersListEntity` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroMarkersListEntityNode {
    pub marker_entity_key: Option<String>,
    pub external_video_id: Option<String>,
    pub marker_type: Option<String>,
    pub markers: Vec<Value>,
    pub max_height_dp: Option<i64>,
    pub min_height_dp: Option<i64>,
    pub show_hide_animation_duration_millis: Option<i64>,
    pub timed_marker_decorations: Vec<Value>,
}

impl MacroMarkersListEntityNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("macroMarkersListEntity").or_else(|| val.get("MacroMarkersListEntity")).unwrap_or(val);
        let markers_list = node.get("markersList");
        
        let markers = markers_list.and_then(|m| m.get("markers"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
            
        let timed_marker_decorations = markers_list
            .and_then(|m| m.get("markersDecoration"))
            .and_then(|d| d.get("timedMarkerDecorations"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
            
        let heatmap_metadata = markers_list.and_then(|m| m.get("markersMetadata")).and_then(|m| m.get("heatmapMetadata"));

        Some(Self {
            marker_entity_key: node.get("key").and_then(|v| v.as_str().map(String::from)),
            external_video_id: node.get("externalVideoId").and_then(|v| v.as_str().map(String::from)),
            marker_type: markers_list.and_then(|m| m.get("markerType")).and_then(|v| v.as_str().map(String::from)),
            markers,
            max_height_dp: heatmap_metadata.and_then(|m| m.get("maxHeightDp")).and_then(|v| v.as_i64()).or(Some(40)),
            min_height_dp: heatmap_metadata.and_then(|m| m.get("minHeightDp")).and_then(|v| v.as_i64()).or(Some(4)),
            show_hide_animation_duration_millis: heatmap_metadata.and_then(|m| m.get("showHideAnimationDurationMillis")).and_then(|v| v.as_i64()).or(Some(200)),
            timed_marker_decorations,
        })
    }
}

/// Strongly typed `MenuTitle` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuTitleNode {
    pub title: Option<TextNode>,
}

impl MenuTitleNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("menuTitleRenderer").or_else(|| val.get("menuTitle")).unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `MerchandiseItem` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchandiseItemNode {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub price: Option<String>,
    pub vendor_name: Option<String>,
    pub button_text: Option<String>,
    pub button_accessibility_text: Option<String>,
    pub from_vendor_text: Option<String>,
    pub additional_fees_text: Option<String>,
    pub region_format: Option<String>,
    pub endpoint: Option<Value>,
}

impl MerchandiseItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("merchandiseItemRenderer").or_else(|| val.get("merchandiseItem")).unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(|v| v.as_str().map(String::from)),
            description: node.get("description").and_then(|v| v.as_str().map(String::from)),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            price: node.get("price").and_then(|v| v.as_str().map(String::from)),
            vendor_name: node.get("vendorName").and_then(|v| v.as_str().map(String::from)),
            button_text: node.get("buttonText").and_then(|v| v.as_str().map(String::from)),
            button_accessibility_text: node.get("buttonAccessibilityText").and_then(|v| v.as_str().map(String::from)),
            from_vendor_text: node.get("fromVendorText").and_then(|v| v.as_str().map(String::from)),
            additional_fees_text: node.get("additionalFeesText").and_then(|v| v.as_str().map(String::from)),
            region_format: node.get("regionFormat").and_then(|v| v.as_str().map(String::from)),
            endpoint: node.get("buttonCommand").cloned(),
        })
    }
}

/// Strongly typed `MerchandiseShelf` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchandiseShelfNode {
    pub title: Option<String>,
    pub menu: Option<Value>,
    pub items: Vec<Value>,
}

impl MerchandiseShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("merchandiseShelfRenderer").or_else(|| val.get("merchandiseShelf")).unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(|v| v.as_str().map(String::from)),
            menu: node.get("actionButton").cloned(),
            items: node.get("items").and_then(|v| v.as_array()).cloned().unwrap_or_default(),
        })
    }
}

/// Strongly typed `Message` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageNode {
    pub text: Option<TextNode>,
}

impl MessageNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("messageRenderer").or_else(|| val.get("message")).unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `MetadataRow` AST node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataRowNode {
    pub title: Option<TextNode>,
    pub contents: Vec<TextNode>,
}

impl MetadataRowNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("metadataRowRenderer").or_else(|| val.get("metadataRow")).unwrap_or(val);
        let contents = node.get("contents")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(TextNode::from_value).collect())
            .unwrap_or_default();
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            contents,
        })
    }
}
