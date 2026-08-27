use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed DefaultPromoPanel AST node (`defaultPromoPanelRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultPromoPanelNode {
    pub title: Option<TextNode>,
    pub description: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub large_form_factor_background_thumbnail: Option<Value>,
    pub small_form_factor_background_thumbnail: Option<Value>,
    pub scrim_color_values: Option<Vec<Value>>,
    pub min_panel_display_duration_ms: Option<u64>,
    pub min_video_play_duration_ms: Option<u64>,
    pub scrim_duration: Option<u64>,
    pub metadata_order: Option<String>,
    pub panel_layout: Option<String>,
}

impl DefaultPromoPanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("defaultPromoPanelRenderer").unwrap_or(val);
        Some(Self {
            title: TextNode::from_value(&node["title"]),
            description: TextNode::from_value(&node["description"]),
            endpoint: node["navigationEndpoint"].clone().into(),
            large_form_factor_background_thumbnail: node["largeFormFactorBackgroundThumbnail"].clone().into(),
            small_form_factor_background_thumbnail: node["smallFormFactorBackgroundThumbnail"].clone().into(),
            scrim_color_values: node["scrimColorValues"].as_array().cloned(),
            min_panel_display_duration_ms: node["minPanelDisplayDurationMs"].as_u64(),
            min_video_play_duration_ms: node["minVideoPlayDurationMs"].as_u64(),
            scrim_duration: node["scrimDuration"].as_u64(),
            metadata_order: node["metadataOrder"].as_str().map(String::from),
            panel_layout: node["panelLayout"].as_str().map(String::from),
        })
    }
}

/// Strongly typed DescriptionPreviewView AST node (`descriptionPreviewViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionPreviewViewNode {
    pub description: Option<TextNode>,
    pub max_lines: Option<u64>,
    pub truncation_text: Option<TextNode>,
    pub always_show_truncation_text: bool,
    pub more_endpoint: Option<Value>,
    pub renderer_context: Option<Value>,
}

impl DescriptionPreviewViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("descriptionPreviewViewRenderer").unwrap_or(val);
        Some(Self {
            description: TextNode::from_value(&node["description"]),
            max_lines: node["maxLines"].as_str().and_then(|s| s.parse().ok()).or_else(|| node["maxLines"].as_u64()),
            truncation_text: TextNode::from_value(&node["truncationText"]),
            always_show_truncation_text: node["alwaysShowTruncationText"].as_bool().unwrap_or(false),
            more_endpoint: node["rendererContext"]["commandContext"]["onTap"]["innertubeCommand"]["showEngagementPanelEndpoint"].clone().into(),
            renderer_context: node["rendererContext"].clone().into(),
        })
    }
}

/// Strongly typed DialogHeaderView AST node (`dialogHeaderViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogHeaderViewNode {
    pub headline: Option<TextNode>,
}

impl DialogHeaderViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dialogHeaderViewRenderer").unwrap_or(val);
        Some(Self {
            headline: TextNode::from_value(&node["headline"]),
        })
    }
}

/// Strongly typed DialogView AST node (`dialogViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogViewNode {
    pub header: Option<Value>,
    pub footer: Option<Value>,
    pub custom_content: Option<Value>,
}

impl DialogViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dialogViewRenderer").unwrap_or(val);
        Some(Self {
            header: node["header"].clone().into(),
            footer: node["footer"].clone().into(),
            custom_content: node["customContent"].clone().into(),
        })
    }
}

/// Strongly typed DislikeButtonView AST node (`dislikeButtonViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DislikeButtonViewNode {
    pub toggle_button: Option<Value>,
    pub dislike_entity_key: Option<String>,
}

impl DislikeButtonViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dislikeButtonViewRenderer").unwrap_or(val);
        Some(Self {
            toggle_button: node["toggleButtonViewModel"].clone().into(),
            dislike_entity_key: node["dislikeEntityKey"].as_str().map(String::from),
        })
    }
}

