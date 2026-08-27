use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed GuideEntry AST node (`guideEntryRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideEntryNode {
    pub title: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub icon_type: Option<String>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub badges: Option<Value>,
    pub is_primary: bool,
}

impl GuideEntryNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("guideEntryRenderer").unwrap_or(val);
        
        Some(Self {
            title: node.get("formattedTitle").and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").or_else(|| node.get("serviceEndpoint")).cloned(),
            icon_type: node
                .get("icon")
                .and_then(|i| i.get("iconType"))
                .and_then(|t| t.as_str().map(String::from)),
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            badges: node.get("badges").cloned(),
            is_primary: node.get("isPrimary").and_then(|b| b.as_bool()).unwrap_or(false),
        })
    }
}

/// Strongly typed GuideSection AST node (`guideSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideSectionNode {
    pub title: Option<TextNode>,
    pub items: Vec<Value>,
}

impl GuideSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("guideSectionRenderer").unwrap_or(val);
        
        Some(Self {
            title: node.get("formattedTitle").and_then(TextNode::from_value),
            items: node
                .get("items")
                .and_then(|i| i.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed GuideSubscriptionsSection AST node (`guideSubscriptionsSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuideSubscriptionsSectionNode {
    pub title: Option<TextNode>,
    pub items: Vec<Value>,
}

impl GuideSubscriptionsSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("guideSubscriptionsSectionRenderer").unwrap_or(val);
        
        Some(Self {
            title: node.get("formattedTitle").and_then(TextNode::from_value),
            items: node
                .get("items")
                .and_then(|i| i.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed HashtagHeader AST node (`hashtagHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagHeaderNode {
    pub hashtag: Option<TextNode>,
    pub hashtag_info: Option<TextNode>,
}

impl HashtagHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("hashtagHeaderRenderer").unwrap_or(val);
        
        Some(Self {
            hashtag: node.get("hashtag").and_then(TextNode::from_value),
            hashtag_info: node.get("hashtagInfoText").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed HashtagTile AST node (`hashtagTileRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashtagTileNode {
    pub hashtag: Option<TextNode>,
    pub hashtag_info_text: Option<TextNode>,
    pub hashtag_thumbnail: Option<ThumbnailListNode>,
    pub endpoint: Option<Value>,
    pub hashtag_background_color: Option<u64>,
    pub hashtag_video_count: Option<TextNode>,
    pub hashtag_channel_count: Option<TextNode>,
}

impl HashtagTileNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("hashtagTileRenderer").unwrap_or(val);
        
        Some(Self {
            hashtag: node.get("hashtag").and_then(TextNode::from_value),
            hashtag_info_text: node.get("hashtagInfoText").and_then(TextNode::from_value),
            hashtag_thumbnail: node.get("hashtagThumbnail").map(ThumbnailListNode::from_value),
            endpoint: node.get("onTapCommand").cloned(),
            hashtag_background_color: node.get("hashtagBackgroundColor").and_then(|n| n.as_u64()),
            hashtag_video_count: node.get("hashtagVideoCount").and_then(TextNode::from_value),
            hashtag_channel_count: node.get("hashtagChannelCount").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed HeatMarker AST node (`heatMarkerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeatMarkerNode {
    pub time_range_start_millis: Option<u64>,
    pub marker_duration_millis: Option<u64>,
    pub heat_marker_intensity_score_normalized: Option<f64>,
}

impl HeatMarkerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("heatMarkerRenderer").unwrap_or(val);
        
        Some(Self {
            time_range_start_millis: node
                .get("startMillis")
                .and_then(|s| s.as_str())
                .and_then(|s| s.parse::<u64>().ok()),
            marker_duration_millis: node
                .get("durationMillis")
                .and_then(|s| s.as_str())
                .and_then(|s| s.parse::<u64>().ok()),
            heat_marker_intensity_score_normalized: node.get("intensityScoreNormalized").and_then(|n| n.as_f64()),
        })
    }
}

/// Strongly typed HeroPlaylistThumbnail AST node (`heroPlaylistThumbnailRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeroPlaylistThumbnailNode {
    pub thumbnails: Option<ThumbnailListNode>,
    pub on_tap_endpoint: Option<Value>,
}

impl HeroPlaylistThumbnailNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("heroPlaylistThumbnailRenderer").unwrap_or(val);
        
        Some(Self {
            thumbnails: node.get("thumbnail").map(ThumbnailListNode::from_value),
            on_tap_endpoint: node.get("onTap").cloned(),
        })
    }
}

/// Details for `HighlightsCarouselPanelNode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelThumbnailNode {
    pub image: Option<ThumbnailListNode>,
    pub endpoint: Option<Value>,
    pub on_long_press_endpoint: Option<Value>,
    pub content_mode: Option<String>,
    pub crop_options: Option<String>,
}

/// Details for `HighlightsCarouselPanelNode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelBackgroundImageNode {
    pub image: Option<ThumbnailListNode>,
    pub gradient_image: Option<ThumbnailListNode>,
}

/// Details for `HighlightsCarouselPanelNode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelCtaNode {
    pub icon_name: Option<String>,
    pub title: Option<String>,
    pub endpoint: Option<Value>,
    pub accessibility_text: Option<String>,
    pub state: Option<String>,
}

