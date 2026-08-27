use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;

/// Strongly typed MenuFlexibleItem AST node (`menuFlexibleItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuFlexibleItemNode {
    pub menu_item: Option<Value>,
    pub top_level_button: Option<Value>,
}

impl MenuFlexibleItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("menuFlexibleItemRenderer").unwrap_or(val);
        Some(Self {
            menu_item: node.get("menuItem").cloned(),
            top_level_button: node.get("topLevelButton").cloned(),
        })
    }
}

/// Strongly typed MenuNavigationItem AST node (`menuNavigationItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuNavigationItemNode {
    pub text: Option<TextNode>,
    pub icon: Option<Value>,
    pub navigation_endpoint: Option<Value>,
}

impl MenuNavigationItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("menuNavigationItemRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            icon: node.get("icon").cloned(),
            navigation_endpoint: node.get("navigationEndpoint").cloned(),
        })
    }
}

/// Strongly typed MenuPopup AST node (`menuPopupRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuPopupNode {
    pub items: Vec<Value>,
}

impl MenuPopupNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("menuPopupRenderer").unwrap_or(val);
        Some(Self {
            items: node
                .get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MenuServiceItem AST node (`menuServiceItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuServiceItemNode {
    pub text: Option<TextNode>,
    pub icon: Option<Value>,
    pub service_endpoint: Option<Value>,
    pub navigation_endpoint: Option<Value>,
}

impl MenuServiceItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("menuServiceItemRenderer").unwrap_or(val);
        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
            icon: node.get("icon").cloned(),
            service_endpoint: node.get("serviceEndpoint").cloned(),
            navigation_endpoint: node.get("navigationEndpoint").cloned(),
        })
    }
}

/// Strongly typed MenuServiceItemDownload AST node (`menuServiceItemDownloadRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuServiceItemDownloadNode {
    pub has_separator: bool,
    pub endpoint: Option<Value>,
}

impl MenuServiceItemDownloadNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("menuServiceItemDownloadRenderer").unwrap_or(val);
        Some(Self {
            has_separator: node
                .get("hasSeparator")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            endpoint: node
                .get("navigationEndpoint")
                .or(node.get("serviceEndpoint"))
                .cloned(),
        })
    }
}

/// Strongly typed MultiPageMenu AST node (`multiPageMenuRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiPageMenuNode {
    pub header: Option<Value>,
    pub sections: Vec<Value>,
    pub style: Option<String>,
}

impl MultiPageMenuNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("multiPageMenuRenderer").unwrap_or(val);
        Some(Self {
            header: node.get("header").cloned(),
            sections: node
                .get("sections")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            style: node.get("style").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed MultiPageMenuNotificationSection AST node (`multiPageMenuNotificationSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiPageMenuNotificationSectionNode {
    pub notification_section_title: Option<TextNode>,
    pub items: Vec<Value>,
}

impl MultiPageMenuNotificationSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("multiPageMenuNotificationSectionRenderer").unwrap_or(val);
        Some(Self {
            notification_section_title: node
                .get("notificationSectionTitle")
                .and_then(TextNode::from_value),
            items: node
                .get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed SimpleMenuHeader AST node (`simpleMenuHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleMenuHeaderNode {
    pub title: Option<TextNode>,
    pub buttons: Vec<Value>,
}

impl SimpleMenuHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("simpleMenuHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            buttons: node
                .get("buttons")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed MobileTopbar AST node (`mobileTopbarRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileTopbarNode {
    pub placeholder_text: Option<TextNode>,
    pub buttons: Vec<Value>,
    pub logo_type: Option<String>,
}

impl MobileTopbarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("mobileTopbarRenderer").unwrap_or(val);
        let logo_type = node
            .get("logo")
            .and_then(|l| l.get("iconType"))
            .and_then(|t| t.as_str().map(String::from));

        Some(Self {
            placeholder_text: node.get("placeholderText").and_then(TextNode::from_value),
            buttons: node
                .get("buttons")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            logo_type,
        })
    }
}

/// Strongly typed MultiPageMenuSection AST node (`multiPageMenuSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiPageMenuSectionNode {
    pub items: Vec<Value>,
}

impl MultiPageMenuSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("multiPageMenuSectionRenderer").unwrap_or(val);
        Some(Self {
            items: node
                .get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed PivotBar AST node (`pivotBarRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotBarNode {
    pub items: Vec<Value>,
}

impl PivotBarNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pivotBarRenderer").unwrap_or(val);
        Some(Self {
            items: node
                .get("items")
                .and_then(|v| v.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed PivotBarItem AST node (`pivotBarItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotBarItemNode {
    pub pivot_identifier: Option<String>,
    pub endpoint: Option<Value>,
    pub title: Option<TextNode>,
    pub accessibility_label: Option<String>,
    pub icon_type: Option<String>,
    pub accessibility: Option<Value>,
}

impl PivotBarItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("pivotBarItemRenderer").unwrap_or(val);

        let accessibility_label = node
            .get("accessibility")
            .and_then(|a| a.get("accessibilityData"))
            .and_then(|ad| ad.get("label"))
            .and_then(|l| l.as_str().map(String::from));

        let icon_type = node
            .get("icon")
            .and_then(|i| i.get("iconType"))
            .and_then(|t| t.as_str().map(String::from));

        Some(Self {
            pivot_identifier: node
                .get("pivotIdentifier")
                .and_then(|v| v.as_str().map(String::from)),
            endpoint: node.get("navigationEndpoint").cloned(),
            title: node.get("title").and_then(TextNode::from_value),
            accessibility_label,
            icon_type,
            accessibility: node.get("accessibility").cloned(),
        })
    }
}

/// Strongly typed TopbarMenuButton AST node (`topbarMenuButtonRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopbarMenuButtonNode {
    pub icon_type: Option<String>,
    pub menu_renderer: Option<Value>,
    pub target_id: Option<String>,
}

impl TopbarMenuButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("topbarMenuButtonRenderer").unwrap_or(val);

        let icon_type = node
            .get("icon")
            .and_then(|i| i.get("iconType"))
            .and_then(|t| t.as_str().map(String::from));

        Some(Self {
            icon_type,
            menu_renderer: node.get("menuRenderer").cloned(),
            target_id: node.get("targetId").and_then(|v| v.as_str().map(String::from)),
        })
    }
}
