use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "UCX6OQ3DkcsbYNE6H8uQQuVA".to_string()); // MrBeast

    println!("=== Testing Channel Extended Tabs for [{}] ===\n", channel_id);

    let yt = Innertube::new().await?;

    // 1. Channel About
    println!("--- Channel Profile & About ---");
    let about = yt.get_channel_about(&channel_id).await?;
    println!("Title: {}", about.title);
    if let Some(sub) = &about.subscriber_count {
        println!("Subscribers: {}", sub);
    }
    if let Some(handle) = &about.custom_url {
        println!("Handle: {}", handle);
    }
    if let Some(desc) = &about.description {
        println!("Description: {}", desc.lines().next().unwrap_or(""));
    }

    // 2. Channel Videos Tab
    println!("\n--- Channel Videos Tab ---");
    let videos_res = yt.get_channel_videos(&channel_id, None).await?;
    println!("Loaded {} videos:", videos_res.videos.len());
    for (i, v) in videos_res.videos.iter().take(5).enumerate() {
        let dur = v.duration.as_deref().unwrap_or("-");
        let views = v.views.as_deref().unwrap_or("-");
        let pub_time = v.published_time.as_deref().unwrap_or("-");
        println!("  [{:2}] {} [{}] (Views: {}, Published: {}) (ID: {})", i + 1, v.title, dur, views, pub_time, v.video_id);
    }

    // 3. Channel Shorts Tab
    println!("\n--- Channel Shorts Tab ---");
    let shorts_res = yt.get_channel_shorts(&channel_id, None).await?;
    println!("Loaded {} shorts:", shorts_res.shorts.len());
    for (i, s) in shorts_res.shorts.iter().take(5).enumerate() {
        let views = s.views.as_deref().unwrap_or("-");
        println!("  [{:2}] {} (Views: {}) (ID: {})", i + 1, s.title, views, s.video_id);
    }

    Ok(())
}
