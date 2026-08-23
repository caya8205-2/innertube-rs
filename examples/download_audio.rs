use std::fs::File;
use std::io::Write;
use innertube_rs::models::format::{FormatFilter, FormatType, QualityPreference};
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Chunk Size on pzyKjoElqxI ===");
    let yt = Innertube::new().await?;
    let video_id = "pzyKjoElqxI";

    let audio_filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: Some("mp4".to_string()),
    };

    let stream_url = yt.get_stream_url(video_id, &audio_filter).await?;
    let info = yt.get_video_info(video_id).await?;
    let fmt = innertube_rs::endpoints::player::select_format(&info, &audio_filter)?;
    let total: u64 = fmt.content_length.as_ref().unwrap().parse()?;
    println!("Total audio size: {} bytes ({:.2} MB)", total, total as f64 / 1_048_576.0);

    let chunk_size: u64 = 1024 * 1024; // 1MB chunks
    let mut downloaded: u64 = 0;
    let mut file = File::create("target/pzyKjoElqxI_audio.m4a")?;

    while downloaded < total {
        let end = std::cmp::min(downloaded + chunk_size - 1, total - 1);
        println!("Fetching range: bytes={}-{}...", downloaded, end);
        let resp = yt.session.http_client
            .get(&stream_url)
            .header("Range", format!("bytes={}-{}", downloaded, end))
            .send()
            .await?;

        println!("HTTP Status: {}", resp.status());
        if !resp.status().is_success() && resp.status() != 206 {
            panic!("Failed at range {}-{}: {}", downloaded, end, resp.status());
        }

        let bytes = resp.bytes().await?;
        file.write_all(&bytes)?;
        downloaded += bytes.len() as u64;
        println!("Progress: {:.1}% ({}/{} bytes)", (downloaded as f64 / total as f64) * 100.0, downloaded, total);
    }

    println!("\n=== AUDIO DOWNLOAD 100% COMPLETE! File size: {} bytes ===", downloaded);
    Ok(())
}
