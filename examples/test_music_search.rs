use innertube_rs::{Innertube, MusicSearchFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "YOASOBI".to_string());

    println!("=== Testing YouTube Music Filtered Search for [{}] ===\n", query);

    let yt = Innertube::new().await?;

    // 1. Search Songs
    println!("--- Searching Songs ---");
    let song_results = yt.search_music(&query, Some(MusicSearchFilter::Songs)).await?;
    println!("Found {} songs:", song_results.songs.len());
    for (i, song) in song_results.songs.iter().take(5).enumerate() {
        let artists: Vec<&str> = song.artists.iter().map(|a| a.name.as_str()).collect();
        let dur = song.duration.as_deref().unwrap_or("unknown");
        let album = song.album.as_ref().map(|a| a.title.as_str()).unwrap_or("-");
        let explicit = if song.is_explicit { " 🔞[E]" } else { "" };
        println!("  [{:2}]{} {} — {} (Album: {}) [{}] (ID: {})", i + 1, explicit, song.title, artists.join(", "), album, dur, song.video_id);
    }

    // 2. Search Albums
    println!("\n--- Searching Albums ---");
    let album_results = yt.search_music(&query, Some(MusicSearchFilter::Albums)).await?;
    println!("Found {} albums:", album_results.albums.len());
    for (i, album) in album_results.albums.iter().take(5).enumerate() {
        let artist = album.artist.as_deref().unwrap_or("Various Artists");
        println!("  [{:2}] {} by {} (ID: {})", i + 1, album.title, artist, album.browse_id);
    }

    // 3. Search Artists
    println!("\n--- Searching Artists ---");
    let artist_results = yt.search_music(&query, Some(MusicSearchFilter::Artists)).await?;
    println!("Found {} artists:", artist_results.artists.len());
    for (i, artist) in artist_results.artists.iter().take(5).enumerate() {
        println!("  [{:2}] {} (ID: {})", i + 1, artist.name, artist.browse_id);
    }

    // 4. Search Playlists
    println!("\n--- Searching Playlists ---");
    let playlist_results = yt.search_music(&query, Some(MusicSearchFilter::Playlists)).await?;
    println!("Found {} playlists:", playlist_results.playlists.len());
    for (i, pl) in playlist_results.playlists.iter().take(5).enumerate() {
        let author = pl.author.as_deref().unwrap_or("Unknown");
        println!("  [{:2}] {} by {} (ID: {})", i + 1, pl.title, author, pl.browse_id);
    }

    Ok(())
}
