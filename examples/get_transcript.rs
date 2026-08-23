use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "MFl-hJdwdzE".to_string());

    println!("=== Fetching Subtitles & Transcript for Video [{}] ===\n", video_id);

    let yt = Innertube::new().await?;

    let tracks = innertube_rs::endpoints::transcript::get_transcript_tracks(&yt.session, &video_id).await?;
    println!("Available Caption Tracks ({} languages):", tracks.len());
    for (i, track) in tracks.iter().enumerate() {
        let kind = track.kind.as_deref().unwrap_or("standard");
        println!("  [{:2}] {} [{}] ({})", i + 1, track.name, track.language_code, kind);
    }

    if tracks.is_empty() {
        println!("No caption tracks found for this video.");
        return Ok(());
    }

    println!("\nFetching default transcript...");
    let transcript = innertube_rs::endpoints::transcript::get_transcript(&yt.session, &video_id, None).await?;
    println!("Fetched {} timed transcript segments!\n", transcript.segments.len());

    println!("Preview of first 5 segments:");
    for (i, seg) in transcript.segments.iter().take(5).enumerate() {
        let start_sec = seg.start_ms as f64 / 1000.0;
        let end_sec = seg.end_ms as f64 / 1000.0;
        println!("  [{:2}] {:05.2}s -> {:05.2}s: {}", i + 1, start_sec, end_sec, seg.text);
    }

    let srt_preview: String = transcript.to_srt().chars().take(300).collect();
    println!("\nSRT Export Preview:\n{}", srt_preview);
    println!("...");

    Ok(())
}
