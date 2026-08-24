use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// Represents an Author or Channel owner (1:1 port of `src/parser/classes/misc/Author.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AuthorNode {
    pub id: Option<String>,
    pub name: String,
    pub url: Option<String>,
    pub thumbnails: ThumbnailListNode,
    pub is_verified: bool,
    pub is_verified_artist: bool,
}

impl AuthorNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        // Try direct TextNode parsing
        let text_node = TextNode::from_value(val);
        let name = text_node.as_ref().map(|t| t.text.clone()).unwrap_or_default();

        let id = val.pointer("/runs/0/navigationEndpoint/browseEndpoint/browseId")
            .or_else(|| val.pointer("/navigationEndpoint/browseEndpoint/browseId"))
            .or_else(|| val.pointer("/browseEndpoint/browseId"))
            .or_else(|| val.get("channelId"))
            .or_else(|| val.get("browseId"))
            .or_else(|| val.get("id"))
            .and_then(|id| id.as_str())
            .map(|s| s.to_string());

        let url = val.pointer("/runs/0/navigationEndpoint/browseEndpoint/canonicalBaseUrl")
            .or_else(|| val.pointer("/navigationEndpoint/browseEndpoint/canonicalBaseUrl"))
            .or_else(|| val.pointer("/canonicalBaseUrl"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let thumbnails = ThumbnailListNode::from_value(val);

        let is_verified = val.pointer("/ownerBadges")
            .or_else(|| val.pointer("/badges"))
            .and_then(|b| b.as_array())
            .map(|arr| {
                arr.iter().any(|badge| {
                    badge.pointer("/metadataBadgeRenderer/style").and_then(|s| s.as_str()) == Some("BADGE_STYLE_TYPE_VERIFIED")
                })
            })
            .unwrap_or(false);

        let is_verified_artist = val.pointer("/ownerBadges")
            .or_else(|| val.pointer("/badges"))
            .and_then(|b| b.as_array())
            .map(|arr| {
                arr.iter().any(|badge| {
                    badge.pointer("/metadataBadgeRenderer/style").and_then(|s| s.as_str()) == Some("BADGE_STYLE_TYPE_VERIFIED_ARTIST")
                })
            })
            .unwrap_or(false);

        if name.is_empty() && id.is_none() {
            return None;
        }

        Some(Self {
            id,
            name,
            url,
            thumbnails,
            is_verified,
            is_verified_artist,
        })
    }
}