/// Sub-node used within HighlightsCarouselNode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightsCarouselPanelNode {
    pub thumbnail: Option<PanelThumbnailNode>,
    pub background_image: Option<PanelBackgroundImageNode>,
    pub strapline: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub text_on_tap_endpoint: Option<Value>,
    pub cta: Option<PanelCtaNode>,
}

impl HighlightsCarouselPanelNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("panelRenderer").unwrap_or(val);

        let thumbnail = node.get("thumbnail").map(|t| PanelThumbnailNode {
            image: t.get("image").map(ThumbnailListNode::from_value),
            endpoint: t.get("onTap").cloned(),
            on_long_press_endpoint: t.get("onLongPress").cloned(),
            content_mode: t.get("contentMode").and_then(|s| s.as_str().map(String::from)),
            crop_options: t.get("cropOptions").and_then(|s| s.as_str().map(String::from)),
        });

        let background_image = node.get("backgroundImage").map(|bg| PanelBackgroundImageNode {
            image: bg.get("image").map(ThumbnailListNode::from_value),
            gradient_image: bg.get("gradientImage").map(ThumbnailListNode::from_value),
        });

        let cta = node.get("cta").map(|c| PanelCtaNode {
            icon_name: c.get("iconName").and_then(|s| s.as_str().map(String::from)),
            title: c.get("title").and_then(|s| s.as_str().map(String::from)),
            endpoint: c.get("onTap").cloned(),
            accessibility_text: c.get("accessibilityText").and_then(|s| s.as_str().map(String::from)),
            state: c.get("state").and_then(|s| s.as_str().map(String::from)),
        });

        Some(Self {
            thumbnail,
            background_image,
            strapline: node.get("strapline").and_then(|s| s.as_str().map(String::from)),
            title: node.get("title").and_then(|s| s.as_str().map(String::from)),
            description: node.get("description").and_then(|s| s.as_str().map(String::from)),
            text_on_tap_endpoint: node.get("textOnTap").cloned(),
            cta,
        })
    }
}

/// Strongly typed HighlightsCarousel AST node (`highlightsCarouselRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightsCarouselNode {
    pub panels: Vec<HighlightsCarouselPanelNode>,
}

impl HighlightsCarouselNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("highlightsCarouselRenderer").unwrap_or(val);
        
        // highlightsCarousel.panels is how JS accessed it, but since this is highlightsCarouselRenderer,
        // it may have `highlightsCarousel` sub-object or just `panels`.
        // In JS: `data.highlightsCarousel.panels.map(...)`.
        let panels_arr = node
            .get("highlightsCarousel")
            .and_then(|hc| hc.get("panels"))
            .or_else(|| node.get("panels"))
            .and_then(|p| p.as_array());

        let panels = panels_arr
            .map(|a| a.iter().filter_map(HighlightsCarouselPanelNode::from_value).collect())
            .unwrap_or_default();

        Some(Self { panels })
    }
}

/// Strongly typed HorizontalList AST node (`horizontalListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalListNode {
    pub visible_item_count: Option<String>,
    pub items: Vec<Value>,
}

impl HorizontalListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("horizontalListRenderer").unwrap_or(val);
        
        Some(Self {
            visible_item_count: node.get("visibleItemCount").and_then(|v| v.as_str().map(String::from)),
            items: node
                .get("items")
                .and_then(|i| i.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
        })
    }
}

/// Strongly typed HorizontalMovieList AST node (`horizontalMovieListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalMovieListNode {
    pub items: Vec<Value>,
    pub previous_button: Option<Value>,
    pub next_button: Option<Value>,
}

impl HorizontalMovieListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("horizontalMovieListRenderer").unwrap_or(val);
        
        Some(Self {
            items: node
                .get("items")
                .and_then(|i| i.as_array())
                .map(|a| a.to_vec())
                .unwrap_or_default(),
            previous_button: node.get("previousButton").cloned(),
            next_button: node.get("nextButton").cloned(),
        })
    }
}

/// Strongly typed HowThisWasMadeSectionView AST node (`howThisWasMadeSectionView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HowThisWasMadeSectionViewNode {
    pub section_title: Option<TextNode>,
    pub body_text: Option<TextNode>,
    pub body_header: Option<TextNode>,
}

impl HowThisWasMadeSectionViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("howThisWasMadeSectionView").unwrap_or(val);
        
        Some(Self {
            section_title: node.get("sectionText").and_then(TextNode::from_value),
            body_text: node.get("bodyText").and_then(TextNode::from_value),
            body_header: node.get("bodyHeader").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed HypeFanCreditsSectionView AST node (`hypeFanCreditsSectionView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HypeFanCreditsSectionViewNode {
    pub header: Option<Value>,
}

impl HypeFanCreditsSectionViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("hypeFanCreditsSectionView").unwrap_or(val);
        
        Some(Self {
            header: node.get("header").cloned(),
        })
    }
}
