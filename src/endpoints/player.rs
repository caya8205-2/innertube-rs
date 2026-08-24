use crate::constants::clients;
use crate::core::session::{Session, SessionOptions};
use crate::error::{InnertubeError, Result};
use crate::models::format::{
    FormatFilter, FormatOptions, FormatType, QualityPreference, StreamingFormat,
};
use crate::models::video::{GetVideoInfoOptions, PlayerResponse};
use crate::utils::decipher::PlayerDecipherer;
use serde_json::json;

/// Fetch player metadata and streaming formats for a video from `/youtubei/v1/player` with optional client and PO-token options.
pub async fn fetch_player_response_with_options(
    session: &Session,
    video_id: &str,
    signature_timestamp: Option<u32>,
    options: Option<&GetVideoInfoOptions>,
) -> Result<PlayerResponse> {
    let mut payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true,
        "playbackContext": {
            "contentPlaybackContext": {
                "vis": 0,
                "splay": false,
                "lactMilliseconds": "-1",
                "html5Preference": "HTML5_PREF_WANTS"
            }
        }
    });

    if let Some(sts) = signature_timestamp {
        payload["playbackContext"]["contentPlaybackContext"]["signatureTimestamp"] = json!(sts);
    }

    if let Some(opt) = options {
        if let Some(ref c) = opt.client {
            payload["client"] = json!(c);
        }
        if let Some(ref pot) = opt.po_token {
            payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
        } else if let Some(ref pot) = session.po_token {
            payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
        }
    } else if let Some(ref pot) = session.po_token {
        payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
    }

    let resp = if let Some(client_name) = options.and_then(|o| o.client.as_deref()) {
        session
            .post_innertube_client(client_name, "/player", payload)
            .await?
    } else {
        session.post_innertube("/player", payload).await?
    };

    if !resp.status().is_success() {
        return Err(InnertubeError::Api {
            status: resp.status().to_string(),
            message: format!("Player endpoint returned HTTP {}", resp.status()),
        });
    }

    let mut player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    // Check if playability status is not OK or adaptive formats have no URLs/ciphers. If so, fallback to ANDROID -> ANDROID_VR -> iOS -> MWEB
    let needs_fallback = player_response.playability_status.status != "OK"
        || player_response.streaming_data.as_ref().is_none_or(|sd| {
            sd.adaptive_formats.is_empty()
                || sd
                    .adaptive_formats
                    .iter()
                    .all(|f| f.url.is_none() && f.signature_cipher.is_none() && f.cipher.is_none())
        });

    if needs_fallback {
        // 1. Fetch standard ANDROID client for reliable progressive formats (itag 18)
        let mut android_prog_formats = Vec::new();
        if let Ok(android_response) = fetch_player_response_android(session, video_id).await {
            if let Some(ref sd) = android_response.streaming_data {
                android_prog_formats = sd.formats.clone();
            }
            if android_response.playability_status.status == "OK" {
                player_response = android_response;
            }
        }

        // 2. Fetch iOS client for high-res direct adaptive formats (1080p, 720p, 480p, AAC audio)
        if let Ok(ios_response) = fetch_player_response_ios(session, video_id).await {
            if ios_response.playability_status.status == "OK" {
                let mut final_ios = ios_response;
                if let Some(ref mut ios_sd) = final_ios.streaming_data {
                    if !android_prog_formats.is_empty() {
                        ios_sd.formats = android_prog_formats.clone();
                    }
                }
                player_response = final_ios;
            } else if let Some(ios_streaming) = ios_response.streaming_data {
                if let Some(ref mut sd) = player_response.streaming_data {
                    sd.adaptive_formats = ios_streaming.adaptive_formats;
                    if !android_prog_formats.is_empty() {
                        sd.formats = android_prog_formats.clone();
                    }
                }
            }
        }

        // 3. Fallback to ANDROID_VR if adaptive formats still missing
        let still_needs_vr = player_response.playability_status.status != "OK"
            || player_response.streaming_data.as_ref().is_none_or(|sd| {
                sd.adaptive_formats.is_empty()
                    || sd.adaptive_formats.iter().all(|f| {
                        f.url.is_none() && f.signature_cipher.is_none() && f.cipher.is_none()
                    })
            });

        if still_needs_vr {
            if let Ok(vr_response) = fetch_player_response_android_vr(session, video_id).await {
                if vr_response.playability_status.status == "OK" {
                    let mut final_vr = vr_response;
                    if let Some(ref mut vr_sd) = final_vr.streaming_data {
                        if !android_prog_formats.is_empty() {
                            vr_sd.formats = android_prog_formats.clone();
                        }
                    }
                    player_response = final_vr;
                } else if let Some(vr_streaming) = vr_response.streaming_data {
                    if let Some(ref mut sd) = player_response.streaming_data {
                        sd.adaptive_formats = vr_streaming.adaptive_formats;
                        if !android_prog_formats.is_empty() {
                            sd.formats = android_prog_formats.clone();
                        }
                    } else {
                        player_response.streaming_data = Some(vr_streaming);
                    }
                }
            }
        }

        // 4. Fallback to MWEB if still needed
        let still_needs_mweb = player_response.playability_status.status != "OK"
            || player_response.streaming_data.as_ref().is_none_or(|sd| {
                sd.adaptive_formats.is_empty()
                    || sd.adaptive_formats.iter().all(|f| {
                        f.url.is_none() && f.signature_cipher.is_none() && f.cipher.is_none()
                    })
            });

        if still_needs_mweb {
            if let Ok(mweb_response) =
                fetch_player_response_mweb(session, video_id, signature_timestamp).await
            {
                if mweb_response.playability_status.status == "OK" {
                    let mut final_mweb = mweb_response;
                    if let Some(ref mut mweb_sd) = final_mweb.streaming_data {
                        if !android_prog_formats.is_empty() {
                            mweb_sd.formats = android_prog_formats.clone();
                        }
                    }
                    player_response = final_mweb;
                } else if let Some(mweb_streaming) = mweb_response.streaming_data {
                    if let Some(ref mut sd) = player_response.streaming_data {
                        sd.adaptive_formats = mweb_streaming.adaptive_formats;
                        if !android_prog_formats.is_empty() {
                            sd.formats = android_prog_formats.clone();
                        }
                    } else {
                        player_response.streaming_data = Some(mweb_streaming);
                    }
                }
            }
        }
    }

    if player_response.playability_status.status != "OK" {
        return Err(InnertubeError::Restricted(format!(
            "Video is not playable: {} ({})",
            player_response.playability_status.status,
            player_response
                .playability_status
                .reason
                .as_deref()
                .unwrap_or("No reason provided")
        )));
    }

    Ok(player_response)
}

