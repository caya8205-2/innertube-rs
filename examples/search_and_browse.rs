use innertube_rs::models::search::SearchResultItem;
use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing innertube-rs Search & Browse ===");

    println!("\n1. Initializing Innertube client...");
    let yt = Innertube::new().await?;
    println!("   >> Innertube client initialized!");

    // 1. Test Search
    let query = "rick astley never gonna give you up";
    println!("\n2. Searching for '{query}'...");
    let search_res = yt.search(query, None).await?;
    println!("   >> Found {} search items!", search_res.items.len());
    for (i, item) in search_res.items.iter().take(3).enumerate() {
        match item {
            SearchResultItem::Video(v) => {
                println!("   [{i}] Video: {} (ID: {}, Author: {}, Duration: {:?})", v.title, v.video_id, v.author, v.duration);
            }
            SearchResultItem::Channel(c) => {
                println!("   [{i}] Channel: {} (ID: {}, Subs: {:?})", c.title, c.channel_id, c.subscriber_count);
            }
            SearchResultItem::Playlist(p) => {
                println!("   [{i}] Playlist: {} (ID: {}, Author: {})", p.title, p.playlist_id, p.author);
            }
        }
    }

    // 2. Test Channel Fetching
    let channel_handle = "@RickAstleyYT";
    println!("\n3. Fetching channel info for {channel_handle}...");
    let channel_info = yt.get_channel(channel_handle).await?;
    println!("   >> Name: {}", channel_info.name);
    println!("   >> Followers: {:?}", channel_info.followers);
    println!("   >> Avatar: {:?}", channel_info.image);
    println!("   >> Top tracks count: {}", channel_info.top_tracks.len());
    println!("   >> Playlists count: {}", channel_info.channel_playlists.len());

    // 3. Test Playlist Fetching using one of the extracted playlists
    if let Some(pl) = channel_info.channel_playlists.first() {
        println!("\n4. Fetching playlist info for '{}' (ID: {})...", pl.name, pl.id);
        let pl_info = yt.get_playlist(&pl.id).await?;
        println!("   >> Playlist Name: {}", pl_info.name);
        println!("   >> Track count: {}", pl_info.tracks.len());
        for (i, track) in pl_info.tracks.iter().take(3).enumerate() {
            println!("      [{i}] {} - {} (ID: {})", track.artist, track.title, track.youtube_id);
        }
    }

    println!("\n=== ALL SEARCH & BROWSE TESTS PASSED 100%! ===");
    Ok(())
}
