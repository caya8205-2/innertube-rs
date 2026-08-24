use std::env;
use innertube_rs::constants::clients;
use innertube_rs::core::session::{Session, SessionOptions};
use innertube_rs::models::video::PlayerResponse;
use serde_json::json;

#[derive(Debug)]
struct ClientTestConfig {
    name: &'static str,
    client_name: &'static str,
    client_version: &'static str,
    client_name_header: &'static str,
    device_category: &'static str,
    user_agent: &'static str,
    android_sdk_version: Option<u32>,
    os_name: &'static str,
    os_version: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let video_id = if args.len() > 1 {
        &args[1]
    } else {
        "e1bCibq2I1g"
    };

    println!("================================================================================");
    println!("  innertube-rs — Multi-Client Diagnostic Tester");
    println!("  Target Video ID: {}", video_id);
    println!("================================================================================\n");

    let configs = vec![
        ClientTestConfig {
            name: "1. iOS Client",
            client_name: clients::IOS_NAME,
            client_version: clients::IOS_VERSION,
            client_name_header: "5",
            device_category: "MOBILE",
            user_agent: clients::IOS_USER_AGENT,
            android_sdk_version: None,
            os_name: "iOS",
            os_version: "16.7.7",
        },
        ClientTestConfig {
            name: "2. Android Client (Standard)",
            client_name: clients::ANDROID_NAME,
            client_version: clients::ANDROID_VERSION,
            client_name_header: "3",
            device_category: "MOBILE",
            user_agent: clients::ANDROID_USER_AGENT,
            android_sdk_version: Some(36),
            os_name: "Android",
            os_version: "16",
        },
        ClientTestConfig {
            name: "3. Android VR Client (Oculus/Meta Quest)",
            client_name: clients::ANDROID_VR_NAME,
            client_version: clients::ANDROID_VR_VERSION,
            client_name_header: "28",
            device_category: "MOBILE",
            user_agent: clients::ANDROID_VR_USER_AGENT,
            android_sdk_version: Some(32),
            os_name: "Android",
            os_version: "12L",
        },
        ClientTestConfig {
            name: "4. MWEB Client (Mobile Web)",
            client_name: clients::MWEB_NAME,
            client_version: clients::MWEB_VERSION,
            client_name_header: "2",
            device_category: "MOBILE",
            user_agent: clients::MWEB_USER_AGENT,
            android_sdk_version: None,
            os_name: "iOS",
            os_version: "16_7_7",
        },
        ClientTestConfig {
            name: "5. WEB Client (Desktop Browser)",
            client_name: clients::WEB_NAME,
            client_version: clients::WEB_VERSION,
            client_name_header: "1",
            device_category: "DESKTOP",
            user_agent: clients::WEB_USER_AGENT,
            android_sdk_version: None,
            os_name: "Windows",
            os_version: "10.0",
        },
    ];

    let yt = innertube_rs::Innertube::new().await?;
    let sig_timestamp = yt.player.decipherer.signature_timestamp;
    let http = reqwest::Client::new();

