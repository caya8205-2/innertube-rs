use std::process::exit;
use serde::Serialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
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
        eprintln!("Usage: innertube <command> <video_id_or_url> [options]");
        eprintln!("Commands:");
        eprintln!("  info <id>                                      Output video metadata as JSON");
        eprintln!("  stream <id> [--format <ext>] [--quality <q>]   Output resolved stream URLs as JSON");
        eprintln!("  download <id> [--output-audio <p>] [--output-video <p>] [--format <ext>]");
        exit(1);
    }

    let command = &args[1];
    let video_id = clean_video_id(&args[2]);

    let mut target_format = "mp3".to_string();
    let mut _target_quality = "best".to_string();
    let mut output_audio: Option<String> = None;
    let mut output_video: Option<String> = None;

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
        } else if args[i] == "--output-audio" {
            if i + 1 < args.len() {
                output_audio = Some(args[i + 1].clone());
                i += 2;
                continue;
            }
        } else if args[i] == "--output-video" {
            if i + 1 < args.len() {
                output_video = Some(args[i + 1].clone());
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

            // 1. Select optimal audio format (prioritize adaptive AAC itag 140 for best quality)
            let audio_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                sd.adaptive_formats.iter().find(|f| f.itag == 140)
                    .or_else(|| sd.adaptive_formats.iter().find(|f| f.mime_type.contains("mp4a")))
                    .or_else(|| sd.adaptive_formats.iter().find(|f| f.is_audio_only()))
                    .or_else(|| sd.formats.iter().find(|f| f.itag == 18 || f.is_audio_video()))
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

            // 2. Select optimal video format
            let video_out = if is_video_target {
                let video_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                    let mut candidates: Vec<&StreamingFormat> = sd.adaptive_formats.iter().filter(|f| f.is_video_only()).collect();
                    if candidates.is_empty() {
                        candidates.extend(sd.formats.iter().filter(|f| f.is_audio_video() || f.is_video_only()));
                    }
                    if target_format == "mp4" {
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
        "download" => {
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

            // 1. Resolve Audio Format (prioritize adaptive AAC itag 140 for best quality)
            let audio_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                sd.adaptive_formats.iter().find(|f| f.itag == 140)
                    .or_else(|| sd.adaptive_formats.iter().find(|f| f.mime_type.contains("mp4a")))
                    .or_else(|| sd.adaptive_formats.iter().find(|f| f.is_audio_only()))
                    .or_else(|| sd.formats.iter().find(|f| f.itag == 18 || f.is_audio_video()))
            });

            let mut audio_mime = "audio/mp4".to_string();
            if let Some(out_path) = output_audio {
                if let Some(fmt) = audio_fmt {
                    audio_mime = fmt.mime_type.clone();
                    let url = match innertube_rs::endpoints::player::resolve_stream_url(fmt, &yt.player.decipherer) {
                        Ok(u) => u,
                        Err(e) => {
                            eprintln!(r#"{{"error": "Failed to decipher audio URL: {}"}}"#, e);
                            exit(1);
                        }
                    };
                    let clen = fmt.content_length.as_ref().and_then(|s| s.parse::<u64>().ok());
                    if let Err(e) = download_stream_to_file(&yt.session.http_client, &url, clen, &out_path, "audio").await {
                        eprintln!(r#"{{"error": "Failed to download audio stream: {}"}}"#, e);
                        exit(1);
                    }
                }
            }

            // 2. Resolve Video Format (if requested)
            let mut video_mime = "video/mp4".to_string();
            if let Some(out_path) = output_video {
                if is_video_target {
                    let video_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                        let mut candidates: Vec<&StreamingFormat> = sd.adaptive_formats.iter().filter(|f| f.is_video_only()).collect();
                        if candidates.is_empty() {
                            candidates.extend(sd.formats.iter().filter(|f| f.is_audio_video() || f.is_video_only()));
                        }
                        if target_format == "mp4" {
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
                        video_mime = fmt.mime_type.clone();
                        let url = match innertube_rs::endpoints::player::resolve_stream_url(fmt, &yt.player.decipherer) {
                            Ok(u) => u,
                            Err(e) => {
                                eprintln!(r#"{{"error": "Failed to decipher video URL: {}"}}"#, e);
                                exit(1);
                            }
                        };
                        let clen = fmt.content_length.as_ref().and_then(|s| s.parse::<u64>().ok());
                        if let Err(e) = download_stream_to_file(&yt.session.http_client, &url, clen, &out_path, "video").await {
                            eprintln!(r#"{{"error": "Failed to download video stream: {}"}}"#, e);
                            exit(1);
                        }
                    }
                }
            }

            println!(
                r#"{{"type":"done","title":{},"author":{},"duration":{},"audioMime":{},"videoMime":{}}}"#,
                serde_json::to_string(&title).unwrap(),
                serde_json::to_string(&author).unwrap(),
                duration,
                serde_json::to_string(&audio_mime).unwrap(),
                serde_json::to_string(&video_mime).unwrap()
            );
        }
        _ => {
            eprintln!(r#"{{"error": "Unknown command: {}"}}"#, command);
            exit(1);
        }
    }
}

async fn download_stream_to_file(
    _client: &reqwest::Client,
    url: &str,
    content_length: Option<u64>,
    out_path: &str,
    stream_type: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(out_path).await?;
    let total = content_length.unwrap_or(0);
    let mut downloaded: u64 = 0;

    // Use Chrome UA for CDN download (matches yt-dlp behavior — CDN validates URL signature
    // but doesn't require matching client UA for the download request itself)
    let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36";

    // Create a dedicated download client (separate cookie jar from session)
    let dl_client = reqwest::Client::builder()
        .user_agent(chrome_ua)
        .build()?;

    // Use YouTube's chunked download pattern: append &range= and &rn= query params
    // This is how yt-dlp downloads with http_chunk_size=10MB
    let chunk_size: u64 = 10 * 1024 * 1024; // 10MB chunks (matching yt-dlp)
    let mut rn: u64 = 0;

    while downloaded < total || (total == 0 && rn == 0) {
        let end = if total > 0 {
            std::cmp::min(downloaded + chunk_size - 1, total - 1)
        } else {
            downloaded + chunk_size - 1
        };

        let chunk_url = format!("{}&range={}-{}&rn={}", url, downloaded, end, rn);

        let mut resp = dl_client
            .get(&chunk_url)
            .send()
            .await?;

        if !resp.status().is_success() && resp.status() != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(format!("Stream download failed: {} (rn={}, range={}-{})", resp.status(), rn, downloaded, end).into());
        }

        let mut chunk_bytes: u64 = 0;
        while let Some(chunk) = resp.chunk().await? {
            file.write_all(&chunk).await?;
            chunk_bytes += chunk.len() as u64;
            downloaded += chunk.len() as u64;
            // Emit progress roughly every 512KB to avoid flooding
            if chunk_bytes % (512 * 1024) < chunk.len() as u64 || downloaded >= total {
                let report_total = if total > 0 { total } else { downloaded };
                println!(r#"{{"type":"progress","stream":"{}","downloaded":{},"total":{}}}"#, stream_type, downloaded, report_total);
            }
        }

        rn += 1;

        // If total is unknown and we got fewer bytes than chunk_size, we're done
        if total == 0 && chunk_bytes < chunk_size {
            break;
        }
    }

    file.flush().await?;
    Ok(())
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
