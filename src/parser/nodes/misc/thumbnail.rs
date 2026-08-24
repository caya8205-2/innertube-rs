use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a single image resolution (1:1 port of `src/parser/classes/misc/Thumbnail.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ThumbnailNode {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Represents a list of thumbnails sorted by resolution.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ThumbnailListNode {
    pub thumbnails: Vec<ThumbnailNode>,
}

impl ThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let url = val.get("url").and_then(|u| u.as_str())?.to_string();
        let width = val.get("width").and_then(|w| w.as_u64()).map(|w| w as u32);
        let height = val.get("height").and_then(|h| h.as_u64()).map(|h| h as u32);

        Some(Self { url, width, height })
    }
}

impl ThumbnailListNode {
    pub fn from_value(val: &Value) -> Self {
        let mut list = Vec::new();

        let raw_arr = val.get("thumbnails")
            .or_else(|| val.get("sources"))
            .or_else(|| val.pointer("/image/sources"))
            .or_else(|| val.pointer("/thumbnail/thumbnails"))
            .or_else(|| val.pointer("/thumbnail/sources"))
            .and_then(|a| a.as_array());

        if let Some(arr) = raw_arr {
            for item in arr {
                if let Some(thumb) = ThumbnailNode::from_value(item) {
                    list.push(thumb);
                }
            }
        } else if let Some(direct_arr) = val.as_array() {
            for item in direct_arr {
                if let Some(thumb) = ThumbnailNode::from_value(item) {
                    list.push(thumb);
                }
            }
        }

        // Sort largest width first (matching YouTube.js behavior)
        list.sort_by_key(|a| std::cmp::Reverse(a.width.unwrap_or(0)));

        Self { thumbnails: list }
    }

    pub fn best_url(&self) -> Option<&str> {
        self.thumbnails.first().map(|t| t.url.as_str())
    }

    pub fn first_url(&self) -> Option<&str> {
        self.thumbnails.last().map(|t| t.url.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.thumbnails.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_thumbnail_list_sorting() {
        let val = json!({
            "thumbnails": [
                { "url": "https://thumb_small.jpg", "width": 120, "height": 90 },
                { "url": "https://thumb_large.jpg", "width": 1280, "height": 720 },
                { "url": "https://thumb_med.jpg", "width": 480, "height": 360 }
            ]
        });

        let list = ThumbnailListNode::from_value(&val);
        assert_eq!(list.thumbnails.len(), 3);
        assert_eq!(list.best_url(), Some("https://thumb_large.jpg"));
        assert_eq!(list.first_url(), Some("https://thumb_small.jpg"));
    }
}
