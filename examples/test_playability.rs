use innertube_rs::constants::clients;
use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "LLFhKaqnWwk";
    let session = Session::create(SessionOptions::default()).await?;

    let mut vr_ctx = session.context.clone();
    vr_ctx.client.client_name = clients::ANDROID_VR_NAME.to_string();
    vr_ctx.client.client_version = clients::ANDROID_VR_VERSION.to_string();
    vr_ctx.client.platform = "MOBILE".to_string();
    vr_ctx.client.user_agent = clients::ANDROID_VR_USER_AGENT.to_string();
    vr_ctx.client.device_make = Some("Oculus".to_string());
    vr_ctx.client.device_model = Some("Quest 3".to_string());
    vr_ctx.client.os_name = "Android".to_string();
    vr_ctx.client.os_version = "12L".to_string();
    vr_ctx.client.android_sdk_version = Some(32);

    let pr: PlayerResponse = session.http_client
        .post(format!("{}/player?key={}", innertube_rs::constants::INNERTUBE_API_BASE_URL, session.api_key))
        .header("User-Agent", clients::ANDROID_VR_USER_AGENT)
        .header("X-Youtube-Client-Name", "81")
        .header("X-Youtube-Client-Version", clients::ANDROID_VR_VERSION)
        .json(&serde_json::json!({
            "context": vr_ctx,
            "videoId": video_id,
            "contentCheckOk": true,
            "racyCheckOk": true
        }))
        .send().await?.json().await?;

    println!("Playability: {}", pr.playability_status.status);
    let sd = pr.streaming_data.as_ref().unwrap();

    // List ALL formats
    println!("\n=== Progressive formats (sd.formats) ===");
    for f in &sd.formats {
        println!("  itag={} mime={} res={}x{} clen={:?}",
            f.itag, f.mime_type, f.width.unwrap_or(0), f.height.unwrap_or(0), f.content_length);
    }

    println!("\n=== Adaptive formats ===");
    for f in &sd.adaptive_formats {
        let kind = if f.is_audio_only() { "audio" } else { "video" };
        println!("  itag={} {} mime={} clen={:?}",
            f.itag, kind, f.mime_type, f.content_length);
    }

    // Try progressive itag 18
    let prog = sd.formats.iter().find(|f| f.itag == 18);
    if let Some(fmt) = prog {
        let url = fmt.url.as_ref().unwrap();
        let clen: u64 = fmt.content_length.as_ref().map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
        println!("\n=== Downloading progressive itag 18 ({:.1} MB) ===", clen as f64 / 1024.0 / 1024.0);

        let dl = reqwest::Client::builder()
            .user_agent(clients::ANDROID_VR_USER_AGENT)
            .build()?;

        let mut resp = dl.get(url)
            .header("Range", format!("bytes=0-{}", if clen > 0 { clen - 1 } else { 10485759 }))
            .send().await?;
        println!("Status: {}", resp.status());

        if resp.status().is_success() || resp.status() == reqwest::StatusCode::PARTIAL_CONTENT {
            let mut downloaded: u64 = 0;
            let mut file = tokio::fs::File::create("test_prog18.tmp").await?;
            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
                downloaded += chunk.len() as u64;
            }
            file.flush().await?;
            println!("Downloaded: {} bytes", downloaded);
            let _ = tokio::fs::remove_file("test_prog18.tmp").await;
        }
    } else {
        println!("\nNo progressive itag 18 found");
    }

    Ok(())
}
