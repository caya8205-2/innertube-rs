use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};

/// Request an InnerTube attestation challenge for a client engagement flow.
///
/// The response schema varies by engagement type, so it is intentionally kept
/// as JSON until the corresponding BotGuard parser is ported.
pub async fn get_attestation_challenge(
    session: &Session,
    engagement_type: &str,
    ids: Option<Value>,
) -> Result<Value> {
    let payload = build_attestation_payload(engagement_type, ids);
    let response = session.post_innertube("/att/get", payload).await?;
    response.json().await.map_err(InnertubeError::Network)
}

pub(crate) fn build_attestation_payload(engagement_type: &str, ids: Option<Value>) -> Value {
    let mut payload = json!({ "engagementType": engagement_type });
    if let Some(ids) = ids {
        payload["ids"] = ids;
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_optional_attestation_ids() {
        let payload =
            build_attestation_payload("ENGAGEMENT_TYPE_VIDEO", Some(json!([{"id": "abc"}])));
        assert_eq!(payload["engagementType"], "ENGAGEMENT_TYPE_VIDEO");
        assert_eq!(payload["ids"][0]["id"], "abc");
    }
}
