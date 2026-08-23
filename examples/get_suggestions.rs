use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "yoasobi".to_string());

    println!("=== Testing Search Autocomplete Suggestions for [{}] ===\n", query);

    let yt = Innertube::new().await?;

    // 1. YouTube Standard Suggestions
    println!("--- Standard YouTube Suggestions ---");
    let yt_suggestions = yt.get_search_suggestions(&query, false).await?;
    println!("Found {} suggestions:", yt_suggestions.suggestions.len());
    for (i, sug) in yt_suggestions.suggestions.iter().enumerate() {
        println!("  [{:2}] {}", i + 1, sug);
    }

    // 2. YouTube Music Suggestions
    println!("\n--- YouTube Music Suggestions ---");
    let music_suggestions = yt.get_search_suggestions(&query, true).await?;
    println!("Found {} suggestions:", music_suggestions.suggestions.len());
    for (i, sug) in music_suggestions.suggestions.iter().enumerate() {
        println!("  [{:2}] {}", i + 1, sug);
    }

    Ok(())
}
