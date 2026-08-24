use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::feed::{BrowseFeed, FilterChip, HashtagFeed, HomeFeed, TrendingFeed, TrendingTab};
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::{NodeListExt, Parser};

/// Fetch the main YouTube Home Feed (`FEwhat_to_watch`).
pub async fn get_home_feed(session: &Session, params: Option<&str>) -> Result<HomeFeed> {
    let mut payload = json!({
        "browseId": "FEwhat_to_watch",
    });

    if let Some(p) = params {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("params".to_string(), json!(p));
        }
    }

    let resp = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_home_feed_response(&raw)
}

/// Fetch continuation page of the YouTube Home Feed.
pub async fn get_home_feed_continuation(session: &Session, continuation_token: &str) -> Result<HomeFeed> {
    let payload = json!({
        "continuation": continuation_token,
    });

    let resp = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_home_feed_response(&raw)
}

/// Fetch YouTube Trending Feed (`FEtrending`).
pub async fn get_trending(session: &Session, tab_params: Option<&str>) -> Result<TrendingFeed> {
    let mut payload = json!({
        "browseId": "FEtrending",
    });

    if let Some(p) = tab_params {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("params".to_string(), json!(p));
        }
    }

    let resp = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_trending_response(&raw)
}

/// Fetch videos for a specific hashtag (`FEhashtag`).
pub async fn get_hashtag_feed(session: &Session, tag: &str) -> Result<HashtagFeed> {
    let clean_tag = tag.trim_start_matches('#').trim();
    let params = crate::utils::proto::encode_hashtag_params(clean_tag)?;
    let payload = json!({
        "browseId": "FEhashtag",
        "params": params,
    });

    let resp = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    parse_hashtag_response(clean_tag, &raw)
}

/// Fetch a generic browse destination and retain its typed media collections.
pub async fn get_browse_feed(session: &Session, browse_id: &str) -> Result<BrowseFeed> {
    let response = session
        .post_innertube("/browse", json!({ "browseId": browse_id }))
        .await?;
    let raw: Value = response.json().await.map_err(InnertubeError::Network)?;
    parse_browse_feed_response(browse_id, &raw)
}

// ---------------------------------------------------------------------------
// Response Parsers
// ---------------------------------------------------------------------------

pub fn parse_home_feed_response(raw: &Value) -> Result<HomeFeed> {
    let mut filter_chips = Vec::new();

    // Extract chips from feedFilterChipBarRenderer / chipCloudRenderer
    if let Some(chips) = raw.pointer("/header/feedTabbedHeaderRenderer/chipCloudRenderer/chips")
        .or_else(|| raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/richGridRenderer/header/feedFilterChipBarRenderer/chips"))
        .and_then(|c| c.as_array())
    {
        for chip in chips {
            if let Some(chip_renderer) = chip.get("chipCloudChipRenderer") {
                let text = TextNode::from_value(chip_renderer.get("text").unwrap_or(&Value::Null))
                    .map(|t| t.text)
                    .unwrap_or_default();

                let is_selected = chip_renderer.get("isSelected").and_then(|s| s.as_bool()).unwrap_or(false);
                let params = chip_renderer.pointer("/navigationEndpoint/continuationEndpoint/continuationCommand/token")
                    .or_else(|| chip_renderer.pointer("/navigationEndpoint/browseEndpoint/params"))
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());

                if !text.is_empty() {
                    filter_chips.push(FilterChip {
                        text,
                        params,
                        is_selected,
                    });
                }
            }
        }
    }

    let parsed_tree = Parser::parse_tree(raw);
    let videos = parsed_tree.find_videos().into_iter().cloned().collect();
    let continuation_token = parsed_tree.find_continuation_token();

    Ok(HomeFeed {
        filter_chips,
        videos,
        continuation_token,
    })
}

