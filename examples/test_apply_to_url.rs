use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yt = Innertube::new().await?;
    let test_url = "https://rr2.googlevideo.com/videoplayback?expire=123&n=jUB4VWXSlSyPbA&c=MWEB";
    let transformed_url = yt.player.decipherer.apply_to_url(test_url, None, None)?;
    
    println!("Original URL:    {}", test_url);
    println!("Transformed URL: {}", transformed_url);
    
    assert!(transformed_url.contains("n=RF_sI3WuCE_8"));
    println!(">>> SUCCESS: URL contains transformed n-token! <<<");
    
    Ok(())
}