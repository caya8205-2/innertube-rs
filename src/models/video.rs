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
    /// `isLive` — actively broadcasting (premieres excluded from
    /// `is_live_content` still count as live content).
    #[serde(default)]
    pub is_live: Option<bool>,
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
    pub playback_tracking: Option<PlaybackTracking>,
}

/// Playback tracking base URLs (legacy `IPlaybackTracking`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackTracking {
    pub videostats_watchtime_url: Option<TrackingUrl>,
    pub videostats_playback_url: Option<TrackingUrl>,
}

/// A `baseUrl`-carrying tracking URL entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackingUrl {
    pub base_url: Option<String>,
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
    /// Proof-of-origin token forwarded to stream URLs as `pot` (legacy
    /// `Player.po_token`), unless the URL is a SABR stream.
    #[serde(default)]
    pub po_token: Option<String>,
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
        crate::endpoints::player::resolve_stream_url_full(
            format,
            decipherer,
            self.po_token.as_deref(),
            None,
        )
    }

    /// Report playback to the watch history stats endpoint (legacy
    /// `MediaInfo.addToWatchHistory`).
    pub async fn add_to_watch_history(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<reqwest::Response> {
        let url = self
            .player_response
            .playback_tracking
            .as_ref()
            .and_then(|pt| pt.videostats_playback_url.as_ref())
            .and_then(|u| u.base_url.as_ref())
            .ok_or_else(|| {
                crate::error::InnertubeError::Other("Playback tracking not available".to_string())
            })?
            .replace("https://s.", "https://www.");

        crate::core::actions::Actions::stats(
            session,
            &url,
            crate::constants::clients::WEB_NAME,
            crate::constants::clients::WEB_VERSION,
            &[
                ("cpn", self.cpn.clone()),
                ("fmt", "251".to_string()),
                ("rtn", "0".to_string()),
                ("rt", "0".to_string()),
            ],
        )
        .await
    }

    /// Update watch time on the stats endpoint (legacy
    /// `MediaInfo.updateWatchTime`; st/et/cmt fixed to 3 decimals, final=1).
    pub async fn update_watch_time(
        &self,
        session: &crate::core::session::Session,
        start_time: f64,
    ) -> crate::error::Result<reqwest::Response> {
        let url = self
            .player_response
            .playback_tracking
            .as_ref()
            .and_then(|pt| pt.videostats_watchtime_url.as_ref())
            .and_then(|u| u.base_url.as_ref())
            .ok_or_else(|| {
                crate::error::InnertubeError::Other("Playback tracking not available".to_string())
            })?
            .replace("https://s.", "https://www.");

        let ts = format!("{start_time:.3}");
        crate::core::actions::Actions::stats(
            session,
            &url,
            crate::constants::clients::WEB_NAME,
            crate::constants::clients::WEB_VERSION,
            &[
                ("cpn", self.cpn.clone()),
                ("st", ts.clone()),
                ("et", ts.clone()),
                ("cmt", ts),
                ("final", "1".to_string()),
            ],
        )
        .await
    }

    /// Download this video (legacy `FormatUtils.download` defaults: 360p,
    /// video+audio, mp4; 10MB `range=` query chunks for ranged/adaptive).
    pub async fn download(
        &self,
        session: &crate::core::session::Session,
        decipherer: &PlayerDecipherer,
        options: &crate::models::format::DownloadOptions,
    ) -> crate::error::Result<bytes::Bytes> {
        crate::utils::format::download(
            session,
            options,
            Some(&self.player_response.playability_status),
            self.player_response.streaming_data.as_ref(),
            decipherer,
            self.po_token.as_deref(),
            Some(&self.cpn),
        )
        .await
    }

    /// Generate a DASH MPD manifest for this video (legacy `MediaInfo.toDash`).
    /// Throws for live content, matching legacy.
    ///
    /// ponytail: storyboard image sets are omitted (player response
    /// storyboards are not modeled yet).
    pub async fn to_dash(
        &self,
        decipherer: &PlayerDecipherer,
        options: crate::utils::streaming_info::StreamingInfoOptions,
    ) -> crate::error::Result<String> {
        let details = self.player_response.video_details.as_ref();
        let is_live = details
            .is_some_and(|v| v.is_live.unwrap_or(false) || v.is_live_content.unwrap_or(false));
        if is_live {
            return Err(crate::error::InnertubeError::Other(
                "Cannot generate DASH manifest for live content".to_string(),
            ));
        }

        let streaming_data = self
            .player_response
            .streaming_data
            .as_ref()
            .ok_or_else(|| {
                crate::error::InnertubeError::NotFound(
                    "Streaming data not available".to_string(),
                )
            })?;

        let caption_tracks = self
            .player_response
            .captions
            .as_ref()
            .map(|c| {
                crate::endpoints::transcript::extract_caption_tracks_from_player(
                    &serde_json::json!({ "captions": c }),
                )
            })
            .transpose()?
            .unwrap_or_default();

        let info = crate::utils::streaming_info::get_streaming_info(
            streaming_data,
            false,
            None,
            crate::utils::streaming_info::StreamingInfoParams {
                decipherer: Some(decipherer),
                cpn: Some(&self.cpn),
                po_token: self.po_token.as_deref(),
                caption_tracks: if caption_tracks.is_empty() {
                    None
                } else {
                    Some(&caption_tracks)
                },
                options,
                ..Default::default()
            },
        )
        .await?;

        Ok(crate::utils::dash::render_dash_manifest(&info))
    }

    /// Retrieve the video transcript via the searchable-transcript
    /// engagement panel continuation (legacy `MediaInfo.getTranscript`).
    pub async fn get_transcript(
        &self,
        session: &crate::core::session::Session,
    ) -> crate::error::Result<crate::models::transcript::Transcript> {
        use crate::error::InnertubeError;

        let watch_next = self.watch_next.as_ref().ok_or_else(|| {
            InnertubeError::Other("Cannot get transcript from basic video info.".to_string())
        })?;
        let token = watch_next
            .transcript_continuation_token
            .as_deref()
            .ok_or_else(|| {
                InnertubeError::Other(
                    "Transcript continuation not found.".to_string(),
                )
            })?;

        let resp = session
            .post_innertube("/next", serde_json::json!({ "continuation": token }))
            .await?;
        let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;

        let segments = parse_transcript_segments(&raw);
        Ok(crate::models::transcript::Transcript {
            video_id: self.id().unwrap_or_default().to_string(),
            language: String::new(),
            segments,
        })
    }
}

