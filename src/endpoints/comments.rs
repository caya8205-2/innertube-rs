use std::collections::HashMap;
use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::comments::{Comment, CommentThread, CommentsResult};

/// Fetch comments for a video ID or continuation token.
pub async fn get_comments(
    session: &Session,
    video_id: &str,
    continuation_token: Option<&str>,
) -> Result<CommentsResult> {
    let continuation = match continuation_token {
        Some(t) => t.to_string(),
        None => {
            // Step 1: Query /next to get initial comment section continuation token
            let initial_payload = json!({
                "videoId": video_id,
                "contentCheckOk": true,
                "racyCheckOk": true
            });
            let resp = session.post_innertube("/next", initial_payload).await?;
            let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
            extract_comment_continuation_token(&raw).ok_or_else(|| {
                InnertubeError::Other(format!("Comments are disabled or not available for video: {}", video_id))
            })?
        }
    };

    // Step 2: Fetch comments using continuation token
    let payload = json!({
        "continuation": continuation,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    let resp = session.post_innertube("/next", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_comments_response(&raw)
}

/// Fetch child comment replies using a reply continuation token.
pub async fn get_comment_replies(session: &Session, continuation_token: &str) -> Result<Vec<Comment>> {
    let payload = json!({
        "continuation": continuation_token,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    let resp = session.post_innertube("/next", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let mut replies = Vec::new();
    let entity_map = build_comment_entity_map(&raw);

    if let Some(endpoints) = raw.get("onResponseReceivedEndpoints").and_then(|e| e.as_array()) {
        for ep in endpoints {
            if let Some(items) = ep.pointer("/appendContinuationItemsAction/continuationItems").and_then(|i| i.as_array()) {
                for item in items {
                    if let Some(cr) = item.get("commentRenderer") {
                        if let Some(c) = parse_comment_renderer(cr) {
                            replies.push(c);
                        }
                    } else if let Some(cvm) = item.pointer("/commentViewModel/commentViewModel") {
                        if let Some(c) = parse_comment_view_model(cvm, &entity_map) {
                            replies.push(c);
                        }
                    }
                }
            }
        }
    }

    Ok(replies)
}

/// Extract comment section continuation token from an initial `/next` response.
pub fn extract_comment_continuation_token(next_json: &Value) -> Option<String> {
    // 1. Look in twoColumnWatchNextResults itemSectionRenderers
    if let Some(contents) = next_json.pointer("/contents/twoColumnWatchNextResults/results/results/contents").and_then(|c| c.as_array()) {
        for section in contents {
            if let Some(isr) = section.get("itemSectionRenderer") {
                let section_id = isr.get("sectionIdentifier").and_then(|s| s.as_str());
                if section_id == Some("comment-item-section") {
                    if let Some(items) = isr.get("contents").and_then(|c| c.as_array()) {
                        for item in items {
                            if let Some(token) = extract_continuation_token(item) {
                                return Some(token);
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Look in engagementPanels
    if let Some(panels) = next_json.get("engagementPanels").and_then(|p| p.as_array()) {
        for panel in panels {
            if let Some(ep) = panel.get("engagementPanelSectionListRenderer") {
                let panel_id = ep.get("targetId").and_then(|t| t.as_str());
                if panel_id == Some("engagement-panel-comments-section") || panel_id == Some("comments-section") {
                    if let Some(sections) = ep.pointer("/content/sectionListRenderer/contents").and_then(|s| s.as_array()) {
                        for section in sections {
                            if let Some(items) = section.pointer("/itemSectionRenderer/contents").and_then(|i| i.as_array()) {
                                for item in items {
                                    if let Some(token) = extract_continuation_token(item) {
                                        return Some(token);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Parse comments response into `CommentsResult`.
pub fn parse_comments_response(raw: &Value) -> Result<CommentsResult> {
    let mut result = CommentsResult::default();
    let entity_map = build_comment_entity_map(raw);

    let endpoints = match raw.get("onResponseReceivedEndpoints").and_then(|e| e.as_array()) {
        Some(e) => e,
        None => return Ok(result),
    };

    for ep in endpoints {
        let items_opt = ep.pointer("/reloadContinuationItemsCommand/continuationItems")
            .or_else(|| ep.pointer("/appendContinuationItemsAction/continuationItems"))
            .and_then(|i| i.as_array());

        if let Some(items) = items_opt {
            for item in items {
                // Header (Total comments count)
                if let Some(header) = item.pointer("/commentsHeaderRenderer/countText") {
                    result.total_comments_text = parse_text(Some(header));
                }

                // Comment Thread
                if let Some(thread) = item.get("commentThreadRenderer") {
                    if let Some(ct) = parse_comment_thread_renderer(thread, &entity_map) {
                        result.comments.push(ct);
                    }
                }

                // Next Page Continuation Token
                if let Some(token) = extract_continuation_token(item) {
                    result.continuation_token = Some(token);
                }
            }
        }
    }

    Ok(result)
}

fn build_comment_entity_map(raw: &Value) -> HashMap<String, &Value> {
    let mut map = HashMap::new();
    if let Some(mutations) = raw.pointer("/frameworkUpdates/entityBatchUpdate/mutations").and_then(|m| m.as_array()) {
        for m in mutations {
            if let (Some(k), Some(payload)) = (m.get("entityKey").and_then(|k| k.as_str()), m.pointer("/payload/commentEntityPayload")) {
                map.insert(k.to_string(), payload);
            }
        }
    }
    map
}

fn parse_comment_thread_renderer(thread: &Value, entity_map: &HashMap<String, &Value>) -> Option<CommentThread> {
    let mut comment_opt = None;

    // 1. Classic commentRenderer
    if let Some(comment_raw) = thread.pointer("/comment/commentRenderer") {
        comment_opt = parse_comment_renderer(comment_raw);
    }
    // 2. Modern commentViewModel
    else if let Some(cvm) = thread.pointer("/commentViewModel/commentViewModel") {
        comment_opt = parse_comment_view_model(cvm, entity_map);
    }

    let comment = comment_opt?;

    let mut replies = Vec::new();
    let mut replies_continuation_token = None;

    if let Some(replies_renderer) = thread.pointer("/replies/commentRepliesRenderer") {
        if let Some(contents) = replies_renderer.get("contents").and_then(|c| c.as_array()) {
            for item in contents {
                if let Some(cr) = item.get("commentRenderer") {
                    if let Some(c) = parse_comment_renderer(cr) {
                        replies.push(c);
                    }
                } else if let Some(cvm) = item.pointer("/commentViewModel/commentViewModel") {
                    if let Some(c) = parse_comment_view_model(cvm, entity_map) {
                        replies.push(c);
                    }
                }
            }
        }

        replies_continuation_token = replies_renderer
            .pointer("/contents/0/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
            .or_else(|| replies_renderer.pointer("/viewReplies/buttonRenderer/command/continuationCommand/token"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }

    Some(CommentThread {
        comment,
        replies,
        replies_continuation_token,
    })
}

fn parse_comment_view_model(cvm: &Value, entity_map: &HashMap<String, &Value>) -> Option<Comment> {
    let comment_id = cvm.get("commentId").and_then(|v| v.as_str())?.to_string();
    let is_pinned = cvm.get("pinnedText").is_some();

    // Look up entity payload by commentKey
    if let Some(comment_key) = cvm.get("commentKey").and_then(|k| k.as_str()) {
        if let Some(payload) = entity_map.get(comment_key) {
            let author_name = payload.pointer("/author/displayName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();

            let author_thumbnail = payload.pointer("/author/avatarThumbnailUrl")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let author_channel_id = payload.pointer("/author/channelId")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let is_author_channel_owner = payload.pointer("/author/isCreator")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let text = payload.pointer("/properties/content/content")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();

            let published_time = payload.pointer("/properties/publishedTime")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let like_count_text = payload.pointer("/toolbar/likeCountNotliked")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let like_count = like_count_text.as_deref().and_then(parse_count_number);
            let reply_count = payload.pointer("/toolbar/replyCount")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<u64>().ok());

            return Some(Comment {
                comment_id,
                author_name,
                author_thumbnail,
                author_channel_id,
                text,
                published_time,
                like_count_text,
                like_count,
                reply_count,
                is_pinned,
                is_author_channel_owner,
                reply_continuation_token: None,
            });
        }
    }

    Some(Comment {
        comment_id,
        author_name: "Unknown".to_string(),
        is_pinned,
        ..Default::default()
    })
}

fn parse_comment_renderer(cr: &Value) -> Option<Comment> {
    let comment_id = cr.get("commentId").and_then(|v| v.as_str())?.to_string();
    let author_name = cr.pointer("/authorText/simpleText")
        .or_else(|| cr.pointer("/authorText/runs/0/text"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let author_thumbnail = cr.pointer("/authorThumbnail/thumbnails/0/url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let author_channel_id = cr.pointer("/authorEndpoint/browseEndpoint/browseId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let text = parse_text(cr.get("contentText")).unwrap_or_default();
    let published_time = cr.pointer("/publishedTimeText/runs/0/text")
        .or_else(|| cr.pointer("/publishedTimeText/simpleText"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let like_count_text = cr.pointer("/voteCount/simpleText")
        .or_else(|| cr.pointer("/voteCount/accessibility/accessibilityData/label"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let like_count = like_count_text.as_deref().and_then(parse_count_number);
    let reply_count = cr.get("replyCount").and_then(|r| r.as_u64());

    let is_pinned = cr.get("pinnedCommentBadge").is_some();
    let is_author_channel_owner = cr.get("authorIsChannelOwner").and_then(|a| a.as_bool()).unwrap_or(false);

    Some(Comment {
        comment_id,
        author_name,
        author_thumbnail,
        author_channel_id,
        text,
        published_time,
        like_count_text,
        like_count,
        reply_count,
        is_pinned,
        is_author_channel_owner,
        reply_continuation_token: None,
    })
}

fn parse_text(val: Option<&Value>) -> Option<String> {
    let val = val?;
    if let Some(s) = val.get("simpleText").and_then(|s| s.as_str()) {
        return Some(s.to_string());
    }
    if let Some(runs) = val.get("runs").and_then(|r| r.as_array()) {
        let texts: Vec<&str> = runs.iter().filter_map(|r| r.get("text").and_then(|t| t.as_str())).collect();
        if !texts.is_empty() {
            return Some(texts.join(""));
        }
    }
    None
}

fn extract_continuation_token(item: &Value) -> Option<String> {
    item.pointer("/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
        .or_else(|| item.pointer("/continuationItemRenderer/button/buttonRenderer/command/continuationCommand/token"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn parse_count_number(text: &str) -> Option<u64> {
    let cleaned = text.trim().replace(',', "");
    if let Ok(num) = cleaned.parse::<u64>() {
        return Some(num);
    }
    if cleaned.ends_with('K') || cleaned.ends_with('k') {
        let n = cleaned[..cleaned.len() - 1].parse::<f64>().ok()?;
        return Some((n * 1_000.0) as u64);
    }
    if cleaned.ends_with('M') || cleaned.ends_with('m') {
        let n = cleaned[..cleaned.len() - 1].parse::<f64>().ok()?;
        return Some((n * 1_000_000.0) as u64);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_comments_response() {
        let fixture = json!({
            "onResponseReceivedEndpoints": [
                {
                    "reloadContinuationItemsCommand": {
                        "continuationItems": [
                            {
                                "commentsHeaderRenderer": {
                                    "countText": {
                                        "runs": [
                                            { "text": "1,520" },
                                            { "text": " Comments" }
                                        ]
                                    }
                                }
                            },
                            {
                                "commentThreadRenderer": {
                                    "comment": {
                                        "commentRenderer": {
                                            "commentId": "Ugx12345Comment",
                                            "authorText": { "simpleText": "@cool_viewer" },
                                            "contentText": {
                                                "runs": [
                                                    { "text": "This is an amazing video!" }
                                                ]
                                            },
                                            "publishedTimeText": { "runs": [{ "text": "2 hours ago" }] },
                                            "voteCount": { "simpleText": "1.2K" },
                                            "replyCount": 5,
                                            "authorIsChannelOwner": true
                                        }
                                    }
                                }
                            },
                            {
                                "continuationItemRenderer": {
                                    "continuationEndpoint": {
                                        "continuationCommand": {
                                            "token": "NEXT_COMMENTS_PAGE_TOKEN"
                                        }
                                    }
                                }
                            }
                        ]
                    }
                }
            ]
        });

        let result = parse_comments_response(&fixture).expect("Failed to parse comments fixture");
        assert_eq!(result.total_comments_text.as_deref(), Some("1,520 Comments"));
        assert_eq!(result.comments.len(), 1);
        assert_eq!(result.comments[0].comment.comment_id, "Ugx12345Comment");
        assert_eq!(result.comments[0].comment.author_name, "@cool_viewer");
        assert_eq!(result.comments[0].comment.text, "This is an amazing video!");
        assert_eq!(result.comments[0].comment.like_count_text.as_deref(), Some("1.2K"));
        assert_eq!(result.comments[0].comment.like_count, Some(1200));
        assert_eq!(result.comments[0].comment.reply_count, Some(5));
        assert!(result.comments[0].comment.is_author_channel_owner);
        assert_eq!(result.continuation_token.as_deref(), Some("NEXT_COMMENTS_PAGE_TOKEN"));
    }
}
