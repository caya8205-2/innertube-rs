use innertube_rs::{Innertube, SessionOptions};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== InnerTube Raw Data Diagnostics & Debugger ===\n");

    innertube_rs::log::load_dotenv();

    let cookie = std::env::var("INNERTUBE_COOKIE").ok();
    if cookie.is_some() {
        println!("Using cookie from INNERTUBE_COOKIE (.env or environment)");
    } else {
        println!("No INNERTUBE_COOKIE found (running anonymously).");
    }

    let options = SessionOptions {
        cookie,
        ..Default::default()
    };

    println!("Initializing InnerTube client...");
    let yt = Innertube::with_options(options).await?;

    println!("Fetching raw response for '/browse' (FEwhat_to_watch)...");
    let response = yt
        .actions()
        .execute("/browse", json!({ "browseId": "FEwhat_to_watch" }))
        .await?;

    println!("Status code: {}", response.status_code);
    println!("Success: {}", response.success);

    let raw = &response.data;

    if let Value::Object(map) = raw {
        println!("\n--- Top-level Response Keys ({} keys) ---", map.len());
        for key in map.keys() {
            let val_type = match &map[key] {
                Value::Null => "null",
                Value::Bool(_) => "bool",
                Value::Number(_) => "number",
                Value::String(_) => "string",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
            };
            println!("  • {key}: ({val_type})");
        }
    }

    let dump_path = innertube_rs::log::dump_json("home_feed_raw.json", raw)
        .map_err(|e| format!("Failed to dump JSON: {e}"))?;

    println!("\nFull raw JSON dumped to:");
    println!("   {}", dump_path.display());
    println!(
        "\nYou can open this file in your editor to inspect the exact InnerTube JSON payload."
    );

    Ok(())
}
