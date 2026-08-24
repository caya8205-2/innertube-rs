use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";

    let session = Session::create(SessionOptions {
        client_name: Some("WEB_CREATOR".to_string()),
        client_version: Some("1.20241203.01.00".to_string()),
        device_category: Some("DESKTOP".to_string()),
        user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36".to_string()),
        generate_session_locally: Some(true),
        ..Default::default()
    }).await?;

    let payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true
    });

    let resp = session.post_innertube("/player", payload).await?;
    println!("WEB_CREATOR Status: {}", resp.status());
    let pr: PlayerResponse = resp.json().await?;
    println!("WEB_CREATOR Playability: {} (reason: {:?})", pr.playability_status.status, pr.playability_status.reason);
    
    if let Some(sd) = pr.streaming_data {
        println!("Formats count: {}", sd.formats.len());
        println!("Adaptive formats count: {}", sd.adaptive_formats.len());
        for f in &sd.formats {
            println!("  Prog format itag {}: height={:?}, mime={}, has_url={}", f.itag, f.height, f.mime_type, f.url.is_some());
            if let Some(ref u) = f.url {
                let dl = reqwest::Client::new();
                let chunk_url = format!("{}&range=6291456-7340031", u);
                let test_resp = dl.get(&chunk_url).send().await?;
                println!("    >>> Chunk 6 Download Status: HTTP {} <<<", test_resp.status());
            }
        }
        for f in &sd.adaptive_formats {
            if f.height == Some(720) || f.height == Some(1080) {
                println!("  Adaptive format itag {}: height={:?}, mime={}, has_url={}", f.itag, f.height, f.mime_type, f.url.is_some());
                if let Some(ref u) = f.url {
                    let dl = reqwest::Client::new();
                    let chunk_url = format!("{}&range=6291456-7340031", u);
                    let test_resp = dl.get(&chunk_url).send().await?;
                    println!("    >>> Adaptive 720p Chunk 6 Download Status: HTTP {} <<<", test_resp.status());
                }
            }
        }
    }

    Ok(())
}