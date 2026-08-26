use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::text::TextNode;

/// Represents a tab in twoColumnBrowseResultsRenderer or browse results (`Tab.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TabNode {
    pub title: String,
    pub selected: bool,
    pub endpoint_params: Option<String>,
    pub content: Option<Value>,
}

impl TabNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("tabRenderer").unwrap_or(val);

        let title = target
            .get("title")
            .and_then(|t| t.as_str())
            .or_else(|| {
                target
                    .pointer("/title/runs/0/text")
                    .and_then(|t| t.as_str())
            })
            .unwrap_or("")
            .to_string();

        let selected = target
            .get("selected")
            .and_then(|s| s.as_bool())
            .unwrap_or(false);

        let endpoint_params = target
            .pointer("/endpoint/browseEndpoint/params")
            .and_then(|p| p.as_str())
            .map(|s| s.to_string());

        let content = target.get("content").cloned();

        Some(Self {
            title,
            selected,
            endpoint_params,
            content,
        })
    }
}

/// Represents a SectionList container (`SectionList.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SectionListNode {
    pub contents: Vec<Value>,
    pub continuation_token: Option<String>,
}

impl SectionListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("sectionListRenderer").unwrap_or(val);
        let contents = target
            .get("contents")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        let continuation_token = target
            .pointer("/continuations/0/nextContinuationData/continuation")
            .or_else(|| target.pointer("/contents/0/continuationItemRenderer/continuationEndpoint/continuationCommand/token"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        Some(Self {
            contents,
            continuation_token,
        })
    }
}

/// Represents an ItemSection container (`ItemSection.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ItemSectionNode {
    pub contents: Vec<Value>,
    pub target_id: Option<String>,
}

impl ItemSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("itemSectionRenderer").unwrap_or(val);
        let contents = target
            .get("contents")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        let target_id = target
            .get("targetId")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        Some(Self {
            contents,
            target_id,
        })
    }
}

/// Represents a RichGrid container (`RichGrid.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RichGridNode {
    pub contents: Vec<Value>,
    pub header: Option<Value>,
}

impl RichGridNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("richGridRenderer").unwrap_or(val);
        let contents = target
            .get("contents")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        let header = target.get("header").cloned();

        Some(Self { contents, header })
    }
}

/// Represents a Shelf container (`Shelf.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ShelfNode {
    pub title: String,
    pub contents: Vec<Value>,
}

impl ShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("shelfRenderer").unwrap_or(val);
        let title = TextNode::from_value(target.get("title").unwrap_or(&Value::Null))
            .map(|t| t.text)
            .unwrap_or_default();

        let contents = target
            .pointer("/content/verticalListRenderer/items")
            .or_else(|| target.pointer("/content/horizontalListRenderer/items"))
            .or_else(|| target.pointer("/content/gridRenderer/items"))
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        Some(Self { title, contents })
    }
}

/// Represents a RichShelf container (`RichShelf.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RichShelfNode {
    pub title: String,
    pub contents: Vec<Value>,
}

impl RichShelfNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("richShelfRenderer").unwrap_or(val);
        let title = TextNode::from_value(target.get("title").unwrap_or(&Value::Null))
            .map(|t| t.text)
            .unwrap_or_default();

        let contents = target
            .get("contents")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        Some(Self { title, contents })
    }
}

/// An individual chip cloud filter button (`ChipCloudChipRenderer.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChipCloudChipNode {
    pub text: String,
    pub is_selected: bool,
    pub continuation_token: Option<String>,
}

impl ChipCloudChipNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("chipCloudChipRenderer").unwrap_or(val);
        let text = target
            .get("text")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let is_selected = target
            .get("isSelected")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let continuation_token = target
            .pointer("/navigationEndpoint/continuationEndpoint/continuationCommand/token")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            text,
            is_selected,
            continuation_token,
        })
    }
}

/// A container cloud of chip filters (`ChipCloud.ts` / `chipCloudRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ChipCloudNode {
    pub chips: Vec<ChipCloudChipNode>,
}

impl ChipCloudNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let target = val.get("chipCloudRenderer").unwrap_or(val);
        let chips_arr = target.get("chips").and_then(Value::as_array)?;

        let mut chips = Vec::new();
        for item in chips_arr {
            if let Some(chip) = ChipCloudChipNode::from_value(item) {
                chips.push(chip);
            }
        }

        Some(Self { chips })
    }
}

/// Strongly typed FeedFilterChipBar AST node (`feedFilterChipBarRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedFilterChipBarNode {
    pub contents: Vec<Value>,
}

impl FeedFilterChipBarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("feedFilterChipBarRenderer").unwrap_or(val);
        let contents = node
            .get("contents")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self { contents })
    }
}