/// Parse a generic browse response for account and discovery feed destinations.
pub fn parse_browse_feed_response(browse_id: &str, raw: &Value) -> Result<BrowseFeed> {
    let tree = Parser::parse_tree(raw);
    Ok(BrowseFeed {
        browse_id: browse_id.to_string(),
        videos: tree.find_videos().into_iter().cloned().collect(),
        channels: tree.find_channels().into_iter().cloned().collect(),
        playlists: tree.find_playlists().into_iter().cloned().collect(),
        continuation_token: tree.find_continuation_token(),
    })
}

pub fn parse_trending_response(raw: &Value) -> Result<TrendingFeed> {
    let mut tabs = Vec::new();
    let mut current_tab = "Now".to_string();

    if let Some(tab_arr) = raw.pointer("/contents/twoColumnBrowseResultsRenderer/tabs").and_then(|t| t.as_array()) {
        for tab in tab_arr {
            if let Some(tr) = tab.get("tabRenderer") {
                let title = tr.get("title").and_then(|t| t.as_str()).unwrap_or("").to_string();
                let is_selected = tr.get("selected").and_then(|s| s.as_bool()).unwrap_or(false);
                let params = tr.pointer("/endpoint/browseEndpoint/params")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());

                if is_selected && !title.is_empty() {
                    current_tab = title.clone();
                }

                if !title.is_empty() {
                    tabs.push(TrendingTab {
                        title,
                        params,
                        is_selected,
                    });
                }
            }
        }
    }

    let parsed_tree = Parser::parse_tree(raw);
    let videos = parsed_tree.find_videos().into_iter().cloned().collect();

    Ok(TrendingFeed {
        current_tab,
        tabs,
        videos,
    })
}

pub fn parse_hashtag_response(tag: &str, raw: &Value) -> Result<HashtagFeed> {
    let mut header_title = None;
    let mut video_count_text = None;
    let mut channel_count_text = None;

    if let Some(header) = raw.pointer("/header/pageHeaderRenderer/content/pageHeaderViewModel") {
        header_title = header.pointer("/title/dynamicTextViewModel/text/content")
            .or_else(|| header.pointer("/title/content"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string());

        if let Some(rows) = header.pointer("/metadata/contentMetadataViewModel/metadataRows").and_then(|r| r.as_array()) {
            if let Some(parts) = rows.first().and_then(|r| r.get("metadataParts")).and_then(|p| p.as_array()) {
                if let Some(p0) = parts.first().and_then(|p| p.pointer("/text/content")).and_then(|t| t.as_str()) {
                    video_count_text = Some(p0.to_string());
                }
                if let Some(p1) = parts.get(1).and_then(|p| p.pointer("/text/content")).and_then(|t| t.as_str()) {
                    channel_count_text = Some(p1.to_string());
                }
            }
        }
    }

    let parsed_tree = Parser::parse_tree(raw);
    let videos = parsed_tree.find_videos().into_iter().cloned().collect();
    let continuation_token = parsed_tree.find_continuation_token();

    Ok(HashtagFeed {
        hashtag: tag.to_string(),
        header_title,
        video_count_text,
        channel_count_text,
        videos,
        continuation_token,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_generic_browse_media_collections() {
        let fixture = json!({
            "items": [
                {
                    "videoRenderer": {
                        "videoId": "dQw4w9WgXcQ",
                        "title": { "simpleText": "Video" }
                    }
                },
                {
                    "channelRenderer": {
                        "channelId": "UC_test",
                        "title": { "simpleText": "Channel" }
                    }
                },
                {
                    "playlistRenderer": {
                        "playlistId": "PL_test",
                        "title": { "simpleText": "Playlist" }
                    }
                }
            ]
        });

        let feed = parse_browse_feed_response("FEsubscriptions", &fixture)
            .expect("fixture should parse");
        assert_eq!(feed.browse_id, "FEsubscriptions");
        assert_eq!(feed.videos.len(), 1);
        assert_eq!(feed.channels.len(), 1);
        assert_eq!(feed.playlists.len(), 1);
    }
}
