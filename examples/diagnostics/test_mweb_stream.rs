use std::env;
use innertube_rs::constants::clients;
use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let video_id = if args.len() > 1 {
        &args[1]
    } else {
        "e1bCibq2I1g"
    };

    println!("=== Testing MWEB High-Definition Stream on {} ===", video_id);
    let yt = innertube_rs::Innertube::new().await?;

    let mweb_session = Session::create(SessionOptions {
        client_name: Some(clients::MWEB_NAME.to_string()),
        client_version: Some(clients::MWEB_VERSION.to_string()),
        device_category: Some("MOBILE".to_string()),
        user_agent: Some(clients::MWEB_USER_AGENT.to_string()),
        generate_session_locally: Some(true),
        ..Default::default()
    }).await?;

    let payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true,
        "playbackContext": {
            "contentPlaybackContext": {
                "html5Preference": "HTML5_PREF_WANTS",
                "signatureTimestamp": yt.player.decipherer.signature_timestamp
            }
        }
    });

    let resp = mweb_session.post_innertube("/player", payload).await?;
    let pr: PlayerResponse = resp.json().await?;
    let sd = pr.streaming_data.unwrap();

    let f_hd = sd.adaptive_formats.iter().find(|f| {
        (f.height == Some(1080) || f.height == Some(720)) && f.mime_type.contains("mp4") && f.url.is_some()
    });

    if let Some(format) = f_hd {
        let resolved_url = innertube_rs::endpoints::player::resolve_stream_url(format, &yt.player.decipherer)?;
        let total: u64 = format.content_length.as_ref().and_then(|s| s.parse().ok()).unwrap_or(0);
        println!("Selected format itag {}: height={:?}, total size: {} MB", format.itag, format.height, total / 1024 / 1024);

        let dl = reqwest::Client::builder().user_agent(clients::MWEB_USER_AGENT).build()?;
        let chunk_size = 1024 * 1024;
        let mut downloaded = 0;

        for i in 0..15 {
            let end = std::cmp::min(downloaded + chunk_size - 1, total.saturating_sub(1));
            let chunk_url = format!("{}&range={}-{}&rn={}", resolved_url, downloaded, end, i);
            let resp = dl.get(&chunk_url)
                .header("Origin", "https://m.youtube.com")
                .header("Referer", "https://m.youtube.com")
                .header("Accept", "*/*")
                .send().await?;

            println!("Chunk {:2} ({:2}-{:2} MB): HTTP {}", i, downloaded / 1024 / 1024, end / 1024 / 1024, resp.status());
            if !resp.status().is_success() {
                println!("  --> Failed at chunk {}: HTTP {}", i, resp.status());
                break;
            }
            let b = resp.bytes().await?;
            downloaded += b.len() as u64;
        }

        println!("\nTotal Downloaded: {} MB / {} MB", downloaded / 1024 / 1024, total / 1024 / 1024);
    } else {
        println!("No HD MP4 adaptive format with direct URL found.");
    }

    Ok(())
}
