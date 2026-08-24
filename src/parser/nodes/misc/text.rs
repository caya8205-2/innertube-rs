use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::navigation::NavigationEndpointNode;

/// Represents formatted text runs (1:1 port of `src/parser/classes/misc/Text.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextNode {
    pub text: String,
    pub runs: Vec<TextRunNode>,
    pub rtl: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextRunNode {
    pub text: String,
    pub bold: bool,
    pub italics: bool,
    pub strikethrough: bool,
    pub endpoint: Option<NavigationEndpointNode>,
}

impl TextNode {
    pub fn new(text: impl Into<String>) -> Self {
        let t = text.into();
        Self {
            text: t,
            runs: Vec::new(),
            rtl: false,
        }
    }

    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // Direct string
        if let Some(s) = val.as_str() {
            return Some(Self::new(s));
        }

        // runs array
        if let Some(runs_arr) = val.get("runs").and_then(|r| r.as_array()) {
            let mut full_text = String::new();
            let mut runs = Vec::new();

            for r in runs_arr {
                if let Some(text_str) = r.get("text").and_then(|t| t.as_str()) {
                    full_text.push_str(text_str);

                    let bold = r.get("bold").and_then(|b| b.as_bool()).unwrap_or(false);
                    let italics = r.get("italics").and_then(|i| i.as_bool()).unwrap_or(false);
                    let strikethrough = r.get("strikethrough").and_then(|s| s.as_bool()).unwrap_or(false);

                    let endpoint = r.get("navigationEndpoint")
                        .or_else(|| r.get("endpoint"))
                        .and_then(NavigationEndpointNode::from_value);

                    runs.push(TextRunNode {
                        text: text_str.to_string(),
                        bold,
                        italics,
                        strikethrough,
                        endpoint,
                    });
                }
            }

            return Some(Self {
                text: full_text,
                runs,
                rtl: val.get("rtl").and_then(|r| r.as_bool()).unwrap_or(false),
            });
        }

        // simpleText
        if let Some(st) = val.get("simpleText").and_then(|s| s.as_str()) {
            return Some(Self::new(st));
        }

        // content / dynamicTextViewModel (modern ViewModels)
        if let Some(content) = val.get("content").and_then(|c| c.as_str()) {
            return Some(Self::new(content));
        }

        if let Some(text_val) = val.pointer("/dynamicTextViewModel/text/content").and_then(|c| c.as_str()) {
            return Some(Self::new(text_val));
        }

        // accessibility data fallback
        if let Some(acc) = val.pointer("/accessibility/accessibilityData/label").and_then(|l| l.as_str()) {
            return Some(Self::new(acc));
        }

        None
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl std::fmt::Display for TextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_text_node_runs() {
        let val = json!({
            "runs": [
                { "text": "Hello ", "bold": true },
                { "text": "World!", "italics": true }
            ]
        });

        let node = TextNode::from_value(&val).unwrap();
        assert_eq!(node.text, "Hello World!");
        assert_eq!(node.runs.len(), 2);
        assert!(node.runs[0].bold);
        assert!(node.runs[1].italics);
    }

    #[test]
    fn test_text_node_simple_text() {
        let val = json!({
            "simpleText": "Never Gonna Give You Up"
        });

        let node = TextNode::from_value(&val).unwrap();
        assert_eq!(node.text, "Never Gonna Give You Up");
        assert_eq!(node.runs.len(), 0);
    }
}
