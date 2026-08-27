use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text::TextNode;
use super::thumbnail::ThumbnailListNode;

/// Strongly typed BrowserMediaSession AST node (`browserMediaSession`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserMediaSessionNode {
    pub album: Option<TextNode>,
    pub thumbnails: Option<ThumbnailListNode>,
}

impl BrowserMediaSessionNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("browserMediaSession").unwrap_or(val);
        Some(Self {
            album: node.get("album").and_then(TextNode::from_value),
            thumbnails: node
                .get("thumbnailDetails")
                .map(ThumbnailListNode::from_value),
        })
    }
}

/// Strongly typed ChannelVideoPlayer AST node (`channelVideoPlayerRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideoPlayerNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub description: Option<TextNode>,
    pub view_count: Option<TextNode>,
    pub published_time: Option<TextNode>,
}

impl ChannelVideoPlayerNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("channelVideoPlayerRenderer")
            .or_else(|| val.get("channelVideoPlayer"))
            .unwrap_or(val);

        let id = node
            .get("videoId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            id,
            title: node.get("title").and_then(TextNode::from_value),
            description: node.get("description").and_then(TextNode::from_value),
            view_count: node.get("viewCountText").and_then(TextNode::from_value),
            published_time: node.get("publishedTimeText").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed ChildVideo AST node (`childVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildVideoNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub duration_text: Option<String>,
    pub endpoint: Option<Value>,
}

impl ChildVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("childVideoRenderer")
            .or_else(|| val.get("childVideo"))
            .unwrap_or(val);

        let id = node
            .get("videoId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let duration_text = node
            .pointer("/lengthText/simpleText")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| {
                node.get("lengthText")
                    .and_then(TextNode::from_value)
                    .map(|t| t.text)
            });

        let endpoint = node.get("navigationEndpoint").cloned();

        Some(Self {
            id,
            title: node.get("title").and_then(TextNode::from_value),
            duration_text,
            endpoint,
        })
    }
}

/// Strongly typed EndScreenVideo AST node (`endScreenVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndScreenVideoNode {
    pub id: Option<String>,
    pub title: Option<TextNode>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub thumbnail_overlays: Vec<Value>,
    pub short_byline_text: Option<TextNode>,
    pub owner_badges: Vec<Value>,
    pub endpoint: Option<Value>,
    pub short_view_count: Option<TextNode>,
    pub badges: Vec<Value>,
    pub duration_text: Option<TextNode>,
    pub length_in_seconds: Option<u64>,
}

impl EndScreenVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("endScreenVideoRenderer")
            .or_else(|| val.get("endscreenVideoRenderer"))
            .or_else(|| val.get("endScreenVideo"))
            .unwrap_or(val);

        let id = node
            .get("videoId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let thumbnails = node
            .get("thumbnail")
            .or_else(|| node.get("thumbnails"))
            .map(ThumbnailListNode::from_value);

        let thumbnail_overlays = node
            .get("thumbnailOverlays")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let short_byline_text = node
            .get("shortBylineText")
            .and_then(TextNode::from_value);

        let owner_badges = node
            .get("ownerBadges")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let endpoint = node.get("navigationEndpoint").cloned();
        let short_view_count = node
            .get("shortViewCountText")
            .and_then(TextNode::from_value);
        let badges = node
            .get("badges")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let duration_text = node.get("lengthText").and_then(TextNode::from_value);
        let length_in_seconds = node
            .get("lengthInSeconds")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("lengthInSeconds")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        Some(Self {
            id,
            title: node.get("title").and_then(TextNode::from_value),
            thumbnails,
            thumbnail_overlays,
            short_byline_text,
            owner_badges,
            endpoint,
            short_view_count,
            badges,
            duration_text,
            length_in_seconds,
        })
    }
}

/// Strongly typed ExpandableVideoDescriptionBody AST node (`expandableVideoDescriptionBodyRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandableVideoDescriptionBodyNode {
    pub show_more_text: Option<TextNode>,
    pub show_less_text: Option<TextNode>,
    pub attributed_description_body_text: Option<TextNode>,
}

impl ExpandableVideoDescriptionBodyNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("expandableVideoDescriptionBodyRenderer")
            .or_else(|| val.get("expandableVideoDescriptionBody"))
            .unwrap_or(val);

        Some(Self {
            show_more_text: node.get("showMoreText").and_then(TextNode::from_value),
            show_less_text: node.get("showLessText").and_then(TextNode::from_value),
            attributed_description_body_text: node
                .get("attributedDescriptionBodyText")
                .and_then(TextNode::from_value),
        })
    }
}

