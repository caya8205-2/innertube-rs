use serde::{Deserialize, Serialize};
use crate::parser::nodes::misc::author::AuthorNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// An option / choice in a community poll (`Poll.ts`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PollChoice {
    pub text: String,
    pub vote_percentage: Option<String>,
    pub vote_ratio: Option<f64>,
    pub image: Option<ThumbnailListNode>,
}

/// A poll attached to a community post (`Poll.ts`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommunityPoll {
    pub poll_type: Option<String>,
    pub total_votes_text: Option<String>,
    pub choices: Vec<PollChoice>,
}

/// An image attached to a community post (`BackstageImage.ts` / `PostMultiImage.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostImage {
    pub thumbnails: ThumbnailListNode,
    pub accessibility_text: Option<String>,
}

/// A YouTube Community Post (`BackstagePost.ts` / `Post.ts`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommunityPost {
    pub id: String,
    pub author: Option<AuthorNode>,
    pub content_text: String,
    pub published_time: Option<String>,
    pub vote_count: Option<String>,
    pub comment_count: Option<String>,
    pub poll: Option<CommunityPoll>,
    pub images: Vec<PostImage>,
    pub video_id: Option<String>,
}

/// Response containing a list of community posts and continuation token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommunityPostsResponse {
    pub posts: Vec<CommunityPost>,
    pub continuation_token: Option<String>,
}
