use innertube_rs::{log, Innertube, SessionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing YouTube Home Feed (FEwhat_to_watch) ===");

    innertube_rs::log::load_dotenv();

    let yt = if let Ok(cookie) = std::env::var("INNERTUBE_COOKIE") {
        println!("Authenticated with INNERTUBE_COOKIE from .env");
        Innertube::with_options(SessionOptions {
            cookie: Some(cookie),
            ..Default::default()
        })
        .await?
    } else {
        println!("Running anonymously (no INNERTUBE_COOKIE found in .env)");
        Innertube::new().await?
    };

    let feed = yt.get_home_feed(None).await?;

    let _ = innertube_rs::log::dump_json("feed.json", &feed.videos);

    let _ = log::dump_json("feed", &feed.videos);

    println!(
        "\n--- Available Category Filters ({} chips) ---",
        feed.filter_chips.len()
    );
    for (i, chip) in feed.filter_chips.iter().take(8).enumerate() {
        let sel = if chip.is_selected { "[SELECTED]" } else { "" };
        println!("  [{:2}] {} {}", i + 1, chip.text, sel);
    }

    println!(
        "\n--- Home Feed Videos ({} videos loaded) ---",
        feed.videos.len()
    );
    for (i, v) in feed.videos.iter().take(6).enumerate() {
        let author = v
            .author
            .as_ref()
            .map(|a| a.name.as_str())
            .unwrap_or("Unknown");
        let dur = v.duration.as_deref().unwrap_or("-");
        let views = v.view_count.as_deref().unwrap_or("-");
        println!(
            "  [{:2}] {} — {} [{}] (Views: {}, ID: {})",
            i + 1,
            v.title,
            author,
            dur,
            views,
            v.id
        );
    }

    if let Some(token) = &feed.continuation_token {
        println!(
            "\n✨ Home Feed continuation token: {}...",
            &token[..token.len().min(35)]
        );
    }

    Ok(())
}
