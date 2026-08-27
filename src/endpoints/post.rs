use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::comments::{CommentsResult, PostCommentSort};
use crate::models::post::CommunityPostsResponse;
use crate::parser::{NodeListExt, Parser};
use crate::utils::proto::{
    encode_community_post_comments_continuation, encode_community_post_params,
};

/// Fetch the Community Post detail page using the same protobuf browse parameters as YouTube.js.
pub async fn get_post(
    session: &Session,
    post_id: &str,
    channel_id: &str,
) -> Result<CommunityPostsResponse> {
    let params = encode_community_post_params(post_id, channel_id)?;
    let response = session
        .post_innertube(
            "/browse",
            json!({
                "browseId": "FEpost_detail",
                "params": params,
            }),
        )
        .await?;
    let raw: Value = response.json().await.map_err(InnertubeError::Network)?;
    parse_post_response(&raw)
}

/// Parse a Community Post detail response.
pub fn parse_post_response(raw: &Value) -> Result<CommunityPostsResponse> {
    let tree = Parser::parse_tree(raw);
    let mut posts = Vec::new();
    for node in &tree {
        match node {
            crate::parser::YTNode::Post(p) => posts.push(p.post.clone()),
            crate::parser::YTNode::BackstagePost(bp) => {
                posts.push(crate::models::post::CommunityPost {
                    id: bp.id.clone().unwrap_or_default(),
                    author: bp.author_text.as_ref().map(|t| crate::parser::nodes::misc::author::AuthorNode {
                        id: None,
                        name: t.text.clone(),
                        url: None,
                        thumbnails: bp.author_thumbnail.clone().unwrap_or_default(),
                        is_verified: false,
                        is_verified_artist: false,
                    }),
                    content_text: bp.content.as_ref().map(|t| t.text.clone()).unwrap_or_default(),
                    published_time: bp.published.as_ref().map(|t| t.text.clone()),
                    vote_count: bp.vote_count.as_ref().map(|t| t.text.clone()),
                    comment_count: None,
                    poll: None,
                    images: Vec::new(),
                    video_id: None,
                });
            }
            _ => {}
        }
    }
    Ok(CommunityPostsResponse {
        posts,
        continuation_token: tree.find_continuation_token(),
    })
}

/// Fetch comments attached to a Community Post.
pub async fn get_post_comments(
    session: &Session,
    post_id: &str,
    channel_id: &str,
    sort: PostCommentSort,
) -> Result<CommentsResult> {
    let continuation = encode_community_post_comments_continuation(post_id, channel_id, sort)?;
    let response = session
        .post_innertube(
            "/browse",
            json!({
                "continuation": continuation,
            }),
        )
        .await?;
    let raw: Value = response.json().await.map_err(InnertubeError::Network)?;
    crate::endpoints::comments::parse_comments_response(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_post_detail_response() {
        let fixture = json!({
            "backstagePostThreadRenderer": {
                "post": {
                    "backstagePostRenderer": {
                        "postId": "UgkxMjM0NTY3ODkw",
                        "contentText": { "simpleText": "A post" }
                    }
                }
            },
            "continuationItemRenderer": {
                "continuationEndpoint": { "continuationCommand": { "token": "next" } }
            }
        });

        let parsed = parse_post_response(&fixture).expect("fixture should parse");
        assert_eq!(parsed.posts.len(), 1);
        assert_eq!(parsed.posts[0].content_text, "A post");
        assert_eq!(parsed.continuation_token.as_deref(), Some("next"));
    }
}