/// Strongly typed DismissableDialog AST node (`dismissableDialogRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DismissableDialogNode {
    pub title: Option<String>,
    pub sections: Vec<Value>,
    pub metadata: Option<Value>,
    pub display_style: Option<String>,
}

impl DismissableDialogNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dismissableDialogRenderer").unwrap_or(val);
        let sections = node["sections"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        Some(Self {
            title: node["title"].as_str().map(String::from),
            sections,
            metadata: node["metadata"].clone().into(),
            display_style: node["displayStyle"].as_str().map(String::from),
        })
    }
}

/// Strongly typed DismissableDialogContentSection AST node (`dismissableDialogContentSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DismissableDialogContentSectionNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
}

impl DismissableDialogContentSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dismissableDialogContentSectionRenderer").unwrap_or(val);
        Some(Self {
            title: TextNode::from_value(&node["title"]),
            subtitle: TextNode::from_value(&node["subtitle"]),
        })
    }
}

/// Strongly typed DownloadButton AST node (`downloadButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadButtonNode {
    pub style: Option<String>,
    pub size: Option<String>,
    pub endpoint: Option<Value>,
    pub target_id: Option<String>,
}

impl DownloadButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("downloadButtonRenderer").unwrap_or(val);
        Some(Self {
            style: node["style"].as_str().map(String::from),
            size: node["size"].as_str().map(String::from),
            endpoint: node["command"].clone().into(),
            target_id: node["targetId"].as_str().map(String::from),
        })
    }
}

/// Strongly typed DownloadListItemView AST node (`downloadListItemViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadListItemViewNode {
    pub renderer_context: Option<Value>,
}

impl DownloadListItemViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("downloadListItemViewRenderer").unwrap_or(val);
        Some(Self {
            renderer_context: node["rendererContext"].clone().into(),
        })
    }
}

/// Strongly typed Dropdown AST node (`dropdownRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropdownNode {
    pub label: Option<String>,
    pub entries: Vec<Value>,
}

impl DropdownNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dropdownRenderer").unwrap_or(val);
        let entries = node["entries"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        Some(Self {
            label: node["label"].as_str().map(String::from),
            entries,
        })
    }
}

/// Strongly typed DropdownItem AST node (`dropdownItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropdownItemNode {
    pub label: Option<TextNode>,
    pub selected: bool,
    pub value: Option<Value>,
    pub icon_type: Option<String>,
    pub description: Option<TextNode>,
    pub endpoint: Option<Value>,
}

impl DropdownItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dropdownItemRenderer").unwrap_or(val);
        let value = if !node["int32Value"].is_null() {
            Some(node["int32Value"].clone())
        } else if !node["stringValue"].is_null() {
            Some(node["stringValue"].clone())
        } else {
            None
        };
        Some(Self {
            label: TextNode::from_value(&node["label"]),
            selected: node["isSelected"].as_bool().unwrap_or(false),
            value,
            icon_type: node["icon"]["iconType"].as_str().map(String::from),
            description: TextNode::from_value(&node["descriptionText"]),
            endpoint: node["onSelectCommand"].clone().into(),
        })
    }
}

/// Strongly typed DropdownView AST node (`dropdownViewRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropdownViewNode {
    pub label: Option<TextNode>,
    pub placeholder_text: Option<TextNode>,
    pub disabled: bool,
    pub options: Vec<Value>,
    pub dropdown_type: Option<String>,
    pub id: Option<String>,
}

impl DropdownViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("dropdownViewRenderer").unwrap_or(val);
        let options = node["options"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        Some(Self {
            label: TextNode::from_value(&node["label"]),
            placeholder_text: TextNode::from_value(&node["placeholderText"]),
            disabled: node["disabled"].as_bool().unwrap_or(false),
            options,
            dropdown_type: node["type"].as_str().map(String::from),
            id: node["id"].as_str().map(String::from),
        })
    }
}
