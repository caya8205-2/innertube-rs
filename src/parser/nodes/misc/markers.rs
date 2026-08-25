use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed Chapter AST node (`chapterRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterNode {
    pub title: String,
    pub time_range_start_millis: u64,
    pub thumbnails: ThumbnailListNode,
}

impl ChapterNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("chapterRenderer").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let time_range_start_millis = node
            .get("timeRangeStartMillis")
            .and_then(Value::as_u64)
            .or_else(|| node.get("timeRangeStartMillis").and_then(Value::as_str).and_then(|s| s.parse().ok()))
            .unwrap_or(0);

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(node));

        Some(Self {
            title,
            time_range_start_millis,
            thumbnails,
        })
    }
}

/// Strongly typed Heatmap AST node (`heatmapRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatmapNode {
    pub max_height_dp: Option<f64>,
    pub min_height_dp: Option<f64>,
    pub show_heatmap_on_seek: Option<bool>,
}

impl HeatmapNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("heatmapRenderer").unwrap_or(val);
        let max_height_dp = node.get("maxHeightDp").and_then(Value::as_f64);
        let min_height_dp = node.get("minHeightDp").and_then(Value::as_f64);
        let show_heatmap_on_seek = node.get("showHeatmapOnSeek").and_then(Value::as_bool);

        Some(Self {
            max_height_dp,
            min_height_dp,
            show_heatmap_on_seek,
        })
    }
}

/// Strongly typed MacroMarkersList AST node (`macroMarkersListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroMarkersListNode {
    pub title: Option<String>,
    pub contents: Vec<Value>,
}

impl MacroMarkersListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("macroMarkersListRenderer").unwrap_or(val);
        let title = node.get("title").and_then(TextNode::from_value).map(|t| t.text);
        let contents = node.get("contents").and_then(Value::as_array).cloned().unwrap_or_default();

        Some(Self { title, contents })
    }
}

/// Strongly typed MacroMarkersListItem AST node (`macroMarkersListItemRenderer`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroMarkersListItemNode {
    pub title: String,
    pub time_description: Option<String>,
    pub thumbnails: ThumbnailListNode,
}

impl MacroMarkersListItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("macroMarkersListItemRenderer").unwrap_or(val);

        let title = node
            .get("title")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .or_else(|| node.get("title").and_then(Value::as_str).map(ToString::to_string))
            .unwrap_or_default();

        let time_description = node
            .get("timeDescription")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let thumbnails = ThumbnailListNode::from_value(node.get("thumbnail").unwrap_or(node));

        Some(Self {
            title,
            time_description,
            thumbnails,
        })
    }
}
