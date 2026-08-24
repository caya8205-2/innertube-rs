use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::text::TextNode;

/// Represents thumbnail time status overlay (`ThumbnailOverlayTimeStatus.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ThumbnailOverlayTimeStatusNode {
    pub text: String,
    pub style: Option<String>,
}

impl ThumbnailOverlayTimeStatusNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("thumbnailOverlayTimeStatusRenderer").unwrap_or(val);

        let text = target.get("text")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let style = target.get("style")
            .and_then(Value::as_str)
            .map(|s| s.to_string());

        Some(Self { text, style })
    }
}

/// Represents thumbnail playback progress bar overlay (`ThumbnailOverlayProgressBar.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ThumbnailOverlayProgressBarNode {
    pub percent_duration_watched: u32,
}

impl ThumbnailOverlayProgressBarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("thumbnailOverlayProgressBarRenderer")
            .or_else(|| val.get("thumbnailOverlayProgressBarViewModel"))
            .unwrap_or(val);

        let percent = target.get("percentDurationWatched")
            .and_then(Value::as_u64)
            .map(|p| p as u32)
            .unwrap_or(0);

        Some(Self {
            percent_duration_watched: percent,
        })
    }
}
