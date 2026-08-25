use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::post::{CommunityPoll, CommunityPost, PollChoice, PostImage};
use crate::parser::nodes::misc::author::AuthorNode;
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;

/// AST Node for Community Posts (`BackstagePost.ts` / `Post.ts` / `SharedPost.ts`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostNode {
    pub post: CommunityPost,
}

impl PostNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("backstagePostRenderer")
            .or_else(|| val.get("postRenderer"))
            .or_else(|| val.get("sharedPostRenderer"))
            .or_else(|| val.pointer("/backstagePostThreadRenderer/post/backstagePostRenderer"))
            .unwrap_or(val);

        let id = node.get("postId")
            .or_else(|| node.get("id"))
            .and_then(Value::as_str)?
            .to_string();

        let author = node.get("authorText")
            .and_then(AuthorNode::from_value)
            .or_else(|| AuthorNode::from_value(node));

        let content_text = TextNode::from_value(node.get("contentText").unwrap_or(&Value::Null))
            .map(|t| t.text)
            .unwrap_or_default();

        let published_time = TextNode::from_value(node.get("publishedTimeText").unwrap_or(&Value::Null))
            .map(|t| t.text);

        let vote_count = TextNode::from_value(node.get("voteCount").unwrap_or(&Value::Null))
            .map(|t| t.text);

        let comment_count = TextNode::from_value(
            node.pointer("/actionButtons/commentActionButtonsRenderer/replyButton/buttonRenderer/text")
                .unwrap_or(&Value::Null),
        )
        .map(|t| t.text);

        let mut poll = None;
        let mut images = Vec::new();
        let mut video_id = None;

        // Extract Attachment (Poll, Images, Video)
        if let Some(att) = node.get("backstageAttachment") {
            // 1. Poll
            if let Some(p) = att.get("pollRenderer") {
                let mut choices = Vec::new();
                if let Some(ch_arr) = p.get("choices").and_then(|c| c.as_array()) {
                    for ch in ch_arr {
                        let text = TextNode::from_value(ch.get("text").unwrap_or(&Value::Null))
                            .map(|t| t.text)
                            .unwrap_or_default();

                        let vote_pct = TextNode::from_value(ch.get("votePercentageIfSelected").unwrap_or(&Value::Null))
                            .or_else(|| TextNode::from_value(ch.get("votePercentage").unwrap_or(&Value::Null)))
                            .map(|t| t.text);

                        let vote_ratio = ch.get("voteRatioIfSelected")
                            .or_else(|| ch.get("voteRatio"))
                            .and_then(Value::as_f64);

                        let img = ch.get("image").map(ThumbnailListNode::from_value);

                        choices.push(PollChoice {
                            text,
                            vote_percentage: vote_pct,
                            vote_ratio,
                            image: img,
                        });
                    }
                }

                let total_votes = TextNode::from_value(p.get("totalVotes").unwrap_or(&Value::Null)).map(|t| t.text);
                let poll_type = p.get("type").and_then(Value::as_str).map(|s| s.to_string());

                poll = Some(CommunityPoll {
                    poll_type,
                    total_votes_text: total_votes,
                    choices,
                });
            }

            // 2. Images (Single or Multi)
            if let Some(bi) = att.get("backstageImageRenderer") {
                let thumbs = ThumbnailListNode::from_value(bi.get("image").unwrap_or(bi));
                let a11y = bi.pointer("/accessibility/accessibilityData/label").and_then(Value::as_str).map(|s| s.to_string());
                images.push(PostImage {
                    thumbnails: thumbs,
                    accessibility_text: a11y,
                });
            } else if let Some(pmi) = att.get("postMultiImageRenderer") {
                if let Some(img_arr) = pmi.get("images").and_then(|i| i.as_array()) {
                    for item in img_arr {
                        if let Some(bi) = item.get("backstageImageRenderer") {
                            let thumbs = ThumbnailListNode::from_value(bi.get("image").unwrap_or(bi));
                            let a11y = bi.pointer("/accessibility/accessibilityData/label").and_then(Value::as_str).map(|s| s.to_string());
                            images.push(PostImage {
                                thumbnails: thumbs,
                                accessibility_text: a11y,
                            });
                        }
                    }
                }
            }

            // 3. Attached Video
            if let Some(vid) = att.pointer("/videoRenderer/videoId")
                .or_else(|| att.pointer("/compactVideoRenderer/videoId"))
                .and_then(Value::as_str)
            {
                video_id = Some(vid.to_string());
            }
        }

        Some(PostNode {
            post: CommunityPost {
                id,
                author,
                content_text,
                published_time,
                vote_count,
                comment_count,
                poll,
                images,
                video_id,
            },
        })
    }

    pub fn post_id(&self) -> &str {
        &self.post.id
    }

    pub fn author_name(&self) -> Option<&str> {
        self.post.author.as_ref().map(|a| a.name.as_str())
    }

    pub fn text(&self) -> &str {
        &self.post.content_text
    }
}

impl std::ops::Deref for PostNode {
    type Target = CommunityPost;

    fn deref(&self) -> &Self::Target {
        &self.post
    }
}

impl std::ops::DerefMut for PostNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.post
    }
}