/// Featured channel data within `PlayerAnnotationsExpandedNode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAnnotationsFeaturedChannelNode {
    pub start_time_ms: Option<u64>,
    pub end_time_ms: Option<u64>,
    pub watermark: Option<ThumbnailListNode>,
    pub channel_name: Option<String>,
    pub endpoint: Option<Value>,
    pub subscribe_button: Option<Value>,
}

/// Strongly typed PlayerAnnotationsExpanded AST node (`playerAnnotationsExpandedRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAnnotationsExpandedNode {
    pub featured_channel: Option<PlayerAnnotationsFeaturedChannelNode>,
    pub allow_swipe_dismiss: bool,
    pub annotation_id: Option<String>,
}

impl PlayerAnnotationsExpandedNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerAnnotationsExpandedRenderer")
            .or_else(|| val.get("playerAnnotationsExpanded"))
            .unwrap_or(val);

        let featured_channel = node.get("featuredChannel").map(|fc| {
            let start_time_ms = fc
                .get("startTimeMs")
                .and_then(Value::as_u64)
                .or_else(|| {
                    fc.get("startTimeMs")
                        .and_then(Value::as_str)
                        .and_then(|s| s.parse().ok())
                });
            let end_time_ms = fc
                .get("endTimeMs")
                .and_then(Value::as_u64)
                .or_else(|| {
                    fc.get("endTimeMs")
                        .and_then(Value::as_str)
                        .and_then(|s| s.parse().ok())
                });
            let watermark = fc.get("watermark").map(ThumbnailListNode::from_value);
            let channel_name = fc
                .get("channelName")
                .and_then(Value::as_str)
                .map(ToString::to_string);
            let endpoint = fc.get("navigationEndpoint").cloned();
            let subscribe_button = fc.get("subscribeButton").cloned();

            PlayerAnnotationsFeaturedChannelNode {
                start_time_ms,
                end_time_ms,
                watermark,
                channel_name,
                endpoint,
                subscribe_button,
            }
        });

        let allow_swipe_dismiss = node
            .get("allowSwipeDismiss")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let annotation_id = node
            .get("annotationId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            featured_channel,
            allow_swipe_dismiss,
            annotation_id,
        })
    }
}

/// Strongly typed PlayerCaptchaView AST node (`playerCaptchaView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCaptchaViewNode {
    pub captcha_loading_message: Option<TextNode>,
    pub challenge_reason: Option<TextNode>,
    pub captcha_successful_message: Option<TextNode>,
    pub captcha_cookie_set_failure_message: Option<TextNode>,
    pub captcha_failed_message: Option<TextNode>,
}

