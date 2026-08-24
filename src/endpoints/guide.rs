use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::guide::{GuideItem, GuideResponse, GuideSection};
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::text::TextNode;

/// Fetch the YouTube Guide navigation menu (/guide endpoint).
pub async fn get_guide(session: &Session) -> Result<GuideResponse> {
    let payload = json!({});
    let resp = session.post_innertube("/guide", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_guide_response(&raw)
}

/// Parse `/guide` response into `GuideResponse`.
pub fn parse_guide_response(raw: &Value) -> Result<GuideResponse> {
    let mut sections = Vec::new();

    if let Some(items) = raw.get("items").and_then(|i| i.as_array()) {
        for section_val in items {
            let section_target = section_val.get("guideSectionRenderer")
                .or_else(|| section_val.get("guideSubscriptionsSectionRenderer"))
                .unwrap_or(section_val);

            let title = section_target.get("formattedTitle")
                .or_else(|| section_target.get("title"))
                .and_then(TextNode::from_value)
                .map(|t| t.text);

            let mut guide_items = Vec::new();

            if let Some(entries) = section_target.get("items").and_then(|i| i.as_array()) {
                for entry in entries {
                    if let Some(ger) = entry.get("guideEntryRenderer") {
                        let item_title = ger.get("formattedTitle")
                            .or_else(|| ger.get("title"))
                            .and_then(TextNode::from_value)
                            .map(|t| t.text)
                            .unwrap_or_default();

                        let endpoint = ger.get("navigationEndpoint")
                            .and_then(NavigationEndpointNode::from_value);

                        let icon_type = ger.pointer("/icon/iconType")
                            .and_then(|i| i.as_str())
                            .map(|s| s.to_string());

                        let is_selected = ger.get("isSelected").and_then(|s| s.as_bool()).unwrap_or(false);

                        if !item_title.is_empty() {
                            guide_items.push(GuideItem {
                                title: item_title,
                                endpoint,
                                icon_type,
                                is_selected,
                            });
                        }
                    }
                }
            }

            if !guide_items.is_empty() || title.is_some() {
                sections.push(GuideSection {
                    title,
                    items: guide_items,
                });
            }
        }
    }

    Ok(GuideResponse { sections })
}
