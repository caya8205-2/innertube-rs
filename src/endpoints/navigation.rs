use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;

/// Resolve a YouTube URL into its InnerTube navigation endpoint.
pub async fn resolve_url(session: &Session, url: &str) -> Result<NavigationEndpointNode> {
    let response = session
        .post_innertube("/navigation/resolve_url", json!({ "url": url }))
        .await?;
    let raw: Value = response.json().await.map_err(InnertubeError::Network)?;
    parse_resolved_url(&raw)
}

/// Parse the endpoint returned by `/navigation/resolve_url`.
pub fn parse_resolved_url(raw: &Value) -> Result<NavigationEndpointNode> {
    raw.get("endpoint")
        .or_else(|| raw.pointer("/data/endpoint"))
        .and_then(NavigationEndpointNode::from_value)
        .ok_or_else(|| {
            InnertubeError::NotFound(
                "resolve_url response did not include a navigation endpoint".to_string(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resolved_watch_url() {
        let fixture = json!({
            "endpoint": {
                "watchEndpoint": { "videoId": "dQw4w9WgXcQ" }
            }
        });

        let endpoint = parse_resolved_url(&fixture).expect("fixture should parse");
        assert_eq!(
            endpoint.watch.expect("watch endpoint").video_id,
            "dQw4w9WgXcQ"
        );
    }
}
