use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed ShareEntityServiceEndpoint AST node (`shareEntityServiceEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareEntityServiceEndpointNode {
    pub serialized_share_entity: Option<String>,
    pub commands: Option<Vec<Value>>,
}

impl ShareEntityServiceEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("shareEntityServiceEndpoint").unwrap_or(val);
        Some(Self {
            serialized_share_entity: node
                .get("serializedShareEntity")
                .and_then(|v| v.as_str().map(String::from)),
            commands: node.get("commands").and_then(|v| v.as_array().cloned()),
        })
    }
}

/// Strongly typed SignalServiceEndpoint AST node (`signalServiceEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalServiceEndpointNode {
    pub signal: Option<String>,
    pub actions: Option<Vec<Value>>,
}

impl SignalServiceEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("signalServiceEndpoint").unwrap_or(val);
        Some(Self {
            signal: node.get("signal").and_then(|v| v.as_str().map(String::from)),
            actions: node.get("actions").and_then(|v| v.as_array().cloned()),
        })
    }
}

/// Strongly typed UnsubscribeEndpoint AST node (`unsubscribeEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeEndpointNode {
    pub channel_ids: Option<Vec<String>>,
    pub params: Option<String>,
}

impl UnsubscribeEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("unsubscribeEndpoint").unwrap_or(val);
        Some(Self {
            channel_ids: node.get("channelIds").and_then(|v| v.as_array()).map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            }),
            params: node.get("params").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed WatchNextEndpoint AST node (`watchNextEndpoint`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextEndpointNode {
    pub video_id: Option<String>,
}

impl WatchNextEndpointNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("watchNextEndpoint").unwrap_or(val);
        Some(Self {
            video_id: node.get("videoId").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed AccessibilityId AST node for AccessibilityData.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityIdNode {
    pub accessibility_id_type: Option<String>,
}

/// Strongly typed AccessibilityData AST node (`accessibilityData`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityDataNode {
    pub accessibility_identifier: Option<String>,
    pub identifier: Option<AccessibilityIdNode>,
    pub label: Option<String>,
}

impl AccessibilityDataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accessibilityData").unwrap_or(val);
        let identifier = node.get("identifier").map(|id_node| AccessibilityIdNode {
            accessibility_id_type: id_node
                .get("accessibilityIdType")
                .and_then(|v| v.as_str().map(String::from)),
        });

        Some(Self {
            accessibility_identifier: node
                .get("accessibilityIdentifier")
                .and_then(|v| v.as_str().map(String::from)),
            identifier,
            label: node.get("label").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed AccessibilityContext AST node (`accessibilityContext`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityContextNode {
    pub context_data: Option<Value>,
}

impl AccessibilityContextNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("accessibilityContext").unwrap_or(val);
        Some(Self {
            context_data: node.get("contextData").cloned(),
        })
    }
}

/// Strongly typed ChildElement AST node (`childElement`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildElementNode {
    pub text: Option<String>,
    pub properties: Option<Value>,
    pub child_elements: Option<Vec<ChildElementNode>>,
}

impl ChildElementNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("childElement").unwrap_or(val);
        let text = node
            .get("type")
            .and_then(|v| v.get("textType"))
            .and_then(|v| v.get("text"))
            .and_then(|v| v.get("content"))
            .and_then(|v| v.as_str().map(String::from));

        let child_elements = node.get("childElements").and_then(|v| v.as_array()).map(|arr| {
            arr.iter().filter_map(ChildElementNode::from_value).collect()
        });

        Some(Self {
            text,
            properties: node.get("properties").cloned(),
            child_elements,
        })
    }
}

/// Strongly typed Emoji AST node for EmojiRun.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiNode {
    pub emoji_id: Option<String>,
    pub shortcuts: Option<Vec<String>>,
    pub search_terms: Option<Vec<String>>,
    pub image: ThumbnailListNode,
    pub is_custom: bool,
}

/// Strongly typed EmojiRun AST node (`emojiRun`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiRunNode {
    pub text: Option<String>,
    pub emoji: Option<EmojiNode>,
}

