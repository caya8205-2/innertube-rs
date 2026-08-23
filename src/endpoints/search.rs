use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::search::{SearchResultItem, SearchResults, SearchVideoItem, SearchChannelItem, SearchPlaylistItem};
use crate::models::video::Thumbnail;

/// Execute a search query against `/youtubei/v1/search`.
pub async fn search(
    session: &Session,
    query: &str,
    continuation_token: Option<&str>,
) -> Result<SearchResults> {
    let mut payload = json!({
        "query": query
    });

    if let Some(token) = continuation_token {
        payload["continuation"] = json!(token);
    }

    let resp = session.post_innertube("/search", payload).await?;

    if !resp.status().is_success() {
        return Err(InnertubeError::Api {
            status: resp.status().to_string(),
            message: format!("Search endpoint returned HTTP {}", resp.status()),
        });
    }

    let val: Value = resp.json().await.map_err(InnertubeError::Network)?;
    let mut items = Vec::new();
    let mut next_continuation = None;

    // Traverse response AST recursively to collect videoRenderer, channelRenderer, playlistRenderer, lockupViewModel
    parse_search_nodes(&val, &mut items, &mut next_continuation);

    Ok(SearchResults {
        query: query.to_string(),
        items,
        continuation_token: next_continuation,
    })
}

fn parse_search_nodes(
    value: &Value,
    items: &mut Vec<SearchResultItem>,
    continuation: &mut Option<String>,
) {
    if let Some(arr) = value.as_array() {
        for v in arr {
            parse_search_nodes(v, items, continuation);
        }
    } else if let Some(obj) = value.as_object() {
        // 1. videoRenderer
        if let Some(vr) = obj.get("videoRenderer") {
            if let Some(video_id) = vr.get("videoId").and_then(Value::as_str) {
                let title = vr.pointer("/title/runs/0/text")
                    .or_else(|| vr.pointer("/title/simpleText"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let author = vr.pointer("/ownerText/runs/0/text")
                    .or_else(|| vr.pointer("/longBylineText/runs/0/text"))
                    .or_else(|| vr.pointer("/shortBylineText/runs/0/text"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let channel_id = vr.pointer("/ownerText/runs/0/navigationEndpoint/browseEndpoint/browseId")
                    .or_else(|| vr.pointer("/longBylineText/runs/0/navigationEndpoint/browseEndpoint/browseId"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let duration = vr.pointer("/lengthText/simpleText")
                    .or_else(|| vr.pointer("/lengthText/runs/0/text"))
                    .and_then(Value::as_str)
                    .map(String::from);

                let view_count = vr.pointer("/viewCountText/simpleText")
                    .or_else(|| vr.pointer("/viewCountText/runs/0/text"))
                    .and_then(Value::as_str)
                    .map(String::from);

                let published_time = vr.pointer("/publishedTimeText/simpleText")
                    .or_else(|| vr.pointer("/publishedTimeText/runs/0/text"))
                    .and_then(Value::as_str)
                    .map(String::from);

                let thumbnails = extract_thumbnails(vr.pointer("/thumbnail/thumbnails"));

                items.push(SearchResultItem::Video(SearchVideoItem {
                    video_id: video_id.to_string(),
                    title,
                    author,
                    channel_id,
                    duration,
                    view_count,
                    published_time,
                    thumbnails,
                }));
            }
        }

        // 2. channelRenderer
        if let Some(cr) = obj.get("channelRenderer") {
            if let Some(channel_id) = cr.get("channelId").and_then(Value::as_str) {
                let title = cr.pointer("/title/simpleText")
                    .or_else(|| cr.pointer("/title/runs/0/text"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let subscriber_count = cr.pointer("/subscriberCountText/simpleText")
                    .or_else(|| cr.pointer("/subscriberCountText/runs/0/text"))
                    .and_then(Value::as_str)
                    .map(String::from);

                let video_count = cr.pointer("/videoCountText/runs/0/text")
                    .and_then(Value::as_str)
                    .map(String::from);

                let thumbnails = extract_thumbnails(cr.pointer("/thumbnail/thumbnails"));

                items.push(SearchResultItem::Channel(SearchChannelItem {
                    channel_id: channel_id.to_string(),
                    title,
                    subscriber_count,
                    video_count,
                    thumbnails,
                }));
            }
        }

        // 3. playlistRenderer
        if let Some(pr) = obj.get("playlistRenderer") {
            if let Some(playlist_id) = pr.get("playlistId").and_then(Value::as_str) {
                let title = pr.pointer("/title/simpleText")
                    .or_else(|| pr.pointer("/title/runs/0/text"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let author = pr.pointer("/longBylineText/runs/0/text")
                    .or_else(|| pr.pointer("/shortBylineText/runs/0/text"))
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();

                let video_count = pr.pointer("/videoCount")
                    .or_else(|| pr.pointer("/videoCountText/runs/0/text"))
                    .and_then(Value::as_str)
                    .map(String::from);

                let thumbnails = extract_thumbnails(pr.pointer("/thumbnails/0/thumbnails"));

                items.push(SearchResultItem::Playlist(SearchPlaylistItem {
                    playlist_id: playlist_id.to_string(),
                    title,
                    author,
                    video_count,
                    thumbnails,
                }));
            }
        }

        // 4. continuation
        if let Some(tok) = value.pointer("/continuationItemRenderer/continuationEndpoint/continuationCommand/token")
            .and_then(Value::as_str)
        {
            *continuation = Some(tok.to_string());
        }

        // Recurse into children
        for (_, v) in obj {
            parse_search_nodes(v, items, continuation);
        }
    }
}

fn extract_thumbnails(val: Option<&Value>) -> Vec<Thumbnail> {
    let mut res = Vec::new();
    if let Some(arr) = val.and_then(Value::as_array) {
        for t in arr {
            if let (Some(url), Some(width), Some(height)) = (
                t.get("url").and_then(Value::as_str),
                t.get("width").and_then(Value::as_u64),
                t.get("height").and_then(Value::as_u64),
            ) {
                res.push(Thumbnail {
                    url: url.to_string(),
                    width: width as u32,
                    height: height as u32,
                });
            }
        }
    }
    res
}
