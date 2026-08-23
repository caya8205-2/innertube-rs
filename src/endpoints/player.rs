use serde_json::json;
use crate::constants::clients;
use crate::core::session::{Session, SessionOptions};
use crate::error::{InnertubeError, Result};
use crate::models::format::{FormatFilter, FormatType, QualityPreference, StreamingFormat};
use crate::models::video::PlayerResponse;
use crate::utils::decipher::PlayerDecipherer;

/// Fetch player metadata and streaming formats for a video from `/youtubei/v1/player`.
pub async fn fetch_player_response(
    session: &Session,
    video_id: &str,
    signature_timestamp: Option<u32>,
) -> Result<PlayerResponse> {
    let mut payload = json!({
        "videoId": video_id,
        "playbackContext": {
            "contentPlaybackContext": {
                "html5Preference": "HTML5_PREF_WANTS"
            }
        }
    });

    if let Some(sts) = signature_timestamp {
        payload["playbackContext"]["contentPlaybackContext"]["signatureTimestamp"] = json!(sts);
    }

    let resp = session.post_innertube("/player", payload).await?;

    if !resp.status().is_success() {
        return Err(InnertubeError::Api {
            status: resp.status().to_string(),
            message: format!("Player endpoint returned HTTP {}", resp.status()),
        });
    }

    let mut player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    // Check if adaptive formats have URLs or ciphers. If not, fallback to ANDROID_VR, then MWEB, then iOS
    let needs_fallback = player_response.streaming_data.as_ref().is_none_or(|sd| {
        sd.adaptive_formats.is_empty() || sd.adaptive_formats.iter().all(|f| f.url.is_none() && f.signature_cipher.is_none() && f.cipher.is_none())
    });

    if needs_fallback {
        if let Ok(vr_response) = fetch_player_response_android_vr(session, video_id).await {
            if let Some(vr_streaming) = vr_response.streaming_data {
                if let Some(ref mut sd) = player_response.streaming_data {
                    sd.formats = vr_streaming.formats;
                    sd.adaptive_formats = vr_streaming.adaptive_formats;
                } else {
                    player_response.streaming_data = Some(vr_streaming);
                }
            }
        } else if let Ok(mweb_response) = fetch_player_response_mweb(session, video_id, signature_timestamp).await {
            if let Some(mweb_streaming) = mweb_response.streaming_data {
                if let Some(ref mut sd) = player_response.streaming_data {
                    sd.formats = mweb_streaming.formats;
                    sd.adaptive_formats = mweb_streaming.adaptive_formats;
                } else {
                    player_response.streaming_data = Some(mweb_streaming);
                }
            }
        } else if let Ok(ios_response) = fetch_player_response_ios(session, video_id).await {
            if let Some(ios_streaming) = ios_response.streaming_data {
                if let Some(ref mut sd) = player_response.streaming_data {
                    sd.formats = ios_streaming.formats;
                    sd.adaptive_formats = ios_streaming.adaptive_formats;
                } else {
                    player_response.streaming_data = Some(ios_streaming);
                }
            }
        }
    }

    if player_response.playability_status.status != "OK" {
        return Err(InnertubeError::Restricted(format!(
            "Video is not playable: {} ({})",
            player_response.playability_status.status,
            player_response.playability_status.reason.as_deref().unwrap_or("No reason provided")
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

    let url = format!("{}/player?key={}", crate::constants::INNERTUBE_API_BASE_URL, session.api_key);
    let resp = session.http_client
        .post(&url)
        .header("User-Agent", clients::MWEB_USER_AGENT)
        .header("X-Youtube-Client-Name", "2")
        .header("X-Youtube-Client-Version", clients::MWEB_VERSION)
        .json(&payload)
        .send()
        .await
        .map_err(InnertubeError::Network)?;
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

    let payload = json!({
        "context": vr_context,
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    let url = format!("{}/player?key={}", crate::constants::INNERTUBE_API_BASE_URL, session.api_key);
    let resp = session.http_client
        .post(&url)
        .header("User-Agent", clients::ANDROID_VR_USER_AGENT)
        .header("X-Youtube-Client-Name", "81")
        .header("X-Youtube-Client-Version", clients::ANDROID_VR_VERSION)
        .json(&payload)
        .send()
        .await
        .map_err(InnertubeError::Network)?;
    let player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    Ok(player_response)
}

/// Fallback player fetch using iOS client to get direct stream URLs for all adaptive formats.
async fn fetch_player_response_ios(
    _session: &Session,
    video_id: &str,
) -> Result<PlayerResponse> {
    let ios_options = SessionOptions {
        client_name: Some(clients::IOS_NAME.to_string()),
        client_version: Some(clients::IOS_VERSION.to_string()),
        device_category: Some("MOBILE".to_string()),
        user_agent: Some(clients::IOS_USER_AGENT.to_string()),
        generate_session_locally: Some(true),
        ..Default::default()
    };

    let ios_session = Session::create(ios_options).await?;

    let payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    let resp = ios_session.post_innertube("/player", payload).await?;
    let player_response: PlayerResponse = resp.json().await.map_err(InnertubeError::Network)?;

    Ok(player_response)
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
