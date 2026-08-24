use innertube_rs::Innertube;
use innertube_rs::models::live_chat::LiveChatMessage;
use innertube_rs::models::search::SearchResultItem;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    println!("=== Testing YouTube Real-Time Live Chat Engine ===");

    // 1. Find a live stream dynamically
    println!("1. Searching for an active live stream ('news live')...");
    let search_res = yt.search("news live", None).await?;
    let live_video_id = search_res.items.iter().find_map(|item| match item {
        SearchResultItem::Video(v) => Some(v.video_id.clone()),
        _ => None,
    }).unwrap_or_else(|| "21X5lGlDOfg".to_string());

    println!("   >> Selected Live Video ID: [{}]", live_video_id);

    // 2. Extract live chat continuation token
    println!("2. Extracting live chat continuation token...");
    let token_opt = yt.get_live_chat_token(&live_video_id).await?;

    let token = match token_opt {
        Some(t) => {
            println!("   >> Live chat token found: {}...", &t[..t.len().min(35)]);
            t
        }
        None => {
            println!("   >> No active live chat found on this stream.");
            return Ok(());
        }
    };

    // 3. Poll live chat messages
    println!("3. Polling batch of live chat messages...\n");
    let resp = yt.get_live_chat(&token).await?;

    println!("Loaded {} live chat messages:", resp.messages.len());

    for (i, msg) in resp.messages.iter().take(10).enumerate() {
        match msg {
            LiveChatMessage::Text(t) => {
                let author = t.author.as_ref().map(|a| a.name.as_str()).unwrap_or("Anonymous");
                let badge = if t.is_owner { " [OWNER]" } else if t.is_moderator { " [MOD]" } else { "" };
                println!("  [{:2}] {}{}: {}", i + 1, author, badge, t.message);
            }
            LiveChatMessage::SuperChat(sc) => {
                let author = sc.author.as_ref().map(|a| a.name.as_str()).unwrap_or("SuperChatter");
                let msg_text = sc.message.as_deref().unwrap_or("");
                println!("  [{:2}] 💰 SUPER CHAT ({}) from {}: {}", i + 1, sc.purchase_amount_text, author, msg_text);
            }
            LiveChatMessage::Membership(m) => {
                let author = m.author.as_ref().map(|a| a.name.as_str()).unwrap_or("New Member");
                println!("  [{:2}] 🌟 MEMBERSHIP: {} ({})", i + 1, author, m.header_subtext.as_deref().unwrap_or(""));
            }
            LiveChatMessage::System(s) => {
                println!("  [{:2}] ℹ️ SYSTEM: {}", i + 1, s);
            }
        }
    }

    if let Some(next) = resp.continuation_token {
        println!("\n✨ Next poll interval: {} ms (next token: {}...)", resp.poll_timeout_ms, &next[..next.len().min(35)]);
    }

    Ok(())
}
