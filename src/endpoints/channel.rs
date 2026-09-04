use serde_json::json;
use serde_json::Value;

use crate::core::session::Session;
use crate::error::Result;
use crate::models::channel::{
    ChannelAbout, ChannelShortItem, ChannelShortsResponse, ChannelVideoItem, ChannelVideosResponse,
};
use crate::parser::nodes::channel::ChannelHeaderNode;
use crate::parser::{NodeListExt, Parser};

/// Follow a `navigateAction` redirect on a browse response (legacy
/// `Innertube.getChannel` / `Actions.execute` redirect handling).
async fn follow_channel_redirect(session: &Session, raw: Value) -> Result<Value> {
    crate::core::actions::follow_navigate_redirect(session, raw).await
}

/// Fetch channel profile and about details.
pub async fn get_channel_about(session: &Session, channel_id: &str) -> Result<ChannelAbout> {
    let payload = json!({
        "browseId": channel_id,
    });

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;
    let raw = follow_channel_redirect(session, raw).await?;

    parse_channel_about_response(channel_id, &raw)
}

/// Fetch channel videos (Videos tab) with pagination support.
pub async fn get_channel_videos(
    session: &Session,
    channel_id: &str,
    continuation_token: Option<&str>,
) -> Result<ChannelVideosResponse> {
    let payload = if let Some(token) = continuation_token {
        json!({
            "continuation": token,
        })
    } else {
        json!({
            "browseId": channel_id,
            "params": "EgZ2aWRlb3PyBgQKAjoA", // Videos tab
        })
    };

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;
    let raw = if continuation_token.is_none() {
        follow_channel_redirect(session, raw).await?
    } else {
        raw
    };

    parse_channel_videos_response(channel_id, &raw)
}

/// Fetch channel shorts (Shorts tab) with pagination support.
pub async fn get_channel_shorts(
    session: &Session,
    channel_id: &str,
    continuation_token: Option<&str>,
) -> Result<ChannelShortsResponse> {
    let payload = if let Some(token) = continuation_token {
        json!({
            "continuation": token,
        })
    } else {
        json!({
            "browseId": channel_id,
            "params": "EgZzaG9ydHPyBgUKA5oBAA%3D%3D", // Shorts tab
        })
    };

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;
    let raw = if continuation_token.is_none() {
        follow_channel_redirect(session, raw).await?
    } else {
        raw
    };

    parse_channel_shorts_response(channel_id, &raw)
}

/// Parse channel about / header response using modular AST node.
pub fn parse_channel_about_response(channel_id: &str, raw: &Value) -> Result<ChannelAbout> {
    let header_node = ChannelHeaderNode::from_value(raw);

    let mut about = ChannelAbout {
        channel_id: channel_id.to_string(),
        title: header_node.as_ref().map(|h| h.title.clone()).unwrap_or_else(|| "Unknown Channel".to_string()),
        subscriber_count: header_node.as_ref().and_then(|h| h.subscriber_count.clone()),
        custom_url: header_node.as_ref().and_then(|h| h.handle.clone()),
        description: header_node.as_ref().and_then(|h| h.description.clone()),
        avatar: header_node.as_ref().and_then(|h| h.avatar.best_url()).map(|s| s.to_string()),
        banner: header_node.as_ref().and_then(|h| h.banner.best_url()).map(|s| s.to_string()),
        ..Default::default()
    };

    // Microformat / metadata fallback
    if let Some(micro) = raw.pointer("/microformat/microformatDataRenderer") {
        if about.title.is_empty() || about.title == "Unknown Channel" {
            if let Some(t) = micro.get("title").and_then(|t| t.as_str()) {
                about.title = t.to_string();
            }
        }
        if about.description.is_none() {
            about.description = micro.get("description").and_then(|d| d.as_str()).map(|s| s.to_string());
        }
    }

    Ok(about)
}

