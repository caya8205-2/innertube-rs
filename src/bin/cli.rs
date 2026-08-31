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
    if args.len() < 3 && (args.len() < 2 || (args.len() >= 2 && args[1] != "help" && args[1] != "--help")) {
        eprintln!("Usage: innertube <command> <query_or_id_or_url> [options]");
        eprintln!("Commands:");
        eprintln!("  info <id>                                      Output video metadata as JSON");
        eprintln!("  stream <id> [--format <ext>] [--quality <q>]   Output resolved stream URLs as JSON");
        eprintln!("  search <query> [--limit <n>]                   Output search results as JSON");
        eprintln!("  playlist <id_or_url> [--limit <n>]             Output playlist details and tracks as JSON");
        eprintln!("  download <id> [--output-audio <p>] [--output-video <p>] [--format <ext>]");
        exit(1);
    }

    let command = &args[1];
    let video_id = clean_video_id(&args[2]);

    let mut target_format = "mp3".to_string();
    let mut target_quality = "best".to_string();
    let mut target_limit: usize = 10;
    let mut output_audio: Option<String> = None;
    let mut output_video: Option<String> = None;
    let mut po_token: Option<String> = std::env::var("INNERTUBE_PO_TOKEN").or_else(|_| std::env::var("POT")).ok();
    let mut cookie_input: Option<String> = std::env::var("INNERTUBE_COOKIES").ok();

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
                target_quality = args[i + 1].to_lowercase();
                i += 2;
                continue;
            }
        } else if args[i] == "--limit" || args[i] == "-l" || args[i] == "-n" {
            if i + 1 < args.len() {
                if let Ok(lim) = args[i + 1].parse::<usize>() {
                    target_limit = lim;
                }
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
        } else if (args[i] == "--po-token" || args[i] == "--pot") && i + 1 < args.len() {
            po_token = Some(args[i + 1].clone());
            i += 2;
            continue;
        } else if (args[i] == "--cookies" || args[i] == "--cookie") && i + 1 < args.len() {
            cookie_input = Some(args[i + 1].clone());
            i += 2;
            continue;
        }
        i += 1;
    }

    let parsed_cookie = cookie_input.map(|c| parse_cookies_input(&c));

    let session_opts = innertube_rs::core::session::SessionOptions {
        po_token: po_token.clone(),
        cookie: parsed_cookie.clone(),
        ..Default::default()
    };

    let yt = match Innertube::with_options(session_opts).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!(r#"{{"error": "Failed to initialize Innertube: {}"}}"#, e);
            exit(1);
        }
    };

    match command.as_str() {
        "search" => {
            let search_query = &args[2];
            match yt.search(search_query, None).await {
                Ok(results) => {
                    let mut tracks: Vec<serde_json::Value> = Vec::new();
                    for item in results.items.into_iter() {
                        if tracks.len() >= target_limit {
                            break;
                        }
                        if let innertube_rs::models::search::SearchResultItem::Video(v) = item {
                            let duration_sec = parse_duration_to_seconds(v.duration.as_deref().unwrap_or(""));
                            let thumb = v.thumbnails.iter().max_by_key(|t| t.width).map(|t| t.url.clone()).unwrap_or_default();
                            tracks.push(serde_json::json!({
                                "id": v.video_id,
                                "title": v.title,
                                "artist": v.author,
                                "duration": duration_sec,
                                "thumbnail": thumb,
                            }));
                        }
                    }
                    println!("{}", serde_json::to_string(&tracks).unwrap_or_default());
                }
                Err(e) => {
                    eprintln!(r#"{{"error": "{}"}}"#, e);
                    exit(1);
                }
            }
        }
        "playlist" => {
            let playlist_id = clean_playlist_id(&args[2]);
            match yt.get_playlist(&playlist_id).await {
                Ok(mut playlist) => {
                    let mut tracks: Vec<serde_json::Value> = Vec::new();
                    for v in &playlist.videos {
                        if tracks.len() >= target_limit {
                            break;
                        }
                        let duration_sec = v.duration_ms.map(|ms| ms / 1000).unwrap_or_else(|| {
                            parse_duration_to_seconds(v.duration.as_deref().unwrap_or(""))
                        });
                        tracks.push(serde_json::json!({
                            "id": v.id,
                            "title": v.title,
                            "artist": v.author,
                            "duration": duration_sec,
                            "thumbnail": v.thumbnail.clone().unwrap_or_default(),
                        }));
                    }

                    // Paginate if requested limit is higher than current page
                    while tracks.len() < target_limit && playlist.has_continuation() {
                        match playlist.get_continuation(&yt.session).await {
                            Ok(continuation) => {
                                for v in &continuation.videos {
                                    if tracks.len() >= target_limit {
                                        break;
                                    }
                                    let duration_sec = v.duration_ms.map(|ms| ms / 1000).unwrap_or_else(|| {
                                        parse_duration_to_seconds(v.duration.as_deref().unwrap_or(""))
                                    });
                                    tracks.push(serde_json::json!({
                                        "id": v.id,
                                        "title": v.title,
                                        "artist": v.author,
                                        "duration": duration_sec,
                                        "thumbnail": v.thumbnail.clone().unwrap_or_default(),
                                    }));
                                }
                                playlist.continuation_token = continuation.continuation_token;
                            }
                            Err(_) => break,
                        }
                    }

                    let output = serde_json::json!({
                        "name": playlist.title,
                        "author": playlist.author.unwrap_or_default(),
                        "tracks": tracks,
                    });
                    println!("{}", serde_json::to_string(&output).unwrap_or_default());
                }
                Err(e) => {
                    eprintln!(r#"{{"error": "{}"}}"#, e);
                    exit(1);
                }
            }
        }
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
            let target_height = parse_target_height(&target_quality);

            // 1. Select optimal audio format (prioritize adaptive AAC itag 140 / progressive itag 18)
            let audio_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                sd.adaptive_formats.iter().find(|f| f.itag == 140)
                    .or_else(|| sd.adaptive_formats.iter().find(|f| f.mime_type.contains("mp4a")))
                    .or_else(|| sd.formats.iter().find(|f| f.itag == 18 || f.is_audio_video()))
                    .or_else(|| sd.adaptive_formats.iter().find(|f| f.is_audio_only()))
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

            // 2. Select optimal video format matching requested target_quality / target_height
            let video_out = if is_video_target {
                let video_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                    let mut candidates: Vec<&StreamingFormat> = sd.adaptive_formats.iter().filter(|f| f.is_video_only()).collect();
                    candidates.extend(sd.formats.iter().filter(|f| f.is_audio_video() || f.is_video_only()));

                    if let Some(th) = target_height {
                        let matching: Vec<&StreamingFormat> = candidates.iter().copied().filter(|f| f.height.unwrap_or(0) == th).collect();
                        if !matching.is_empty() {
                            candidates = matching;
                        } else {
                            let below: Vec<&StreamingFormat> = candidates.iter().copied().filter(|f| f.height.unwrap_or(0) <= th).collect();
                            if !below.is_empty() {
                                candidates = below;
                            }
                        }
                    }

                    if target_format == "mp4" {
                        candidates.sort_by_key(|f| {
                            let is_avc = f.mime_type.contains("avc1") || f.mime_type.contains("h264");
                            let height = f.height.unwrap_or(0);
                            (std::cmp::Reverse(height), if is_avc { 0 } else { 1 }, std::cmp::Reverse(f.bitrate))
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
            let target_height = parse_target_height(&target_quality);

            // Handle Video Download
            if let Some(out_video_path) = output_video {
                if is_video_target {
                    let video_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                        let mut candidates: Vec<&StreamingFormat> = sd.adaptive_formats.iter().filter(|f| f.is_video_only()).collect();
                        candidates.extend(sd.formats.iter().filter(|f| f.is_audio_video() || f.is_video_only()));

                        if let Some(th) = target_height {
                            let matching: Vec<&StreamingFormat> = candidates.iter().copied().filter(|f| f.height.unwrap_or(0) == th).collect();
                            if !matching.is_empty() {
                                candidates = matching;
                            } else {
                                let below: Vec<&StreamingFormat> = candidates.iter().copied().filter(|f| f.height.unwrap_or(0) <= th).collect();
                                if !below.is_empty() {
                                    candidates = below;
                                }
                            }
                        }

                        if target_format == "mp4" {
                            candidates.sort_by_key(|f| {
                                let is_avc = f.mime_type.contains("avc1") || f.mime_type.contains("h264");
                                let height = f.height.unwrap_or(0);
                                (std::cmp::Reverse(height), if is_avc { 0 } else { 1 }, std::cmp::Reverse(f.bitrate))
                            });
                        } else {
                            candidates.sort_by_key(|f| {
                                let height = f.height.unwrap_or(0);
                                (std::cmp::Reverse(height), std::cmp::Reverse(f.bitrate))
                            });
                        }
                        candidates.first().copied()
                    });

                    if let Some(v_fmt) = video_fmt {
                        let v_url = match innertube_rs::endpoints::player::resolve_stream_url(v_fmt, &yt.player.decipherer) {
                            Ok(u) => u,
                            Err(e) => {
                                eprintln!(r#"{{"error": "Failed to resolve video URL: {}"}}"#, e);
                                exit(1);
                            }
                        };
                        let clen = v_fmt.content_length.as_ref().and_then(|s| s.parse::<u64>().ok());
                        let pot_ref = po_token.as_deref();
                        let cookie_ref = parsed_cookie.as_deref();

                        if let Err(e) = download_stream_to_file(&yt.session.http_client, &v_url, clen, &out_video_path, "video", pot_ref, cookie_ref).await {
                            // Only fallback to progressive (360p) if user didn't explicitly request > 360p
                            let requested_high_res = target_height.is_some_and(|h| h > 360);
                            let prog_fallback = if !requested_high_res {
                                info.streaming_data.as_ref().and_then(|sd| {
                                    sd.formats.iter().find(|f| f.itag == 18 || f.is_audio_video())
                                })
                            } else {
                                None
                            };

                            if let Some(prog) = prog_fallback {
                                eprintln!(r#"{{"info": "Adaptive video stream restricted, falling back to progressive format"}}"#);
                                let p_url = match innertube_rs::endpoints::player::resolve_stream_url(prog, &yt.player.decipherer) {
                                    Ok(u) => u,
                                    Err(err) => {
                                        eprintln!(r#"{{"error": "Failed to resolve progressive fallback URL: {}"}}"#, err);
                                        exit(1);
                                    }
                                };
                                let p_clen = prog.content_length.as_ref().and_then(|s| s.parse::<u64>().ok());
                                if let Err(err) = download_stream_to_file(&yt.session.http_client, &p_url, p_clen, &out_video_path, "video", pot_ref, cookie_ref).await {
                                    eprintln!(r#"{{"error": "Failed to download progressive fallback video: {}"}}"#, err);
                                    exit(1);
                                }

                                if let Some(ref out_audio_path) = output_audio {
                                    if let Err(err) = tokio::fs::copy(&out_video_path, out_audio_path).await {
                                        eprintln!(r#"{{"error": "Failed to copy audio stream: {}"}}"#, err);
                                        exit(1);
                                    }
                                    let rep_total = p_clen.unwrap_or(0);
                                    println!(r#"{{"type":"progress","stream":"audio","downloaded":{},"total":{}}}"#, rep_total, rep_total);
                                }

                                println!(
                                    r#"{{"type":"done","title":{},"author":{},"duration":{},"audioMime":{},"videoMime":{}}}"#,
                                    serde_json::to_string(&title).unwrap(),
                                    serde_json::to_string(&author).unwrap(),
                                    duration,
                                    serde_json::to_string(&prog.mime_type).unwrap(),
                                    serde_json::to_string(&prog.mime_type).unwrap()
                                );
                                return;
                            } else {
                                eprintln!(r#"{{"error": "Failed to download video stream: {}"}}"#, e);
                                exit(1);
                            }
                        }

                        let mut audio_mime = "audio/mp4".to_string();
                        if let Some(out_audio_path) = output_audio {
                            if v_fmt.is_audio_video() {
                                // Progressive format: copy video file to audio path
                                if let Err(e) = tokio::fs::copy(&out_video_path, &out_audio_path).await {
                                    eprintln!(r#"{{"error": "Failed to prepare audio stream: {}"}}"#, e);
                                    exit(1);
                                }
                                let rep_total = clen.unwrap_or(0);
                                println!(r#"{{"type":"progress","stream":"audio","downloaded":{},"total":{}}}"#, rep_total, rep_total);
                                audio_mime = v_fmt.mime_type.clone();
                            } else {
                                // Adaptive video: download matching audio stream (prioritizing progressive itag 18 or adaptive itag 140)
                                let audio_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                                    sd.formats.iter().find(|f| f.itag == 18 || f.is_audio_video())
                                        .or_else(|| sd.adaptive_formats.iter().find(|f| f.itag == 140))
                                        .or_else(|| sd.adaptive_formats.iter().find(|f| f.mime_type.contains("mp4a")))
                                        .or_else(|| sd.adaptive_formats.iter().find(|f| f.is_audio_only()))
                                });

                                if let Some(a_fmt) = audio_fmt {
                                    audio_mime = a_fmt.mime_type.clone();
                                    let a_url = match innertube_rs::endpoints::player::resolve_stream_url(a_fmt, &yt.player.decipherer) {
                                        Ok(u) => u,
                                        Err(e) => {
                                            eprintln!(r#"{{"error": "Failed to resolve audio URL: {}"}}"#, e);
                                            exit(1);
                                        }
                                    };
                                    let a_clen = a_fmt.content_length.as_ref().and_then(|s| s.parse::<u64>().ok());
                                    if let Err(e) = download_stream_to_file(&yt.session.http_client, &a_url, a_clen, &out_audio_path, "audio", pot_ref, cookie_ref).await {
                                        eprintln!(r#"{{"error": "Failed to download audio stream: {}"}}"#, e);
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
                            serde_json::to_string(&v_fmt.mime_type).unwrap()
                        );
                        return;
                    }
                }
            }

            // Handle Audio-Only Download
            if let Some(out_audio_path) = output_audio {
                let audio_fmt: Option<&StreamingFormat> = info.streaming_data.as_ref().and_then(|sd| {
                    sd.formats.iter().find(|f| f.itag == 18 || f.is_audio_video())
                        .or_else(|| sd.adaptive_formats.iter().find(|f| f.itag == 140))
                        .or_else(|| sd.adaptive_formats.iter().find(|f| f.mime_type.contains("mp4a")))
                        .or_else(|| sd.adaptive_formats.iter().find(|f| f.is_audio_only()))
                });

                let mut audio_mime = "audio/mp4".to_string();
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
                    let pot_ref = po_token.as_deref();
                    let cookie_ref = parsed_cookie.as_deref();
                    if let Err(e) = download_stream_to_file(&yt.session.http_client, &url, clen, &out_audio_path, "audio", pot_ref, cookie_ref).await {
                        eprintln!(r#"{{"error": "Failed to download audio stream: {}"}}"#, e);
                        exit(1);
                    }
                }

                println!(
                    r#"{{"type":"done","title":{},"author":{},"duration":{},"audioMime":{},"videoMime":""}}"#,
                    serde_json::to_string(&title).unwrap(),
                    serde_json::to_string(&author).unwrap(),
                    duration,
                    serde_json::to_string(&audio_mime).unwrap()
                );
            }
        }
        _ => {
            eprintln!(r#"{{"error": "Unknown command: {}"}}"#, command);
            exit(1);
        }
    }
}

fn parse_target_height(quality: &str) -> Option<u32> {
    let q = quality.trim().to_lowercase();
    if q.is_empty() || q == "best" || q == "highest" || q == "max" {
        return None;
    }
    if q == "worst" || q == "lowest" || q == "min" {
        return Some(144);
    }
    let digits: String = q.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse::<u32>().ok()
}

fn user_agent_for_stream_url(url: &str) -> &'static str {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some((_, client_val)) = parsed.query_pairs().find(|(k, _)| k == "c") {
            match client_val.as_ref() {
                "ANDROID" => return innertube_rs::constants::clients::ANDROID_USER_AGENT,
                "ANDROID_VR" => return innertube_rs::constants::clients::ANDROID_VR_USER_AGENT,
                "IOS" => return innertube_rs::constants::clients::IOS_USER_AGENT,
                "MWEB" => return innertube_rs::constants::clients::MWEB_USER_AGENT,
                _ => {}
            }
        }
    }
    if url.contains("c=ANDROID_VR") {
        innertube_rs::constants::clients::ANDROID_VR_USER_AGENT
    } else if url.contains("c=ANDROID") {
        innertube_rs::constants::clients::ANDROID_USER_AGENT
    } else if url.contains("c=IOS") {
        innertube_rs::constants::clients::IOS_USER_AGENT
    } else {
        innertube_rs::constants::DEFAULT_USER_AGENT
    }
}

fn parse_cookies_input(input: &str) -> String {
    let path = std::path::Path::new(input);
    if path.exists() && path.is_file() {
        if let Ok(content) = std::fs::read_to_string(path) {
            let mut cookies = Vec::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || (trimmed.starts_with('#') && !trimmed.starts_with("#HttpOnly_")) {
                    continue;
                }
                let clean_line = trimmed.trim_start_matches("#HttpOnly_");
                let parts: Vec<&str> = clean_line.split('\t').collect();
                if parts.len() >= 7 {
                    let name = parts[5];
                    let value = parts[6];
                    cookies.push(format!("{}={}", name, value));
                }
            }
            if !cookies.is_empty() {
                return cookies.join("; ");
            }
        }
    }
    input.to_string()
}

async fn download_stream_to_file(
    _client: &reqwest::Client,
    url: &str,
    content_length: Option<u64>,
    out_path: &str,
    stream_type: &str,
    po_token: Option<&str>,
    cookie: Option<&str>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(out_path).await?;
    let mut total = content_length.unwrap_or(0);
    let mut downloaded: u64 = 0;

    let ua = user_agent_for_stream_url(url);
    let dl_client = reqwest::Client::builder()
        .user_agent(ua)
        .build()?;

    let stream_url = if let Some(pot) = po_token {
        if !url.contains("pot=") {
            if url.contains('?') {
                format!("{}&pot={}", url, pot)
            } else {
                format!("{}?pot={}", url, pot)
            }
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    };

    let chunk_size: u64 = 1024 * 1024; // 1MB chunks
    let mut chunk_index: u32 = 0;

    while downloaded < total || (total == 0 && downloaded == 0) {
        let end = if total > 0 {
            std::cmp::min(downloaded + chunk_size - 1, total - 1)
        } else {
            downloaded + chunk_size - 1
        };

        let chunk_url = if stream_url.contains('?') {
            format!("{}&range={}-{}&rn={}", stream_url, downloaded, end, chunk_index)
        } else {
            format!("{}?range={}-{}&rn={}", stream_url, downloaded, end, chunk_index)
        };

        let mut req = dl_client.get(&chunk_url)
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com")
            .header("Accept", "*/*");

        if let Some(c) = cookie {
            req = req.header("Cookie", c);
        }

        let mut resp = req.send().await?;

        if total == 0 {
            if let Some(cr) = resp.headers().get("content-range").and_then(|h| h.to_str().ok()) {
                if let Some(slash_idx) = cr.rfind('/') {
                    if let Ok(parsed_total) = cr[slash_idx + 1..].trim().parse::<u64>() {
                        total = parsed_total;
                    }
                }
            }
        }

        if !resp.status().is_success() && resp.status() != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(format!("Stream download failed: {} (range={}-{}, ua={})", resp.status(), downloaded, end, ua).into());
        }

        let mut chunk_bytes: u64 = 0;
        while let Some(chunk) = resp.chunk().await? {
            file.write_all(&chunk).await?;
            chunk_bytes += chunk.len() as u64;
            downloaded += chunk.len() as u64;
            if chunk_bytes % (512 * 1024) < chunk.len() as u64 || (total > 0 && downloaded >= total) {
                let report_total = if total > 0 { total } else { downloaded };
                println!(r#"{{"type":"progress","stream":"{}","downloaded":{},"total":{}}}"#, stream_type, downloaded, report_total);
            }
        }

        chunk_index += 1;

        if total > 0 && downloaded >= total {
            break;
        }
        if total == 0 && chunk_bytes < chunk_size {
            break;
        }
    }

    file.flush().await?;
    Ok(())
}

fn parse_duration_to_seconds(d: &str) -> u64 {
    let parts: Vec<&str> = d.trim().split(':').collect();
    if parts.is_empty() {
        return 0;
    }
    if parts.len() == 1 {
        return parts[0].parse::<u64>().unwrap_or(0);
    }
    if parts.len() == 2 {
        let m = parts[0].parse::<u64>().unwrap_or(0);
        let s = parts[1].parse::<u64>().unwrap_or(0);
        return m * 60 + s;
    }
    if parts.len() == 3 {
        let h = parts[0].parse::<u64>().unwrap_or(0);
        let m = parts[1].parse::<u64>().unwrap_or(0);
        let s = parts[2].parse::<u64>().unwrap_or(0);
        return h * 3600 + m * 60 + s;
    }
    0
}

fn clean_playlist_id(input: &str) -> String {
    let s = input.trim();
    if let Some(pos) = s.find("list=") {
        let rest = &s[pos + 5..];
        let end = rest.find('&').unwrap_or(rest.len());
        return rest[..end].to_string();
    }
    s.to_string()
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
