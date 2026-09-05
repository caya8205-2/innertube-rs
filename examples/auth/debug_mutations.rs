//! Debug authenticated mutation responses: prints raw status + body for
//! like (TV and WEB client), subscribe, and create_playlist.
//! Requires INNERTUBE_COOKIE (and performs real mutations).

use innertube_rs::{Innertube, SessionOptions};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie = std::env::var("INNERTUBE_COOKIE")
        .expect("set INNERTUBE_COOKIE to your browser YouTube cookie");

    let yt = Innertube::with_options(SessionOptions {
        cookie: Some(cookie.clone()),
        ..Default::default()
    })
    .await?;

    // Auth diagnostics: which cookies exist and whether the SID hash applies.
    for name in ["SAPISID", "__Secure-1PAPISID", "__Secure-3PAPISID", "SID"] {
        println!(
            "cookie {name}: {}",
            if innertube_rs::utils::auth::get_cookie(&cookie, name).is_some() {
                "present"
            } else {
                "MISSING"
            }
        );
    }

    // Authenticated read probe: FElibrary must be signed in.
    let probe = yt
        .session
        .post_innertube("/browse", json!({ "browseId": "FElibrary", "skip_auth_check": true }))
        .await;
    match probe {
        Ok(r) => {
            let body = r.text().await.unwrap_or_default();
            let logged_in = body.contains("\"logged_in\",\"value\":\"1\"")
                || body.contains(r#""value":"1""#) && body.contains("logged_in");
            println!("== FElibrary probe -> logged_in flag present: {logged_in}\n");
        }
        Err(e) => println!("== FElibrary probe -> ERROR {e}\n"),
    }

    let video_id = "dQw4w9WgXcQ";
    let channel_id = "UCuAXFkgsw1L7xaCfnd5JJOw";

    for (label, endpoint, client, payload) in [
        (
            "like (TV client)",
            "/like/like",
            Some("TV"),
            json!({ "target": video_id }),
        ),
        (
            "like (WEB client)",
            "/like/like",
            None,
            json!({ "target": video_id }),
        ),
        (
            "like (object target)",
            "/like/like",
            None,
            json!({ "target": { "videoId": video_id } }),
        ),
        (
            "subscribe",
            "/subscription/subscribe",
            None,
            json!({ "channelIds": [channel_id], "params": "EgIIAhgA" }),
        ),
        (
            "create_playlist",
            "/playlist/create",
            None,
            json!({ "title": "zz debug playlist", "videoIds": [video_id] }),
        ),
    ] {
        let resp = match client {
            Some(c) => yt.session.post_innertube_client(c, endpoint, payload).await,
            None => yt.session.post_innertube(endpoint, payload).await,
        };

        match resp {
            Ok(r) => {
                let status = r.status();
                let body = r.text().await.unwrap_or_default();
                println!("== {label} -> HTTP {status}\n{}\n", &body.chars().take(600).collect::<String>());
            }
            Err(e) => println!("== {label} -> ERROR {e}\n"),
        }
    }

    Ok(())
}
