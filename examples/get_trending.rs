use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing YouTube Trending Feed (FEtrending) ===");

    let yt = Innertube::new().await?;
    let trending = yt.get_trending(None).await?;

    println!("\n--- Trending Tabs ({} tabs) ---", trending.tabs.len());
    for (i, tab) in trending.tabs.iter().enumerate() {
        let sel = if tab.is_selected { "[CURRENT]" } else { "" };
        println!("  [{:2}] {} {}", i + 1, tab.title, sel);
    }

    println!("\n--- Trending Videos in [{}] ({} loaded) ---", trending.current_tab, trending.videos.len());
    for (i, v) in trending.videos.iter().take(8).enumerate() {
        let author = v.author.as_ref().map(|a| a.name.as_str()).unwrap_or("Unknown");
        let dur = v.duration.as_deref().unwrap_or("-");
        let views = v.view_count.as_deref().unwrap_or("-");
        println!("  [{:2}] {} — {} [{}] (Views: {}, ID: {})", i + 1, v.title, author, dur, views, v.id);
    }

    Ok(())
}
