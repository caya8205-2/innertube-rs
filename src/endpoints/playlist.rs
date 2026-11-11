use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::playlist::{PlaylistContinuation, PlaylistVideoItem, PlaylistView};
use crate::parser::nodes::misc::alert::AlertNode;
use crate::parser::nodes::playlist::{is_mix_playlist_id, PlaylistNode};
use crate::parser::{NodeListExt, Parser, YTNode};

/// Fetch full YouTube playlist metadata and video list.
pub async fn get_playlist(session: &Session, playlist_id: &str) -> Result<PlaylistView> {
    let clean_id = if playlist_id.starts_with("VL") {
        playlist_id.to_string()
    } else {
        format!("VL{}", playlist_id)
    };

    let payload = json!({
        "browseId": clean_id,
    });

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_playlist_browse_response(&clean_id, &raw)
}

/// Fetch next page of playlist videos using a continuation token.
pub async fn get_playlist_continuation(
    session: &Session,
    continuation_token: &str,
) -> Result<PlaylistContinuation> {
    let payload = json!({
        "continuation": continuation_token,
    });

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;

    parse_playlist_continuation_response(&raw)
}

/// Parse playlist browse response into `PlaylistView` using modular AST nodes.
pub fn parse_playlist_browse_response(playlist_id: &str, raw: &Value) -> Result<PlaylistView> {
    // 1. Detect error alerts (e.g. "The playlist does not exist.")
    if let Some(alerts) = raw.get("alerts").and_then(|a| a.as_array()) {
        for alert_val in alerts {
            if let Some(alert) = AlertNode::from_value(alert_val) {
                if alert.alert_type.as_deref() == Some("ERROR") {
                    return Err(InnertubeError::NotFound(alert.text));
                }
            }
        }
    }

    // 2. Parse playlist metadata from AST header node
    let header_node = PlaylistNode::from_value(raw);
    let parsed_tree = Parser::parse_tree(raw);

    // The requested ID is the ground truth for Mix detection: a /browse response
    // for VLRD... is a Mix even when the header node is missing.
    let is_mix = header_node.as_ref().is_some_and(|h| h.is_mix) || is_mix_playlist_id(playlist_id);

    let mut view = PlaylistView {
        id: playlist_id.to_string(),
        title: header_node.as_ref().map(|h| h.title.clone()).unwrap_or_else(|| "Untitled Playlist".to_string()),
        author: header_node.as_ref().and_then(|h| h.author.as_ref()).map(|a| a.name.clone()),
        author_id: header_node.as_ref().and_then(|h| h.author.as_ref()).and_then(|a| a.id.clone()),
        description: header_node.as_ref().and_then(|h| h.description.clone()),
        video_count: header_node.as_ref().and_then(|h| h.video_count),
        view_count: header_node.as_ref().and_then(|h| h.view_count.clone()),
        last_updated: header_node.as_ref().and_then(|h| h.last_updated.clone()),
        thumbnail: header_node.as_ref().and_then(|h| h.thumbnails.best_url().map(|s| s.to_string())),
        videos: Vec::new(),
        continuation_token: parsed_tree.find_continuation_token(),
        is_editable: !is_mix && find_playlist_video_list_editable(raw),
        is_mix,
    };

    // 3. Collect video items and assign 1-based index if missing
    for node in &parsed_tree {
        match node {
            YTNode::PlaylistVideo(pv) => {
                let next_idx = (view.videos.len() + 1) as u32;
                view.videos.push(PlaylistVideoItem {
                    id: pv.id.clone(),
                    title: pv.title.clone(),
                    author: pv.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: pv.author.as_ref().and_then(|a| a.id.clone()),
                    duration: pv.duration.clone(),
                    duration_ms: pv.duration_ms,
                    thumbnail: pv.thumbnails.best_url().map(|s| s.to_string()),
                    index: pv.index.or(Some(next_idx)),
                    is_playable: pv.is_playable,
                    set_video_id: pv.set_video_id.clone(),
                });
            }
            YTNode::Video(v) => {
                let next_idx = (view.videos.len() + 1) as u32;
                view.videos.push(PlaylistVideoItem {
                    id: v.id.clone(),
                    title: v.title.clone(),
                    author: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: v.author.as_ref().and_then(|a| a.id.clone()),
                    duration: v.duration.clone(),
                    duration_ms: v.duration_ms,
                    thumbnail: v.thumbnails.best_url().map(|s| s.to_string()),
                    index: Some(next_idx),
                    is_playable: true,
                    set_video_id: None,
                });
            }
            _ => {}
        }
    }

    Ok(view)
}

/// Recursively locate `playlistVideoListRenderer.isEditable`.
fn find_playlist_video_list_editable(raw: &Value) -> bool {
    if let Some(list) = raw.get("playlistVideoListRenderer") {
        if let Some(editable) = list.get("isEditable").and_then(Value::as_bool) {
            return editable;
        }
    }
    match raw {
        Value::Object(map) => map.values().any(find_playlist_video_list_editable),
        Value::Array(items) => items.iter().any(find_playlist_video_list_editable),
        _ => false,
    }
}

