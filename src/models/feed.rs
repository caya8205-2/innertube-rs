use serde::{Deserialize, Serialize};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::parser::nodes::containers::ChipCloudChipNode;
use crate::parser::nodes::misc::carousels::ChipViewNode;
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::misc::panels::ListItemViewNode;
use crate::parser::nodes::video::VideoNode;
use crate::parser::nodes::{ChannelNode, PlaylistNode};
use crate::parser::YTNode;

/// Generic parsed feed matching YouTube.js `Feed<T>` mixin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Feed<T = serde_json::Value> {
    pub items: Vec<T>,
    pub videos: Vec<VideoNode>,
    pub channels: Vec<ChannelNode>,
    pub playlists: Vec<PlaylistNode>,
    pub continuation_token: Option<String>,
}

impl<T> Feed<T> {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }
}

/// Generic browse feed used by legacy account and discovery destinations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrowseFeed {
    pub browse_id: String,
    pub videos: Vec<VideoNode>,
    pub channels: Vec<ChannelNode>,
    pub playlists: Vec<PlaylistNode>,
    /// Community posts (legacy `Feed.posts`: Post, SharedPost,
    /// BackstagePost).
    pub posts: Vec<FeedPost>,
    pub continuation_token: Option<String>,
}

impl BrowseFeed {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch next batch of browse feed contents.
    pub async fn get_continuation(&self, session: &Session) -> Result<BrowseFeed> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            InnertubeError::NotFound("There are no continuations available for this BrowseFeed".into())
        })?;
        crate::endpoints::feed::get_browse_continuation(session, token).await
    }
}

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

impl HomeFeed {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch next batch of home feed videos.
    pub async fn get_continuation(&self, session: &Session) -> Result<HomeFeed> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            InnertubeError::NotFound("There are no continuations available for this HomeFeed".into())
        })?;
        crate::endpoints::feed::get_home_feed_continuation(session, token).await
    }

    /// Legacy `FilterableFeed.getFilteredFeed` for the home feed: `Ok(None)`
    /// when the chip is already selected; otherwise browses
    /// `FEwhat_to_watch` with the chip's params.
    pub async fn get_filtered_feed(
        &self,
        session: &Session,
        filter: &str,
    ) -> Result<Option<HomeFeed>> {
        let available: Vec<&str> = self.filter_chips.iter().map(|c| c.text.as_str()).collect();
        let chip = self
            .filter_chips
            .iter()
            .find(|c| c.text == filter)
            .ok_or_else(|| {
                InnertubeError::NotFound(format!(
                    "Filter '{filter}' not found. Available filters: {available:?}"
                ))
            })?;

        if chip.is_selected {
            return Ok(None);
        }

        crate::endpoints::feed::get_home_feed(session, chip.params.as_deref())
            .await
            .map(Some)
    }
}

/// A tab in YouTube Trending page (Now, Music, Gaming, Movies).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrendingTab {
    pub title: String,
    pub params: Option<String>,
    pub is_selected: bool,
    /// Legacy `endpoint.metadata.url` (e.g. `/feed/trending?bp=...`).
    pub url: Option<String>,
}

/// A primary filter node (legacy `ChipCloudChip | ChipView | ListItemView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterNode {
    Chip(Box<ChipCloudChipNode>),
    ChipView(ChipViewNode),
    ListItem(ListItemViewNode),
}

impl FilterNode {
    /// Display label (legacy: chip text or list item title).
    pub fn label(&self) -> String {
        match self {
            Self::Chip(c) => c.text.clone(),
            Self::ChipView(c) => c.text.clone().unwrap_or_default(),
            Self::ListItem(i) => i
                .title
                .as_ref()
                .map(|t| t.text.clone())
                .unwrap_or_default(),
        }
    }

    fn is_selected(&self) -> bool {
        match self {
            Self::Chip(c) => c.is_selected,
            Self::ChipView(c) => c.selected,
            Self::ListItem(i) => i.is_selected,
        }
    }

    fn endpoint(&self) -> Option<NavigationEndpointNode> {
        match self {
            Self::Chip(c) => c.endpoint.clone(),
            Self::ChipView(c) => c
                .tap_command
                .as_ref()
                .and_then(NavigationEndpointNode::from_value),
            Self::ListItem(i) => i
                .renderer_context
                .as_ref()
                .and_then(|rc| rc.pointer("/commandContext/onTap"))
                .and_then(NavigationEndpointNode::from_value),
        }
    }
}

/// A community post in a feed (legacy `Feed.posts` coverage: Post,
/// SharedPost, BackstagePost).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FeedPost {
    Post(crate::parser::nodes::post::PostNode),
    SharedPost(crate::parser::nodes::misc::music_shorts_misc::SharedPostNode),
    BackstagePost(crate::parser::nodes::misc::music_shorts_misc::BackstagePostNode),
}

