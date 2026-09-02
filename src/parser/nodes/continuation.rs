use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a continuation token (`ContinuationItem.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContinuationNode {
    pub token: String,
    pub endpoint_type: Option<String>,
}

impl ContinuationNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        if val.is_null() {
            return None;
        }

        let target = val.get("continuationItemRenderer")
            .or_else(|| val.get("continuationItemViewModel"))
            .unwrap_or(val);

        let token = target.pointer("/continuationEndpoint/continuationCommand/token")
            .or_else(|| target.pointer("/continuationEndpoint/command/token"))
            .or_else(|| target.pointer("/continuationEndpoint/browseContinuationEndpoint/continuation"))
            .or_else(|| target.pointer("/continuationEndpoint/nextContinuationData/continuation"))
            .or_else(|| target.pointer("/continuationEndpoint/searchContinuationEndpoint/continuation"))
            .or_else(|| target.pointer("/nextContinuationData/continuation"))
            .or_else(|| target.pointer("/nextRadioContinuationData/continuation"))
            .or_else(|| target.pointer("/reloadContinuationData/continuation"))
            .and_then(|t| t.as_str())?
            .to_string();

        let endpoint_type = target.pointer("/continuationEndpoint/continuationCommand/request")
            .and_then(|r| r.as_str())
            .map(|s| s.to_string());

        Some(Self {
            token,
            endpoint_type,
        })
    }
}