impl PlayerCaptchaViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerCaptchaView")
            .or_else(|| val.get("playerCaptchaViewRenderer"))
            .unwrap_or(val);

        Some(Self {
            captcha_loading_message: node
                .get("captchaLoadingMessage")
                .and_then(TextNode::from_value),
            challenge_reason: node.get("challengeReason").and_then(TextNode::from_value),
            captcha_successful_message: node
                .get("captchaSuccessfulMessage")
                .and_then(TextNode::from_value),
            captcha_cookie_set_failure_message: node
                .get("captchaCookieSetFailureMessage")
                .and_then(TextNode::from_value),
            captcha_failed_message: node
                .get("captchaFailedMessage")
                .and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed PlayerControlsOverlay AST node (`playerControlsOverlayRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerControlsOverlayNode {
    pub overflow: Option<Value>,
}

impl PlayerControlsOverlayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerControlsOverlayRenderer")
            .or_else(|| val.get("playerControlsOverlay"))
            .unwrap_or(val);

        Some(Self {
            overflow: node.get("overflow").cloned(),
        })
    }
}

/// Strongly typed PlayerLegacyDesktopYpcOffer AST node (`playerLegacyDesktopYpcOfferRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLegacyDesktopYpcOfferNode {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub offer_description: Option<String>,
    pub offer_id: Option<String>,
}

impl PlayerLegacyDesktopYpcOfferNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerLegacyDesktopYpcOfferRenderer")
            .or_else(|| val.get("playerLegacyDesktopYpcOffer"))
            .unwrap_or(val);

        Some(Self {
            title: node
                .get("itemTitle")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            thumbnail: node
                .get("itemThumbnail")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            offer_description: node
                .get("offerDescription")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            offer_id: node
                .get("offerId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}

/// Embed metadata within `PlayerMicroformatNode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatEmbedNode {
    pub iframe_url: Option<String>,
    pub flash_url: Option<String>,
    pub flash_secure_url: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
}

/// Channel metadata within `PlayerMicroformatNode`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatChannelNode {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Strongly typed PlayerMicroformat AST node (`playerMicroformatRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatNode {
    pub title: Option<TextNode>,
    pub description: Option<TextNode>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub embed: Option<PlayerMicroformatEmbedNode>,
    pub length_seconds: Option<u64>,
    pub channel: Option<PlayerMicroformatChannelNode>,
    pub is_family_safe: bool,
    pub is_unlisted: bool,
    pub has_ypc_metadata: bool,
    pub view_count: Option<u64>,
    pub category: Option<String>,
    pub publish_date: Option<String>,
    pub upload_date: Option<String>,
    pub available_countries: Vec<String>,
    pub start_timestamp: Option<String>,
    pub end_timestamp: Option<String>,
}

impl PlayerMicroformatNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerMicroformatRenderer")
            .or_else(|| val.get("playerMicroformat"))
            .unwrap_or(val);

        let embed = node.get("embed").map(|e| PlayerMicroformatEmbedNode {
            iframe_url: e
                .get("iframeUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            flash_url: e
                .get("flashUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            flash_secure_url: e
                .get("flashSecureUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            width: e.get("width").and_then(Value::as_u64).or_else(|| {
                e.get("width")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            }),
            height: e.get("height").and_then(Value::as_u64).or_else(|| {
                e.get("height")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            }),
        });

        let channel = Some(PlayerMicroformatChannelNode {
            id: node
                .get("externalChannelId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            name: node
                .get("ownerChannelName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            url: node
                .get("ownerProfileUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        });

        let length_seconds = node
            .get("lengthSeconds")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("lengthSeconds")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let view_count = node.get("viewCount").and_then(Value::as_u64).or_else(|| {
            node.get("viewCount")
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
        });

        let available_countries = node
            .get("availableCountries")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(ToString::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let start_timestamp = node
            .pointer("/liveBroadcastDetails/startTimestamp")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let end_timestamp = node
            .pointer("/liveBroadcastDetails/endTimestamp")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            description: node.get("description").and_then(TextNode::from_value),
            thumbnails: node
                .get("thumbnail")
                .or_else(|| node.get("thumbnails"))
                .map(ThumbnailListNode::from_value),
            embed,
            length_seconds,
            channel,
            is_family_safe: node
                .get("isFamilySafe")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_unlisted: node
                .get("isUnlisted")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            has_ypc_metadata: node
                .get("hasYpcMetadata")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            view_count,
            category: node
                .get("category")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            publish_date: node
                .get("publishDate")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            upload_date: node
                .get("uploadDate")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            available_countries,
            start_timestamp,
            end_timestamp,
        })
    }
}

/// Strongly typed PlayerOverflow AST node (`playerOverflowRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOverflowNode {
    pub endpoint: Option<Value>,
    pub enable_listen_first: bool,
}

impl PlayerOverflowNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerOverflowRenderer")
            .or_else(|| val.get("playerOverflow"))
            .unwrap_or(val);

        Some(Self {
            endpoint: node.get("endpoint").cloned(),
            enable_listen_first: node
                .get("enableListenFirst")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        })
    }
}

/// Strongly typed PlayerOverlayAutoplay AST node (`playerOverlayAutoplayRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOverlayAutoplayNode {
    pub title: Option<TextNode>,
    pub video_id: Option<String>,
    pub video_title: Option<TextNode>,
    pub short_view_count: Option<TextNode>,
    pub prefer_immediate_redirect: Option<Value>,
    pub count_down_secs_for_fullscreen: Option<u64>,
    pub published: Option<TextNode>,
    pub background: Option<ThumbnailListNode>,
    pub thumbnail_overlays: Vec<Value>,
    pub byline: Option<TextNode>,
    pub cancel_button: Option<Value>,
    pub next_button: Option<Value>,
    pub close_button: Option<Value>,
}

impl PlayerOverlayAutoplayNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerOverlayAutoplayRenderer")
            .or_else(|| val.get("playerOverlayAutoplay"))
            .unwrap_or(val);

        let count_down_secs_for_fullscreen = node
            .get("countDownSecsForFullscreen")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("countDownSecsForFullscreen")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            video_id: node
                .get("videoId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            video_title: node.get("videoTitle").and_then(TextNode::from_value),
            short_view_count: node
                .get("shortViewCountText")
                .and_then(TextNode::from_value),
            prefer_immediate_redirect: node.get("preferImmediateRedirect").cloned(),
            count_down_secs_for_fullscreen,
            published: node
                .get("publishedTimeText")
                .and_then(TextNode::from_value),
            background: node.get("background").map(ThumbnailListNode::from_value),
            thumbnail_overlays: node
                .get("thumbnailOverlays")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            byline: node.get("byline").and_then(TextNode::from_value),
            cancel_button: node.get("cancelButton").cloned(),
            next_button: node.get("nextButton").cloned(),
            close_button: node.get("closeButton").cloned(),
        })
    }
}

