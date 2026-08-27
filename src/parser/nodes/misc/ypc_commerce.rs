use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed `VerticalWatchCardList` AST node (`verticalWatchCardListRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerticalWatchCardListNode {
    pub items: Vec<Value>,
    pub view_all_text: Option<TextNode>,
    pub view_all_endpoint: Option<Value>,
}

impl VerticalWatchCardListNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("verticalWatchCardListRenderer").unwrap_or(val);
        Some(Self {
            items: node.get("items").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
            view_all_text: node.get("viewAllText").and_then(TextNode::from_value),
            view_all_endpoint: node.get("viewAllEndpoint").cloned(),
        })
    }
}

/// Strongly typed `VideoAttributesSectionView` AST node (`videoAttributesSectionViewModel`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAttributesSectionViewNode {
    pub header_title: Option<String>,
    pub header_subtitle: Option<String>,
    pub video_attributes: Vec<Value>,
    pub previous_button: Option<Value>,
    pub next_button: Option<Value>,
}

impl VideoAttributesSectionViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoAttributesSectionViewModel").unwrap_or(val);
        Some(Self {
            header_title: node.get("headerTitle").and_then(|v| v.as_str()).map(String::from),
            header_subtitle: node.get("headerSubtitle").and_then(|v| v.as_str()).map(String::from),
            video_attributes: node.get("videoAttributeViewModels").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
            previous_button: node.get("previousButton").cloned(),
            next_button: node.get("nextButton").cloned(),
        })
    }
}

/// Strongly typed `VideoDescriptionCourseSection` AST node (`videoDescriptionCourseSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDescriptionCourseSectionNode {
    pub section_title: Option<TextNode>,
    pub media_lockups: Vec<Value>,
}

impl VideoDescriptionCourseSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoDescriptionCourseSectionRenderer").unwrap_or(val);
        Some(Self {
            section_title: node.get("sectionTitle").and_then(TextNode::from_value),
            media_lockups: node.get("mediaLockups").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
        })
    }
}

/// Strongly typed `VideoDescriptionInfocardsSection` AST node (`videoDescriptionInfocardsSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDescriptionInfocardsSectionNode {
    pub section_title: Option<TextNode>,
    pub creator_videos_button: Option<Value>,
    pub creator_about_button: Option<Value>,
    pub section_subtitle: Option<TextNode>,
    pub channel_avatar: ThumbnailListNode,
    pub channel_endpoint: Option<Value>,
}

impl VideoDescriptionInfocardsSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoDescriptionInfocardsSectionRenderer").unwrap_or(val);
        Some(Self {
            section_title: node.get("sectionTitle").and_then(TextNode::from_value),
            creator_videos_button: node.get("creatorVideosButton").cloned(),
            creator_about_button: node.get("creatorAboutButton").cloned(),
            section_subtitle: node.get("sectionSubtitle").and_then(TextNode::from_value),
            channel_avatar: ThumbnailListNode::from_value(node.get("channelAvatar").unwrap_or(&Value::Null)),
            channel_endpoint: node.get("channelEndpoint").cloned(),
        })
    }
}

/// Strongly typed `VideoDescriptionMusicSection` AST node (`videoDescriptionMusicSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDescriptionMusicSectionNode {
    pub carousel_lockups: Vec<Value>,
    pub section_title: Option<TextNode>,
}

impl VideoDescriptionMusicSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoDescriptionMusicSectionRenderer").unwrap_or(val);
        Some(Self {
            carousel_lockups: node.get("carouselLockups").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
            section_title: node.get("sectionTitle").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed `VideoDescriptionTranscriptSection` AST node (`videoDescriptionTranscriptSectionRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDescriptionTranscriptSectionNode {
    pub section_title: Option<TextNode>,
    pub sub_header_text: Option<TextNode>,
    pub primary_button: Option<Value>,
}

impl VideoDescriptionTranscriptSectionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoDescriptionTranscriptSectionRenderer").unwrap_or(val);
        Some(Self {
            section_title: node.get("sectionTitle").and_then(TextNode::from_value),
            sub_header_text: node.get("subHeaderText").and_then(TextNode::from_value),
            primary_button: node.get("primaryButton").cloned(),
        })
    }
}

/// Strongly typed `VideoDescriptionYouchatSectionView` AST node (`videoDescriptionYouchatSectionViewModel`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDescriptionYouchatSectionViewNode {
    pub section_title: Option<TextNode>,
    pub sub_header_text: Option<TextNode>,
    pub primary_button: Option<Value>,
}

impl VideoDescriptionYouchatSectionViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoDescriptionYouchatSectionViewModel").unwrap_or(val);
        Some(Self {
            section_title: node.get("sectionTitle").and_then(TextNode::from_value),
            sub_header_text: node.get("subHeaderText").and_then(TextNode::from_value),
            primary_button: node.get("primaryButton").cloned(),
        })
    }
}

/// Strongly typed `VideoMetadataCarouselView` AST node (`videoMetadataCarouselViewModel`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadataCarouselViewNode {
    pub carousel_titles: Vec<Value>,
    pub carousel_items: Vec<Value>,
}

impl VideoMetadataCarouselViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("videoMetadataCarouselViewModel").unwrap_or(val);
        Some(Self {
            carousel_titles: node.get("carouselTitles").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
            carousel_items: node.get("carouselItems").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
        })
    }
}

/// Strongly typed `ViewCountFactoid` AST node (`viewCountFactoidRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewCountFactoidNode {
    pub view_count_entity_key: Option<String>,
    pub factoid: Option<Value>,
    pub view_count_type: Option<String>,
}

impl ViewCountFactoidNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("viewCountFactoidRenderer").unwrap_or(val);
        Some(Self {
            view_count_entity_key: node.get("viewCountEntityKey").and_then(|v| v.as_str()).map(String::from),
            factoid: node.get("factoid").cloned(),
            view_count_type: node.get("viewCountType").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed `WatchCardRichHeader` AST node (`watchCardRichHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchCardRichHeaderNode {
    pub title: Option<TextNode>,
    pub title_endpoint: Option<Value>,
    pub subtitle: Option<TextNode>,
    pub author: Option<Value>,
    pub style: Option<String>,
}

impl WatchCardRichHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("watchCardRichHeaderRenderer").unwrap_or(val);
        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            title_endpoint: node.get("titleNavigationEndpoint").cloned(),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            author: Some(node.clone()),
            style: node.get("style").and_then(|v| v.as_str()).map(String::from),
        })
    }
}

/// Strongly typed `WatchCardSectionSequence` AST node (`watchCardSectionSequenceRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchCardSectionSequenceNode {
    pub lists: Vec<Value>,
}

impl WatchCardSectionSequenceNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("watchCardSectionSequenceRenderer").unwrap_or(val);
        Some(Self {
            lists: node.get("lists").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
        })
    }
}

/// Strongly typed `WatchNextEndScreen` AST node (`watchNextEndScreenRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchNextEndScreenNode {
    pub results: Vec<Value>,
    pub title: Option<TextNode>,
}

impl WatchNextEndScreenNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("watchNextEndScreenRenderer").unwrap_or(val);
        Some(Self {
            results: node.get("results").and_then(|v| v.as_array()).map(|v| v.to_vec()).unwrap_or_default(),
            title: node.get("title").and_then(TextNode::from_value),
        })
    }
}
