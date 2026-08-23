use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    println!("Testing pzyKjoElqxI with default Innertube::new()...");
    let info = yt.get_video_info("pzyKjoElqxI").await?;
    println!("pzyKjoElqxI Playability: {}", info.playability_status.status);
    println!("Title: {:?}", info.video_details.as_ref().map(|d| &d.title));
    let formats = info.streaming_data.as_ref().map(|sd| sd.adaptive_formats.len()).unwrap_or(0);
    println!("Adaptive formats count: {}", formats);

    println!("\nTesting dQw4w9WgXcQ with default Innertube::new()...");
    let info2 = yt.get_video_info("dQw4w9WgXcQ").await?;
    println!("dQw4w9WgXcQ Playability: {}", info2.playability_status.status);
    println!("Title: {:?}", info2.video_details.as_ref().map(|d| &d.title));
    let formats2 = info2.streaming_data.as_ref().map(|sd| sd.adaptive_formats.len()).unwrap_or(0);
    println!("Adaptive formats count: {}", formats2);

    Ok(())
}
