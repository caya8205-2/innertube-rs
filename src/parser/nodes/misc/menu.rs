use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;

/// Represents a menu item with a navigation or service endpoint (`MenuServiceItem.ts` & `MenuNavigationItem.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MenuItemNode {
    pub text: String,
    pub endpoint: Option<NavigationEndpointNode>,
    pub icon_type: Option<String>,
}

impl MenuItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("menuServiceItemRenderer")
            .or_else(|| val.get("menuNavigationItemRenderer"))
            .or_else(|| val.get("menuServiceItemDownloadRenderer"))
            .unwrap_or(val);

        let text = target.get("text")
            .or_else(|| target.get("formattedText"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let endpoint = target.get("serviceEndpoint")
            .or_else(|| target.get("navigationEndpoint"))
            .or_else(|| target.get("command"))
            .and_then(NavigationEndpointNode::from_value);

        let icon_type = target.pointer("/icon/iconType")
            .and_then(Value::as_str)
            .map(|s| s.to_string());

        Some(Self {
            text,
            endpoint,
            icon_type,
        })
    }
}

/// Represents an action menu container (`Menu.ts` & `MenuPopup.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MenuNode {
    pub items: Vec<MenuItemNode>,
    pub top_level_buttons: Vec<Value>,
}

impl MenuNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("menuRenderer")
            .or_else(|| val.get("menuPopupRenderer"))
            .unwrap_or(val);

        let mut items = Vec::new();
        if let Some(items_arr) = target.get("items").and_then(Value::as_array) {
            for item in items_arr {
                if let Some(mi) = MenuItemNode::from_value(item) {
                    items.push(mi);
                }
            }
        }

        let top_level_buttons = target
            .get("topLevelButtons")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self {
            items,
            top_level_buttons,
        })
    }
}