/// Parse channel videos response into `ChannelVideosResponse` using modular AST nodes.
pub fn parse_channel_videos_response(channel_id: &str, raw: &Value) -> Result<ChannelVideosResponse> {
    let parsed_tree = Parser::parse_tree(raw);
    let mut resp = ChannelVideosResponse {
        channel_id: channel_id.to_string(),
        ..Default::default()
    };

    for v in parsed_tree.find_videos() {
        resp.videos.push(ChannelVideoItem {
            video_id: v.id.clone(),
            title: v.title.clone(),
            published_time: v.published_time.clone(),
            duration: v.duration.clone(),
            views: v.view_count.clone(),
            thumbnail: v.thumbnails.best_url().map(|s| s.to_string()),
        });
    }

    resp.continuation_token = parsed_tree.find_continuation_token();
    Ok(resp)
}

/// Parse channel shorts response into `ChannelShortsResponse` using modular AST nodes.
pub fn parse_channel_shorts_response(channel_id: &str, raw: &Value) -> Result<ChannelShortsResponse> {
    let parsed_tree = Parser::parse_tree(raw);
    let mut resp = ChannelShortsResponse {
        channel_id: channel_id.to_string(),
        ..Default::default()
    };

    for s in parsed_tree.find_shorts() {
        resp.shorts.push(ChannelShortItem {
            video_id: s.id.clone(),
            title: s.title.clone(),
            views: s.view_count.clone(),
            thumbnail: s.thumbnails.best_url().map(|u| u.to_string()),
        });
    }

    resp.continuation_token = parsed_tree.find_continuation_token();
    Ok(resp)
}

/// Fetch channel community posts (Community tab) with pagination support.
pub async fn get_channel_community(
    session: &Session,
    channel_id: &str,
    continuation_token: Option<&str>,
) -> Result<crate::models::post::CommunityPostsResponse> {
    let payload = if let Some(token) = continuation_token {
        json!({
            "continuation": token,
        })
    } else {
        json!({
            "browseId": channel_id,
            "params": "Egljb21tdW5pdHkyBgQKAjoA", // Community tab
        })
    };

    let resp: reqwest::Response = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await?;
    let raw = if continuation_token.is_none() {
        follow_channel_redirect(session, raw).await?
    } else {
        raw
    };

    parse_channel_community_response(&raw)
}

/// Parse channel community posts response into `CommunityPostsResponse` using modular AST nodes.
pub fn parse_channel_community_response(raw: &Value) -> Result<crate::models::post::CommunityPostsResponse> {
    let parsed_tree = Parser::parse_tree(raw);
    let mut resp = crate::models::post::CommunityPostsResponse::default();

    for p in parsed_tree.find_posts() {
        resp.posts.push(p.post.clone());
    }

    resp.continuation_token = parsed_tree.find_continuation_token();
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_channel_about_fixture() {
        let fixture = json!({
            "header": {
                "c4TabbedHeaderRenderer": {
                    "title": "YOASOBI",
                    "subscriberCountText": { "simpleText": "6.5M subscribers" },
                    "avatar": { "thumbnails": [{ "url": "https://avatar.jpg" }] },
                    "banner": { "thumbnails": [{ "url": "https://banner.jpg" }] }
                }
            },
            "microformat": {
                "microformatDataRenderer": {
                    "description": "Official YOASOBI Channel."
                }
            }
        });

        let about = parse_channel_about_response("UCbqY3RHKkPS8dJCrfAfSk6Q", &fixture).unwrap();
        assert_eq!(about.title, "YOASOBI");
        assert_eq!(about.subscriber_count.as_deref(), Some("6.5M subscribers"));
        assert_eq!(about.description.as_deref(), Some("Official YOASOBI Channel."));
        assert_eq!(about.avatar.as_deref(), Some("https://avatar.jpg"));
        assert_eq!(about.banner.as_deref(), Some("https://banner.jpg"));
    }
}
