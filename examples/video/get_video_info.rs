use innertube_rs::models::format::{FormatFilter, FormatType, QualityPreference};
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing innertube-rs Video & Stream URL Extraction ===");

    println!("\n1. Initializing Innertube client...");
    let yt = Innertube::new().await?;
    println!("   >> Innertube client initialized!");

    // Test with a famous public YouTube video: Rick Astley - Never Gonna Give You Up
    let video_id = "dQw4w9WgXcQ";

    println!("\n2. Fetching metadata for video ID: {video_id}...");
    let info = yt.get_video_info(video_id).await?;
    if let Some(ref details) = info.video_details {
        println!("   >> Title: {}", details.title);
        println!("   >> Author: {}", details.author);
        println!("   >> Duration: {}s", details.length_seconds);
        println!("   >> View count: {:?}", details.view_count);
    }

    if let Some(ref streaming) = info.streaming_data {
        println!("   >> Formats count: {}", streaming.formats.len());
        println!("   >> Adaptive formats count: {}", streaming.adaptive_formats.len());
    }

    println!("\n3. Resolving and deciphering highest quality Audio-Only stream URL...");
    let audio_filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: None,
    };

    let stream_url = yt.get_stream_url(video_id, &audio_filter).await?;
    println!("   >> Resolved stream URL (first 120 chars): {}...", &stream_url[..120.min(stream_url.len())]);

    println!("\n4. Verifying playable stream URL with HTTP HEAD request...");
    let head_resp = yt.session.http_client.head(&stream_url).send().await?;
    println!("   >> HTTP Status: {}", head_resp.status());
    if let Some(len) = head_resp.headers().get("content-length") {
        let len_bytes: f64 = len.to_str()?.parse()?;
        println!("   >> Content-Length: {} bytes (~{:.2} MB)", len.to_str()?, len_bytes / (1024.0 * 1024.0));
    }

    assert!(head_resp.status().is_success(), "Stream URL HEAD request must succeed (200 OK)");

    println!("\n5. Resolving and deciphering highest quality 1080p/Video stream URL...");
    let video_filter = FormatFilter {
        format_type: FormatType::VideoOnly,
        quality: QualityPreference::Highest,
        container: None,
    };

    let video_stream_url = yt.get_stream_url(video_id, &video_filter).await?;
    println!("   >> Resolved Video stream URL (first 120 chars): {}...", &video_stream_url[..120.min(video_stream_url.len())]);

    let video_head_resp = yt.session.http_client.head(&video_stream_url).send().await?;
    println!("   >> Video HTTP Status: {}", video_head_resp.status());
    if let Some(len) = video_head_resp.headers().get("content-length") {
        let len_bytes: f64 = len.to_str()?.parse()?;
        println!("   >> Video Content-Length: {} bytes (~{:.2} MB)", len.to_str()?, len_bytes / (1024.0 * 1024.0));
    }

    assert!(video_head_resp.status().is_success(), "Video stream URL HEAD request must succeed (200 OK)");

    println!("\n=== ALL VIDEO & AUDIO STREAM EXTRACTIONS PASSED 100%! ===");
    Ok(())
}
