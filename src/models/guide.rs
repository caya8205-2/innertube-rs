use serde::{Deserialize, Serialize};

use crate::parser::nodes::misc::navigation::NavigationEndpointNode;

/// A single entry item in the YouTube guide sidebar (`GuideEntry.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GuideItem {
    pub title: String,
    pub endpoint: Option<NavigationEndpointNode>,
    pub icon_type: Option<String>,
    pub is_selected: bool,
}

/// A section in the YouTube guide sidebar (`GuideSection.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GuideSection {
    pub title: Option<String>,
    pub items: Vec<GuideItem>,
}

/// YouTube Guide response (`Guide.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GuideResponse {
    pub sections: Vec<GuideSection>,
}