/// Strongly typed PlayerOverlayVideoDetails AST node (`playerOverlayVideoDetailsRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOverlayVideoDetailsNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
}

impl PlayerOverlayVideoDetailsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("playerOverlayVideoDetailsRenderer")
            .or_else(|| val.get("playerOverlayVideoDetails"))
            .unwrap_or(val);

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed SlimVideoMetadata AST node (`slimVideoMetadataRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlimVideoMetadataNode {
    pub title: Option<TextNode>,
    pub collapsed_subtitle: Option<TextNode>,
    pub expanded_subtitle: Option<TextNode>,
    pub owner: Option<Value>,
    pub description: Option<TextNode>,
    pub video_id: Option<String>,
    pub date: Option<TextNode>,
}

impl SlimVideoMetadataNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("slimVideoMetadataRenderer")
            .or_else(|| val.get("slimVideoMetadata"))
            .or_else(|| val.get("slimVideoInformationRenderer"))
            .unwrap_or(val);

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            collapsed_subtitle: node
                .get("collapsedSubtitle")
                .and_then(TextNode::from_value),
            expanded_subtitle: node
                .get("expandedSubtitle")
                .and_then(TextNode::from_value),
            owner: node.get("owner").cloned(),
            description: node.get("description").and_then(TextNode::from_value),
            video_id: node
                .get("videoId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            date: node.get("dateText").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed VideoAttributeView AST node (`videoAttributeView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAttributeViewNode {
    pub image: Option<ThumbnailListNode>,
    pub image_style: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub secondary_subtitle: Option<String>,
    pub orientation: Option<String>,
    pub sizing_rule: Option<String>,
    pub overflow_menu_on_tap: Option<Value>,
    pub overflow_menu_a11y_label: Option<String>,
}

impl VideoAttributeViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoAttributeView")
            .or_else(|| val.get("videoAttributeViewRenderer"))
            .unwrap_or(val);

        let secondary_subtitle = node
            .pointer("/secondarySubtitle/content")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| {
                node.get("secondarySubtitle")
                    .and_then(Value::as_str)
                    .map(ToString::to_string)
            });

        Some(Self {
            image: node.get("image").map(ThumbnailListNode::from_value),
            image_style: node
                .get("imageStyle")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            title: node
                .get("title")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            subtitle: node
                .get("subtitle")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            secondary_subtitle,
            orientation: node
                .get("orientation")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            sizing_rule: node
                .get("sizingRule")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            overflow_menu_on_tap: node.get("overflowMenuOnTap").cloned(),
            overflow_menu_a11y_label: node
                .get("overflowMenuA11yLabel")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}

/// Strongly typed VideoCard AST node (`videoCardRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoCardNode {
    pub video_id: Option<String>,
    pub title: Option<TextNode>,
    pub metadata_text: Option<TextNode>,
    pub description_snippet: Option<TextNode>,
    pub thumbnails: Option<ThumbnailListNode>,
    pub thumbnail_overlays: Vec<Value>,
    pub byline_text: Option<TextNode>,
    pub short_view_count: Option<TextNode>,
    pub published: Option<TextNode>,
    pub endpoint: Option<Value>,
    pub length_text: Option<TextNode>,
    pub badges: Vec<Value>,
    pub menu: Option<Value>,
}

