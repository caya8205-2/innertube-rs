use std::process::exit;
use serde::Serialize;
use innertube_rs::models::format::{FormatFilter, FormatType, QualityPreference, StreamingFormat};
use innertube_rs::Innertube;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StreamInfoOutput {
    video_id: String,
    title: String,
    author: String,
    duration_seconds: u64,
    audio: Option<StreamFormatOutput>,
    video: Option<StreamFormatOutput>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StreamFormatOutput {
    url: String,
    mime_type: String,
    bitrate: u64,
    content_length: Option<u64>,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: innertube <command> <video_id_or_url> [--format <ext>] [--quality <q>]");
        eprintln!("Commands:");
        eprintln!("  info <id>               Output video metadata as JSON");
        eprintln!("  stream <id> [options]   Output resolved audio/video stream URLs as JSON");
        exit(1);
    }

    let command = &args[1];
    let video_id = clean_video_id(&args[2]);

    let mut target_format = "mp3".to_string();
    let mut _target_quality = "best".to_string();

    let mut i = 3;
    while i < args.len() {
        if args[i] == "--format" || args[i] == "-f" {
            if i + 1 < args.len() {
                target_format = args[i + 1].to_lowercase();
                i += 2;
                continue;
            }
        } else if args[i] == "--quality" || args[i] == "-q" {
            if i + 1 < args.len() {
                _target_quality = args[i + 1].to_lowercase();
                i += 2;
                continue;
            }
        }
        i += 1;
    }

    let yt = match Innertube::new().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!(r#"{{"error": "Failed to initialize Innertube: {}"}}"#, e);
            exit(1);
        }
    };

    match command.as_str() {
        "info" => {
            match yt.get_video_info(&video_id).await {
                Ok(info) => {
                    println!("{}", serde_json::to_string_pretty(&info).unwrap_or_default());
                }
                Err(e) => {
                    eprintln!(r#"{{"error": "{}"}}"#, e);
                    exit(1);
                }
            }
        }
        "stream" => {
            let info = match yt.get_video_info(&video_id).await {
                Ok(i) => i,
                Err(e) => {
                    eprintln!(r#"{{"error": "{}"}}"#, e);
                    exit(1);
                }
            };

            let title = info.video_details.as_ref().map(|d| d.title.clone()).unwrap_or_else(|| video_id.clone());
            let author = info.video_details.as_ref().map(|d| d.author.clone()).unwrap_or_default();
            let duration = info.video_details.as_ref()
                .and_then(|d| d.length_seconds.parse::<u64>().ok())
                .unwrap_or(0);

            let is_video_target = matches!(target_format.as_str(), "mp4" | "webm" | "mkv");

            // 1. Select optimal audio format
            let audio_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                let mut candidates: Vec<&StreamingFormat> = sd.adaptive_formats.iter().filter(|f| f.is_audio_only()).collect();
                if target_format == "mp4" || target_format == "m4a" || target_format == "aac" {
                    // Prefer AAC for MP4 container
                    candidates.sort_by_key(|f| {
                        let is_aac = f.mime_type.contains("mp4a");
                        (if is_aac { 0 } else { 1 }, std::cmp::Reverse(f.bitrate))
                    });
                } else {
                    // Prefer Opus for webm / ogg / general
                    candidates.sort_by_key(|f| {
                        let is_opus = f.mime_type.contains("opus");
                        (if is_opus { 0 } else { 1 }, std::cmp::Reverse(f.bitrate))
                    });
                }
                candidates.first().copied()
            });

            let audio_out = if let Some(fmt) = audio_fmt {
                match innertube_rs::endpoints::player::resolve_stream_url(fmt, &yt.player.decipherer) {
                    Ok(url) => Some(StreamFormatOutput {
                        url,
                        mime_type: fmt.mime_type.clone(),
                        bitrate: fmt.bitrate,
                        content_length: fmt.content_length.as_ref().and_then(|s| s.parse::<u64>().ok()),
                    }),
                    Err(_) => None,
                }
            } else {
                // Fallback to format filter
                let filter = FormatFilter {
                    format_type: FormatType::AudioOnly,
                    quality: QualityPreference::Highest,
                    container: None,
                };
                if let Ok(url) = yt.get_stream_url(&video_id, &filter).await {
                    let fmt = innertube_rs::endpoints::player::select_format(&info, &filter).ok();
                    Some(StreamFormatOutput {
                        url,
                        mime_type: fmt.map(|f| f.mime_type.clone()).unwrap_or_else(|| "audio/mp4".to_string()),
                        bitrate: fmt.map(|f| f.bitrate).unwrap_or(128000),
                        content_length: fmt.and_then(|f| f.content_length.as_ref()).and_then(|s| s.parse::<u64>().ok()),
                    })
                } else {
                    None
                }
            };

            // 2. Select optimal video format (if requested or available)
            let video_out = if is_video_target {
                let video_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                    let mut candidates: Vec<&StreamingFormat> = sd.adaptive_formats.iter().filter(|f| f.is_video_only()).collect();
                    if target_format == "mp4" {
                        // Prioritize AVC/H.264 for MP4 so ffmpeg can copy stream without transcoding
                        candidates.sort_by_key(|f| {
                            let is_avc = f.mime_type.contains("avc1") || f.mime_type.contains("h264");
                            let height = f.height.unwrap_or(0);
                            (if is_avc { 0 } else { 1 }, std::cmp::Reverse(height), std::cmp::Reverse(f.bitrate))
                        });
                    } else {
                        candidates.sort_by_key(|f| {
                            let height = f.height.unwrap_or(0);
                            (std::cmp::Reverse(height), std::cmp::Reverse(f.bitrate))
                        });
                    }
                    candidates.first().copied()
                });

                if let Some(fmt) = video_fmt {
                    match innertube_rs::endpoints::player::resolve_stream_url(fmt, &yt.player.decipherer) {
                        Ok(url) => Some(StreamFormatOutput {
                            url,
                            mime_type: fmt.mime_type.clone(),
                            bitrate: fmt.bitrate,
                            content_length: fmt.content_length.as_ref().and_then(|s| s.parse::<u64>().ok()),
                        }),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let output = StreamInfoOutput {
                video_id,
                title,
                author,
                duration_seconds: duration,
                audio: audio_out,
                video: video_out,
            };

            println!("{}", serde_json::to_string(&output).unwrap_or_default());
        }
        _ => {
            eprintln!(r#"{{"error": "Unknown command: {}"}}"#, command);
            exit(1);
        }
    }
}

fn clean_video_id(input: &str) -> String {
    let s = input.trim();
    if s.len() == 11 && !s.contains('/') {
        return s.to_string();
    }

    if let Some(pos) = s.find("v=") {
        let rest = &s[pos + 2..];
        let end = rest.find('&').unwrap_or(rest.len());
        return rest[..end].to_string();
    }

    if let Some(pos) = s.find("youtu.be/") {
        let rest = &s[pos + 9..];
        let end = rest.find('?').unwrap_or(rest.len());
        return rest[..end].to_string();
    }

    if let Some(pos) = s.find("shorts/") {
        let rest = &s[pos + 7..];
        let end = rest.find('?').unwrap_or(rest.len());
        return rest[..end].to_string();
    }

    s.to_string()
}
