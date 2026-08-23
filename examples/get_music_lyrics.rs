use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "dQw4w9WgXcQ".to_string());

    println!("=== Testing YouTube Music Lyrics for [{}] ===\n", video_id);

    let yt = Innertube::new().await?;

    match yt.get_music_lyrics(&video_id).await {
        Ok(lyrics) => {
            println!("Title: {}", lyrics.title.as_deref().unwrap_or("Lyrics"));
            if let Some(footer) = &lyrics.footer {
                println!("Source: {}", footer);
            }
            println!("\n--- Lyrics Preview ---");
            let preview_lines: Vec<&str> = lyrics.lyrics_text.lines().take(15).collect();
            println!("{}", preview_lines.join("\n"));
            if lyrics.lyrics_text.lines().count() > 15 {
                println!("...");
            }
        }
        Err(e) => {
            println!("ℹ️ {}", e);
        }
    }

    Ok(())
}
