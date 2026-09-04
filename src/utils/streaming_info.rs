use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::format::{FormatRange, StreamingFormat};
use crate::models::transcript::TranscriptTrack;
use crate::models::video::StreamingData;
use crate::parser::nodes::misc::player_overlay::PlayerStoryboardSpecNode;
use crate::utils::decipher::PlayerDecipherer;
use crate::utils::format::stream_headers;

/// A segment timeline entry (legacy `Segment`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub duration: u64,
    pub repeat_count: Option<u64>,
}

/// Segment template for OTF / post-live-DVR streams (legacy
/// `SegmentTemplate`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentTemplate {
    pub init_url: Option<String>,
    pub media_url: String,
    pub timeline: Vec<Segment>,
}

/// Legacy `SegmentInfo` union: byte-range base URL or a resolved segment
/// template.
///
/// ponytail: legacy defers OTF/DVR template resolution behind
/// `getSegmentTemplate()`; we resolve eagerly during construction when a
/// session is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentInfo {
    Base {
        base_url: String,
        index_range: FormatRange,
        init_range: FormatRange,
    },
    Template(SegmentTemplate),
}

impl SegmentInfo {
    pub fn is_otf(&self) -> bool {
        matches!(self, Self::Template(_))
    }
}

/// CICP-mapped color info (legacy `ColorInfo`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColorInfo {
    pub primaries: Option<&'static str>,
    pub transfer_characteristics: Option<&'static str>,
    pub matrix_coefficients: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioRepresentation {
    pub uid: String,
    pub bitrate: u64,
    pub codecs: Option<String>,
    pub audio_sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub segment_info: SegmentInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioSet {
    pub mime_type: String,
    pub language: Option<String>,
    pub codecs: Option<String>,
    pub audio_sample_rate: Option<u32>,
    pub track_name: Option<String>,
    pub track_roles: Vec<String>,
    pub channels: Option<u32>,
    pub drm_families: Option<Vec<String>>,
    pub drm_track_type: Option<String>,
    pub representations: Vec<AudioRepresentation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoRepresentation {
    pub uid: String,
    pub bitrate: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codecs: Option<String>,
    pub fps: Option<u32>,
    pub segment_info: SegmentInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoSet {
    pub mime_type: String,
    pub color_info: ColorInfo,
    pub codecs: Option<String>,
    pub fps: Option<u32>,
    pub drm_families: Option<Vec<String>>,
    pub drm_track_type: Option<String>,
    pub representations: Vec<VideoRepresentation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRepresentation {
    pub uid: String,
    pub bitrate: Option<u64>,
    pub sheet_width: u32,
    pub sheet_height: u32,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub rows: u32,
    pub columns: u32,
    pub template_url: String,
    pub template_duration: u64,
}

impl ImageRepresentation {
    pub fn get_url(&self, n: u32) -> String {
        self.template_url.replace("$Number$", &n.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageSet {
    pub mime_type: String,
    pub representations: Vec<ImageRepresentation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRepresentation {
    pub uid: String,
    pub base_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSet {
    pub mime_type: String,
    pub language: String,
    pub track_name: String,
    pub track_roles: Vec<String>,
    pub representation: TextRepresentation,
}

/// Legacy `StreamingInfo`.
#[derive(Debug, Clone)]
pub struct StreamingInfo {
    pub duration_secs: f64,
    pub audio_sets: Vec<AudioSet>,
    pub video_sets: Vec<VideoSet>,
    pub image_sets: Vec<ImageSet>,
    pub text_sets: Vec<TextSet>,
}

/// Legacy `StreamingInfoOptions`. `*_multiple` labels use `{name}` as the
/// display-name placeholder.
#[derive(Debug, Clone, Default)]
pub struct StreamingInfoOptions {
    pub captions_format: Option<String>,
    pub label_original: Option<String>,
    pub label_drc: Option<String>,
    pub label_drc_multiple: Option<String>,
    pub label_vb: Option<String>,
    pub label_vb_multiple: Option<String>,
    pub is_sabr: bool,
}

/// A parsed storyboard board (legacy `PlayerStoryboardSpec` board parsing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storyboard {
    pub is_live: bool,
    pub template_url: String,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub thumbnail_count: u32,
    pub columns: u32,
    pub rows: u32,
    pub storyboard_count: u32,
}

/// Parse the storyboard spec string into boards (legacy
/// `PlayerStoryboardSpec` constructor: `url|w#h#count#cols#rows#interval#name#sigh|...`).
pub fn parse_storyboard_spec(node: &PlayerStoryboardSpecNode) -> Vec<Storyboard> {
    let Some(ref spec) = node.spec else {
        return Vec::new();
    };
    let is_live = spec.contains("playerLiveStoryboardSpec") || node.spec.is_none();
    let mut parts = spec.split('|');
    let Some(base_url) = parts.next() else {
        return Vec::new();
    };

    parts
        .enumerate()
        .filter_map(|(i, part)| {
            let fields: Vec<&str> = part.split('#').collect();
            if fields.len() < 8 {
                return None;
            }
            let thumbnail_width = fields[0].parse().ok()?;
            let thumbnail_height = fields[1].parse().ok()?;
            let thumbnail_count: u32 = fields[2].parse().ok()?;
            let columns: u32 = fields[3].parse().ok()?;
            let rows: u32 = fields[4].parse().ok()?;
            let name = fields[6];
            let sigh = fields[7];

            let url = url::Url::parse(base_url).ok().map(|mut u| {
                u.query_pairs_mut().append_pair("sigh", sigh);
                u.to_string()
            })?;
            let template_url = url.replace("$L", &i.to_string()).replace("$N", name);

            let storyboard_count = thumbnail_count.div_ceil(columns * rows);

            Some(Storyboard {
                is_live,
                template_url,
                thumbnail_width,
                thumbnail_height,
                thumbnail_count,
                columns,
                rows,
                storyboard_count,
            })
        })
        .collect()
}

fn get_string_between<'a>(input: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_idx = input.find(start)? + start.len();
    let rest = &input[start_idx..];
    let end_idx = rest.find(end)?;
    Some(&rest[..end_idx])
}

fn format_bitrate_grouping(f: &StreamingFormat) -> String {
    let mime = f.mime_type.split(';').next().unwrap_or("");
    let codec = get_string_between(&f.mime_type, "codecs=\"", "\"")
        .and_then(|c| c.split('.').next())
        .unwrap_or("");
    let color = f
        .color_info
        .as_ref()
        .map(|c| {
            [
                c.primaries.as_deref().unwrap_or(""),
                c.transfer_characteristics.as_deref().unwrap_or(""),
                c.matrix_coefficients.as_deref().unwrap_or(""),
            ]
            .join("-")
        })
        .unwrap_or_default();
    let track_id = f.audio_track.as_ref().map(|t| t.id.as_str()).unwrap_or("");
    let drc = if f.is_drc() { "drc" } else { "" };
    let vb = if f.is_vb() { "vb" } else { "" };
    format!("{mime}-{codec}-{color}-{track_id}-{drc}-{vb}")
}

fn hoist_codecs(formats: &[&StreamingFormat], hoisted: &mut Vec<&'static str>) -> Option<String> {
    if formats.len() > 1 {
        let codecs: std::collections::HashSet<Option<String>> = formats
            .iter()
            .map(|f| get_string_between(&f.mime_type, "codecs=\"", "\"").map(String::from))
            .collect();
        if codecs.len() == 1 {
            hoisted.push("codecs");
            return codecs.into_iter().next().flatten();
        }
    }
    None
}

fn hoist_u32(
    formats: &[&StreamingFormat],
    property: &'static str,
    get: impl Fn(&StreamingFormat) -> Option<u32>,
    hoisted: &mut Vec<&'static str>,
) -> Option<u32> {
    if formats.len() > 1 {
        let values: std::collections::HashSet<Option<u32>> = formats.iter().map(|f| get(f)).collect();
        if values.len() == 1 {
            hoisted.push(property);
            return values.into_iter().next().flatten();
        }
    }
    None
}

fn map_color_info(format: &StreamingFormat) -> ColorInfo {
    let mut info = ColorInfo::default();

    if let Some(ref color) = format.color_info {
        info.primaries = color.primaries.as_deref().and_then(|p| match p {
            "BT709" => Some("1"),
            "BT2020" => Some("9"),
            _ => None,
        });
        info.transfer_characteristics =
            color.transfer_characteristics.as_deref().and_then(|t| match t {
                "BT709" => Some("1"),
                "BT2020_10" => Some("14"),
                "SMPTEST2084" => Some("16"),
                "ARIB_STD_B67" => Some("18"),
                _ => None,
            });
        // ponytail: legacy logs a warning for unknown matrix coefficients
        // and omits the value; we omit silently (no log facade).
        info.matrix_coefficients =
            color.matrix_coefficients.as_deref().and_then(|m| match m {
                "BT709" => Some("1"),
                "BT2020_NCL" => Some("14"),
                _ => None,
            });
    } else if get_string_between(&format.mime_type, "codecs=\"", "\"")
        .is_some_and(|c| c.starts_with("avc1"))
    {
        // YouTube's h264 streams are always SDR.
        info.transfer_characteristics = Some("1");
    }

    info
}

fn track_roles(format: &StreamingFormat, has_drc_streams: bool) -> Vec<String> {
    if format.audio_track.is_none() && !has_drc_streams {
        return Vec::new();
    }

    let mut roles = vec![
        if format.is_original() { "main" } else { "alternate" }.to_string()
    ];
    if format.is_dubbed() || format.is_auto_dubbed() {
        roles.push("dub".to_string());
    }
    if format.is_descriptive() {
        roles.push("description".to_string());
    }
    if format.is_drc() || format.is_vb() {
        roles.push("enhanced-audio-intelligibility".to_string());
    }
    roles
}

/// Resolve a format's stream URL (decipher when a decipherer is given) or
/// build the SABR pseudo-URL (legacy `sabr://video|audio?key=itag:xtags`).
fn resolve_format_url(
    format: &StreamingFormat,
    decipherer: Option<&PlayerDecipherer>,
    po_token: Option<&str>,
    cpn: Option<&str>,
    is_sabr: bool,
) -> Result<String> {
    if is_sabr {
        let key = format!("{}:{}", format.itag, format.xtags.as_deref().unwrap_or(""));
        return Ok(format!(
            "sabr://{}?key={key}",
            if format.has_video() { "video" } else { "audio" }
        ));
    }

    let mut url = match decipherer {
        Some(d) => crate::endpoints::player::resolve_stream_url_full(format, d, po_token, None)?,
        None => format
            .get_raw_cipher_url()
            .map(|(u, _, _)| u)
            .ok_or_else(|| InnertubeError::Format("Format has no URL".to_string()))?,
    };

    url = match url::Url::parse(&url) {
        Ok(mut u) => {
            u.query_pairs_mut().append_pair("cpn", cpn.unwrap_or(""));
            u.to_string()
        }
        Err(_) => format!("{url}&cpn={}", cpn.unwrap_or("")),
    };

    Ok(url)
}

/// Fetch the OTF segment template (`&rn=0&sq=0` scrape of
/// `Segment-Durations-Ms`).
async fn get_otf_segment_template(session: &Session, url: &str) -> Result<SegmentTemplate> {
    let resp = session
        .http_client
        .get(format!("{url}&rn=0&sq=0"))
        .headers(stream_headers())
        .send()
        .await
        .map_err(InnertubeError::Network)?;

    // Resolved URL after redirects, without the probe params.
    let resolved_url = resp
        .url()
        .as_str()
        .replace("&rn=0", "")
        .replace("&sq=0", "");

    let text = resp.text().await.map_err(InnertubeError::Network)?;
    let durations_str = get_string_between(&text, "Segment-Durations-Ms:", "\r\n")
        .ok_or_else(|| InnertubeError::Format(format!(
            "Failed to extract the segment durations from this OTF stream ({url})"
        )))?;

    let mut timeline = Vec::new();
    for part in durations_str.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let repeat_count = get_string_between(trimmed, "(r=", ")").and_then(|s| s.parse().ok());
        let duration_str = trimmed.split('(').next().unwrap_or(trimmed);
        let duration: u64 = duration_str
            .parse()
            .map_err(|_| InnertubeError::Format("Invalid OTF segment duration".to_string()))?;
        timeline.push(Segment {
            duration,
            repeat_count,
        });
    }

    Ok(SegmentTemplate {
        init_url: Some(format!("{resolved_url}&sq=0")),
        media_url: format!("{resolved_url}&sq=$Number$"),
        timeline,
    })
}

/// Post-live DVR info from `X-Head-Time-Millis` / `X-Head-Seqnum` headers.
async fn get_post_live_dvr_info(session: &Session, url: &str) -> Result<(f64, u64)> {
    let resp = session
        .http_client
        .head(format!("{url}&rn=0&sq=0"))
        .headers(stream_headers())
        .send()
        .await
        .map_err(InnertubeError::Network)?;

    let parse_header = |name: &str| -> Option<u64> {
        resp.headers().get(name)?.to_str().ok()?.parse().ok()
    };

    let duration_ms = parse_header("X-Head-Time-Millis");
    let segment_count = parse_header("X-Head-Seqnum");

    match (duration_ms, segment_count) {
        (Some(ms), Some(count)) => Ok((ms as f64 / 1000.0, count)),
        _ => Err(InnertubeError::Format(
            "Failed to extract the duration or segment count for this Post Live DVR video"
                .to_string(),
        )),
    }
}

struct BuildContext<'a> {
    session: Option<&'a Session>,
    decipherer: Option<&'a PlayerDecipherer>,
    po_token: Option<&'a str>,
    cpn: Option<&'a str>,
    is_sabr: bool,
    is_post_live_dvr: bool,
    dvr_info: Option<(f64, u64)>,
}

impl<'a> BuildContext<'a> {
    async fn segment_info(&mut self, format: &StreamingFormat) -> Result<SegmentInfo> {
        let url = resolve_format_url(
            format,
            self.decipherer,
            self.po_token,
            self.cpn,
            self.is_sabr,
        )?;

        if format.is_type_otf.unwrap_or(false) {
            let session = self.session.ok_or_else(|| {
                InnertubeError::Other(
                    "Unable to get segment durations for this OTF stream without a session"
                        .to_string(),
                )
            })?;
            return Ok(SegmentInfo::Template(
                get_otf_segment_template(session, &url).await?,
            ));
        }

        if self.is_post_live_dvr {
            let session = self.session.ok_or_else(|| {
                InnertubeError::Other(
                    "Unable to get segment count for this Post Live DVR video without a session"
                        .to_string(),
                )
            })?;
            let target_duration_sec = format.target_duration_sec.ok_or_else(|| {
                InnertubeError::Format("Format is missing target_duration_sec".to_string())
            })?;
            if self.dvr_info.is_none() {
                self.dvr_info = Some(get_post_live_dvr_info(session, &url).await?);
            }
            let (_, segment_count) = self.dvr_info.unwrap_or((0.0, 0));
            return Ok(SegmentInfo::Template(SegmentTemplate {
                init_url: None,
                media_url: format!("{url}&sq=$Number$"),
                timeline: vec![Segment {
                    duration: (target_duration_sec * 1000.0) as u64,
                    repeat_count: Some(segment_count),
                }],
            }));
        }

        let (Some(index_range), Some(init_range)) = (&format.index_range, &format.init_range)
        else {
            return Err(InnertubeError::Format(
                "Index and init ranges not available".to_string(),
            ));
        };

        Ok(SegmentInfo::Base {
            base_url: url,
            index_range: index_range.clone(),
            init_range: init_range.clone(),
        })
    }
}

/// Inputs for [`get_streaming_info`] (legacy positional args grouped).
#[derive(Default)]
pub struct StreamingInfoParams<'a> {
    pub session: Option<&'a Session>,
    pub decipherer: Option<&'a PlayerDecipherer>,
    pub cpn: Option<&'a str>,
    pub po_token: Option<&'a str>,
    pub storyboards: Option<&'a PlayerStoryboardSpecNode>,
    pub caption_tracks: Option<&'a [TranscriptTrack]>,
    pub options: StreamingInfoOptions,
}

/// Legacy `StreamingInfo.getStreamingInfo` port.
///
/// `format_filter` rejects formats (legacy `rejectFormat` semantics).
/// The URL transformer of legacy is not ported (identity); add when a
/// consumer needs it.
pub async fn get_streaming_info(
    streaming_data: &StreamingData,
    is_post_live_dvr: bool,
    format_filter: Option<&dyn Fn(&StreamingFormat) -> bool>,
    params: StreamingInfoParams<'_>,
) -> Result<StreamingInfo> {
    let StreamingInfoParams {
        session,
        decipherer,
        cpn,
        po_token,
        storyboards,
        caption_tracks,
        options,
    } = params;
    let options = &options;
    let formats: Vec<&StreamingFormat> = match format_filter {
        Some(filter) => streaming_data
            .adaptive_formats
            .iter()
            .filter(|f| !filter(f))
            .collect(),
        None => streaming_data.adaptive_formats.iter().collect(),
    };

    if formats.is_empty() {
        return Err(InnertubeError::NotFound(
            "No adaptive formats available".to_string(),
        ));
    }

    let has_multiple_audio_tracks = formats.iter().any(|f| f.audio_track.is_some());

    // Group by mime-codec-colorInfo-audioTrackId-drc-vb.
    let mut groups: Vec<Vec<&StreamingFormat>> = Vec::new();
    for format in &formats {
        let has_ranges = format.index_range.is_some() && format.init_range.is_some();
        if !has_ranges && !format.is_type_otf.unwrap_or(false) && !is_post_live_dvr {
            continue;
        }
        let group_id = format_bitrate_grouping(format);
        match groups.iter_mut().find(|g| format_bitrate_grouping(g[0]) == group_id) {
            Some(group) => group.push(format),
            None => groups.push(vec![format]),
        }
    }

    let mut audio_groups: Vec<Vec<&StreamingFormat>> = Vec::new();
    let mut video_groups: Vec<Vec<&StreamingFormat>> = Vec::new();
    for group in groups {
        if group[0].has_audio() {
            // Skip broken no-track group when multiple audio tracks exist.
            if has_multiple_audio_tracks && group[0].audio_track.is_none() {
                continue;
            }
            audio_groups.push(group);
        } else {
            video_groups.push(group);
        }
    }

    let has_drc = audio_groups.iter().flatten().any(|f| f.is_drc());
    let has_vb = audio_groups.iter().flatten().any(|f| f.is_vb());

    let label_original = || options.label_original.clone().unwrap_or_else(|| "Original".to_string());
    let label_drc = || options.label_drc.clone().unwrap_or_else(|| "Stable Volume".to_string());
    let label_vb = || options.label_vb.clone().unwrap_or_else(|| "Voice Boost".to_string());

    let mut ctx = BuildContext {
        session,
        decipherer,
        po_token,
        cpn,
        is_sabr: options.is_sabr,
        is_post_live_dvr,
        dvr_info: None,
    };

    let mut audio_sets = Vec::new();
    for group in audio_groups {
        let first = group[0];
        let mut hoisted: Vec<&'static str> = Vec::new();

        let track_name = if let Some(ref track) = first.audio_track {
            if has_drc && first.is_drc() {
                Some(
                    options
                        .label_drc_multiple
                        .clone()
                        .unwrap_or_else(|| "{name} (Stable Volume)".to_string())
                        .replace("{name}", &track.display_name),
                )
            } else if has_vb && first.is_vb() {
                Some(
                    options
                        .label_vb_multiple
                        .clone()
                        .unwrap_or_else(|| "{name} (Voice Boost)".to_string())
                        .replace("{name}", &track.display_name),
                )
            } else {
                Some(track.display_name.clone())
            }
        } else if has_drc || has_vb {
            if has_drc && first.is_drc() {
                Some(label_drc())
            } else if has_vb && first.is_vb() {
                Some(label_vb())
            } else {
                Some(label_original())
            }
        } else {
            None
        };

        let set = AudioSet {
            mime_type: first.mime_type.split(';').next().unwrap_or("").to_string(),
            language: first.language(),
            codecs: hoist_codecs(&group, &mut hoisted),
            audio_sample_rate: hoist_u32(
                &group,
                "audio_sample_rate",
                |f| f.audio_sample_rate.as_deref().and_then(|s| s.parse().ok()),
                &mut hoisted,
            ),
            track_name,
            track_roles: track_roles(first, has_drc),
            channels: hoist_u32(&group, "AudioChannelConfiguration", |f| {
                Some(f.audio_channels.unwrap_or(2))
            }, &mut hoisted),
            drm_families: first.drm_families.clone(),
            drm_track_type: first.drm_track_type.clone(),
            representations: {
                let mut reps = Vec::new();
                for format in &group {
                    let mut uid_parts = vec![format.itag.to_string()];
                    if let Some(ref track) = format.audio_track {
                        uid_parts.push(track.id.clone());
                    }
                    if format.is_drc() {
                        uid_parts.push("drc".to_string());
                    }
                    if format.is_vb() {
                        uid_parts.push("vb".to_string());
                    }
                    reps.push(AudioRepresentation {
                        uid: uid_parts.join("-"),
                        bitrate: format.bitrate,
                        codecs: (!hoisted.contains(&"codecs"))
                            .then(|| {
                                get_string_between(&format.mime_type, "codecs=\"", "\"")
                                    .map(String::from)
                            })
                            .flatten(),
                        audio_sample_rate: (!hoisted.contains(&"audio_sample_rate"))
                            .then(|| {
                                format.audio_sample_rate.as_deref().and_then(|s| s.parse().ok())
                            })
                            .flatten(),
                        channels: (!hoisted.contains(&"AudioChannelConfiguration"))
                            .then_some(format.audio_channels.unwrap_or(2)),
                        segment_info: ctx.segment_info(format).await?,
                    });
                }
                reps
            },
        };
        audio_sets.push(set);
    }

    let mut video_sets = Vec::new();
    for group in video_groups {
        let first = group[0];
        let mut hoisted: Vec<&'static str> = Vec::new();

        let set = VideoSet {
            mime_type: first.mime_type.split(';').next().unwrap_or("").to_string(),
            color_info: map_color_info(first),
            codecs: hoist_codecs(&group, &mut hoisted),
            fps: hoist_u32(&group, "fps", |f| f.fps, &mut hoisted),
            drm_families: first.drm_families.clone(),
            drm_track_type: first.drm_track_type.clone(),
            representations: {
                let mut reps = Vec::new();
                for format in &group {
                    reps.push(VideoRepresentation {
                        uid: format.itag.to_string(),
                        bitrate: format.bitrate,
                        width: format.width,
                        height: format.height,
                        codecs: (!hoisted.contains(&"codecs"))
                            .then(|| {
                                get_string_between(&format.mime_type, "codecs=\"", "\"")
                                    .map(String::from)
                            })
                            .flatten(),
                        fps: if hoisted.contains(&"fps") { None } else { format.fps },
                        segment_info: ctx.segment_info(format).await?,
                    });
                }
                reps
            },
        };
        video_sets.push(set);
    }

    // Storyboard image sets (skipped without a session, per legacy).
    let mut image_sets = Vec::new();
    if let (Some(boards_node), Some(session)) = (storyboards, session) {
        let boards = parse_storyboard_spec(boards_node);
        let duration_secs = if boards.iter().all(|b| !b.is_live) {
            formats[0]
                .approx_duration_ms
                .as_deref()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0)
                / 1000.0
        } else {
            formats[0].target_duration_sec.ok_or_else(|| {
                InnertubeError::Format("Format is missing target_duration_sec".to_string())
            })?
        };

        let mut mime_groups: Vec<(String, Vec<&Storyboard>)> = Vec::new();
        for board in &boards {
            let extension = url::Url::parse(&board.template_url)
                .ok()
                .and_then(|u| u.path().rsplit('.').next().map(String::from))
                .unwrap_or_default();
            let mime = format!(
                "image/{}",
                if extension == "jpg" { "jpeg" } else { &extension }
            );
            match mime_groups.iter_mut().find(|(m, _)| *m == mime) {
                Some((_, list)) => list.push(board),
                None => mime_groups.push((mime, vec![board])),
            }
        }

        for (mime, boards) in mime_groups {
            let mut reps = Vec::new();
            for board in boards {
                // Estimated bitrate from the largest HEAD content-length
                // over up to 10 boards.
                // Legacy: min(vod ? storyboard_count : 5, 10).
                let request_limit = if board.is_live {
                    5
                } else {
                    board.storyboard_count.min(10)
                };
                let mut max_content_length = 0u64;
                let mut actual_mime: Option<String> = None;
                for i in 0..request_limit {
                    let url = board.template_url.replace("$M", &i.to_string());
                    if let Ok(resp) = session
                        .http_client
                        .head(&url)
                        .headers(stream_headers())
                        .send()
                        .await
                    {
                        if actual_mime.is_none() {
                            actual_mime = resp
                                .headers()
                                .get("Content-Type")
                                .and_then(|v| v.to_str().ok())
                                .map(String::from);
                        }
                        if let Some(len) = resp
                            .headers()
                            .get("Content-Length")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                        {
                            max_content_length = max_content_length.max(len);
                        }
                    }
                }

                let _ = &actual_mime; // probed but probable mime kept (legacy getMimeType)
                let bitrate = ((max_content_length as f64
                    / (board.rows * board.columns) as f64)
                    * 8.0)
                    .ceil() as u64;

                let template_duration = if !board.is_live {
                    duration_secs / board.storyboard_count as f64
                } else {
                    duration_secs * (board.columns * board.rows) as f64
                };

                reps.push(ImageRepresentation {
                    uid: format!(
                        "thumbnails_{}x{}",
                        board.thumbnail_width, board.thumbnail_height
                    ),
                    bitrate: Some(bitrate),
                    sheet_width: board.thumbnail_width * board.columns,
                    sheet_height: board.thumbnail_height * board.rows,
                    thumbnail_width: board.thumbnail_width,
                    thumbnail_height: board.thumbnail_height,
                    rows: board.rows,
                    columns: board.columns,
                    template_url: board.template_url.replace("$M", "$Number$"),
                    template_duration: template_duration.round() as u64,
                });
            }
            image_sets.push(ImageSet {
                mime_type: mime,
                representations: reps,
            });
        }
    }

    // Caption text sets.
    let mut text_sets = Vec::new();
    if let (Some(tracks), Some(captions_format)) = (caption_tracks, options.captions_format.as_deref()) {
        if captions_format != "vtt" && captions_format != "ttml" {
            return Err(InnertubeError::Format(format!(
                "Invalid captions format: {captions_format}"
            )));
        }
        let mime_type = if captions_format == "vtt" {
            "text/vtt"
        } else {
            "application/ttml+xml"
        };

        for track in tracks {
            let mut parsed = url::Url::parse(&track.base_url)
                .map_err(|e| InnertubeError::Format(format!("Invalid caption URL: {e}")))?;
            parsed.query_pairs_mut().append_pair("fmt", captions_format);

            let mut roles = vec!["caption".to_string()];
            let has_tlang = parsed.query_pairs().any(|(k, _)| k == "tlang");
            if has_tlang {
                roles.push("dub".to_string());
            }

            text_sets.push(TextSet {
                mime_type: mime_type.to_string(),
                language: track.language_code.clone(),
                track_name: track.name.clone(),
                track_roles: roles,
                representation: TextRepresentation {
                    uid: format!(
                        "text-{}",
                        track.vss_id.as_deref().unwrap_or(&track.language_code)
                    ),
                    base_url: parsed.to_string(),
                },
            });
        }
    }

    let duration_secs = if is_post_live_dvr {
        ctx.dvr_info.map(|(d, _)| d).unwrap_or(0.0)
    } else {
        formats[0]
            .approx_duration_ms
            .as_deref()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
            / 1000.0
    };

    Ok(StreamingInfo {
        duration_secs,
        audio_sets,
        video_sets,
        image_sets,
        text_sets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::format::{AudioTrack, FormatRange};

    /// Encode xtags the way YouTube does: base64url protobuf FormatXTags.
    fn encode_xtags(pairs: &[(&str, &str)]) -> String {
        use base64::Engine;
        use prost::Message;

        let msg = crate::proto::misc::FormatXTags {
            xtags: pairs
                .iter()
                .map(|(k, v)| crate::proto::misc::KeyValuePair {
                    key: Some(k.to_string()),
                    value: Some(v.to_string()),
                })
                .collect(),
        };
        base64::engine::general_purpose::URL_SAFE.encode(msg.encode_to_vec())
    }

    fn audio_format(itag: u32, bitrate: u64, codec: &str) -> StreamingFormat {
        StreamingFormat {
            itag,
            url: Some(format!("https://googlevideo.com/v?itag={itag}")),
            mime_type: format!("audio/mp4; codecs=\"{codec}\""),
            bitrate,
            audio_quality: Some("AUDIO_QUALITY_MEDIUM".to_string()),
            audio_bitrate: Some(128),
            audio_sample_rate: Some("44100".to_string()),
            audio_channels: Some(2),
            approx_duration_ms: Some("213000".to_string()),
            index_range: Some(FormatRange {
                start: "592".to_string(),
                end: "1000".to_string(),
            }),
            init_range: Some(FormatRange {
                start: "0".to_string(),
                end: "591".to_string(),
            }),
            ..Default::default()
        }
    }

    fn video_format(itag: u32, bitrate: u64, codec: &str, height: u32) -> StreamingFormat {
        StreamingFormat {
            itag,
            url: Some(format!("https://googlevideo.com/v?itag={itag}")),
            mime_type: format!("video/mp4; codecs=\"{codec}\""),
            bitrate,
            quality_label: Some(format!("{height}p")),
            width: Some(height * 16 / 9),
            height: Some(height),
            fps: Some(30),
            approx_duration_ms: Some("213000".to_string()),
            index_range: Some(FormatRange {
                start: "592".to_string(),
                end: "1000".to_string(),
            }),
            init_range: Some(FormatRange {
                start: "0".to_string(),
                end: "591".to_string(),
            }),
            ..Default::default()
        }
    }

    fn data(formats: Vec<StreamingFormat>) -> StreamingData {
        StreamingData {
            expires_in_seconds: None,
            formats: vec![],
            adaptive_formats: formats,
            dash_manifest_url: None,
            hls_manifest_url: None,
        }
    }

    #[tokio::test]
    async fn groups_and_hoists_like_legacy() {
        let sd = data(vec![
            audio_format(140, 128_000, "mp4a.40.2"),
            audio_format(139, 64_000, "mp4a.40.2"),
            video_format(137, 2_000_000, "avc1.640028", 1080),
            video_format(136, 1_000_000, "avc1.640028", 720),
        ]);

        let info = get_streaming_info(&sd, false, None, StreamingInfoParams::default())
            .await
            .unwrap();

        assert_eq!(info.duration_secs, 213.0);

        // One audio set with hoisted codecs/sample-rate/channels.
        assert_eq!(info.audio_sets.len(), 1);
        let audio = &info.audio_sets[0];
        assert_eq!(audio.mime_type, "audio/mp4");
        assert_eq!(audio.codecs.as_deref(), Some("mp4a.40.2"));
        assert_eq!(audio.audio_sample_rate, Some(44100));
        assert_eq!(audio.channels, Some(2));
        assert_eq!(audio.representations.len(), 2);
        // Hoisted attrs are not repeated per representation.
        assert!(audio.representations[0].codecs.is_none());
        assert!(audio.representations[0].channels.is_none());

        // One video set, avc1 gets BT709 transfer fallback.
        assert_eq!(info.video_sets.len(), 1);
        let video = &info.video_sets[0];
        assert_eq!(video.color_info.transfer_characteristics, Some("1"));
        assert_eq!(video.fps, Some(30));
        assert_eq!(video.representations.len(), 2);

        // Segment info carries base URL with cpn appended.
        match &video.representations[0].segment_info {
            SegmentInfo::Base { base_url, .. } => assert!(base_url.contains("cpn=")),
            _ => panic!("expected base segment info"),
        }
    }

    #[tokio::test]
    async fn sabr_urls_use_itag_xtags_key() {
        let mut f = audio_format(140, 128_000, "mp4a.40.2");
        let xtags = encode_xtags(&[("acont", "original"), ("lang", "en")]);
        f.xtags = Some(xtags.clone());
        let sd = data(vec![f]);

        let info = get_streaming_info(&sd, false, None, StreamingInfoParams { options: StreamingInfoOptions { is_sabr: true, ..Default::default() }, ..Default::default() })
        .await
        .unwrap();

        match &info.audio_sets[0].representations[0].segment_info {
            SegmentInfo::Base { base_url, .. } => {
                assert_eq!(base_url, &format!("sabr://audio?key=140:{xtags}"));
            }
            _ => panic!("SABR must use base segment info with sabr:// URL"),
        }
        // xtags decode path works for language.
        assert_eq!(info.audio_sets[0].language.as_deref(), Some("en"));
    }

    #[tokio::test]
    async fn multi_track_groups_and_drc_labels() {
        let mut original = audio_format(140, 128_000, "mp4a.40.2");
        original.audio_track = Some(AudioTrack {
            audio_is_default: true,
            display_name: "English".to_string(),
            id: "en.4".to_string(),
        });
        original.xtags = Some(encode_xtags(&[("acont", "original"), ("lang", "en")]));

        let mut drc = audio_format(999, 128_000, "mp4a.40.2");
        drc.audio_track = Some(AudioTrack {
            audio_is_default: true,
            display_name: "English".to_string(),
            id: "en.4".to_string(),
        });
        drc.xtags = Some(encode_xtags(&[("drc", "1"), ("lang", "en")]));

        let sd = data(vec![original, drc]);

        let info = get_streaming_info(&sd, false, None, StreamingInfoParams::default())
            .await
            .unwrap();

        assert_eq!(info.audio_sets.len(), 2);

        let main_set = info
            .audio_sets
            .iter()
            .find(|s| s.track_name.as_deref() == Some("English"))
            .expect("original track set");
        assert_eq!(main_set.track_roles, vec!["main".to_string()]);

        let drc_set = info
            .audio_sets
            .iter()
            .find(|s| s.track_name.as_deref() == Some("English (Stable Volume)"))
            .expect("drc set");
        assert!(drc_set
            .track_roles
            .contains(&"enhanced-audio-intelligibility".to_string()));
        assert_eq!(drc_set.representations[0].uid, "999-en.4-drc");
    }

    #[test]
    fn storyboard_spec_parses_boards() {
        let node = PlayerStoryboardSpecNode {
            spec: Some(
                "https://i.ytimg.com/sb/vid/storyboard3_L$L/$N.jpg?sqp=abc|48#27#100#5#5#0#en#sig1|80#45#50#10#10#0#en#sig2"
                    .to_string(),
            ),
        };
        let boards = parse_storyboard_spec(&node);
        assert_eq!(boards.len(), 2);
        assert_eq!(boards[0].thumbnail_width, 48);
        assert_eq!(boards[0].columns, 5);
        assert_eq!(boards[0].storyboard_count, 4); // ceil(100 / 25)
        assert!(boards[0].template_url.contains("storyboard3_L0/en.jpg"));
        assert!(boards[0].template_url.contains("sigh=sig1"));
        assert_eq!(boards[1].thumbnail_height, 45);
    }

    #[tokio::test]
    async fn text_sets_append_fmt_and_roles() {
        let sd = data(vec![audio_format(140, 128_000, "mp4a.40.2")]);
        let tracks = vec![
            TranscriptTrack {
                language_code: "en".to_string(),
                name: "English".to_string(),
                kind: None,
                base_url: "https://www.youtube.com/api/timedtext?v=1".to_string(),
                is_translatable: true,
                vss_id: Some(".en".to_string()),
            },
            TranscriptTrack {
                language_code: "id".to_string(),
                name: "Indonesian (auto-translated)".to_string(),
                kind: None,
                base_url: "https://www.youtube.com/api/timedtext?v=1&tlang=id".to_string(),
                is_translatable: true,
                vss_id: Some("a.id".to_string()),
            },
        ];

        let info = get_streaming_info(
            &sd,
            false,
            None,
            StreamingInfoParams {
                caption_tracks: Some(&tracks),
                options: StreamingInfoOptions {
                    captions_format: Some("vtt".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert_eq!(info.text_sets.len(), 2);
        assert_eq!(info.text_sets[0].mime_type, "text/vtt");
        assert_eq!(info.text_sets[0].representation.uid, "text-.en");
        assert!(info.text_sets[0].representation.base_url.contains("fmt=vtt"));
        assert_eq!(info.text_sets[1].track_roles, vec!["caption", "dub"]);

        // Invalid captions format must error.
        let err = get_streaming_info(
            &sd,
            false,
            None,
            StreamingInfoParams {
                caption_tracks: Some(&tracks),
                options: StreamingInfoOptions {
                    captions_format: Some("srt".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await;
        assert!(err.is_err());
    }
}