/// Legacy `FilterableFeed` mixin: primary/secondary filter extraction and
/// filtered feed navigation.
#[derive(Debug, Clone, Default)]
pub struct FilterableFeed {
    pub primary_filters: Vec<FilterNode>,
    pub secondary_filters: Vec<ChipViewNode>,
    /// Current page nodes (needed when applying a secondary filter to an
    /// already-selected primary filter — legacy uses `this.page`).
    pub page_nodes: Vec<YTNode>,
}

impl FilterableFeed {
    /// Extract filter nodes from a parsed page tree (legacy
    /// `filter_nodes` getter rules).
    pub fn from_nodes(nodes: &[YTNode]) -> Result<Self> {
        let chipbars = nodes
            .iter()
            .filter(|n| matches!(n, YTNode::FeedFilterChipBar(_)))
            .count();
        if chipbars > 1 {
            return Err(InnertubeError::Other(
                "There are too many feed filter chipbars, you'll need to find the correct one yourself in this.page"
                    .to_string(),
            ));
        }

        let mut primary_filters = Vec::new();
        let mut secondary_filters = Vec::new();

        if chipbars == 1 {
            for node in nodes {
                if let YTNode::ChipCloudChip(chip) = node {
                    primary_filters.push(FilterNode::Chip(Box::new(chip.clone())));
                }
            }
        } else {
            let chips: Vec<&ChipViewNode> = nodes
                .iter()
                .filter_map(|n| match n {
                    YTNode::ChipView(c) => Some(c),
                    _ => None,
                })
                .collect();

            if let Some(first) = chips.first() {
                let has_dropdown = first.display_type.as_deref().is_some_and(|t| {
                    t == "CHIP_VIEW_MODEL_DISPLAY_TYPE_DROP_DOWN"
                        || t == "CHIP_VIEW_MODEL_DISPLAY_TYPE_DROP_DOWN_WITH_CLEAR"
                });

                if has_dropdown {
                    // Dropdown items: tap command -> showSheetCommand ->
                    // sheetView -> listView -> listItemView items.
                    if let Some(items) = first.tap_command.as_ref().and_then(|tc| {
                        tc.pointer(
                            "/innertubeCommand/showSheetCommand/inlineContent/sheetView/content/listView/items",
                        )
                        .or_else(|| {
                            tc.pointer(
                                "/showSheetCommand/inlineContent/sheetView/content/listView/items",
                            )
                        })
                    }).and_then(|v| v.as_array())
                    {
                        for item in items {
                            if let Some(list_item) = ListItemViewNode::from_value(item) {
                                primary_filters.push(FilterNode::ListItem(list_item));
                            }
                        }
                    }
                    secondary_filters = chips.iter().skip(1).map(|c| (*c).clone()).collect();
                } else {
                    primary_filters = chips.iter().map(|c| FilterNode::ChipView((*c).clone())).collect();
                }
            }
        }

        Ok(Self {
            primary_filters,
            secondary_filters,
            page_nodes: nodes.to_vec(),
        })
    }

    /// Available primary filter labels.
    pub fn filters(&self) -> Vec<String> {
        self.primary_filters.iter().map(FilterNode::label).collect()
    }

    /// Available secondary filter labels.
    pub fn secondary_filter_labels(&self) -> Vec<String> {
        self.secondary_filters
            .iter()
            .map(|c| c.text.clone().unwrap_or_default())
            .collect()
    }

    /// Apply a filter (legacy `getFilteredFeed`). Returns `Ok(None)` when
    /// the filter is already selected and no secondary filter was given
    /// (legacy returns `this`).
    pub async fn get_filtered_feed(
        &self,
        session: &Session,
        filter: &str,
        secondary_filter: Option<&str>,
    ) -> Result<Option<Vec<YTNode>>> {
        if !self.filters().iter().any(|f| f == filter) {
            return Err(InnertubeError::NotFound(format!(
                "Filter '{filter}' not found. Available filters: {:?}",
                self.filters()
            )));
        }

        let node = self
            .primary_filters
            .iter()
            .find(|n| n.label() == filter)
            .expect("filter existence checked above");
        let endpoint = node.endpoint().ok_or_else(|| {
            InnertubeError::Other(
                "Could not find endpoint for the specified filter".to_string(),
            )
        })?;
        let is_selected = node.is_selected();

        if is_selected && secondary_filter.is_none() {
            return Ok(None);
        }

        let nodes = if is_selected {
            None
        } else {
            Some(call_endpoint_nodes(session, &endpoint).await?)
        };

        if let Some(secondary) = secondary_filter {
            // Legacy: `response = isSelected ? this.page : await call(...)` —
            // an already-selected primary filter uses the current page as base.
            let base_nodes = match nodes {
                Some(n) => n,
                None => self.page_nodes.clone(),
            };
            let feed = FilterableFeed::from_nodes(&base_nodes)?;
            if !feed.secondary_filter_labels().iter().any(|f| f == secondary) {
                return Err(InnertubeError::NotFound(format!(
                    "Secondary filter '{secondary}' not found. Available filters: {:?}",
                    feed.secondary_filter_labels()
                )));
            }
            let secondary_node = feed
                .secondary_filters
                .iter()
                .find(|c| c.text.as_deref() == Some(secondary));
            if let Some(chip) = secondary_node {
                if !chip.selected {
                    let endpoint = chip
                        .tap_command
                        .as_ref()
                        .and_then(NavigationEndpointNode::from_value)
                        .ok_or_else(|| {
                            InnertubeError::Other(
                                "Could not find an endpoint for the specified secondary filter"
                                    .to_string(),
                            )
                        })?;
                    return Ok(Some(call_endpoint_nodes(session, &endpoint).await?));
                }
            }
            return Ok(Some(base_nodes));
        }

        Ok(nodes)
    }
}