/// Parse continuation response into `PlaylistContinuation` using modular AST nodes.
pub fn parse_playlist_continuation_response(raw: &Value) -> Result<PlaylistContinuation> {
    let parsed_tree = Parser::parse_tree(raw);
    let mut result = PlaylistContinuation::default();

    for node in &parsed_tree {
        match node {
            YTNode::PlaylistVideo(pv) => {
                let next_idx = (result.videos.len() + 1) as u32;
                result.videos.push(PlaylistVideoItem {
                    id: pv.id.clone(),
                    title: pv.title.clone(),
                    author: pv.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: pv.author.as_ref().and_then(|a| a.id.clone()),
                    duration: pv.duration.clone(),
                    duration_ms: pv.duration_ms,
                    thumbnail: pv.thumbnails.best_url().map(|s| s.to_string()),
                    index: pv.index.or(Some(next_idx)),
                    is_playable: pv.is_playable,
                    set_video_id: pv.set_video_id.clone(),
                });
            }
            YTNode::Video(v) => {
                let next_idx = (result.videos.len() + 1) as u32;
                result.videos.push(PlaylistVideoItem {
                    id: v.id.clone(),
                    title: v.title.clone(),
                    author: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: v.author.as_ref().and_then(|a| a.id.clone()),
                    duration: v.duration.clone(),
                    duration_ms: v.duration_ms,
                    thumbnail: v.thumbnails.best_url().map(|s| s.to_string()),
                    index: Some(next_idx),
                    is_playable: true,
                    set_video_id: None,
                });
            }
            _ => {}
        }
    }

    result.continuation_token = parsed_tree.find_continuation_token();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_playlist_fixture() {
        let fixture = json!({
            "header": {
                "playlistHeaderRenderer": {
                    "title": { "runs": [{ "text": "My Top Playlist" }] },
                    "ownerText": { "runs": [{ "text": "Caya Dev", "navigationEndpoint": { "browseEndpoint": { "browseId": "UC12345" } } }] },
                    "numVideosText": { "runs": [{ "text": "25 videos" }] }
                }
            },
            "contents": {
                "twoColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "itemSectionRenderer": {
                                            "contents": [{
                                                "playlistVideoListRenderer": {
                                                    "isEditable": true,
                                                    "contents": [
                                                        {
                                                            "playlistVideoRenderer": {
                                                                "videoId": "vid123",
                                                                "setVideoId": "set_vid123",
                                                                "title": { "runs": [{ "text": "Track 1" }] },
                                                                "shortBylineText": { "runs": [{ "text": "Artist 1" }] },
                                                                "lengthText": { "simpleText": "3:45" },
                                                                "lengthSeconds": "225"
                                                            }
                                                        },
                                                        {
                                                            "continuationItemRenderer": {
                                                                "continuationEndpoint": {
                                                                    "continuationCommand": {
                                                                        "token": "token_next_page_123"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    ]
                                                }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            }
        });

        let pl = parse_playlist_browse_response("PL_test_123", &fixture).unwrap();
        assert_eq!(pl.title, "My Top Playlist");
        assert_eq!(pl.author.as_deref(), Some("Caya Dev"));
        assert_eq!(pl.video_count, Some(25));
        assert_eq!(pl.videos.len(), 1);
        assert_eq!(pl.videos[0].id, "vid123");
        assert_eq!(pl.videos[0].title, "Track 1");
        assert_eq!(pl.videos[0].duration.as_deref(), Some("3:45"));
        assert_eq!(pl.videos[0].duration_ms, Some(225_000));
        assert_eq!(pl.videos[0].set_video_id.as_deref(), Some("set_vid123"));
        assert!(pl.is_editable);
        assert_eq!(pl.continuation_token.as_deref(), Some("token_next_page_123"));
    }

    #[test]
    fn test_parse_playlist_error_alert() {
        let fixture = json!({
            "alerts": [
                {
                    "alertRenderer": {
                        "type": "ERROR",
                        "text": {
                            "runs": [
                                { "text": "The playlist does not exist." }
                            ]
                        }
                    }
                }
            ]
        });

        let err = parse_playlist_browse_response("PL_invalid", &fixture).unwrap_err();
        match err {
            InnertubeError::NotFound(msg) => assert_eq!(msg, "The playlist does not exist."),
            other => panic!("Expected NotFound error, got: {:?}", other),
        }
    }

    #[test]
    fn test_parse_playlist_modern_viewmodel_and_sidebar() {
        let fixture = json!({
            "sidebar": {
                "playlistSidebarRenderer": {
                    "items": [
                        {
                            "playlistSidebarPrimaryInfoRenderer": {
                                "title": { "runs": [{ "text": "Modern Top Hits" }] },
                                "stats": [
                                    { "runs": [{ "text": "50" }, { "text": " videos" }] },
                                    { "simpleText": "1,234,567 views" },
                                    { "runs": [{ "text": "Updated today" }] }
                                ],
                                "thumbnailRenderer": {
                                    "playlistVideoThumbnailRenderer": {
                                        "thumbnail": {
                                            "thumbnails": [
                                                { "url": "https://i.ytimg.com/vi/thumb123/hqdefault.jpg", "width": 480, "height": 360 }
                                            ]
                                        }
                                    }
                                }
                            }
                        },
                        {
                            "playlistSidebarSecondaryInfoRenderer": {
                                "videoOwner": {
                                    "videoOwnerRenderer": {
                                        "title": { "runs": [{ "text": "Music Hub" }] },
                                        "navigationEndpoint": {
                                            "browseEndpoint": { "browseId": "UC_channel_123" }
                                        }
                                    }
                                }
                            }
                        }
                    ]
                }
            },
            "contents": {
                "twoColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "itemSectionRenderer": {
                                            "contents": [
                                                {
                                                    "lockupViewModel": {
                                                        "contentId": "track_1",
                                                        "contentType": "LOCKUP_CONTENT_TYPE_VIDEO",
                                                        "metadata": {
                                                            "lockupMetadataViewModel": {
                                                                "title": { "content": "Hit Track 1" },
                                                                "metadata": {
                                                                    "contentMetadataViewModel": {
                                                                        "metadataRows": [{
                                                                            "metadataParts": [{
                                                                                "text": { "content": "Artist A" }
                                                                            }]
                                                                        }]
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                {
                                                    "continuationItemViewModel": {
                                                        "continuationCommand": {
                                                            "innertubeCommand": {
                                                                "continuationCommand": {
                                                                    "token": "token_page_2_viewmodel"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            ]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            }
        });

        let pl = parse_playlist_browse_response("PL_modern_123", &fixture).unwrap();
        assert_eq!(pl.title, "Modern Top Hits");
        assert_eq!(pl.author.as_deref(), Some("Music Hub"));
        assert_eq!(pl.author_id.as_deref(), Some("UC_channel_123"));
        assert_eq!(pl.video_count, Some(50));
        assert_eq!(pl.view_count.as_deref(), Some("1,234,567 views"));
        assert_eq!(pl.last_updated.as_deref(), Some("Updated today"));
        assert_eq!(pl.thumbnail.as_deref(), Some("https://i.ytimg.com/vi/thumb123/hqdefault.jpg"));
        assert_eq!(pl.continuation_token.as_deref(), Some("token_page_2_viewmodel"));
        assert_eq!(pl.videos.len(), 1);
        assert_eq!(pl.videos[0].id, "track_1");
        assert_eq!(pl.videos[0].title, "Hit Track 1");
        assert_eq!(pl.videos[0].author, "Artist A");
        assert_eq!(pl.videos[0].index, Some(1)); // 1-based index assigned
        assert!(!pl.is_mix);
    }

    #[test]
    fn test_parse_mix_playlist_from_requested_id() {
        let fixture = json!({
            "header": {
                "playlistHeaderRenderer": {
                    "title": { "runs": [{ "text": "Mix - Caya Dev" }] }
                }
            },
            "contents": {
                "twoColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [{
                                        "itemSectionRenderer": {
                                            "contents": [{
                                                "playlistVideoListRenderer": {
                                                    "isEditable": true,
                                                    "contents": [{
                                                        "playlistVideoRenderer": {
                                                            "videoId": "8XXPxLLgwt8",
                                                            "title": { "runs": [{ "text": "Track 1" }] }
                                                        }
                                                    }]
                                                }
                                            }]
                                        }
                                    }]
                                }
                            }
                        }
                    }]
                }
            }
        });

        let pl = parse_playlist_browse_response("VLRDEM5KXlGSfIinpcCL9eVl4XVw", &fixture).unwrap();
        assert!(pl.is_mix);
        assert!(!pl.is_editable, "Mix playlists must never be editable");
    }

    #[test]
    fn test_parse_regular_playlist_ignores_mix_cards_in_body() {
        let fixture = json!({
            "header": {
                "playlistHeaderRenderer": {
                    "playlistId": "PL_regular_123",
                    "title": { "runs": [{ "text": "My Playlist" }] }
                }
            },
            "contents": {
                "gridRadioRenderer": {
                    "playlistId": "RDEM5KXlGSfIinpcCL9eVl4XVw",
                    "title": { "runs": [{ "text": "Mix - Something" }] }
                }
            }
        });

        let pl = parse_playlist_browse_response("VLPL_regular_123", &fixture).unwrap();
        assert!(!pl.is_mix);
    }
}
