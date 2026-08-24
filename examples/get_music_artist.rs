use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artist_id = std::env::args().nth(1).unwrap_or_else(|| "UCbqY3RHKkPS8dJCrfAfSk6Q".to_string()); // YOASOBI

    println!("=== Testing YouTube Music Artist Page for [{}] ===", artist_id);

    let yt = Innertube::new().await?;
    let artist = yt.get_music_artist(&artist_id).await?;

    println!("\n--- Artist Details ---");
    println!("Name: {}", artist.name);
    println!("Subscribers: {}", artist.subscribers.as_deref().unwrap_or("-"));
    println!("Description: {}", artist.description.as_deref().unwrap_or("-"));

    println!("\n--- Top Songs ({} tracks) ---", artist.top_songs.len());
    for (i, t) in artist.top_songs.iter().take(5).enumerate() {
        let dur = t.duration.as_deref().unwrap_or("-");
        println!("  [{:2}] {} [{}] (ID: {})", i + 1, t.title, dur, t.video_id);
    }

    println!("\n--- Albums ({} albums) ---", artist.albums.len());
    for (i, alb) in artist.albums.iter().take(4).enumerate() {
        println!("  [{:2}] {} (ID: {})", i + 1, alb.title, alb.browse_id);
    }

    println!("\n--- Singles & EPs ({} singles) ---", artist.singles.len());
    for (i, s) in artist.singles.iter().take(4).enumerate() {
        println!("  [{:2}] {} (ID: {})", i + 1, s.title, s.browse_id);
    }

    println!("\n--- Similar Artists ({} artists) ---", artist.similar_artists.len());
    for (i, a) in artist.similar_artists.iter().take(4).enumerate() {
        println!("  [{:2}] {} (ID: {})", i + 1, a.name, a.browse_id);
    }

    Ok(())
}
