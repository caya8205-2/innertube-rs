use innertube_rs::constants::clients;
use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";
    let yt = innertube_rs::Innertube::new().await?;

    let web_session = Session::create(SessionOptions {
        client_name: Some(clients::WEB_NAME.to_string()),
        client_version: Some(clients::WEB_VERSION.to_string()),
        device_category: Some("DESKTOP".to_string()),
        user_agent: Some(clients::WEB_USER_AGENT.to_string()),
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

    let resp = web_session.post_innertube("/player", payload).await?;
    let pr: PlayerResponse = resp.json().await?;
    let sd = pr.streaming_data.unwrap();

    let f136 = sd.adaptive_formats.iter().find(|f| f.height == Some(720) && (f.mime_type.contains("avc1") || f.mime_type.contains("mp4"))).unwrap();
    println!("WEB Format f136 has signatureCipher: {}", f136.signature_cipher.is_some());
    println!("WEB Format f136 has url: {}", f136.url.is_some());
    
    let resolved_url = innertube_rs::endpoints::player::resolve_stream_url(f136, &yt.player.decipherer)?;
    println!("Resolved URL:\n{}\n", resolved_url);
    
    let total: u64 = f136.content_length.as_ref().unwrap().parse().unwrap();
    println!("Total 720p size: {} MB", total / 1024 / 1024);
    
    let dl = reqwest::Client::builder().user_agent(clients::WEB_USER_AGENT).build()?;
    let chunk_size = 1024 * 1024;
    let mut downloaded = 0;
    
    for i in 0..20 {
        let end = std::cmp::min(downloaded + chunk_size - 1, total - 1);
        let chunk_url = format!("{}&range={}-{}&rn={}", resolved_url, downloaded, end, i);
        let resp = dl.get(&chunk_url)
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com")
            .header("Accept", "*/*")
            .send().await?;
        println!("Chunk {} ({}-{} MB): HTTP {}", i, downloaded / 1024 / 1024, end / 1024 / 1024, resp.status());
        if !resp.status().is_success() {
            println!("  --> FAILED at chunk {}: HTTP {}", i, resp.status());
            break;
        }
        let b = resp.bytes().await?;
        downloaded += b.len() as u64;
    }
    
    println!("\nTotal WEB Downloaded: {} MB / {} MB", downloaded / 1024 / 1024, total / 1024 / 1024);
    if downloaded >= 15 * 1024 * 1024 {
        println!("============================================================");
        println!(">>> SUCCESS: WEB DECIPHERED STREAM DOWNLOADED PAST 15MB! <<<");
        println!("============================================================");
    }
    
    Ok(())
}