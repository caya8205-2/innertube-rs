use reqwest::header::{HeaderMap, HeaderValue};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::format::{DownloadOptions, FormatOptions, StreamingFormat};
use crate::models::video::{PlayabilityStatus, StreamingData};
use crate::utils::decipher::PlayerDecipherer;

/// Legacy `Constants.STREAM_HEADERS`.
pub fn stream_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert(
        "origin",
        HeaderValue::from_static(crate::constants::YOUTUBE_BASE_URL),
    );
    headers.insert(
        "referer",
        HeaderValue::from_static(crate::constants::YOUTUBE_BASE_URL),
    );
    headers.insert("DNT", HeaderValue::from_static("?1"));
    headers
}

/// Legacy `FormatUtils.chooseFormat` — exact filter/sort semantics.
pub fn choose_format<'a>(
    options: &FormatOptions,
    streaming_data: &'a StreamingData,
) -> Result<&'a StreamingFormat> {
    let mut formats: Vec<&'a StreamingFormat> = Vec::new();
    formats.extend(streaming_data.formats.iter());
    formats.extend(streaming_data.adaptive_formats.iter());

    if let Some(itag) = options.itag {
        return formats
            .into_iter()
            .find(|f| f.itag == itag)
            .ok_or_else(|| InnertubeError::NotFound("No matching formats found".to_string()));
    }

    let type_opt = options.format_type;
    let requires_audio = type_opt.is_none_or(|t| {
        matches!(
            t,
            crate::models::format::FormatType::AudioOnly
                | crate::models::format::FormatType::AudioVideo
        )
    });
    let requires_video = type_opt.is_none_or(|t| {
        matches!(
            t,
            crate::models::format::FormatType::VideoOnly
                | crate::models::format::FormatType::AudioVideo
        )
    });
    let language = options.language.as_deref().unwrap_or("original");
    let quality = options.quality.as_deref().unwrap_or("best");

    let mut best_width: i64 = -1;
    let is_best = matches!(quality, "best" | "bestefficiency");
    let use_most_efficient = quality != "best";

    let mut candidates: Vec<&StreamingFormat> = formats
        .into_iter()
        .filter(|f| {
            if requires_audio && !f.has_audio() {
                return false;
            }
            if requires_video && !f.has_video() {
                return false;
            }
            if let Some(ref codec) = options.codec {
                if !f.mime_type.contains(codec.as_str()) {
                    return false;
                }
            }
            let format_opt = options.format.as_deref();
            if format_opt != Some("any")
                && !f.mime_type.contains(format_opt.unwrap_or("mp4"))
            {
                return false;
            }
            if !is_best && f.quality_label.as_deref() != Some(quality) {
                return false;
            }
            if let Some(width) = f.width {
                if best_width < width as i64 {
                    best_width = width as i64;
                }
            }
            true
        })
        .collect();

    if candidates.is_empty() {
        return Err(InnertubeError::NotFound(
            "No matching formats found".to_string(),
        ));
    }

    if is_best && requires_video {
        candidates.retain(|f| f.width.map(|w| w as i64) == Some(best_width));
    }

    if requires_audio && !requires_video {
        let audio_only: Vec<&StreamingFormat> = candidates
            .iter()
            .copied()
            .filter(|f| {
                if language != "original" {
                    !f.has_video() && !f.has_text() && f.language().as_deref() == Some(language)
                } else {
                    !f.has_video() && !f.has_text() && f.is_original()
                }
            })
            .collect();
        if !audio_only.is_empty() {
            candidates = audio_only;
        }
    }

    if use_most_efficient {
        candidates.sort_by_key(|f| f.bitrate);
    } else {
        candidates.sort_by_key(|f| std::cmp::Reverse(f.bitrate));
    }

    candidates
        .first()
        .copied()
        .ok_or_else(|| InnertubeError::NotFound("No matching formats found".to_string()))
}

/// Legacy `FormatUtils.download`: playability guards, default options
/// (360p / video+audio / mp4), `cpn` appended, STREAM_HEADERS.
///
/// Non-ranged combined downloads return a single GET response. Ranged or
/// adaptive downloads use 10MB chunks via the `range=` **query param**
/// (never a Range header), collected into one buffer.
///
/// ponytail: chunks are collected in memory; legacy streams them. Add a
/// streaming adapter when a stream-combinators dependency lands.
pub async fn download(
    session: &Session,
    options: &DownloadOptions,
    playability_status: Option<&PlayabilityStatus>,
    streaming_data: Option<&StreamingData>,
    decipherer: &PlayerDecipherer,
    po_token: Option<&str>,
    cpn: Option<&str>,
) -> Result<bytes::Bytes> {
    if let Some(ps) = playability_status {
        if ps.status == "UNPLAYABLE" {
            return Err(InnertubeError::Restricted("Video is unplayable".to_string()));
        }
        if ps.status == "LOGIN_REQUIRED" {
            return Err(InnertubeError::AuthenticationRequired(
                "Video is login required".to_string(),
            ));
        }
    }
    let streaming_data = streaming_data.ok_or_else(|| {
        InnertubeError::NotFound("Streaming data not available.".to_string())
    })?;

    let mut opts = options.clone();
    if opts.format_options.quality.is_none() {
        opts.format_options.quality = Some("360p".to_string());
    }
    if opts.format_options.format_type.is_none() {
        opts.format_options.format_type = Some(crate::models::format::FormatType::AudioVideo);
    }
    if opts.format_options.format.is_none() {
        opts.format_options.format = Some("mp4".to_string());
    }

    let format = choose_format(&opts.format_options, streaming_data)?;
    let mut format_url = crate::endpoints::player::resolve_stream_url_full(
        format,
        decipherer,
        po_token,
        None,
    )?;

    if let Some(cpn) = cpn {
        format_url = format!("{format_url}&cpn={cpn}");
    }

    let content_length: u64 = format
        .content_length
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let combined_no_range =
        opts.format_options.format_type == Some(crate::models::format::FormatType::AudioVideo)
            && options.range.is_none();

    if combined_no_range {
        let resp = session
            .http_client
            .get(&format_url)
            .headers(stream_headers())
            .send()
            .await
            .map_err(InnertubeError::Network)?;
        let resp = Session::ensure_success("download", resp).await?;
        return resp.bytes().await.map_err(InnertubeError::Network);
    }

    const CHUNK_SIZE: u64 = 10 * 1_048_576;
    let mut chunk_start = options.range.map(|r| r.start).unwrap_or(0);
    let mut chunk_end = options.range.map(|r| r.end).unwrap_or(CHUNK_SIZE);
    let mut buffer = Vec::new();

    loop {
        let resp = session
            .http_client
            .get(format!("{format_url}&range={chunk_start}-{chunk_end}"))
            .headers(stream_headers())
            .send()
            .await
            .map_err(InnertubeError::Network)?;
        let resp = Session::ensure_success("download chunk", resp).await?;
        let chunk = resp.bytes().await.map_err(InnertubeError::Network)?;
        buffer.extend_from_slice(&chunk);

        if chunk_end >= content_length || options.range.is_some() {
            break;
        }
        chunk_start = chunk_end + 1;
        chunk_end += CHUNK_SIZE;
    }

    Ok(bytes::Bytes::from(buffer))
}
