use std::collections::HashMap;
use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::comments::{Comment, CommentThread, CommentsResult};
use crate::parser::nodes::comments::CommentNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::{NodeListExt, Parser, YTNode};

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

    let parsed_tree = Parser::parse_tree(&raw);
    for node in &parsed_tree {
        if let YTNode::Comment(c) = node {
            replies.push(convert_node_to_comment(c));
        }
    }

    if replies.is_empty() {
        // Fallback for commentViewModel in entityBatchUpdate
        if let Some(endpoints) = raw.get("onResponseReceivedEndpoints").and_then(|e| e.as_array()) {
            for ep in endpoints {
                if let Some(items) = ep.pointer("/appendContinuationItemsAction/continuationItems").and_then(|i| i.as_array()) {
                    for item in items {
                        if let Some(cvm) = item.pointer("/commentViewModel/commentViewModel") {
                            if let Some(c) = parse_comment_view_model(cvm, &entity_map) {
                                replies.push(c);
                            }
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
            let panel_id = panel.pointer("/engagementPanelSectionListRenderer/panelIdentifier")
                .and_then(|i| i.as_str())
                .unwrap_or("");
            if panel_id.contains("comment") {
                if let Some(token) = panel.pointer("/engagementPanelSectionListRenderer/content/sectionListRenderer/contents/0/itemSectionRenderer/continuations/0/nextContinuationData/continuation")
                    .or_else(|| panel.pointer("/engagementPanelSectionListRenderer/content/sectionListRenderer/contents/0/continuationItemRenderer/continuationEndpoint/continuationCommand/token"))
                    .and_then(|t| t.as_str())
                {
                    return Some(token.to_string());
                }
            }
        }
    }

    None
}

/// Parse comments response into `CommentsResult` using modular AST nodes.
pub fn parse_comments_response(raw: &Value) -> Result<CommentsResult> {
    let mut total_comments_text = None;
    let mut comments = Vec::new();
    let entity_map = build_comment_entity_map(raw);

    // 1. Extract Header total comments count
    if let Some(endpoints) = raw.get("onResponseReceivedEndpoints").and_then(|e| e.as_array()) {
        for ep in endpoints {
            let items = ep.pointer("/reloadContinuationItemsCommand/continuationItems")
                .or_else(|| ep.pointer("/appendContinuationItemsAction/continuationItems"))
                .and_then(|i| i.as_array());

            if let Some(item_arr) = items {
                for item in item_arr {
                    if let Some(header) = item.get("commentsHeaderRenderer") {
                        total_comments_text = TextNode::from_value(header.get("countText").unwrap_or(&Value::Null))
                            .map(|t| t.text);
                    }
                }
            }
        }
    }

    // 2. Extract comments threads using modular AST parser
    let parsed_tree = Parser::parse_tree(raw);
    for node in &parsed_tree {
        if let YTNode::CommentThread(ct) = node {
            comments.push(CommentThread {
                comment: convert_node_to_comment(&ct.comment),
                replies: ct.replies.iter().map(convert_node_to_comment).collect(),
                replies_continuation_token: ct.continuation_token.clone(),
            });
        }
    }

    // 3. Fallback for commentThreadRenderer / commentViewModel in modern responses
    if comments.is_empty() {
        if let Some(endpoints) = raw.get("onResponseReceivedEndpoints").and_then(|e| e.as_array()) {
            for ep in endpoints {
                let items = ep.pointer("/reloadContinuationItemsCommand/continuationItems")
                    .or_else(|| ep.pointer("/appendContinuationItemsAction/continuationItems"))
                    .and_then(|i| i.as_array());

                if let Some(item_arr) = items {
                    for item in item_arr {
                        if let Some(ctr) = item.get("commentThreadRenderer") {
                            if let Some(t) = parse_comment_thread_renderer(ctr, &entity_map) {
                                comments.push(t);
                            }
                        }
                    }
                }
            }
        }
    }

    let next_continuation = parsed_tree.find_continuation_token();

    Ok(CommentsResult {
        total_comments_text,
        comments,
        continuation_token: next_continuation,
    })
}

fn convert_node_to_comment(c: &CommentNode) -> Comment {
    Comment {
        comment_id: c.comment_id.clone(),
        author_name: c.author_name.clone(),
        author_thumbnail: c.author_thumbnail.clone(),
        author_channel_id: c.author_id.clone(),
        text: c.text.clone(),
        published_time: c.published_time.clone(),
        like_count_text: c.like_count.clone(),
        like_count: c.like_count.as_deref().and_then(parse_like_count),
        reply_count: c.reply_count.map(|r| r as u64),
        is_pinned: c.is_pinned,
        is_author_channel_owner: c.is_author_channel_owner,
        reply_continuation_token: c.reply_continuation_token.clone(),
    }
}

fn parse_comment_thread_renderer(ctr: &Value, entity_map: &HashMap<String, String>) -> Option<CommentThread> {
    let comment = if let Some(cr) = ctr.pointer("/comment/commentRenderer") {
        CommentNode::from_value(cr).map(|c| convert_node_to_comment(&c))?
    } else if let Some(cvm) = ctr.pointer("/commentViewModel/commentViewModel") {
        parse_comment_view_model(cvm, entity_map)?
    } else {
        return None;
    };

    let reply_token = ctr.pointer("/replies/commentRepliesRenderer/contents/0/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    Some(CommentThread {
        comment,
        replies: Vec::new(),
        replies_continuation_token: reply_token,
    })
}

fn parse_comment_view_model(cvm: &Value, entity_map: &HashMap<String, String>) -> Option<Comment> {
    let comment_id = cvm.get("commentId").or_else(|| cvm.get("commentKey")).and_then(|c| c.as_str())?.to_string();
    let author_name = cvm.pointer("/author/avatarViewModel/avatarImage/accessibility/accessibilityData/label")
        .or_else(|| cvm.pointer("/author/displayName/content"))
        .and_then(|a| a.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let author_thumbnail = cvm.pointer("/author/avatarViewModel/avatarImage/sources/0/url")
        .and_then(|u| u.as_str())
        .map(|s| s.to_string());

    let author_channel_id = cvm.pointer("/author/channelCommand/innertubeCommand/browseEndpoint/browseId")
        .and_then(|id| id.as_str())
        .map(|s| s.to_string());

    let entity_key = cvm.pointer("/content/content/entityKey").and_then(|k| k.as_str());
    let text = if let Some(key) = entity_key {
        entity_map.get(key).cloned().unwrap_or_default()
    } else {
        cvm.pointer("/content/content/content").and_then(|c| c.as_str()).unwrap_or("").to_string()
    };

    let published_time = cvm.pointer("/publishedTime/content").and_then(|p| p.as_str()).map(|s| s.to_string());
    let like_count_text = cvm.pointer("/toolbar/likeCountNotliked/content").and_then(|l| l.as_str()).map(|s| s.to_string());
    let is_pinned = cvm.pointer("/pinnedText/content").is_some();
    let is_author_channel_owner = cvm.pointer("/author/isCreator").and_then(|c| c.as_bool()).unwrap_or(false);

    let reply_count = cvm.pointer("/toolbar/replyCount/content")
        .and_then(|r| r.as_str())
        .and_then(|s| s.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().ok());

    Some(Comment {
        comment_id,
        author_name,
        author_thumbnail,
        author_channel_id,
        text,
        published_time,
        like_count_text: like_count_text.clone(),
        like_count: like_count_text.as_deref().and_then(parse_like_count),
        reply_count,
        is_pinned,
        is_author_channel_owner,
        reply_continuation_token: None,
    })
}

fn build_comment_entity_map(raw: &Value) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(mutations) = raw.pointer("/frameworkUpdates/entityBatchUpdate/mutations").and_then(|m| m.as_array()) {
        for mutation in mutations {
            if let Some(payload) = mutation.get("payload") {
                if let Some(comment_entity) = payload.get("commentEntityPayload") {
                    if let Some(key) = comment_entity.get("key").and_then(|k| k.as_str()) {
                        if let Some(content) = comment_entity.pointer("/properties/content/content").and_then(|c| c.as_str()) {
                            map.insert(key.to_string(), content.to_string());
                        }
                    }
                }
            }
        }
    }
    map
}

fn extract_continuation_token(item: &Value) -> Option<String> {
    item.pointer("/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
        .or_else(|| item.pointer("/continuationItemRenderer/continuationEndpoint/nextContinuationData/continuation"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
}

fn parse_like_count(text: &str) -> Option<u64> {
    let clean = text.trim().to_uppercase();
    if clean.ends_with('K') {
        let num: f64 = clean.strip_suffix('K')?.parse().ok()?;
        Some((num * 1_000.0) as u64)
    } else if clean.ends_with('M') {
        let num: f64 = clean.strip_suffix('M')?.parse().ok()?;
        Some((num * 1_000_000.0) as u64)
    } else {
        clean.replace(',', "").parse().ok()
    }
}
