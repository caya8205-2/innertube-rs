use serde_json::{json, Value};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::search::{SearchResultItem, SearchResults, SearchVideoItem, SearchChannelItem, SearchPlaylistItem};
use crate::models::video::Thumbnail;
use crate::parser::{NodeListExt, Parser, YTNode};

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
    let parsed_tree = Parser::parse_tree(&val);
    let mut items = Vec::new();

    for node in &parsed_tree {
        match node {
            YTNode::Video(v) => {
                items.push(SearchResultItem::Video(SearchVideoItem {
                    video_id: v.id.clone(),
                    title: v.title.clone(),
                    author: v.author.as_ref().map(|a| a.name.clone()).unwrap_or_default(),
                    channel_id: v.author.as_ref().and_then(|a| a.id.clone()).unwrap_or_default(),
                    duration: v.duration.clone(),
                    view_count: v.view_count.clone(),
                    published_time: v.published_time.clone(),
                    thumbnails: v.thumbnails.thumbnails.iter().map(|t| Thumbnail {
                        url: t.url.clone(),
                        width: t.width.unwrap_or(0),
                        height: t.height.unwrap_or(0),
                    }).collect(),
                }));
            }
            YTNode::ChannelCard(c) => {
                items.push(SearchResultItem::Channel(SearchChannelItem {
                    channel_id: c.id.clone(),
                    title: c.title.clone(),
                    subscriber_count: c.subscriber_count.clone(),
                    video_count: c.video_count.clone(),
                    thumbnails: c.avatar.thumbnails.iter().map(|t| Thumbnail {
                        url: t.url.clone(),
                        width: t.width.unwrap_or(0),
                        height: t.height.unwrap_or(0),
                    }).collect(),
                }));
            }
            YTNode::Playlist(p) => {
                items.push(SearchResultItem::Playlist(SearchPlaylistItem {
                    playlist_id: p.id.clone(),
                    title: p.title.clone(),
                    author: p.author.as_ref().map(|a| a.name.clone()).unwrap_or_default(),
                    video_count: p.video_count.map(|c| format!("{} videos", c)),
                    thumbnails: p.thumbnails.thumbnails.iter().map(|t| Thumbnail {
                        url: t.url.clone(),
                        width: t.width.unwrap_or(0),
                        height: t.height.unwrap_or(0),
                    }).collect(),
                }));
            }
            _ => {}
        }
    }

    let continuation_token = parsed_tree.find_continuation_token();

    Ok(SearchResults {
        query: query.to_string(),
        items,
        continuation_token,
    })
}
