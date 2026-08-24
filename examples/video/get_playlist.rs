use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playlist_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "PLcwSN1dMiPp7-zS0pz_uiRreJopx9EHpY".to_string()); // YOASOBI playlist

    println!("=== Testing YouTube Playlist Scraper for [{}] ===\n", playlist_id);

    let yt = Innertube::new().await?;
    let playlist = yt.get_playlist(&playlist_id).await?;

    println!("Playlist Title: {}", playlist.title);
    if let Some(author) = &playlist.author {
        println!("Author: {} (ID: {:?})", author, playlist.author_id);
    }
    if let Some(count) = playlist.video_count {
        println!("Total Video Count: {}", count);
    }
    if let Some(thumb) = &playlist.thumbnail {
        println!("Thumbnail: {}", thumb);
    }

    println!("\nFirst Page Videos ({} videos loaded):", playlist.videos.len());
    for (i, video) in playlist.videos.iter().take(5).enumerate() {
        let dur = video.duration.as_deref().unwrap_or("-");
        println!("  [{:2}] {} — {} [{}] (ID: {})", i + 1, video.title, video.author, dur, video.id);
    }

    // Test continuation if available
    if let Some(token) = &playlist.continuation_token {
        println!("\n--- Fetching Page 2 using Continuation Token ---");
        let cont = yt.get_playlist_continuation(token).await?;
        println!("Loaded {} additional videos:", cont.videos.len());
        for (i, video) in cont.videos.iter().take(5).enumerate() {
            let dur = video.duration.as_deref().unwrap_or("-");
            println!("  [{:2}] {} — {} [{}] (ID: {})", i + 1, video.title, video.author, dur, video.id);
        }
    }

    Ok(())
}
