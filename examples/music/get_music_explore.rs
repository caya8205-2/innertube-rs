use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing YouTube Music Explore & Charts ===\n");

    let yt = Innertube::new().await?;
    let explore = yt.get_music_explore().await?;

    println!("Top Songs ({} loaded):", explore.top_songs.len());
    for (i, song) in explore.top_songs.iter().take(5).enumerate() {
        let artists: Vec<&str> = song.artists.iter().map(|a| a.name.as_str()).collect();
        println!("  [{:2}] {} — {} (ID: {})", i + 1, song.title, artists.join(", "), song.video_id);
    }

    println!("\nNew Releases & Featured Albums ({} loaded):", explore.new_releases.len());
    for (i, album) in explore.new_releases.iter().take(5).enumerate() {
        let artist = album.artist.as_deref().unwrap_or("Various");
        println!("  [{:2}] {} by {} (ID: {})", i + 1, album.title, artist, album.browse_id);
    }

    Ok(())
}
