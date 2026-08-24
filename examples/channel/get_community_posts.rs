use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel_id = std::env::args().nth(1).unwrap_or_else(|| "UCX6OQ3DkcsbYNE6H8uQQuVA".to_string()); // MrBeast

    println!("=== Testing YouTube Channel Community Posts for [{}] ===", channel_id);

    let yt = Innertube::new().await?;
    let resp = yt.get_channel_community(&channel_id, None).await?;

    println!("\nFound {} community posts:", resp.posts.len());

    for (i, p) in resp.posts.iter().take(5).enumerate() {
        let author = p.author.as_ref().map(|a| a.name.as_str()).unwrap_or("Channel Owner");
        let pub_time = p.published_time.as_deref().unwrap_or("-");
        let likes = p.vote_count.as_deref().unwrap_or("0");
        let comments = p.comment_count.as_deref().unwrap_or("0");

        println!("\n📝 Post [{}] by {} ({})", i + 1, author, pub_time);
        let preview = if p.content_text.len() > 120 {
            format!("{}...", &p.content_text[..120])
        } else {
            p.content_text.clone()
        };
        println!("   \"{}\"", preview.replace('\n', " "));
        println!("   ❤️ Likes: {} | 💬 Comments: {}", likes, comments);

        if let Some(poll) = &p.poll {
            let total = poll.total_votes_text.as_deref().unwrap_or("-");
            println!("   📊 Poll (Total votes: {}):", total);
            for (c_idx, choice) in poll.choices.iter().enumerate() {
                let pct = choice.vote_percentage.as_deref().unwrap_or("-");
                println!("      [{}] {} ({})", c_idx + 1, choice.text, pct);
            }
        }

        if !p.images.is_empty() {
            println!("   🖼️ Images attached: {}", p.images.len());
        }
    }

    if let Some(token) = &resp.continuation_token {
        println!("\n✨ Continuation token: {}...", &token[..token.len().min(35)]);
    }

    Ok(())
}
