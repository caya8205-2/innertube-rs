use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::live_chat::LiveChatResponse;
use crate::parser::nodes::livechat::LiveChatMessageNode;

/// Fetch a batch of live chat messages using a live chat continuation token.
pub async fn get_live_chat(session: &Session, continuation_token: &str) -> Result<LiveChatResponse> {
    let payload = json!({
        "continuation": continuation_token,
    });

    let resp = session.post_innertube("/live_chat/get_live_chat", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_live_chat_response(&raw)
}

/// Extract initial live chat continuation token from `/next` or `/player` response of a live stream.
pub fn extract_live_chat_continuation_token(raw: &Value) -> Option<String> {
    // 1. From liveChatRenderer in twoColumnWatchNextResults
    if let Some(token) = raw.pointer("/contents/twoColumnWatchNextResults/conversationBar/liveChatRenderer/continuations/0/reloadContinuationData/continuation")
        .or_else(|| raw.pointer("/contents/twoColumnWatchNextResults/conversationBar/liveChatRenderer/continuations/0/invalidationContinuationData/continuation"))
        .or_else(|| raw.pointer("/contents/twoColumnWatchNextResults/conversationBar/liveChatRenderer/continuations/0/timedContinuationData/continuation"))
        .and_then(Value::as_str)
    {
        return Some(token.to_string());
    }

    // 2. From engagement panels
    if let Some(panels) = raw.get("engagementPanels").and_then(|p| p.as_array()) {
        for panel in panels {
            let panel_id = panel.pointer("/engagementPanelSectionListRenderer/panelIdentifier")
                .and_then(Value::as_str)
                .unwrap_or("");
            if panel_id.contains("live_chat") {
                if let Some(token) = panel.pointer("/engagementPanelSectionListRenderer/content/liveChatRenderer/continuations/0/reloadContinuationData/continuation")
                    .or_else(|| panel.pointer("/engagementPanelSectionListRenderer/content/liveChatRenderer/continuations/0/timedContinuationData/continuation"))
                    .and_then(Value::as_str)
                {
                    return Some(token.to_string());
                }
            }
        }
    }

    None
}

/// Parse live chat response into `LiveChatResponse`.
pub fn parse_live_chat_response(raw: &Value) -> Result<LiveChatResponse> {
    let mut messages = Vec::new();
    let mut next_token = None;
    let mut timeout_ms = 1000;

    let actions = raw.pointer("/continuationContents/liveChatContinuation/actions")
        .or_else(|| raw.pointer("/continuationContents/liveChatContinuation/initialDisplayState/actions"))
        .or_else(|| raw.pointer("/onResponseReceivedEndpoints/0/reloadContinuationItemsCommand/continuationItems"))
        .or_else(|| raw.pointer("/onResponseReceivedEndpoints/0/appendContinuationItemsAction/continuationItems"))
        .and_then(|a| a.as_array());

    if let Some(act_arr) = actions {
        for action in act_arr {
            let item = action.pointer("/addChatItemAction/item")
                .or_else(|| action.pointer("/addLiveChatTickerItemAction/item"))
                .unwrap_or(action);

            if let Some(node) = LiveChatMessageNode::from_value(item) {
                messages.push(node.message);
            }
        }
    }

    if let Some(continuations) = raw.pointer("/continuationContents/liveChatContinuation/continuations").and_then(|c| c.as_array()) {
        for cont in continuations {
            if let Some(inval) = cont.get("invalidationContinuationData")
                .or_else(|| cont.get("timedContinuationData"))
                .or_else(|| cont.get("liveChatReplayContinuationData"))
            {
                if let Some(t) = inval.get("continuation").and_then(Value::as_str) {
                    next_token = Some(t.to_string());
                }
                if let Some(ms) = inval.get("timeoutMs").and_then(Value::as_u64) {
                    timeout_ms = ms;
                }
            }
        }
    }

    Ok(LiveChatResponse {
        messages,
        continuation_token: next_token,
        poll_timeout_ms: timeout_ms,
    })
}
