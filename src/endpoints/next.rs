use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::next::{AutoplayVideo, PlaylistPanelItem, RelatedVideo, WatchNextResults};
use crate::models::video::Thumbnail;

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

/// Parse raw `/next` response into consolidated `WatchNextResults`.
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
            results.current_title = parse_text(primary_info.get("title"));
        }
        if let Some(owner) = two_col.pointer("/results/results/contents/1/videoSecondaryInfoRenderer/owner/videoOwnerRenderer") {
            results.current_author = parse_text(owner.get("title"));
        }

        // Extract Autoplay
        if let Some(autoplay_val) = two_col.get("autoplay") {
            results.autoplay = parse_autoplay(autoplay_val);
        }

        // Extract Recommendations & Continuations from secondaryResults
        if let Some(secondary_items) = two_col.pointer("/secondaryResults/secondaryResults/results").and_then(|r| r.as_array()) {
            for item in secondary_items {
                if let Some(lvm) = item.get("lockupViewModel") {
                    if let Some(rv) = parse_lockup_view_model(lvm) {
                        results.related_videos.push(rv);
                    }
                } else if let Some(cvr) = item.get("compactVideoRenderer") {
                    if let Some(rv) = parse_compact_video_renderer(cvr) {
                        results.related_videos.push(rv);
                    }
                } else if let Some(token) = parse_continuation_token(item) {
                    results.continuation_token = Some(token);
                }
            }
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

    // 3. Fallback for Continuations payload
    if let Some(on_response_endpoints) = raw.get("onResponseReceivedEndpoints").and_then(|r| r.as_array()) {
        for ep in on_response_endpoints {
            if let Some(actions) = ep.pointer("/appendContinuationItemsAction/continuationItems").and_then(|a| a.as_array()) {
                for item in actions {
                    if let Some(lvm) = item.get("lockupViewModel") {
                        if let Some(rv) = parse_lockup_view_model(lvm) {
                            results.related_videos.push(rv);
                        }
                    } else if let Some(cvr) = item.get("compactVideoRenderer") {
                        if let Some(rv) = parse_compact_video_renderer(cvr) {
                            results.related_videos.push(rv);
                        }
                    } else if let Some(token) = parse_continuation_token(item) {
                        results.continuation_token = Some(token);
                    }
                }
            }
        }
    }

    Ok(results)
}

fn parse_text(val: Option<&Value>) -> Option<String> {
    let val = val?;
    if let Some(s) = val.get("simpleText").and_then(|s| s.as_str()) {
        return Some(s.to_string());
    }
    if let Some(content) = val.get("content").and_then(|s| s.as_str()) {
        return Some(content.to_string());
    }
    if let Some(runs) = val.get("runs").and_then(|r| r.as_array()) {
        let texts: Vec<&str> = runs.iter().filter_map(|r| r.get("text").and_then(|t| t.as_str())).collect();
        if !texts.is_empty() {
            return Some(texts.join(""));
        }
    }
    None
}

fn parse_thumbnails(val: Option<&Value>) -> Vec<Thumbnail> {
    let mut list = Vec::new();
    let val = match val {
        Some(v) => v,
        None => return list,
    };

    if let Some(arr) = val.get("thumbnails").and_then(|t| t.as_array()) {
        for item in arr {
            if let Some(url) = item.get("url").and_then(|u| u.as_str()) {
                list.push(Thumbnail {
                    url: url.to_string(),
                    width: item.get("width").and_then(|w| w.as_u64()).map(|w| w as u32).unwrap_or(0),
                    height: item.get("height").and_then(|h| h.as_u64()).map(|h| h as u32).unwrap_or(0),
                });
            }
        }
    } else if let Some(sources) = val.pointer("/image/sources").and_then(|s| s.as_array()) {
        for item in sources {
            if let Some(url) = item.get("url").and_then(|u| u.as_str()) {
                list.push(Thumbnail {
                    url: url.to_string(),
                    width: item.get("width").and_then(|w| w.as_u64()).map(|w| w as u32).unwrap_or(0),
                    height: item.get("height").and_then(|h| h.as_u64()).map(|h| h as u32).unwrap_or(0),
                });
            }
        }
    }
    list
}

fn parse_duration_to_seconds(text: &str) -> Option<u64> {
    let parts: Vec<&str> = text.split(':').collect();
    match parts.len() {
        1 => parts[0].parse::<u64>().ok(),
        2 => {
            let m = parts[0].parse::<u64>().ok()?;
            let s = parts[1].parse::<u64>().ok()?;
            Some(m * 60 + s)
        }
        3 => {
            let h = parts[0].parse::<u64>().ok()?;
            let m = parts[1].parse::<u64>().ok()?;
            let s = parts[2].parse::<u64>().ok()?;
            Some(h * 3600 + m * 60 + s)
        }
        _ => None,
    }
}

