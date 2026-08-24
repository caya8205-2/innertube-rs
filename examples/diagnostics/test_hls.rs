use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let video_id = "e1bCibq2I1g";
    let yt = Innertube::new().await?;
    let info = yt.get_video_info(video_id).await?;
    let sd = info.streaming_data.unwrap();
    println!("dashManifestUrl: {:?}", sd.dash_manifest_url);
    println!("hlsManifestUrl: {:?}", sd.hls_manifest_url);

    Ok(())
}