    for cfg in configs {
        println!("--------------------------------------------------------------------------------");
        println!("Testing: {} [{}]", cfg.name, cfg.client_name);
        println!("--------------------------------------------------------------------------------");

        let session = Session::create(SessionOptions {
            client_name: Some(cfg.client_name.to_string()),
            client_version: Some(cfg.client_version.to_string()),
            device_category: Some(cfg.device_category.to_string()),
            user_agent: Some(cfg.user_agent.to_string()),
            generate_session_locally: Some(true),
            ..Default::default()
        }).await?;

        let mut context = session.context.clone();
        context.client.client_name = cfg.client_name.to_string();
        context.client.client_version = cfg.client_version.to_string();
        context.client.user_agent = cfg.user_agent.to_string();
        context.client.os_name = cfg.os_name.to_string();
        context.client.os_version = cfg.os_version.to_string();
        if let Some(sdk) = cfg.android_sdk_version {
            context.client.android_sdk_version = Some(sdk);
        }

        let payload = json!({
            "context": context,
            "videoId": video_id,
            "contentCheckOk": true,
            "racyCheckOk": true,
            "playbackContext": {
                "contentPlaybackContext": {
                    "html5Preference": "HTML5_PREF_WANTS",
                    "signatureTimestamp": sig_timestamp
                }
            }
        });

        let url = format!("{}/player?key={}", innertube_rs::constants::INNERTUBE_API_BASE_URL, session.api_key);
        let resp = session.http_client
            .post(&url)
            .header("User-Agent", cfg.user_agent)
            .header("X-Youtube-Client-Name", cfg.client_name_header)
            .header("X-Youtube-Client-Version", cfg.client_version)
            .json(&payload)
            .send()
            .await;

        let resp = match resp {
            Ok(r) => r,
            Err(e) => {
                println!("  [FAIL] HTTP request failed: {}", e);
                continue;
            }
        };

        let status = resp.status();
        let pr: Result<PlayerResponse, _> = resp.json().await;

        match pr {
            Ok(player_response) => {
                let playability = &player_response.playability_status;
                println!("  HTTP Status:         {}", status);
                println!("  Playability Status:  {} (Reason: {:?})", playability.status, playability.reason);

                if let Some(ref sd) = player_response.streaming_data {
                    println!("  Progressive Formats: {} found", sd.formats.len());
                    for f in &sd.formats {
                        let has_url = f.url.is_some();
                        let has_cipher = f.signature_cipher.is_some() || f.cipher.is_some();
                        println!("    • itag {:3} | {:>4}p | mime: {:22} | direct_url: {:<5} | cipher: {}",
                            f.itag,
                            f.height.unwrap_or(0),
                            f.mime_type.split(';').next().unwrap_or(""),
                            has_url,
                            has_cipher
                        );
                    }

                    println!("  Adaptive Formats:    {} found", sd.adaptive_formats.len());
                    let mut sample_res = Vec::new();
                    for f in &sd.adaptive_formats {
                        if let Some(h) = f.height {
                            if !sample_res.contains(&h) && (h == 360 || h == 720 || h == 1080) {
                                sample_res.push(h);
                                let has_url = f.url.is_some();
                                let has_cipher = f.signature_cipher.is_some() || f.cipher.is_some();
                                println!("    • itag {:3} | {:>4}p | mime: {:22} | direct_url: {:<5} | cipher: {}",
                                    f.itag,
                                    h,
                                    f.mime_type.split(';').next().unwrap_or(""),
                                    has_url,
                                    has_cipher
                                );

                                // Test CDN Range Chunk 0 and Chunk 6
                                if let Ok(u) = innertube_rs::endpoints::player::resolve_stream_url(f, &yt.player.decipherer) {
                                    let test_chunk0 = format!("{}&range=0-1048575&rn=0", u);
                                    let test_chunk6 = format!("{}&range=6291456-7340031&rn=6", u);

                                    let r0 = http.get(&test_chunk0)
                                        .header("User-Agent", cfg.user_agent)
                                        .send().await
                                        .map(|r| r.status().to_string())
                                        .unwrap_or_else(|e| format!("Err: {e}"));

                                    let r6 = http.get(&test_chunk6)
                                        .header("User-Agent", cfg.user_agent)
                                        .send().await
                                        .map(|r| r.status().to_string())
                                        .unwrap_or_else(|e| format!("Err: {e}"));

                                    println!("      └─ CDN Test [{}p]: Chunk 0 (0-1MB): {} | Chunk 6 (6-7MB): {}", h, r0, r6);
                                }
                            }
                        }
                    }
                } else {
                    println!("  [WARN] No streamingData returned for this client.");
                }
            }
            Err(err) => {
                println!("  [FAIL] Failed to parse player response: {}", err);
            }
        }
        println!();
    }

    println!("================================================================================");
    println!("  Diagnostic Completed!");
    println!("================================================================================");
    Ok(())
}
