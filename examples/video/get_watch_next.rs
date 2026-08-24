use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "e1bCibq2I1g".to_string());

    println!("=== Fetching Watch Next Details for Video [{}] ===\n", video_id);

    let yt = Innertube::new().await?;
    let watch_next = yt.get_watch_next(&video_id).await?;

    if let Some(title) = &watch_next.current_title {
        println!("Current Video Title : {}", title);
    }
    if let Some(author) = &watch_next.current_author {
        println!("Current Channel     : {}", author);
    }

    if let Some(autoplay) = &watch_next.autoplay {
        println!("\n▶ Autoplay Next Video:");
        println!("  - Title  : {}", autoplay.title);
        println!("  - Channel: {}", autoplay.author);
        println!("  - Link   : https://youtu.be/{}", autoplay.video_id);
    }

    println!("\n📚 Recommended / Related Videos ({} items found):", watch_next.related_videos.len());
    for (i, video) in watch_next.related_videos.iter().enumerate() {
        let live_badge = if video.is_live { " [LIVE]" } else { "" };
        let duration = video.duration_text.as_deref().unwrap_or("N/A");
        let views = video.view_count_text.as_deref().unwrap_or("");
        let published = video.published_time_text.as_deref().unwrap_or("");
        let author_str = if video.author.is_empty() { "Unknown" } else { &video.author };

        println!(
            "  [{:2}] {}{} by {} ({}) {} {} (id: {})",
            i + 1,
            video.title,
            live_badge,
            author_str,
            duration,
            views,
            published,
            video.video_id
        );
    }

    if let Some(token) = &watch_next.continuation_token {
        println!("\n✨ Continuation token available: {}...", &token[..30]);
    }

    if !watch_next.playlist_items.is_empty() {
        println!("\n📋 Playlist Queue Items ({} tracks):", watch_next.playlist_items.len());
        for (i, item) in watch_next.playlist_items.iter().enumerate() {
            let sel = if item.is_selected { " (Playing)" } else { "" };
            println!("  [{:2}] {} by {}{}", i + 1, item.title, item.author, sel);
        }
    }

    println!("\n Watch Next endpoint successfully extracted!");
    Ok(())
}
