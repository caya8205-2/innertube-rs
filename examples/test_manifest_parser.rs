use innertube_rs::Innertube;
use innertube_rs::utils::manifest::{fetch_and_parse_dash, fetch_and_parse_hls};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;

    // Search for an active live stream
    let search_results = yt.search("live lofi stream", None).await?;
    let live_video = search_results.items.iter().find_map(|item| match item {
        innertube_rs::SearchResultItem::Video(v) => Some(v.video_id.clone()),
        _ => None,
    });

    let video_id = match std::env::args().nth(1) {
        Some(id) => id,
        None => live_video.unwrap_or_else(|| "live".to_string()),
    };

    println!("=== Testing HLS & DASH Manifest Parser for Active Live Stream [{}] ===\n", video_id);

    let info = yt.get_video_info(&video_id).await?;

    if let Some(streaming_data) = &info.streaming_data {
        if let Some(hls_url) = &streaming_data.hls_manifest_url {
            println!("Found HLS Manifest URL: {}...\n", &hls_url[..std::cmp::min(80, hls_url.len())]);
            match fetch_and_parse_hls(&yt.session.http_client, hls_url).await {
                Ok(streams) => {
                    println!("Extracted {} HLS stream variants:", streams.len());
                    for (i, s) in streams.iter().enumerate() {
                        let res = match (s.width, s.height) {
                            (Some(w), Some(h)) => format!("{}x{}", w, h),
                            _ => "audio-only".to_string(),
                        };
                        let bw = s.bandwidth.map(|b| format!("{} kbps", b / 1000)).unwrap_or_default();
                        let fps = s.frame_rate.map(|f| format!("{}fps", f)).unwrap_or_default();
                        let codecs = s.codecs.as_deref().unwrap_or("unknown");
                        println!("  [{:2}] itag {:?} — {:10} ({:5}) [{:10}] codecs: {}", i + 1, s.itag, res, fps, bw, codecs);
                    }
                }
                Err(e) => println!("Failed to parse HLS manifest: {}", e),
            }
        } else {
            println!("No HLS manifest URL (video is not a live stream).");
        }

        if let Some(dash_url) = &streaming_data.dash_manifest_url {
            println!("\nFound DASH Manifest URL: {}...\n", &dash_url[..std::cmp::min(80, dash_url.len())]);
            match fetch_and_parse_dash(&yt.session.http_client, dash_url).await {
                Ok(streams) => {
                    println!("Extracted {} DASH stream representations:", streams.len());
                    for (i, s) in streams.iter().enumerate() {
                        let res = match (s.width, s.height) {
                            (Some(w), Some(h)) => format!("{}x{}", w, h),
                            _ => "audio-only".to_string(),
                        };
                        let bw = s.bandwidth.map(|b| format!("{} kbps", b / 1000)).unwrap_or_default();
                        println!("  [{:2}] itag {:?} — {} [{}] ({})", i + 1, s.itag, res, bw, s.mime_type);
                    }
                }
                Err(e) => println!("Failed to parse DASH manifest: {}", e),
            }
        }
    }

    Ok(())
}
