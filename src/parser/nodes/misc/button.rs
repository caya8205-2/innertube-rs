use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;

/// Represents a standard InnerTube button (`Button.ts` & `ButtonView.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ButtonNode {
    pub text: String,
    pub endpoint: Option<NavigationEndpointNode>,
    pub icon_type: Option<String>,
    pub is_disabled: bool,
}

impl ButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("buttonRenderer")
            .or_else(|| val.get("buttonViewModel"))
            .unwrap_or(val);

        let text = target.get("text")
            .or_else(|| target.get("title"))
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let endpoint = target.get("navigationEndpoint")
            .or_else(|| target.get("command"))
            .or_else(|| target.get("onTap"))
            .and_then(NavigationEndpointNode::from_value);

        let icon_type = target.pointer("/icon/iconType")
            .or_else(|| target.pointer("/iconName"))
            .and_then(Value::as_str)
            .map(|s| s.to_string());

        let is_disabled = target.get("isDisabled")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        Some(Self {
            text,
            endpoint,
            icon_type,
            is_disabled,
        })
    }
}

/// Represents a toggle button (`ToggleButton.ts` & `ToggleButtonView.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ToggleButtonNode {
    pub is_toggled: bool,
    pub default_text: String,
    pub toggled_text: Option<String>,
    pub default_endpoint: Option<NavigationEndpointNode>,
    pub toggled_endpoint: Option<NavigationEndpointNode>,
}

impl ToggleButtonNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("toggleButtonRenderer")
            .or_else(|| val.get("toggleButtonViewModel"))
            .unwrap_or(val);

        let is_toggled = target.get("isToggled")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let default_text = target.get("defaultText")
            .and_then(TextNode::from_value)
            .map(|t| t.text)
            .unwrap_or_default();

        let toggled_text = target.get("toggledText")
            .and_then(TextNode::from_value)
            .map(|t| t.text);

        let default_endpoint = target.get("defaultEndpoint")
            .or_else(|| target.get("defaultServiceEndpoint"))
            .and_then(NavigationEndpointNode::from_value);

        let toggled_endpoint = target.get("toggledEndpoint")
            .or_else(|| target.get("toggledServiceEndpoint"))
            .and_then(NavigationEndpointNode::from_value);

        Some(Self {
            is_toggled,
            default_text,
            toggled_text,
            default_endpoint,
            toggled_endpoint,
        })
    }
}
