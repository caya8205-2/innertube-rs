use innertube_rs::core::player::Player;
use innertube_rs::core::session::{Session, SessionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing innertube-rs Session & Player ===");

    // 1. Test Session Creation
    println!("\n1. Bootstrapping Session from YouTube sw.js_data...");
    let session = Session::create(SessionOptions::default()).await?;
    println!("   >> Session initialized successfully!");
    println!("   >> Client Name: {}", session.context.client.client_name);
    println!("   >> Client Version: {}", session.context.client.client_version);
    println!("   >> Visitor Data: {}", session.context.client.visitor_data.as_deref().unwrap_or("None"));
    println!("   >> API Key: {}", session.api_key);

    // 2. Test Player & Decipher Engine
    println!("\n2. Fetching YouTube Player and extracting Decipher Engine...");
    let player = Player::create(&session.http_client, None).await?;
    println!("   >> Player ID: {}", player.player_id);
    println!("   >> Signature Timestamp (STS): {}", player.decipherer.signature_timestamp);

    // 3. Test Decipher execution in QuickJS
    println!("\n3. Testing Decipher transform execution in QuickJS sandbox...");
    let test_n = "d2kH_aBC123xyz";
    let test_sig = "abcdef1234567890abcdef1234567890abcdef12";
    let result = player.decipherer.decipher(Some(test_n), Some("sig"), Some(test_sig))?;
    println!("   >> Transformed n-token: {:?}", result.n);
    println!("   >> Deciphered signature: {:?}", result.sig);

    println!("\n=== ALL CHECKS PASSED SUCCESSFULLY! ===");
    Ok(())
}