impl VideoCardNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoCardRenderer")
            .or_else(|| val.get("videoCard"))
            .unwrap_or(val);

        Some(Self {
            video_id: node
                .get("videoId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            title: node.get("title").and_then(TextNode::from_value),
            metadata_text: node.get("metadataText").and_then(TextNode::from_value),
            description_snippet: node
                .get("descriptionSnippet")
                .and_then(TextNode::from_value),
            thumbnails: node
                .get("thumbnail")
                .or_else(|| node.get("thumbnails"))
                .map(ThumbnailListNode::from_value),
            thumbnail_overlays: node
                .get("thumbnailOverlays")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            byline_text: node.get("bylineText").and_then(TextNode::from_value),
            short_view_count: node
                .get("shortViewCountText")
                .and_then(TextNode::from_value),
            published: node
                .get("publishedTimeText")
                .and_then(TextNode::from_value),
            endpoint: node.get("navigationEndpoint").cloned(),
            length_text: node.get("lengthText").and_then(TextNode::from_value),
            badges: node
                .get("badges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            menu: node.get("menu").cloned(),
        })
    }
}

/// Strongly typed VideoDescriptionHeader AST node (`videoDescriptionHeaderRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDescriptionHeaderNode {
    pub title: Option<TextNode>,
    pub channel: Option<TextNode>,
    pub channel_navigation_endpoint: Option<Value>,
    pub channel_thumbnail: Option<ThumbnailListNode>,
    pub publish_date: Option<TextNode>,
    pub views: Option<TextNode>,
    pub factoids: Vec<Value>,
}

impl VideoDescriptionHeaderNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoDescriptionHeaderRenderer")
            .or_else(|| val.get("videoDescriptionHeader"))
            .unwrap_or(val);

        let factoids = node
            .get("factoid")
            .or_else(|| node.get("factoids"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            channel: node.get("channel").and_then(TextNode::from_value),
            channel_navigation_endpoint: node.get("channelNavigationEndpoint").cloned(),
            channel_thumbnail: node
                .get("channelThumbnail")
                .map(ThumbnailListNode::from_value),
            publish_date: node.get("publishDate").and_then(TextNode::from_value),
            views: node.get("views").and_then(TextNode::from_value),
            factoids,
        })
    }
}

/// Strongly typed VideoInfoCardContent AST node (`videoInfoCardContentRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfoCardContentNode {
    pub title: Option<TextNode>,
    pub channel_name: Option<TextNode>,
    pub view_count: Option<TextNode>,
    pub video_thumbnails: Option<ThumbnailListNode>,
    pub duration: Option<TextNode>,
    pub endpoint: Option<Value>,
}

impl VideoInfoCardContentNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoInfoCardContentRenderer")
            .or_else(|| val.get("videoInfoCardContent"))
            .unwrap_or(val);

        Some(Self {
            title: node.get("videoTitle").and_then(TextNode::from_value),
            channel_name: node.get("channelName").and_then(TextNode::from_value),
            view_count: node.get("viewCountText").and_then(TextNode::from_value),
            video_thumbnails: node
                .get("videoThumbnail")
                .or_else(|| node.get("videoThumbnails"))
                .map(ThumbnailListNode::from_value),
            duration: node.get("lengthString").and_then(TextNode::from_value),
            endpoint: node
                .get("action")
                .or_else(|| node.get("navigationEndpoint"))
                .cloned(),
        })
    }
}

/// Strongly typed VideoSummaryParagraphView AST node (`videoSummaryParagraphView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSummaryParagraphViewNode {
    pub text: Option<TextNode>,
}

impl VideoSummaryParagraphViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoSummaryParagraphView")
            .or_else(|| val.get("videoSummaryParagraphViewRenderer"))
            .unwrap_or(val);

        Some(Self {
            text: node.get("text").and_then(TextNode::from_value),
        })
    }
}

/// Strongly typed VideoSummaryContentView AST node (`videoSummaryContentView`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSummaryContentViewNode {
    pub dislike_button_view: Option<Value>,
    pub like_button_view: Option<Value>,
    pub paragraphs: Vec<VideoSummaryParagraphViewNode>,
}

