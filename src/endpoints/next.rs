use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::next::{AutoplayVideo, PlaylistPanelItem, RelatedVideo, WatchNextResults};
use crate::models::video::Thumbnail;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;
use crate::parser::{NodeListExt, Parser};

/// Fetch Watch Next results (/next endpoint) including related videos, autoplay recommendations, and playlist items.
pub async fn get_watch_next(
    session: &Session,
    video_id: &str,
    playlist_id: Option<&str>,
    playlist_index: Option<usize>,
    continuation: Option<&str>,
) -> Result<WatchNextResults> {
    let mut payload = json!({
        "contentCheckOk": true,
        "racyCheckOk": true,
    });

    if let Some(cont) = continuation {
        payload["continuation"] = json!(cont);
    } else {
        payload["videoId"] = json!(video_id);
        if let Some(pl_id) = playlist_id {
            payload["playlistId"] = json!(pl_id);
            if let Some(idx) = playlist_index {
                payload["playlistIndex"] = json!(idx);
            }
        }
    }

    let resp = session.post_innertube("/next", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_watch_next_response(video_id, &raw)
}

/// Parse raw `/next` response into consolidated `WatchNextResults` using modular AST nodes.
pub fn parse_watch_next_response(video_id: &str, raw: &Value) -> Result<WatchNextResults> {
    let mut results = WatchNextResults {
        current_video_id: video_id.to_string(),
        ..Default::default()
    };

    let contents = match raw.get("contents") {
        Some(c) => c,
        None => return Ok(results),
    };

    // 1. Two Column Layout (Standard Desktop)
    if let Some(two_col) = contents.get("twoColumnWatchNextResults") {
        // Extract current video metadata if available
        if let Some(primary_info) = two_col.pointer("/results/results/contents/0/videoPrimaryInfoRenderer") {
            results.current_title = TextNode::from_value(primary_info.get("title").unwrap_or(&Value::Null)).map(|t| t.text);
        }
        if let Some(owner) = two_col.pointer("/results/results/contents/1/videoSecondaryInfoRenderer/owner/videoOwnerRenderer") {
            results.current_author = TextNode::from_value(owner.get("title").unwrap_or(&Value::Null)).map(|t| t.text);
        }

        // Extract Autoplay
        if let Some(autoplay_val) = two_col.get("autoplay") {
            results.autoplay = parse_autoplay(autoplay_val);
        }

        // Extract Recommendations & Continuations from secondaryResults using Parser::parse_tree
        if let Some(secondary) = two_col.get("secondaryResults") {
            let parsed_tree = Parser::parse_tree(secondary);
            for v in parsed_tree.find_videos() {
                results.related_videos.push(RelatedVideo {
                    video_id: v.id.clone(),
                    title: v.title.clone(),
                    author: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_default(),
                    author_id: v.author.as_ref().and_then(|a| a.id.clone()),
                    duration_text: v.duration.clone(),
                    duration_seconds: v.duration_ms.map(|ms| ms / 1000),
                    thumbnails: v.thumbnails.thumbnails.iter().map(|t| Thumbnail {
                        url: t.url.clone(),
                        width: t.width.unwrap_or(0),
                        height: t.height.unwrap_or(0),
                    }).collect(),
                    view_count_text: v.view_count.clone(),
                    published_time_text: v.published_time.clone(),
                    is_live: v.is_live,
                    is_upcoming: v.is_upcoming,
                });
            }
            results.continuation_token = parsed_tree.find_continuation_token();
        }

        // Extract Playlist Queue if playing in a playlist
        if let Some(playlist_items) = two_col.pointer("/playlist/playlist/contents").and_then(|c| c.as_array()) {
            for item in playlist_items {
                if let Some(pvr) = item.get("playlistPanelVideoRenderer") {
                    if let Some(pi) = parse_playlist_panel_video_renderer(pvr) {
                        results.playlist_items.push(pi);
                    }
                }
            }
        }
    }

    // 2. Single Column Layout (Mobile / Music)
    if let Some(single_col) = contents.get("singleColumnMusicWatchNextResults") {
        if let Some(queue_items) = single_col.pointer("/tabbedRenderer/watchNextTabbedResultsRenderer/tabs/0/tabRenderer/content/musicQueueRenderer/content/playlistPanelRenderer/contents").and_then(|c| c.as_array()) {
            for item in queue_items {
                if let Some(pvr) = item.get("playlistPanelVideoRenderer") {
                    if let Some(pi) = parse_playlist_panel_video_renderer(pvr) {
                        results.playlist_items.push(pi);
                    }
                }
            }
        }
    }

    if results.continuation_token.is_none() {
        results.continuation_token = extract_comments_continuation_token(raw);
    }

    Ok(results)
}

fn parse_autoplay(val: &Value) -> Option<AutoplayVideo> {
    let auto_set = val.pointer("/autoplay/sets/0/nextVideoRenderer")
        .or_else(|| val.pointer("/autoplayRenderer/sets/0/nextVideoRenderer"))
        .or_else(|| val.pointer("/autoplay/autoplay/sets/0/nextVideoRenderer"))?;

    let node = auto_set.get("autoplayVideoRenderer")
        .or_else(|| auto_set.get("autoplayEndpointRenderer"))
        .or_else(|| auto_set.get("lockupViewModel"))
        .unwrap_or(auto_set);

    let video_id = node.pointer("/endpoint/watchEndpoint/videoId")
        .or_else(|| node.pointer("/navigationEndpoint/watchEndpoint/videoId"))
        .or_else(|| node.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId"))
        .or_else(|| node.get("videoId"))
        .and_then(Value::as_str)?
        .to_string();

    let title = node.pointer("/videoTitle/runs/0/text")
        .or_else(|| node.pointer("/videoTitle/simpleText"))
        .or_else(|| node.pointer("/title/runs/0/text"))
        .or_else(|| node.pointer("/title/content"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let author = node.pointer("/byline/runs/0/text")
        .or_else(|| node.pointer("/byline/simpleText"))
        .or_else(|| node.pointer("/shortBylineText/runs/0/text"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let thumbnails = ThumbnailListNode::from_value(
        node.pointer("/thumbnail/thumbnails")
            .or_else(|| node.pointer("/contentImage/thumbnailViewModel/image/sources"))
            .unwrap_or(node)
    ).thumbnails.into_iter().map(|t| Thumbnail {
        url: t.url,
        width: t.width.unwrap_or(0),
        height: t.height.unwrap_or(0),
    }).collect();

    Some(AutoplayVideo {
        video_id,
        title,
        author,
        thumbnails,
    })
}

fn parse_playlist_panel_video_renderer(pvr: &Value) -> Option<PlaylistPanelItem> {
    let video_id = pvr.get("videoId").and_then(Value::as_str)?.to_string();
    let title = TextNode::from_value(pvr.get("title").unwrap_or(&Value::Null)).map(|t| t.text).unwrap_or_default();
    let author = TextNode::from_value(pvr.get("shortBylineText").unwrap_or(&Value::Null)).map(|t| t.text).unwrap_or_default();
    let is_selected = pvr.get("selected").and_then(Value::as_bool).unwrap_or(false);

    let thumbnails = ThumbnailListNode::from_value(pvr.get("thumbnail").unwrap_or(pvr))
        .thumbnails
        .into_iter()
        .map(|t| Thumbnail {
            url: t.url,
            width: t.width.unwrap_or(0),
            height: t.height.unwrap_or(0),
        })
        .collect();

    Some(PlaylistPanelItem {
        video_id,
        title,
        author,
        index: None,
        is_selected,
        thumbnails,
    })
}

fn extract_comments_continuation_token(raw: &Value) -> Option<String> {
    // 1. Check engagement panels
    if let Some(panels) = raw.get("engagementPanels").and_then(|p| p.as_array()) {
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

    // 2. Check itemSectionRenderer in twoColumnWatchNextResults
    if let Some(sections) = raw.pointer("/contents/twoColumnWatchNextResults/results/results/contents").and_then(|c| c.as_array()) {
        for sec in sections {
            let section_id = sec.pointer("/itemSectionRenderer/sectionIdentifier")
                .and_then(|i| i.as_str())
                .unwrap_or("");
            if section_id.contains("comment") {
                if let Some(token) = sec.pointer("/itemSectionRenderer/continuations/0/nextContinuationData/continuation")
                    .or_else(|| sec.pointer("/itemSectionRenderer/contents/0/continuationItemRenderer/continuationEndpoint/continuationCommand/token"))
                    .and_then(|t| t.as_str())
                {
                    return Some(token.to_string());
                }
            }
        }
    }

    None
}