fn parse_lockup_view_model(lvm: &Value) -> Option<RelatedVideo> {
    let content_id = lvm.get("contentId").and_then(|v| v.as_str());
    let tap_vid = lvm.pointer("/rendererContext/commandContext/onTap/innertubeCommand/watchEndpoint/videoId")
        .and_then(|v| v.as_str());
    let video_id = content_id.or(tap_vid)?.to_string();

    let meta = lvm.get("metadata")?.get("lockupMetadataViewModel")?;
    let title = meta.get("title").and_then(|t| t.get("content")).and_then(|v| v.as_str()).unwrap_or("Untitled").to_string();

    let mut author = String::new();
    let mut author_id = None;
    let mut view_count_text = None;
    let mut published_time_text = None;

    let rows_opt = meta.pointer("/metadata/contentMetadataViewModel/metadataRows")
        .or_else(|| meta.pointer("/metadata/metadataRows"))
        .and_then(|r| r.as_array());

    if let Some(rows) = rows_opt {
        if let Some(row0) = rows.first() {
            if let Some(part0) = row0.pointer("/metadataParts/0/text") {
                author = part0.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
                author_id = part0.pointer("/commandRuns/0/onTap/innertubeCommand/browseEndpoint/browseId")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
        }
        if rows.len() > 1 {
            if let Some(parts) = rows[1].get("metadataParts").and_then(|p| p.as_array()) {
                if let Some(p0) = parts.first() {
                    view_count_text = p0.pointer("/text/content").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
                if parts.len() > 1 {
                    published_time_text = parts[1].pointer("/text/content").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
            }
        }
    }

    // Thumbnails & Duration
    let mut thumbnails = Vec::new();
    let mut duration_text = None;
    let mut is_live = false;

    if let Some(content_img) = lvm.get("contentImage") {
        let tvm_opt = content_img.get("thumbnailViewModel")
            .or_else(|| content_img.pointer("/collectionThumbnailViewModel/primaryThumbnail/thumbnailViewModel"));

        if let Some(tvm) = tvm_opt {
            thumbnails = parse_thumbnails(Some(tvm));
            if let Some(overlays) = tvm.get("overlays").and_then(|o| o.as_array()) {
                for overlay in overlays {
                    let badge_text = overlay.pointer("/thumbnailBottomOverlayViewModel/badges/0/thumbnailBadgeViewModel/text")
                        .or_else(|| overlay.pointer("/thumbnailOverlayBadgeViewModel/thumbnailBadges/0/thumbnailBadgeViewModel/text"))
                        .or_else(|| overlay.pointer("/thumbnailOverlayTimeStatusRenderer/text/simpleText"))
                        .and_then(|v| v.as_str());

                    if let Some(b_str) = badge_text {
                        if b_str.eq_ignore_ascii_case("LIVE") {
                            is_live = true;
                        } else {
                            duration_text = Some(b_str.to_string());
                        }
                        break;
                    }
                }
            }
        }
    }

    let duration_seconds = duration_text.as_deref().and_then(parse_duration_to_seconds);

    Some(RelatedVideo {
        video_id,
        title,
        author,
        author_id,
        duration_text,
        duration_seconds,
        thumbnails,
        view_count_text,
        published_time_text,
        is_live,
        is_upcoming: false,
    })
}

fn parse_compact_video_renderer(cvr: &Value) -> Option<RelatedVideo> {
    let video_id = cvr.get("videoId").and_then(|v| v.as_str())?.to_string();
    let title = parse_text(cvr.get("title")).unwrap_or_else(|| "Untitled".to_string());
    let author = parse_text(cvr.get("shortBylineText")).unwrap_or_default();
    let author_id = cvr.pointer("/shortBylineText/runs/0/navigationEndpoint/browseEndpoint/browseId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let duration_text = parse_text(cvr.get("lengthText"));
    let duration_seconds = duration_text.as_deref().and_then(parse_duration_to_seconds);
    let thumbnails = parse_thumbnails(cvr.get("thumbnail"));
    let view_count_text = parse_text(cvr.get("viewCountText"));
    let published_time_text = parse_text(cvr.get("publishedTimeText"));

    let is_live = cvr.get("badges").and_then(|b| b.as_array()).is_some_and(|badges| {
        badges.iter().any(|badge| {
            badge.pointer("/metadataBadgeRenderer/style").and_then(|s| s.as_str()) == Some("BADGE_STYLE_TYPE_LIVE_NOW")
        })
    });

    Some(RelatedVideo {
        video_id,
        title,
        author,
        author_id,
        duration_text,
        duration_seconds,
        thumbnails,
        view_count_text,
        published_time_text,
        is_live,
        is_upcoming: false,
    })
}

fn parse_playlist_panel_video_renderer(pvr: &Value) -> Option<PlaylistPanelItem> {
    let video_id = pvr.get("videoId").and_then(|v| v.as_str())?.to_string();
    let title = parse_text(pvr.get("title")).unwrap_or_else(|| "Untitled".to_string());
    let author = parse_text(pvr.get("shortBylineText")).unwrap_or_default();
    let index = pvr.get("indexText").and_then(|i| i.as_str()).and_then(|s| s.parse::<usize>().ok());
    let is_selected = pvr.get("selected").and_then(|s| s.as_bool()).unwrap_or(false);
    let thumbnails = parse_thumbnails(pvr.get("thumbnail"));

    Some(PlaylistPanelItem {
        video_id,
        title,
        author,
        index,
        is_selected,
        thumbnails,
    })
}

fn parse_autoplay(val: &Value) -> Option<AutoplayVideo> {
    if let Some(cvr) = val.pointer("/autoplay/sets/0/nextVideoRenderer/compactVideoRenderer") {
        let video_id = cvr.get("videoId").and_then(|v| v.as_str())?.to_string();
        let title = parse_text(cvr.get("title")).unwrap_or_else(|| "Untitled".to_string());
        let author = parse_text(cvr.get("shortBylineText")).unwrap_or_default();
        let thumbnails = parse_thumbnails(cvr.get("thumbnail"));
        return Some(AutoplayVideo { video_id, title, author, thumbnails });
    }
    if let Some(av) = val.pointer("/sets/0/autoplayVideo") {
        let video_id = av.pointer("/watchEndpoint/videoId").and_then(|v| v.as_str())?.to_string();
        let title = parse_text(av.get("title")).unwrap_or_else(|| "Untitled".to_string());
        let author = parse_text(av.get("shortBylineText")).unwrap_or_default();
        let thumbnails = parse_thumbnails(av.get("thumbnail"));
        return Some(AutoplayVideo { video_id, title, author, thumbnails });
    }
    None
}

fn parse_continuation_token(item: &Value) -> Option<String> {
    if let Some(cir) = item.get("continuationItemRenderer") {
        return cir.pointer("/continuationEndpoint/continuationCommand/token")
            .or_else(|| cir.pointer("/button/buttonRenderer/command/continuationCommand/token"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_watch_next_compact_renderer() {
        let fixture = json!({
            "contents": {
                "twoColumnWatchNextResults": {
                    "results": {
                        "results": {
                            "contents": [
                                {
                                    "videoPrimaryInfoRenderer": {
                                        "title": { "simpleText": "Current Video Title" }
                                    }
                                },
                                {
                                    "videoSecondaryInfoRenderer": {
                                        "owner": {
                                            "videoOwnerRenderer": {
                                                "title": { "simpleText": "Channel Author" }
                                            }
                                        }
                                    }
                                }
                            ]
                        }
                    },
                    "secondaryResults": {
                        "secondaryResults": {
                            "results": [
                                {
                                    "compactVideoRenderer": {
                                        "videoId": "testVid123",
                                        "title": { "simpleText": "Recommended Video 1" },
                                        "shortBylineText": { "simpleText": "Awesome Creator" },
                                        "lengthText": { "simpleText": "3:45" },
                                        "viewCountText": { "simpleText": "50K views" },
                                        "publishedTimeText": { "simpleText": "2 days ago" }
                                    }
                                },
                                {
                                    "continuationItemRenderer": {
                                        "continuationEndpoint": {
                                            "continuationCommand": {
                                                "token": "NEXT_PAGE_TOKEN_123"
                                            }
                                        }
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        });

        let results = parse_watch_next_response("currentVid123", &fixture).expect("Failed to parse fixture");
        assert_eq!(results.current_video_id, "currentVid123");
        assert_eq!(results.current_title.as_deref(), Some("Current Video Title"));
        assert_eq!(results.current_author.as_deref(), Some("Channel Author"));
        assert_eq!(results.related_videos.len(), 1);
        assert_eq!(results.related_videos[0].video_id, "testVid123");
        assert_eq!(results.related_videos[0].title, "Recommended Video 1");
        assert_eq!(results.related_videos[0].author, "Awesome Creator");
        assert_eq!(results.related_videos[0].duration_text.as_deref(), Some("3:45"));
        assert_eq!(results.related_videos[0].duration_seconds, Some(225));
        assert_eq!(results.continuation_token.as_deref(), Some("NEXT_PAGE_TOKEN_123"));
    }

    #[test]
    fn test_parse_playlist_panel_items() {
        let fixture = json!({
            "contents": {
                "twoColumnWatchNextResults": {
                    "playlist": {
                        "playlist": {
                            "contents": [
                                {
                                    "playlistPanelVideoRenderer": {
                                        "videoId": "pvid1",
                                        "title": { "simpleText": "Track 1" },
                                        "shortBylineText": { "simpleText": "Artist 1" },
                                        "indexText": "1",
                                        "selected": true
                                    }
                                },
                                {
                                    "playlistPanelVideoRenderer": {
                                        "videoId": "pvid2",
                                        "title": { "simpleText": "Track 2" },
                                        "shortBylineText": { "simpleText": "Artist 2" },
                                        "indexText": "2",
                                        "selected": false
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        });

        let results = parse_watch_next_response("pvid1", &fixture).expect("Failed to parse playlist panel");
        assert_eq!(results.playlist_items.len(), 2);
        assert_eq!(results.playlist_items[0].video_id, "pvid1");
        assert_eq!(results.playlist_items[0].title, "Track 1");
        assert!(results.playlist_items[0].is_selected);
        assert_eq!(results.playlist_items[1].video_id, "pvid2");
        assert!(!results.playlist_items[1].is_selected);
    }
}