/// Call a navigation endpoint and return the parsed node tree.
async fn call_endpoint_nodes(
    session: &Session,
    endpoint: &NavigationEndpointNode,
) -> Result<Vec<YTNode>> {
    let path = endpoint.api_path.as_deref().ok_or_else(|| {
        InnertubeError::NotFound("Filter endpoint has no InnerTube API path".to_string())
    })?;
    let resp = session
        .post_innertube(path, endpoint.payload.clone())
        .await?;
    let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;
    Ok(crate::parser::Parser::parse_tree(&raw))
}

/// YouTube Trending page (`Explore.ts`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrendingFeed {
    pub current_tab: String,
    pub tabs: Vec<TrendingTab>,
    pub videos: Vec<VideoNode>,
}

impl TrendingFeed {
    /// Legacy `TabbedFeed.title`: selected tab title.
    pub fn title(&self) -> Option<&str> {
        self.tabs
            .iter()
            .find(|t| t.is_selected)
            .map(|t| t.title.as_str())
    }

    /// Legacy `TabbedFeed.getTabByName` (case-insensitive; `Ok(None)` when
    /// the tab is already selected, i.e. legacy returns `this`).
    pub fn resolve_tab_by_name(&self, title: &str) -> Result<Option<&TrendingTab>> {
        let tab = self
            .tabs
            .iter()
            .find(|t| t.title.eq_ignore_ascii_case(title))
            .ok_or_else(|| InnertubeError::NotFound(format!("Tab \"{title}\" not found")))?;
        Ok((!tab.is_selected).then_some(tab))
    }

    /// Fetch the feed for a tab by name (legacy `getTabByName`); returns
    /// `None` when already selected.
    pub async fn get_tab_by_name(&self, session: &Session, title: &str) -> Result<Option<TrendingFeed>> {
        let Some(tab) = self.resolve_tab_by_name(title)? else {
            return Ok(None);
        };
        crate::endpoints::feed::get_trending(session, tab.params.as_deref())
            .await
            .map(Some)
    }

    /// Legacy `TabbedFeed.getTabByURL`: matches the last URL path segment.
    pub fn resolve_tab_by_url(&self, url: &str) -> Result<Option<&TrendingTab>> {
        let tab = self
            .tabs
            .iter()
            .find(|t| {
                t.url
                    .as_deref()
                    .and_then(|u| u.rsplit('/').next())
                    == Some(url)
            })
            .ok_or_else(|| InnertubeError::NotFound(format!("Tab \"{url}\" not found")))?;
        Ok((!tab.is_selected).then_some(tab))
    }

    /// Legacy `TabbedFeed.hasTabWithURL`.
    pub fn has_tab_with_url(&self, url: &str) -> bool {
        self.tabs
            .iter()
            .any(|t| t.url.as_deref().and_then(|u| u.rsplit('/').next()) == Some(url))
    }
}
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

impl HashtagFeed {
    pub fn has_continuation(&self) -> bool {
        self.continuation_token.is_some()
    }

    /// Fetch next batch of hashtag feed videos.
    pub async fn get_continuation(&self, session: &Session) -> Result<HashtagFeed> {
        let token = self.continuation_token.as_deref().ok_or_else(|| {
            InnertubeError::NotFound("There are no continuations available for this HashtagFeed".into())
        })?;
        let payload = serde_json::json!({
            "continuation": token,
        });
        let resp = session.post_innertube("/browse", payload).await?;
        let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;
        crate::endpoints::feed::parse_hashtag_response(&self.hashtag, &raw)
    }
}
