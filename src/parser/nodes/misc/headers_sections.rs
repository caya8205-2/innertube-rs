use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed `FlexibleActionsView` AST node (`flexibleActionsView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleActionsViewNode {
    pub actions_rows: Vec<ActionRow>,
    pub style: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRow {
    pub actions: Vec<Value>,
}

impl FlexibleActionsViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("flexibleActionsView").unwrap_or(val);
        let actions_rows = node
            .get("actionsRows")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|row| {
                        let actions = row
                            .get("actions")
                            .and_then(|a| a.as_array())
                            .cloned()
                            .unwrap_or_default();
                        ActionRow { actions }
                    })
                    .collect()
            })
            .unwrap_or_default();
        let style = node
            .get("style")
            .and_then(|v| v.as_str().map(String::from));

        Some(Self {
            actions_rows,
            style,
        })
    }
}

/// Strongly typed `Form` AST node (`form`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormNode {
    pub fields: Vec<Value>,
}

impl FormNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("form").unwrap_or(val);
        let fields = node
            .get("fields")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        Some(Self { fields })
    }
}

/// Strongly typed `FormFooterView` AST node (`formFooterView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormFooterViewNode {
    pub panel_footer: Option<Value>,
    pub form_id: Option<String>,
    pub container_type: Option<String>,
}

impl FormFooterViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("formFooterView").unwrap_or(val);
        Some(Self {
            panel_footer: node.get("panelFooter").cloned(),
            form_id: node.get("formId").and_then(|v| v.as_str().map(String::from)),
            container_type: node.get("containerType").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed `FormPopup` AST node (`formPopup`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormPopupNode {
    pub title: Option<TextNode>,
    pub form: Option<Value>,
    pub buttons: Vec<Value>,
}

impl FormPopupNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("formPopup").unwrap_or(val);
        let title = TextNode::from_value(&node.get("title").cloned().unwrap_or_default());
        let form = node.get("form").cloned();
        let buttons = node
            .get("buttons")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        Some(Self {
            title,
            form,
            buttons,
        })
    }
}

/// Strongly typed `GameCard` AST node (`gameCard`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCardNode {
    pub game: Option<Value>,
}

impl GameCardNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gameCard").unwrap_or(val);
        Some(Self {
            game: node.get("game").cloned(),
        })
    }
}

/// Strongly typed `GameDetails` AST node (`gameDetails`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDetailsNode {
    pub title: Option<TextNode>,
    pub box_art: ThumbnailListNode,
    pub box_art_overlay_text: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub is_official_box_art: bool,
}

impl GameDetailsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gameDetails").unwrap_or(val);
        let title = TextNode::from_value(&node.get("title").cloned().unwrap_or_default());
        let box_art = ThumbnailListNode::from_value(&node.get("boxArt").cloned().unwrap_or_default());
        let box_art_overlay_text = TextNode::from_value(&node.get("boxArtOverlayText").cloned().unwrap_or_default());
        let endpoint = node.get("endpoint").cloned();
        let is_official_box_art = node.get("isOfficialBoxArt").and_then(|v| v.as_bool()).unwrap_or(false);

        Some(Self {
            title,
            box_art,
            box_art_overlay_text,
            endpoint,
            is_official_box_art,
        })
    }
}

/// Strongly typed `Grid` AST node (`grid`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridNode {
    pub items: Vec<Value>,
    pub is_collapsible: Option<bool>,
    pub visible_row_count: Option<String>,
    pub target_id: Option<String>,
    pub continuation: Option<String>,
    pub header: Option<Value>,
}

impl GridNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridRenderer").or_else(|| val.get("grid")).unwrap_or(val);
        
        let items = node
            .get("items")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        
        let is_collapsible = node.get("isCollapsible").and_then(|v| v.as_bool());
        let visible_row_count = node.get("visibleRowCount").and_then(|v| v.as_str().map(String::from));
        let target_id = node.get("targetId").and_then(|v| v.as_str().map(String::from));
        
        let continuation = node
            .get("continuations")
            .and_then(|c| c.as_array())
            .and_then(|c| c.first())
            .and_then(|c| c.get("nextContinuationData"))
            .and_then(|n| n.get("continuation"))
            .and_then(|v| v.as_str().map(String::from));
            
        let header = node.get("header").cloned();

        Some(Self {
            items,
            is_collapsible,
            visible_row_count,
            target_id,
            continuation,
            header,
        })
    }
}

