use innertube_rs::constants::clients;
use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";
    println!("=== Testing CDN Response Modes for Video {} ===", video_id);

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

    let f136 = sd.adaptive_formats.iter().find(|f| f.height == Some(720) && (f.mime_type.contains("avc1") || f.mime_type.contains("mp4"))).unwrap();
    let resolved_url = innertube_rs::endpoints::player::resolve_stream_url(f136, &yt.player.decipherer)?;

    println!("Full resolved URL:\n{}\n", resolved_url);

    let dl = reqwest::Client::builder().user_agent(clients::MWEB_USER_AGENT).build()?;

    // Test A: Range Header (bytes=6291456-7340031)
    let resp_a = dl.get(&resolved_url)
        .header("Range", "bytes=6291456-7340031")
        .header("Origin", "https://m.youtube.com")
        .header("Referer", "https://m.youtube.com")
        .send().await?;
    println!("Test A (Range Header 6MB-7MB): Status {}", resp_a.status());

    // Test B: Query Param range=6291456-7340031
    let url_b = format!("{}&range=6291456-7340031", resolved_url);
    let resp_b = dl.get(&url_b)
        .header("Origin", "https://m.youtube.com")
        .header("Referer", "https://m.youtube.com")
        .send().await?;
    println!("Test B (Query Param range 6MB-7MB): Status {}", resp_b.status());

    // Test C: Whole file request without range
    let resp_c = dl.get(&resolved_url)
        .header("Origin", "https://m.youtube.com")
        .header("Referer", "https://m.youtube.com")
        .send().await?;
    println!("Test C (Direct GET without range): Status {}, Content-Length: {:?}", resp_c.status(), resp_c.content_length());

    // Test D: Android client URL with range
    let and_info = yt.get_video_info(video_id).await?;
    let and_sd = and_info.streaming_data.unwrap();
    let and_f = and_sd.adaptive_formats.iter().find(|f| f.height == Some(720) && (f.mime_type.contains("avc1") || f.mime_type.contains("mp4"))).unwrap();
    if let Some(ref and_url) = and_f.url {
        let and_resp = dl.get(format!("{}&range=6291456-7340031", and_url))
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com")
            .send().await?;
        println!("Test D (Android URL 6MB-7MB): Status {}", and_resp.status());
    }

    Ok(())
}
