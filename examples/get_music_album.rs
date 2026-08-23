use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let album_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "MPREb_hqiB0KumHYT".to_string()); // YOASOBI - THE BOOK

    println!("=== Testing YouTube Music Album for [{}] ===\n", album_id);

    let yt = Innertube::new().await?;
    let album = yt.get_music_album(&album_id).await?;

    println!("Album: {}", album.title);
    if let Some(artist) = &album.artist {
        println!("Artist: {}", artist);
    }
    if let Some(year) = &album.year {
        println!("Year: {}", year);
    }
    if let Some(thumb) = &album.thumbnail {
        println!("Thumbnail: {}", thumb);
    }

    println!("\nTracklist ({} tracks):", album.tracks.len());
    for (i, track) in album.tracks.iter().enumerate() {
        let dur = track.duration.as_deref().unwrap_or("-");
        println!("  [{:2}] {} [{}] (ID: {})", i + 1, track.title, dur, track.video_id);
    }

    Ok(())
}
