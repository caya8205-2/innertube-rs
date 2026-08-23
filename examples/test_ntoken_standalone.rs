use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Standalone Signature & N-Token Decipherer ===");
    let yt = Innertube::new().await?;

    let raw_n = "jUB4VWXSlSyPbA";
    let res = yt.player.decipherer.decipher(Some(raw_n), None, None)?;

    println!("Input Raw N-Token:       {}", raw_n);
    println!("Transformed N-Token:     {:?}", res.n);
    println!("Deciphered Signature:    {:?}", res.sig);
    println!("Signature Timestamp:     {}", yt.player.decipherer.signature_timestamp);
    println!("Extracted JS Function:   {}", yt.player.decipherer.nsig_fn_name);

    Ok(())
}
