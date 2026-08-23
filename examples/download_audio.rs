use std::fs::File;
use std::io::Write;
use innertube_rs::models::format::{FormatFilter, FormatType, QualityPreference};
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing innertube-rs Audio Downloader ===");

    println!("\n1. Initializing Innertube client...");
    let yt = Innertube::new().await?;
    println!("   >> Innertube client initialized!");

    let video_id = "dQw4w9WgXcQ";

    println!("\n2. Getting metadata and audio stream URL for video: {video_id}...");
    let audio_filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: Some("mp4".to_string()), // AAC / M4A audio
    };

    let stream_url = yt.get_stream_url(video_id, &audio_filter).await?;
    println!("   >> Resolved audio stream URL successfully!");

    println!("\n3. Downloading audio stream (first 512 KB chunk for test)...");
    let resp = yt.session.http_client
        .get(&stream_url)
        .header("Range", "bytes=0-524287")
        .send()
        .await?;

    println!("   >> HTTP Status: {}", resp.status());
    assert!(resp.status().is_success() || resp.status() == 206);

    let bytes = resp.bytes().await?;
    println!("   >> Downloaded: {} bytes ({:.2} KB)", bytes.len(), bytes.len() as f64 / 1024.0);

    let output_path = "target/test_audio_sample.m4a";
    let mut file = File::create(output_path)?;
    file.write_all(&bytes)?;
    println!("   >> Saved audio sample chunk to: {}", output_path);

    println!("\n=== AUDIO STREAM DOWNLOAD TEST SUCCEEDED 100%! ===");
    Ok(())
}
