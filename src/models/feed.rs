use serde::{Deserialize, Serialize};
use crate::parser::nodes::video::VideoNode;

/// A category filter chip in Home Feed (`ChipCloudChip.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterChip {
    pub text: String,
    pub params: Option<String>,
    pub is_selected: bool,
}

/// YouTube Home Feed (`HomeFeed.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HomeFeed {
    pub filter_chips: Vec<FilterChip>,
    pub videos: Vec<VideoNode>,
    pub continuation_token: Option<String>,
}

/// A tab in YouTube Trending page (Now, Music, Gaming, Movies).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrendingTab {
    pub title: String,
    pub params: Option<String>,
    pub is_selected: bool,
}

/// YouTube Trending page (`Explore.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrendingFeed {
    pub current_tab: String,
    pub tabs: Vec<TrendingTab>,
    pub videos: Vec<VideoNode>,
}

/// YouTube Hashtag page (`HashtagFeed.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HashtagFeed {
    pub hashtag: String,
    pub header_title: Option<String>,
    pub video_count_text: Option<String>,
    pub channel_count_text: Option<String>,
    pub videos: Vec<VideoNode>,
    pub continuation_token: Option<String>,
}
