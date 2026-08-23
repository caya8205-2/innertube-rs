use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";
    let yt = Innertube::new().await?;
    let info = yt.get_video_info(video_id).await?;
    let sd = info.streaming_data.unwrap();
    let f136 = sd.adaptive_formats.iter().find(|f| f.height == Some(720) && (f.mime_type.contains("avc1") || f.mime_type.contains("mp4"))).unwrap();
    let raw_url = f136.url.as_ref().unwrap();
    let total: u64 = f136.content_length.as_ref().unwrap().parse().unwrap();
    println!("Total 720p size: {} MB", total / 1024 / 1024);
    let dl = reqwest::Client::builder()
        .user_agent(innertube_rs::constants::clients::IOS_USER_AGENT)
        .build()?;
    let chunk_size = 1024 * 1024;
    let mut downloaded = 0;
    for i in 0..20 {
        let end = std::cmp::min(downloaded + chunk_size - 1, total - 1);
        let chunk_url = format!("{}&range={}-{}&rn={}", raw_url, downloaded, end, i);
        let resp = dl.get(&chunk_url)
            .header("Origin", "https://www.youtube.com")
            .header("Referer", "https://www.youtube.com")
            .header("Accept", "*/*")
            .header("Sec-Fetch-Dest", "video")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "cross-site")
            .send().await?;
        println!("Chunk {} ({}-{} MB) - Protocol: {:?}: HTTP {}", i, downloaded / 1024 / 1024, end / 1024 / 1024, resp.version(), resp.status());
        if !resp.status().is_success() {
            println!("  --> FAILED at chunk {}: HTTP {}", i, resp.status());
            break;
        }
        let b = resp.bytes().await?;
        downloaded += b.len() as u64;
    }
    println!("\nDownloaded: {} MB / {} MB", downloaded / 1024 / 1024, total / 1024 / 1024);
    if downloaded >= 15 * 1024 * 1024 {
        println!(">>> SUCCESS: Downloaded past 15MB! <<<");
    }

    Ok(())
}