/// Strongly typed `GridHeader` AST node (`gridHeader`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridHeaderNode {
    pub title: Option<TextNode>,
}

impl GridHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridHeaderRenderer").or_else(|| val.get("gridHeader")).unwrap_or(val);
        Some(Self {
            title: TextNode::from_value(&node.get("title").cloned().unwrap_or_default()),
        })
    }
}

/// Strongly typed `GridShelfView` AST node (`gridShelfView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridShelfViewNode {
    pub contents: Vec<Value>,
    pub header: Option<Value>,
    pub content_aspect_ratio: Option<String>,
    pub enable_vertical_expansion: bool,
    pub show_more_button: Option<Value>,
    pub show_less_button: Option<Value>,
    pub min_collapsed_item_count: Option<f64>,
}

impl GridShelfViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("gridShelfView").unwrap_or(val);
        
        let contents = node
            .get("contents")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
            
        Some(Self {
            contents,
            header: node.get("header").cloned(),
            content_aspect_ratio: node.get("contentAspectRatio").and_then(|v| v.as_str().map(String::from)),
            enable_vertical_expansion: node.get("enableVerticalExpansion").and_then(|v| v.as_bool()).unwrap_or(false),
            show_more_button: node.get("showMoreButton").cloned(),
            show_less_button: node.get("showLessButton").cloned(),
            min_collapsed_item_count: node.get("minCollapsedItemCount").and_then(|v| v.as_f64()),
        })
    }
}

/// Strongly typed `GuideCollapsibleEntry` AST node (`guideCollapsibleEntry`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideCollapsibleEntryNode {
    pub expander_item: Option<Value>,
    pub collapser_item: Option<Value>,
    pub expandable_items: Vec<Value>,
}

impl GuideCollapsibleEntryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("guideCollapsibleEntryRenderer").or_else(|| val.get("guideCollapsibleEntry")).unwrap_or(val);
        
        let expandable_items = node
            .get("expandableItems")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        Some(Self {
            expander_item: node.get("expanderItem").cloned(),
            collapser_item: node.get("collapserItem").cloned(),
            expandable_items,
        })
    }
}

/// Strongly typed `GuideCollapsibleSectionEntry` AST node (`guideCollapsibleSectionEntry`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideCollapsibleSectionEntryNode {
    pub header_entry: Option<Value>,
    pub expander_icon: Option<String>,
    pub collapser_icon: Option<String>,
    pub section_items: Vec<Value>,
}

impl GuideCollapsibleSectionEntryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("guideCollapsibleSectionEntryRenderer").or_else(|| val.get("guideCollapsibleSectionEntry")).unwrap_or(val);
        
        let section_items = node
            .get("sectionItems")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        Some(Self {
            header_entry: node.get("headerEntry").cloned(),
            expander_icon: node.get("expanderIcon").and_then(|i| i.get("iconType")).and_then(|v| v.as_str().map(String::from)),
            collapser_icon: node.get("collapserIcon").and_then(|i| i.get("iconType")).and_then(|v| v.as_str().map(String::from)),
            section_items,
        })
    }
}

/// Strongly typed `GuideDownloadsEntry` AST node (`guideDownloadsEntry`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideDownloadsEntryNode {
    pub always_show: bool,
    pub guide_entry: Option<Value>,
}

impl GuideDownloadsEntryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("guideDownloadsEntryRenderer").or_else(|| val.get("guideDownloadsEntry")).unwrap_or(val);
        
        Some(Self {
            always_show: node.get("alwaysShow").and_then(|v| v.as_bool()).unwrap_or(false),
            guide_entry: node.get("entryRenderer").and_then(|v| v.get("guideEntryRenderer")).cloned(),
        })
    }
}