impl VideoSummaryContentViewNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoSummaryContentView")
            .or_else(|| val.get("videoSummaryContentViewRenderer"))
            .unwrap_or(val);

        let paragraphs = node
            .get("paragraphs")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(VideoSummaryParagraphViewNode::from_value)
                    .collect()
            })
            .unwrap_or_default();

        Some(Self {
            dislike_button_view: node.get("dislikeButtonViewModel").cloned(),
            like_button_view: node.get("likeButtonViewModel").cloned(),
            paragraphs,
        })
    }
}

/// Strongly typed WatchCardCompactVideo AST node (`watchCardCompactVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchCardCompactVideoNode {
    pub title: Option<TextNode>,
    pub subtitle: Option<TextNode>,
    pub duration_text: Option<TextNode>,
    pub style: Option<String>,
}

impl WatchCardCompactVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("watchCardCompactVideoRenderer")
            .or_else(|| val.get("watchCardCompactVideo"))
            .unwrap_or(val);

        Some(Self {
            title: node.get("title").and_then(TextNode::from_value),
            subtitle: node.get("subtitle").and_then(TextNode::from_value),
            duration_text: node.get("lengthText").and_then(TextNode::from_value),
            style: node
                .get("style")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        })
    }
}

/// Strongly typed WatchCardHeroVideo AST node (`watchCardHeroVideoRenderer`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchCardHeroVideoNode {
    pub endpoint: Option<Value>,
    pub call_to_action_button: Option<Value>,
    pub hero_image: Option<Value>,
    pub label: Option<String>,
}

impl WatchCardHeroVideoNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("watchCardHeroVideoRenderer")
            .or_else(|| val.get("watchCardHeroVideo"))
            .unwrap_or(val);

        let label = node
            .pointer("/lengthText/accessibility/accessibilityData/label")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        Some(Self {
            endpoint: node.get("navigationEndpoint").cloned(),
            call_to_action_button: node.get("callToActionButton").cloned(),
            hero_image: node.get("heroImage").cloned(),
            label,
        })
    }
}

/// Byte range within `FormatNode`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatRangeNode {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

/// Color info metadata within `FormatNode`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatColorInfoNode {
    pub primaries: Option<String>,
    pub transfer_characteristics: Option<String>,
    pub matrix_coefficients: Option<String>,
}

/// Audio track metadata within `FormatNode`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatAudioTrackNode {
    pub audio_is_default: bool,
    pub display_name: Option<String>,
    pub id: Option<String>,
}

/// Caption track metadata within `FormatNode`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatCaptionTrackNode {
    pub display_name: Option<String>,
    pub vss_id: Option<String>,
    pub language_code: Option<String>,
    pub kind: Option<String>,
    pub id: Option<String>,
}

/// Strongly typed Format AST node (stream / adaptive format).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatNode {
    pub itag: Option<u32>,
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub is_type_otf: bool,
    pub bitrate: Option<u64>,
    pub average_bitrate: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub init_range: Option<FormatRangeNode>,
    pub index_range: Option<FormatRangeNode>,
    pub last_modified_ms: Option<String>,
    pub content_length: Option<u64>,
    pub quality: Option<String>,
    pub quality_label: Option<String>,
    pub projection_type: Option<String>,
    pub fps: Option<u32>,
    pub cipher: Option<String>,
    pub signature_cipher: Option<String>,
    pub audio_quality: Option<String>,
    pub approx_duration_ms: Option<u64>,
    pub audio_sample_rate: Option<u32>,
    pub audio_channels: Option<u32>,
    pub loudness_db: Option<f64>,
    pub stereo_layout: Option<String>,
    pub spatial_audio_type: Option<String>,
    pub max_dvr_duration_sec: Option<u64>,
    pub target_duration_sec: Option<u64>,
    pub xtags: Option<String>,
    pub fair_play_key_uri: Option<String>,
    pub drm_families: Vec<String>,
    pub drm_track_type: Option<String>,
    pub distinct_params: Option<String>,
    pub track_absolute_loudness_lkfs: Option<f64>,
    pub high_replication: Option<bool>,
    pub color_info: Option<FormatColorInfoNode>,
    pub audio_track: Option<FormatAudioTrackNode>,
    pub caption_track: Option<FormatCaptionTrackNode>,
    pub has_audio: bool,
    pub has_video: bool,
    pub has_text: bool,
    pub is_drc: Option<bool>,
    pub is_vb: Option<bool>,
}

