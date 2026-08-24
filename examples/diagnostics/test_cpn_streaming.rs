use innertube_rs::constants::clients;
use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use rand::Rng;
use serde_json::json;

fn generate_cpn() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut rng = rand::rng();
    (0..16)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";
    let cpn = generate_cpn();
    println!("Generated CPN: {}", cpn);
    let session = Session::create(SessionOptions {
        client_name: Some(clients::IOS_NAME.to_string()),
        client_version: Some(clients::IOS_VERSION.to_string()),
        device_category: Some("MOBILE".to_string()),
        user_agent: Some(clients::IOS_USER_AGENT.to_string()),
        generate_session_locally: Some(true),
        ..Default::default()
    }).await?;
    let payload = json!({
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true,
        "cpn": cpn,
        "playbackContext": {
            "contentPlaybackContext": {
                "html5Preference": "HTML5_PREF_WANTS"
            }
        }
    });

    let resp = session.post_innertube("/player", payload).await?;
    let pr: PlayerResponse = resp.json().await?;
    let sd = pr.streaming_data.unwrap();
    let f136 = sd.adaptive_formats.iter().find(|f| f.height == Some(720) && (f.mime_type.contains("avc1") || f.mime_type.contains("mp4"))).unwrap();
    let raw_url = f136.url.as_ref().unwrap();
    let total: u64 = f136.content_length.as_ref().unwrap().parse().unwrap();
    println!("Total 720p size: {} MB", total / 1024 / 1024);
    let dl = reqwest::Client::builder().user_agent(clients::IOS_USER_AGENT).build()?;
    let chunk_size = 1024 * 1024;
    let mut downloaded = 0;
    for i in 0..20 {
        let end = std::cmp::min(downloaded + chunk_size - 1, total - 1);
        let chunk_url = format!("{}&cpn={}&range={}-{}&rn={}", raw_url, cpn, downloaded, end, i);
        let resp = dl.get(&chunk_url)
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com")
            .header("DNT", "1")
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
    println!("\nDownloaded: {} MB / {} MB", downloaded / 1024 / 1024, total / 1024 / 1024);
    if downloaded >= 15 * 1024 * 1024 {
        println!(">>> SUCCESS: CPN streaming downloaded past 15MB! <<<");
    }

    Ok(())
}