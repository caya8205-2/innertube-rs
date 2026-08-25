use serde::{Deserialize, Serialize};
use crate::models::format::StreamingFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub length_seconds: String,
    pub channel_id: String,
    pub is_owner_viewing: Option<bool>,
    pub short_description: Option<String>,
    pub is_crawlable: Option<bool>,
    pub thumbnail: Option<ThumbnailContainer>,
    pub allow_ratings: Option<bool>,
    pub view_count: Option<String>,
    pub author: String,
    pub is_private: Option<bool>,
    pub is_unplugged_corpus: Option<bool>,
    pub is_live_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailContainer {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    pub expires_in_seconds: Option<String>,
    #[serde(default)]
    pub formats: Vec<StreamingFormat>,
    #[serde(default)]
    pub adaptive_formats: Vec<StreamingFormat>,
    pub dash_manifest_url: Option<String>,
    pub hls_manifest_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: String,
    pub reason: Option<String>,
    pub playable_in_embed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub playability_status: PlayabilityStatus,
    pub video_details: Option<VideoDetails>,
    pub streaming_data: Option<StreamingData>,
    pub captions: Option<serde_json::Value>,
}

/// Options for fetching video player data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVideoInfoOptions {
    /// InnerTube client name override (e.g. "ANDROID", "IOS", "WEB").
    pub client: Option<String>,
    /// Proof of Origin token (PO-Token) bound to this video.
    pub po_token: Option<String>,
    /// Optional playback context / lact milliseconds override.
    pub playback_context: Option<serde_json::Value>,
}

use crate::models::format::FormatFilter;
use crate::models::next::WatchNextResults;
use crate::utils::decipher::PlayerDecipherer;

/// High-level parsed video info matching YouTube.js VideoInfo / MediaInfo container.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    /// The underlying player response.
    pub player_response: PlayerResponse,
    /// Watch next details (recommendations, related videos, playlist panel).
    pub watch_next: Option<WatchNextResults>,
    /// Client Playback Nonce (CPN) generated for this playback session.
    pub cpn: String,
}

impl VideoInfo {
    /// Extract video ID.
    pub fn id(&self) -> Option<&str> {
        self.player_response.video_details.as_ref().map(|v| v.video_id.as_str())
    }

    /// Extract video title.
    pub fn title(&self) -> Option<&str> {
        self.player_response.video_details.as_ref().map(|v| v.title.as_str())
    }

    /// Get the video author/channel title if available.
    pub fn author(&self) -> Option<&str> {
        self.player_response
            .video_details
            .as_ref()
            .map(|v| v.author.as_str())
    }

    /// Get the video duration in seconds if available.
    pub fn duration_seconds(&self) -> Option<u64> {
        self.player_response
            .video_details
            .as_ref()
            .and_then(|v| v.length_seconds.parse().ok())
    }

    /// Select a streaming format matching the specified filter.
    pub fn select_format(&self, filter: &FormatFilter) -> crate::error::Result<&StreamingFormat> {
        crate::endpoints::player::select_format(&self.player_response, filter)
    }

    /// Retrieve a decrypted, playable streaming URL matching the specified filter.
    pub fn get_stream_url(
        &self,
        filter: &FormatFilter,
        decipherer: &PlayerDecipherer,
    ) -> crate::error::Result<String> {
        let format = self.select_format(filter)?;
        crate::endpoints::player::resolve_stream_url(format, decipherer)
    }
}

/// High-level Shorts video metadata and reel sequence navigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortFormVideoInfo {
    /// The player response for the short.
    pub player_response: PlayerResponse,
    /// Client Playback Nonce (CPN) generated for this short.
    pub cpn: String,
    /// Watch next feed items / reel sequence entries.
    #[serde(default)]
    pub watch_next_feed: Vec<serde_json::Value>,
    /// Continuation sequence parameter for the next batch of shorts.
    pub continuation_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::format::QualityPreference;
    use crate::models::format::FormatType;
    use crate::models::next::WatchNextResults;

    #[test]
    fn test_video_info_helpers_and_format_selection() {
        let player_response = PlayerResponse {
            playability_status: PlayabilityStatus {
                status: "OK".to_string(),
                reason: None,
                playable_in_embed: Some(true),
            },
            video_details: Some(VideoDetails {
                video_id: "dQw4w9WgXcQ".to_string(),
                title: "Rick Astley - Never Gonna Give You Up".to_string(),
                length_seconds: "213".to_string(),
                channel_id: "UCuAXFkgsw1L7xaCfnd5JJOw".to_string(),
                is_owner_viewing: None,
                short_description: Some("Music video".to_string()),
                is_crawlable: Some(true),
                thumbnail: None,
                allow_ratings: Some(true),
                view_count: Some("1500000000".to_string()),
                author: "Rick Astley".to_string(),
                is_private: Some(false),
                is_unplugged_corpus: None,
                is_live_content: Some(false),
            }),
            streaming_data: Some(StreamingData {
                expires_in_seconds: Some("21540".to_string()),
                formats: vec![StreamingFormat {
                    itag: 18,
                    url: Some("https://example.com/18.mp4".to_string()),
                    signature_cipher: None,
                    cipher: None,
                    mime_type: "video/mp4; codecs=\"avc1.42001E, mp4a.40.2\"".to_string(),
                    bitrate: 500_000,
                    width: Some(640),
                    height: Some(360),
                    quality_label: Some("360p".to_string()),
                    audio_quality: Some("AUDIO_QUALITY_LOW".to_string()),
                    approx_duration_ms: None,
                    audio_sample_rate: None,
                    audio_channels: Some(2),
                    content_length: None,
                    average_bitrate: None,
                }],
                adaptive_formats: vec![],
                dash_manifest_url: None,
                hls_manifest_url: None,
            }),
            captions: None,
        };

        let watch_next = WatchNextResults {
            current_video_id: "dQw4w9WgXcQ".to_string(),
            current_title: Some("Rick Astley - Never Gonna Give You Up".to_string()),
            ..Default::default()
        };

        let video_info = VideoInfo {
            player_response,
            watch_next: Some(watch_next),
            cpn: "abcdef1234567890".to_string(),
        };

        assert_eq!(video_info.id(), Some("dQw4w9WgXcQ"));
        assert_eq!(video_info.title(), Some("Rick Astley - Never Gonna Give You Up"));
        assert!(video_info.watch_next.is_some());

        let filter = FormatFilter {
            format_type: FormatType::AudioVideo,
            quality: QualityPreference::Highest,
            container: None,
        };

        let selected = video_info.select_format(&filter).expect("Failed to select format");
        assert_eq!(selected.itag, 18);
    }
}
