use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";
    println!("=== Testing TVHTML5_SIMPLY_EMBEDDED_PLAYER for {} ===", video_id);

    let tv_session = Session::create(SessionOptions {
        client_name: Some("TVHTML5_SIMPLY_EMBEDDED_PLAYER".to_string()),
        client_version: Some("2.0".to_string()),
        device_category: Some("TV".to_string()),
        user_agent: Some("Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version".to_string()),
        generate_session_locally: Some(true),
        ..Default::default()
    }).await?;

    let payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true,
        "context": {
            "client": {
                "clientName": "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
                "clientVersion": "2.0",
                "clientScreen": "WATCH",
                "hl": "en",
                "gl": "US"
            },
            "thirdParty": {
                "embedUrl": "https://www.youtube.com"
            }
        }
    });

    let resp = tv_session.post_innertube("/player", payload).await?;
    println!("HTTP Status: {}", resp.status());
    let pr: PlayerResponse = resp.json().await?;
    println!("Playability Status: {} (reason: {:?})", pr.playability_status.status, pr.playability_status.reason);

    Ok(())
}