impl FormatNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val.get("format").unwrap_or(val);

        let itag = node
            .get("itag")
            .and_then(Value::as_u64)
            .map(|n| n as u32)
            .or_else(|| {
                node.get("itag")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let url = node
            .get("url")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let mime_type = node
            .get("mimeType")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let is_type_otf = node.get("type").and_then(Value::as_str) == Some("FORMAT_STREAM_TYPE_OTF");

        let bitrate = node.get("bitrate").and_then(Value::as_u64).or_else(|| {
            node.get("bitrate")
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
        });

        let average_bitrate = node
            .get("averageBitrate")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("averageBitrate")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let width = node
            .get("width")
            .and_then(Value::as_u64)
            .map(|n| n as u32)
            .or_else(|| {
                node.get("width")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let height = node
            .get("height")
            .and_then(Value::as_u64)
            .map(|n| n as u32)
            .or_else(|| {
                node.get("height")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let init_range = node.get("initRange").map(|r| FormatRangeNode {
            start: r.get("start").and_then(Value::as_u64).or_else(|| {
                r.get("start")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            }),
            end: r.get("end").and_then(Value::as_u64).or_else(|| {
                r.get("end")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            }),
        });

        let index_range = node.get("indexRange").map(|r| FormatRangeNode {
            start: r.get("start").and_then(Value::as_u64).or_else(|| {
                r.get("start")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            }),
            end: r.get("end").and_then(Value::as_u64).or_else(|| {
                r.get("end")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            }),
        });

        let last_modified_ms = node
            .get("lastModified")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| {
                node.get("lastModified")
                    .and_then(Value::as_u64)
                    .map(|n| n.to_string())
            });

        let content_length = node
            .get("contentLength")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("contentLength")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let quality = node
            .get("quality")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let quality_label = node
            .get("qualityLabel")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let projection_type = node
            .get("projectionType")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let fps = node
            .get("fps")
            .and_then(Value::as_u64)
            .map(|n| n as u32)
            .or_else(|| {
                node.get("fps")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let cipher = node
            .get("cipher")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let signature_cipher = node
            .get("signatureCipher")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let audio_quality = node
            .get("audioQuality")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let approx_duration_ms = node
            .get("approxDurationMs")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("approxDurationMs")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let audio_sample_rate = node
            .get("audioSampleRate")
            .and_then(Value::as_u64)
            .map(|n| n as u32)
            .or_else(|| {
                node.get("audioSampleRate")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let audio_channels = node
            .get("audioChannels")
            .and_then(Value::as_u64)
            .map(|n| n as u32)
            .or_else(|| {
                node.get("audioChannels")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let loudness_db = node.get("loudnessDb").and_then(Value::as_f64);

        let stereo_layout = node
            .get("stereoLayout")
            .and_then(Value::as_str)
            .map(|s| s.replace("STEREO_LAYOUT_", ""));

        let spatial_audio_type = node
            .get("spatialAudioType")
            .and_then(Value::as_str)
            .map(|s| s.replace("SPATIAL_AUDIO_TYPE_", ""));

        let max_dvr_duration_sec = node
            .get("maxDvrDurationSec")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("maxDvrDurationSec")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let target_duration_sec = node
            .get("targetDurationSec")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("targetDurationSec")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let xtags = node
            .get("xtags")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let fair_play_key_uri = node
            .get("fairPlayKeyUri")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let drm_families = node
            .get("drmFamilies")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(ToString::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let drm_track_type = node
            .get("drmTrackType")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let distinct_params = node
            .get("distinctParams")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let track_absolute_loudness_lkfs =
            node.get("trackAbsoluteLoudnessLkfs").and_then(Value::as_f64);

        let high_replication = node.get("highReplication").and_then(Value::as_bool);

        let color_info = node.get("colorInfo").map(|c| FormatColorInfoNode {
            primaries: c
                .get("primaries")
                .and_then(Value::as_str)
                .map(|s| s.replace("COLOR_PRIMARIES_", "")),
            transfer_characteristics: c
                .get("transferCharacteristics")
                .and_then(Value::as_str)
                .map(|s| s.replace("COLOR_TRANSFER_CHARACTERISTICS_", "")),
            matrix_coefficients: c
                .get("matrixCoefficients")
                .and_then(Value::as_str)
                .map(|s| s.replace("COLOR_MATRIX_COEFFICIENTS_", "")),
        });

        let audio_track = node.get("audioTrack").map(|a| FormatAudioTrackNode {
            audio_is_default: a
                .get("audioIsDefault")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            display_name: a
                .get("displayName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            id: a.get("id").and_then(Value::as_str).map(ToString::to_string),
        });

        let caption_track = node.get("captionTrack").map(|c| FormatCaptionTrackNode {
            display_name: c
                .get("displayName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            vss_id: c
                .get("vssId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            language_code: c
                .get("languageCode")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            kind: c
                .get("kind")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            id: c.get("id").and_then(Value::as_str).map(ToString::to_string),
        });

        let has_audio = node.get("audioBitrate").is_some() || node.get("audioQuality").is_some();
        let has_video = node.get("qualityLabel").is_some();
        let has_text = node.get("captionTrack").is_some();
        let is_drc = node.get("isDrc").and_then(Value::as_bool);
        let is_vb = node.get("isVb").and_then(Value::as_bool);

        Some(Self {
            itag,
            url,
            mime_type,
            is_type_otf,
            bitrate,
            average_bitrate,
            width,
            height,
            init_range,
            index_range,
            last_modified_ms,
            content_length,
            quality,
            quality_label,
            projection_type,
            fps,
            cipher,
            signature_cipher,
            audio_quality,
            approx_duration_ms,
            audio_sample_rate,
            audio_channels,
            loudness_db,
            stereo_layout,
            spatial_audio_type,
            max_dvr_duration_sec,
            target_duration_sec,
            xtags,
            fair_play_key_uri,
            drm_families,
            drm_track_type,
            distinct_params,
            track_absolute_loudness_lkfs,
            high_replication,
            color_info,
            audio_track,
            caption_track,
            has_audio,
            has_video,
            has_text,
            is_drc,
            is_vb,
        })
    }
}

/// Strongly typed VideoDetails AST node (`videoDetails`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetailsNode {
    pub id: Option<String>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub duration_seconds: Option<u64>,
    pub keywords: Vec<String>,
    pub is_owner_viewing: bool,
    pub short_description: Option<String>,
    pub thumbnail: Option<ThumbnailListNode>,
    pub allow_ratings: bool,
    pub view_count: Option<u64>,
    pub author: Option<String>,
    pub is_private: bool,
    pub is_live: bool,
    pub is_live_content: bool,
    pub is_live_dvr_enabled: bool,
    pub is_upcoming: bool,
    pub is_crawlable: bool,
    pub is_post_live_dvr: bool,
    pub is_low_latency_live_stream: bool,
    pub live_chunk_readahead: Option<u64>,
}

impl VideoDetailsNode {
    pub fn from_value(val: &Value) -> Option<Self> {
        let node = val
            .get("videoDetails")
            .or_else(|| val.get("videoDetailsRenderer"))
            .unwrap_or(val);

        let duration_seconds = node
            .get("lengthSeconds")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("lengthSeconds")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        let keywords = node
            .get("keywords")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(ToString::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let view_count = node.get("viewCount").and_then(Value::as_u64).or_else(|| {
            node.get("viewCount")
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
        });

        let live_chunk_readahead = node
            .get("liveChunkReadahead")
            .and_then(Value::as_u64)
            .or_else(|| {
                node.get("liveChunkReadahead")
                    .and_then(Value::as_str)
                    .and_then(|s| s.parse().ok())
            });

        Some(Self {
            id: node
                .get("videoId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            channel_id: node
                .get("channelId")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            title: node
                .get("title")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            duration_seconds,
            keywords,
            is_owner_viewing: node
                .get("isOwnerViewing")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            short_description: node
                .get("shortDescription")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            thumbnail: node
                .get("thumbnail")
                .or_else(|| node.get("thumbnails"))
                .map(ThumbnailListNode::from_value),
            allow_ratings: node
                .get("allowRatings")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            view_count,
            author: node
                .get("author")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            is_private: node
                .get("isPrivate")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_live: node
                .get("isLive")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_live_content: node
                .get("isLiveContent")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_live_dvr_enabled: node
                .get("isLiveDvrEnabled")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_upcoming: node
                .get("isUpcoming")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_crawlable: node
                .get("isCrawlable")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_post_live_dvr: node
                .get("isPostLiveDvr")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            is_low_latency_live_stream: node
                .get("isLowLatencyLiveStream")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            live_chunk_readahead,
        })
    }
}
