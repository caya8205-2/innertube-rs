use innertube_rs::Innertube;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing YouTube OAuth2 Device Flow Engine ===");

    let yt = Innertube::new().await?;

    println!("1. Requesting Google OAuth2 TV Device Code...");
    let (client, code) = yt.request_oauth_code().await?;

    println!("\n✅ Successfully generated OAuth2 Device Code!");
    println!("   >> Client ID: {}...", &client.client_id[..client.client_id.len().min(30)]);
    println!("   >> User Code: {}", code.user_code);
    println!("   >> Verification URL: {}", code.verification_url);
    println!("   >> Expires In: {} seconds", code.expires_in);
    println!("   >> Poll Interval: {} seconds", code.interval);
    println!("\nℹ️ To complete login, a user would navigate to {} and enter code [{}]", code.verification_url, code.user_code);

    Ok(())
}
