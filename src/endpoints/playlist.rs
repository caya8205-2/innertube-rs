use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::Result;
use crate::models::playlist::{PlaylistContinuation, PlaylistVideoItem, PlaylistView};
use crate::parser::nodes::playlist::PlaylistNode;
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
    let header_node = PlaylistNode::from_value(raw);
    let parsed_tree = Parser::parse_tree(raw);

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
    };

    for node in &parsed_tree {
        match node {
            YTNode::PlaylistVideo(pv) => {
                view.videos.push(PlaylistVideoItem {
                    id: pv.id.clone(),
                    title: pv.title.clone(),
                    author: pv.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: pv.author.as_ref().and_then(|a| a.id.clone()),
                    duration: pv.duration.clone(),
                    duration_ms: pv.duration_ms,
                    thumbnail: pv.thumbnails.best_url().map(|s| s.to_string()),
                    index: pv.index,
                    is_playable: pv.is_playable,
                });
            }
            YTNode::Video(v) => {
                view.videos.push(PlaylistVideoItem {
                    id: v.id.clone(),
                    title: v.title.clone(),
                    author: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: v.author.as_ref().and_then(|a| a.id.clone()),
                    duration: v.duration.clone(),
                    duration_ms: v.duration_ms,
                    thumbnail: v.thumbnails.best_url().map(|s| s.to_string()),
                    index: None,
                    is_playable: true,
                });
            }
            _ => {}
        }
    }

    Ok(view)
}

/// Parse continuation response into `PlaylistContinuation` using modular AST nodes.
pub fn parse_playlist_continuation_response(raw: &Value) -> Result<PlaylistContinuation> {
    let parsed_tree = Parser::parse_tree(raw);
    let mut result = PlaylistContinuation::default();

    for node in &parsed_tree {
        match node {
            YTNode::PlaylistVideo(pv) => {
                result.videos.push(PlaylistVideoItem {
                    id: pv.id.clone(),
                    title: pv.title.clone(),
                    author: pv.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: pv.author.as_ref().and_then(|a| a.id.clone()),
                    duration: pv.duration.clone(),
                    duration_ms: pv.duration_ms,
                    thumbnail: pv.thumbnails.best_url().map(|s| s.to_string()),
                    index: pv.index,
                    is_playable: pv.is_playable,
                });
            }
            YTNode::Video(v) => {
                result.videos.push(PlaylistVideoItem {
                    id: v.id.clone(),
                    title: v.title.clone(),
                    author: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string()),
                    author_id: v.author.as_ref().and_then(|a| a.id.clone()),
                    duration: v.duration.clone(),
                    duration_ms: v.duration_ms,
                    thumbnail: v.thumbnails.best_url().map(|s| s.to_string()),
                    index: None,
                    is_playable: true,
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
                                                    "contents": [
                                                        {
                                                            "playlistVideoRenderer": {
                                                                "videoId": "vid123",
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
        assert_eq!(pl.continuation_token.as_deref(), Some("token_next_page_123"));
    }
}
