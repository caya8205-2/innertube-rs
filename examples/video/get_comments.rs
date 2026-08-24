use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "e1bCibq2I1g".to_string());

    println!("=== Fetching Comments for Video [{}] ===\n", video_id);

    let yt = Innertube::new().await?;
    let comments_res = yt.get_comments(&video_id).await?;

    if let Some(total) = &comments_res.total_comments_text {
        println!("Total Comments Count: {}\n", total);
    }

    println!("Top Comment Threads ({} loaded):", comments_res.comments.len());
    for (i, thread) in comments_res.comments.iter().enumerate() {
        let pinned = if thread.comment.is_pinned { " 📌[PINNED]" } else { "" };
        let creator = if thread.comment.is_author_channel_owner { " 👑[CREATOR]" } else { "" };
        let likes = thread.comment.like_count_text.as_deref().unwrap_or("0");
        let replies = thread.comment.reply_count.unwrap_or(0);
        let time = thread.comment.published_time.as_deref().unwrap_or("");

        println!(
            "[{:2}]{}{} {} ({}) — 👍 {}",
            i + 1,
            pinned,
            creator,
            thread.comment.author_name,
            time,
            likes
        );
        let first_line = thread.comment.text.lines().next().unwrap_or("");
        println!("     \"{}\"", first_line);
        if replies > 0 {
            println!("     ↳ {} replies", replies);
        }
        println!();
    }

    if let Some(token) = &comments_res.continuation_token {
        println!("✨ Comments continuation token available: {}...", &token[..30]);
    }

    Ok(())
}