/// Parse `transcriptSegmentRenderer` entries from a transcript continuation
/// response.
fn parse_transcript_segments(raw: &serde_json::Value) -> Vec<crate::models::transcript::TranscriptSegment> {
    fn walk(v: &serde_json::Value, out: &mut Vec<crate::models::transcript::TranscriptSegment>) {
        if let Some(seg) = v.get("transcriptSegmentRenderer") {
            let start_ms = seg
                .get("startMs")
                .and_then(|s| s.as_str())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let end_ms = seg
                .get("endMs")
                .and_then(|s| s.as_str())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let text = seg
                .pointer("/snippet/runs")
                .and_then(|r| r.as_array())
                .map(|runs| {
                    runs.iter()
                        .filter_map(|r| r.get("text").and_then(|t| t.as_str()))
                        .collect::<String>()
                })
                .unwrap_or_default();
            out.push(crate::models::transcript::TranscriptSegment {
                start_ms,
                duration_ms: end_ms.saturating_sub(start_ms),
                end_ms,
                text,
            });
            return;
        }
        match v {
            serde_json::Value::Object(map) => map.values().for_each(|x| walk(x, out)),
            serde_json::Value::Array(items) => items.iter().for_each(|x| walk(x, out)),
            _ => {}
        }
    }

    let mut segments = Vec::new();
    walk(raw, &mut segments);
    segments
}

/// YouTube Kids video info (parallel `/player` + `/next` on the YTKIDS
/// client, legacy `Kids.getInfo`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KidsVideoInfo {
    pub player_response: PlayerResponse,
    pub watch_next: Option<serde_json::Value>,
    pub cpn: String,
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
                is_live: None,
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
                    ..Default::default()
                }],
                adaptive_formats: vec![],
                dash_manifest_url: None,
                hls_manifest_url: None,
            }),
            captions: None,
            playback_tracking: None,
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
            po_token: None,
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
