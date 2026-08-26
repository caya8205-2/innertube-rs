use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Strongly typed KidsCategoriesHeader AST node (`kidsCategoriesHeaderRenderer` / `kidsCategoryTabRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsCategoriesHeaderNode {
    pub category_tabs: Vec<Value>,
}

impl KidsCategoriesHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("kidsCategoriesHeaderRenderer")
            .or_else(|| val.get("kidsCategoryTabRenderer"))
            .unwrap_or(val);

        let category_tabs = node
            .get("categoryTabs")
            .or_else(|| node.get("tabs"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self { category_tabs })
    }
}

/// Strongly typed KidsHomeScreen AST node (`kidsHomeScreenRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsHomeScreenNode {
    pub anchors: Vec<Value>,
}

impl KidsHomeScreenNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("kidsHomeScreenRenderer").unwrap_or(val);
        let anchors = node.get("anchors").and_then(Value::as_array).cloned().unwrap_or_default();

        Some(Self { anchors })
    }
}