/// Fallback player fetch using MWEB (Mobile Web) client reusing the session HTTP client and cookie jar.
async fn fetch_player_response_mweb(
    session: &Session,
    video_id: &str,
    signature_timestamp: Option<u32>,
) -> Result<PlayerResponse> {
    let mut mweb_context = session.context.clone();
    mweb_context.client.client_name = clients::MWEB_NAME.to_string();
    mweb_context.client.client_version = clients::MWEB_VERSION.to_string();
    mweb_context.client.platform = "MOBILE".to_string();
    mweb_context.client.user_agent = clients::MWEB_USER_AGENT.to_string();

    let mut payload = json!({
        "context": mweb_context,
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    if let Some(sts) = signature_timestamp {
        payload["playbackContext"] = json!({
            "contentPlaybackContext": {
                "signatureTimestamp": sts
            }
        });
    }

    let url = format!(
        "{}/player?key={}",
        crate::constants::INNERTUBE_API_BASE_URL,
        session.api_key
    );
    let resp = session
        .http_client
        .post(&url)
        .header("User-Agent", clients::MWEB_USER_AGENT)
        .header("X-Youtube-Client-Name", "2")
        .header("X-Youtube-Client-Version", clients::MWEB_VERSION)
        .json(&payload)
        .send()
        .await
        .map_err(InnertubeError::Network)?;
    let resp = Session::ensure_success("player (MWEB fallback)", resp).await?;
    let player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    Ok(player_response)
}

/// Fallback player fetch using standard ANDROID client to get progressive streams (itag 18).
async fn fetch_player_response_android(
    session: &Session,
    video_id: &str,
) -> Result<PlayerResponse> {
    let mut android_context = session.context.clone();
    android_context.client.client_name = clients::ANDROID_NAME.to_string();
    android_context.client.client_version = clients::ANDROID_VERSION.to_string();
    android_context.client.platform = "MOBILE".to_string();
    android_context.client.user_agent = clients::ANDROID_USER_AGENT.to_string();
    android_context.client.os_name = "Android".to_string();
    android_context.client.os_version = "16".to_string();
    android_context.client.android_sdk_version = Some(36);

    let mut payload = json!({
        "context": android_context,
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    if let Some(ref pot) = session.po_token {
        payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
    }

    let url = format!(
        "{}/player?key={}",
        crate::constants::INNERTUBE_API_BASE_URL,
        session.api_key
    );
    let mut req = session
        .http_client
        .post(&url)
        .header("User-Agent", clients::ANDROID_USER_AGENT)
        .header("X-Youtube-Client-Name", "3")
        .header("X-Youtube-Client-Version", clients::ANDROID_VERSION);

    if let Some(ref c) = session.cookie {
        req = req.header("Cookie", c);
    }

    let resp = req
        .json(&payload)
        .send()
        .await
        .map_err(InnertubeError::Network)?;
    let resp = Session::ensure_success("player (ANDROID fallback)", resp).await?;
    let player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    Ok(player_response)
}

/// Fallback player fetch using ANDROID_VR client to get direct, unthrottled stream URLs.
async fn fetch_player_response_android_vr(
    session: &Session,
    video_id: &str,
) -> Result<PlayerResponse> {
    let mut vr_context = session.context.clone();
    vr_context.client.client_name = clients::ANDROID_VR_NAME.to_string();
    vr_context.client.client_version = clients::ANDROID_VR_VERSION.to_string();
    vr_context.client.platform = "MOBILE".to_string();
    vr_context.client.user_agent = clients::ANDROID_VR_USER_AGENT.to_string();
    vr_context.client.device_make = Some("Oculus".to_string());
    vr_context.client.device_model = Some("Quest 3".to_string());
    vr_context.client.os_name = "Android".to_string();
    vr_context.client.os_version = "12L".to_string();
    vr_context.client.android_sdk_version = Some(32);

    let mut payload = json!({
        "context": vr_context,
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    if let Some(ref pot) = session.po_token {
        payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
    }

    let url = format!(
        "{}/player?key={}",
        crate::constants::INNERTUBE_API_BASE_URL,
        session.api_key
    );
    let mut req = session
        .http_client
        .post(&url)
        .header("User-Agent", clients::ANDROID_VR_USER_AGENT)
        .header("X-Youtube-Client-Name", "28")
        .header("X-Youtube-Client-Version", clients::ANDROID_VR_VERSION);

    if let Some(ref c) = session.cookie {
        req = req.header("Cookie", c);
    }

    let resp = req
        .json(&payload)
        .send()
        .await
        .map_err(InnertubeError::Network)?;
    let resp = Session::ensure_success("player (ANDROID_VR fallback)", resp).await?;
    let player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    Ok(player_response)
}

/// Fallback player fetch using iOS client to get direct stream URLs for all adaptive formats.
async fn fetch_player_response_ios(session: &Session, video_id: &str) -> Result<PlayerResponse> {
    let ios_options = SessionOptions {
        client_name: Some(clients::IOS_NAME.to_string()),
        client_version: Some(clients::IOS_VERSION.to_string()),
        device_category: Some("MOBILE".to_string()),
        user_agent: Some(clients::IOS_USER_AGENT.to_string()),
        po_token: session.po_token.clone(),
        cookie: session.cookie.clone(),
        generate_session_locally: Some(true),
        ..Default::default()
    };

    let ios_session = Session::create(ios_options).await?;

    let mut payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    if let Some(ref pot) = session.po_token {
        payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
    }

    let resp = ios_session.post_innertube("/player", payload).await?;
    let player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    Ok(player_response)
}

/// Fetch player metadata and streaming formats for a video from `/youtubei/v1/player`.
pub async fn fetch_player_response(
    session: &Session,
    video_id: &str,
    signature_timestamp: Option<u32>,
) -> Result<PlayerResponse> {
    fetch_player_response_with_options(session, video_id, signature_timestamp, None).await
}

/// Filter formats according to user criteria.
pub fn select_format<'a>(
    player_response: &'a PlayerResponse,
    filter: &FormatFilter,
) -> Result<&'a StreamingFormat> {
    let streaming_data = player_response.streaming_data.as_ref().ok_or_else(|| {
        InnertubeError::NotFound("No streamingData found in player response".into())
    })?;

    let mut candidates: Vec<&'a StreamingFormat> = Vec::new();

    candidates.extend(&streaming_data.formats);
    candidates.extend(&streaming_data.adaptive_formats);

    // Apply type filter
    candidates.retain(|f| match filter.format_type {
        FormatType::AudioOnly => f.is_audio_only(),
        FormatType::VideoOnly => f.is_video_only(),
        FormatType::AudioVideo => f.is_audio_video(),
        FormatType::Any => true,
    });

    // Apply container filter if requested
    if let Some(ref container) = filter.container {
        candidates.retain(|f| f.mime_type.contains(container));
    }

    if candidates.is_empty() {
        return Err(InnertubeError::NotFound(
            "No streaming format matching the specified filter was found".into(),
        ));
    }

    // Sort by bitrate
    match filter.quality {
        QualityPreference::Highest => candidates.sort_by_key(|a| std::cmp::Reverse(a.bitrate)),
        QualityPreference::Lowest => candidates.sort_by_key(|a| a.bitrate),
    }

    Ok(candidates[0])
}

/// Filter formats according to rich FormatOptions.
pub fn select_format_with_options<'a>(
    player_response: &'a PlayerResponse,
    options: &FormatOptions,
) -> Result<&'a StreamingFormat> {
    let streaming_data = player_response.streaming_data.as_ref().ok_or_else(|| {
        InnertubeError::NotFound("No streamingData found in player response".into())
    })?;

    let mut candidates: Vec<&'a StreamingFormat> = Vec::new();
    candidates.extend(&streaming_data.formats);
    candidates.extend(&streaming_data.adaptive_formats);

    if let Some(itag) = options.itag {
        if let Some(matched) = candidates.into_iter().find(|f| f.itag == itag) {
            return Ok(matched);
        } else {
            return Err(InnertubeError::NotFound(format!(
                "No format with itag {} was found",
                itag
            )));
        }
    }

    if let Some(format_type) = options.format_type {
        candidates.retain(|f| match format_type {
            FormatType::AudioOnly => f.is_audio_only(),
            FormatType::VideoOnly => f.is_video_only(),
            FormatType::AudioVideo => f.is_audio_video(),
            FormatType::Any => true,
        });
    }

    if let Some(ref container) = options.format {
        if container != "any" {
            candidates.retain(|f| f.mime_type.contains(container));
        }
    }

    if let Some(ref codec) = options.codec {
        candidates.retain(|f| f.mime_type.contains(codec));
    }

    if let Some(ref q) = options.quality {
        let q_lower = q.to_lowercase();
        if q_lower == "best" || q_lower == "highest" {
            candidates.sort_by_key(|a| std::cmp::Reverse(a.bitrate));
        } else if q_lower == "lowest" || q_lower == "bestefficiency" {
            candidates.sort_by_key(|a| a.bitrate);
        } else if let Some(exact_match) = candidates.iter().find(|f| {
            f.quality_label
                .as_ref()
                .is_some_and(|ql| ql.to_lowercase().contains(&q_lower))
        }) {
            return Ok(exact_match);
        }
    }

    if candidates.is_empty() {
        return Err(InnertubeError::NotFound(
            "No streaming format matching the specified options was found".into(),
        ));
    }

    Ok(candidates[0])
}

/// Resolve final playable stream URL by applying decipher transformations if needed.
pub fn resolve_stream_url(
    format: &StreamingFormat,
    decipherer: &PlayerDecipherer,
) -> Result<String> {
    if let Some((raw_url, sp, s)) = format.get_raw_cipher_url() {
        decipherer.apply_to_url(&raw_url, sp.as_deref(), s.as_deref())
    } else if let Some(ref url) = format.url {
        if url.contains("c=MWEB") || url.contains("c=WEB") {
            decipherer.apply_to_url(url, None, None)
        } else {
            Ok(url.clone())
        }
    } else {
        Err(InnertubeError::Format(
            "Format does not contain a valid URL or signature cipher".into(),
        ))
    }
}

/// Fetch Shorts video metadata and reel sequence navigation.
pub async fn fetch_shorts_video_info(
    session: &Session,
    video_id: &str,
    client: Option<&str>,
) -> Result<crate::models::video::ShortFormVideoInfo> {
    let reel_watch_payload = json!({
        "videoId": video_id,
        "disablePlayerResponse": false,
        "params": "CAUwAg%3D%3D",
        "contentCheckOk": true,
        "racyCheckOk": true,
    });

    let sequence_params = crate::utils::proto::encode_reel_sequence_params(video_id)?;
    let sequence_payload = json!({
        "sequenceParams": sequence_params,
    });

    let reel_watch_resp = if let Some(c) = client {
        session
            .post_innertube_client(c, "/player", reel_watch_payload)
            .await?
    } else {
        session.post_innertube("/player", reel_watch_payload).await?
    };

    let sequence_resp = session
        .post_innertube("/reel/reel_watch_sequence", sequence_payload)
        .await?;

    let player_response: PlayerResponse =
        reel_watch_resp.json().await.map_err(InnertubeError::Network)?;

    let seq_json: serde_json::Value = sequence_resp
        .json()
        .await
        .unwrap_or(serde_json::Value::Null);

    let mut watch_next_feed = Vec::new();
    if let Some(entries) = seq_json.get("entries").and_then(|e| e.as_array()) {
        watch_next_feed = entries.clone();
    }

    let continuation_token = seq_json
        .pointer("/continuationEndpoint/continuationCommand/token")
        .or_else(|| {
            seq_json.pointer(
                "/continuationEndpoint/reelWatchSequenceContinuationEndpoint/sequenceParams",
            )
        })
        .and_then(|t| t.as_str())
        .map(|s| s.to_string());

    let cpn = crate::utils::proto::generate_random_string(16);

    Ok(crate::models::video::ShortFormVideoInfo {
        player_response,
        cpn,
        watch_next_feed,
        continuation_token,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::format::StreamingFormat;
    use crate::models::video::{PlayabilityStatus, StreamingData};

    fn make_test_player_response() -> PlayerResponse {
        PlayerResponse {
            playability_status: PlayabilityStatus {
                status: "OK".to_string(),
                reason: None,
                playable_in_embed: Some(true),
            },
            video_details: None,
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
                adaptive_formats: vec![
                    StreamingFormat {
                        itag: 137,
                        url: Some("https://example.com/137.mp4".to_string()),
                        signature_cipher: None,
                        cipher: None,
                        mime_type: "video/mp4; codecs=\"avc1.640028\"".to_string(),
                        bitrate: 2_000_000,
                        width: Some(1920),
                        height: Some(1080),
                        quality_label: Some("1080p".to_string()),
                        audio_quality: None,
                        approx_duration_ms: None,
                        audio_sample_rate: None,
                        audio_channels: None,
                        content_length: None,
                        average_bitrate: None,
                    },
                    StreamingFormat {
                        itag: 140,
                        url: Some("https://example.com/140.m4a".to_string()),
                        signature_cipher: None,
                        cipher: None,
                        mime_type: "audio/mp4; codecs=\"mp4a.40.2\"".to_string(),
                        bitrate: 128_000,
                        width: None,
                        height: None,
                        quality_label: None,
                        audio_quality: Some("AUDIO_QUALITY_MEDIUM".to_string()),
                        approx_duration_ms: None,
                        audio_sample_rate: Some("44100".to_string()),
                        audio_channels: Some(2),
                        content_length: None,
                        average_bitrate: None,
                    },
                ],
                dash_manifest_url: None,
                hls_manifest_url: None,
            }),
            captions: None,
        }
    }

    #[test]
    fn test_select_format_with_options_itag() {
        let resp = make_test_player_response();
        let opts = FormatOptions {
            itag: Some(140),
            ..Default::default()
        };
        let selected = select_format_with_options(&resp, &opts).expect("should find itag 140");
        assert_eq!(selected.itag, 140);
    }

    #[test]
    fn test_select_format_with_options_audio_only() {
        let resp = make_test_player_response();
        let opts = FormatOptions {
            format_type: Some(FormatType::AudioOnly),
            ..Default::default()
        };
        let selected =
            select_format_with_options(&resp, &opts).expect("should find audio only format");
        assert_eq!(selected.itag, 140);
        assert!(selected.is_audio_only());
    }

    #[test]
    fn test_select_format_with_options_quality_1080p() {
        let resp = make_test_player_response();
        let opts = FormatOptions {
            quality: Some("1080p".to_string()),
            ..Default::default()
        };
        let selected = select_format_with_options(&resp, &opts).expect("should find 1080p format");
        assert_eq!(selected.itag, 137);
    }
}