impl EmojiRunNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("emojiRun").unwrap_or(val);
        let emoji_node = node.get("emoji");
        let text = emoji_node
            .and_then(|e| e.get("emojiId").and_then(|v| v.as_str().map(String::from)))
            .or_else(|| {
                emoji_node
                    .and_then(|e| e.get("shortcuts").and_then(|v| v.as_array()))
                    .and_then(|a| a.first().and_then(|s| s.as_str().map(String::from)))
            })
            .or_else(|| node.get("text").and_then(|v| v.as_str().map(String::from)))
            .unwrap_or_default();

        let emoji = emoji_node.map(|e| EmojiNode {
            emoji_id: e.get("emojiId").and_then(|v| v.as_str().map(String::from)),
            shortcuts: e.get("shortcuts").and_then(|v| v.as_array()).map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            }),
            search_terms: e.get("searchTerms").and_then(|v| v.as_array()).map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            }),
            image: ThumbnailListNode::from_value(e.get("image").unwrap_or(&Value::Null)),
            is_custom: e.get("isCustomEmoji").and_then(|v| v.as_bool()).unwrap_or(false),
        });

        Some(Self {
            text: if text.is_empty() { None } else { Some(text) },
            emoji,
        })
    }
}

/// Strongly typed RendererContext AST node (`rendererContext`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RendererContextNode {
    pub command_context: Option<Value>,
    pub accessibility_context: Option<AccessibilityContextNode>,
}

impl RendererContextNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("rendererContext").unwrap_or(val);
        Some(Self {
            command_context: node.get("commandContext").cloned(),
            accessibility_context: node
                .get("accessibilityContext")
                .and_then(AccessibilityContextNode::from_value),
        })
    }
}

/// Strongly typed CategoryAssets AST node for AnchoredSection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryAssetsNode {
    pub asset_key: Option<String>,
    pub background_color: Option<String>,
}

/// Strongly typed AnchoredSection AST node (`anchoredSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnchoredSectionNode {
    pub title: Option<String>,
    pub content: Option<Value>,
    pub endpoint: Option<Value>,
    pub category_assets: Option<CategoryAssetsNode>,
    pub category_type: Option<String>,
}

impl AnchoredSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("anchoredSectionRenderer").unwrap_or(val);
        let category_assets = node.get("categoryAssets").map(|ca| CategoryAssetsNode {
            asset_key: ca.get("assetKey").and_then(|v| v.as_str().map(String::from)),
            background_color: ca
                .get("backgroundColor")
                .and_then(|v| v.as_str().map(String::from)),
        });

        Some(Self {
            title: node.get("title").and_then(|v| v.as_str().map(String::from)),
            content: node.get("content").cloned(),
            endpoint: node.get("navigationEndpoint").cloned(),
            category_assets,
            category_type: node.get("categoryType").and_then(|v| v.as_str().map(String::from)),
        })
    }
}

/// Strongly typed KidsBlocklistPicker AST node (`kidsBlocklistPickerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsBlocklistPickerNode {
    pub title: Option<TextNode>,
    pub child_rows: Option<Vec<Value>>,
    pub done_button: Option<Value>,
    pub successful_toast_action_message: Option<TextNode>,
}

impl KidsBlocklistPickerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("kidsBlocklistPickerRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            child_rows: node.get("childRows").and_then(|v| v.as_array().cloned()),
            done_button: node.get("doneButton").cloned(),
            successful_toast_action_message: node
                .get("successfulToastActionMessage")
                .and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed KidsBlocklistPickerItem AST node (`kidsBlocklistPickerItemRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsBlocklistPickerItemNode {
    pub child_display_name: Option<TextNode>,
    pub child_account_description: Option<TextNode>,
    pub avatar: ThumbnailListNode,
    pub block_button: Option<Value>,
    pub blocked_entity_key: Option<String>,
}

impl KidsBlocklistPickerItemNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("kidsBlocklistPickerItemRenderer").unwrap_or(val);
        Some(Self {
            child_display_name: node.get("childDisplayName").and_then(TextNode::from_value),
            child_account_description: node
                .get("childAccountDescription")
                .and_then(TextNode::from_value),
            avatar: ThumbnailListNode::from_value(node.get("avatar").unwrap_or(&Value::Null)),
            block_button: node.get("blockButton").cloned(),
            blocked_entity_key: node
                .get("blockedEntityKey")
                .and_then(|v| v.as_str().map(String::from)),
        })
    }
}
