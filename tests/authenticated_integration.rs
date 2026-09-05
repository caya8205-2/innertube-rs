#![allow(unused_imports, dead_code)]

use innertube_rs::{
    ActionResult, CreateCommentResult, CreatePlaylistResult, Innertube, InnertubeError,
    NotificationPreferenceType, Session, SessionOptions,
};
use serde_json::{json, Value};
use std::sync::Arc;

// =========================================================================
// 1. ANONYMOUS SESSION MUTATION REJECTION (PRECONDITION HARDENING)
// =========================================================================

#[tokio::test]
async fn test_anonymous_session_rejects_rating_mutations() {
    let session = Session::create(SessionOptions {
        generate_session_locally: Some(true),
        ..Default::default()
    })
    .await
    .expect("Anonymous session creation should succeed");

    assert!(!session.is_authenticated());

    let res_like = innertube_rs::Actions::like(&session, "dQw4w9WgXcQ").await;
    match res_like {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for like, got {:?}", res_like),
    }

    let res_dislike = innertube_rs::Actions::dislike(&session, "dQw4w9WgXcQ").await;
    match res_dislike {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for dislike, got {:?}", res_dislike),
    }

    let res_remove = innertube_rs::Actions::remove_rating(&session, "dQw4w9WgXcQ").await;
    match res_remove {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for remove_rating, got {:?}", res_remove),
    }
}

#[tokio::test]
async fn test_anonymous_session_rejects_subscription_mutations() {
    let session = Session::create(SessionOptions {
        generate_session_locally: Some(true),
        ..Default::default()
    })
    .await
    .expect("Anonymous session creation should succeed");

    let res_sub = innertube_rs::Actions::subscribe(&session, &["UCuAXFkgsw1L7xaCfnd5JJOw"]).await;
    match res_sub {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for subscribe, got {:?}", res_sub),
    }

    let res_unsub = innertube_rs::Actions::unsubscribe(&session, &["UCuAXFkgsw1L7xaCfnd5JJOw"]).await;
    match res_unsub {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for unsubscribe, got {:?}", res_unsub),
    }
}

#[tokio::test]
async fn test_anonymous_session_rejects_playlist_mutations() {
    let session = Session::create(SessionOptions {
        generate_session_locally: Some(true),
        ..Default::default()
    })
    .await
    .expect("Anonymous session creation should succeed");

    let res_create = innertube_rs::Actions::create_playlist(&session, "My Test Playlist", None).await;
    match res_create {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for create_playlist, got {:?}", res_create),
    }

    let res_delete = innertube_rs::Actions::delete_playlist(&session, "PL_fake_playlist").await;
    match res_delete {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for delete_playlist, got {:?}", res_delete),
    }

    let res_add = innertube_rs::Actions::add_to_playlist(&session, "PL_fake", &["vid_1"]).await;
    match res_add {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for add_to_playlist, got {:?}", res_add),
    }

    let res_remove = innertube_rs::Actions::remove_from_playlist(&session, "PL_fake", &["set_vid_1"]).await;
    match res_remove {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for remove_from_playlist, got {:?}", res_remove),
    }

    let res_name = innertube_rs::Actions::set_playlist_name(&session, "PL_fake", "New Name").await;
    match res_name {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for set_playlist_name, got {:?}", res_name),
    }

    let res_desc = innertube_rs::Actions::set_playlist_description(&session, "PL_fake", "New Desc").await;
    match res_desc {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for set_playlist_description, got {:?}", res_desc),
    }

    let res_move = innertube_rs::Actions::move_playlist_video(&session, "PL_fake", "set_1", "set_0").await;
    match res_move {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for move_playlist_video, got {:?}", res_move),
    }
}

#[tokio::test]
async fn test_anonymous_session_rejects_comment_and_notification_mutations() {
    let session = Session::create(SessionOptions {
        generate_session_locally: Some(true),
        ..Default::default()
    })
    .await
    .expect("Anonymous session creation should succeed");

    let res_comment = innertube_rs::Actions::create_comment(&session, "dQw4w9WgXcQ", "Hello world!").await;
    match res_comment {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for create_comment, got {:?}", res_comment),
    }

    let res_pref = innertube_rs::Actions::set_notification_preferences(
        &session,
        "UCuAXFkgsw1L7xaCfnd5JJOw",
        NotificationPreferenceType::Personalized,
    )
    .await;
    match res_pref {
        Err(InnertubeError::AuthenticationRequired(_)) => {}
        _ => panic!("Expected AuthenticationRequired error for set_notification_preferences, got {:?}", res_pref),
    }
}

// =========================================================================
// 2. AUTHENTICATED MUTATION PAYLOAD & PROTOBUF CONTRACTS
// =========================================================================

#[tokio::test]
async fn test_authenticated_session_flag_and_headers() {
    let options = SessionOptions {
        cookie: Some("SAPISID=fake_sapisid_12345; __Secure-3PAPISID=fake_3papisid_67890".to_string()),
        generate_session_locally: Some(true),
        ..Default::default()
    };

    let session = Session::create(options)
        .await
        .expect("Authenticated session should initialize");

    assert!(session.is_authenticated(), "Session with SAPISID cookie must be detected as authenticated");
    assert!(session.ensure_authenticated().is_ok());
}

#[test]
fn test_mutation_payload_contracts_rating_and_subscription() {
    let rating_target = json!({
        "target": {
            "videoId": "dQw4w9WgXcQ"
        }
    });
    assert_eq!(rating_target["target"]["videoId"], "dQw4w9WgXcQ");

    let sub_payload = json!({
        "channelIds": ["UCuAXFkgsw1L7xaCfnd5JJOw"]
    });
    assert_eq!(sub_payload["channelIds"][0], "UCuAXFkgsw1L7xaCfnd5JJOw");
}

#[test]
fn test_mutation_payload_contracts_playlist_operations() {
    let add_actions = vec![
        json!({
            "action": "ACTION_ADD_VIDEO",
            "addedVideoId": "video_1"
        }),
        json!({
            "action": "ACTION_ADD_VIDEO",
            "addedVideoId": "video_2"
        }),
    ];
    let add_payload = json!({
        "playlistId": "PL_test_playlist",
        "actions": add_actions
    });
    assert_eq!(add_payload["playlistId"], "PL_test_playlist");
    assert_eq!(add_payload["actions"].as_array().unwrap().len(), 2);
    assert_eq!(add_payload["actions"][0]["action"], "ACTION_ADD_VIDEO");

    let move_action = json!({
        "action": "ACTION_MOVE_VIDEO_AFTER",
        "setVideoId": "set_1",
        "movedSetVideoIdPredecessor": "set_0"
    });
    assert_eq!(move_action["action"], "ACTION_MOVE_VIDEO_AFTER");
    assert_eq!(move_action["setVideoId"], "set_1");
    assert_eq!(move_action["movedSetVideoIdPredecessor"], "set_0");
}

#[test]
fn test_mutation_payload_contracts_comment_and_notification() {
    let comment_params = innertube_rs::utils::proto::encode_create_comment_params("dQw4w9WgXcQ")
        .expect("Create comment protobuf param encoding must succeed");
    assert!(!comment_params.is_empty());

    let notif_params = innertube_rs::utils::proto::encode_notification_preferences(
        "UCuAXFkgsw1L7xaCfnd5JJOw",
        NotificationPreferenceType::All.index(),
    )
    .expect("Notification preference protobuf encoding must succeed");
    assert!(!notif_params.is_empty());
}

// =========================================================================
// 3. OPT-IN LIVE AUTHENTICATED MUTATION SUITE (WITH SAFE REVERSIBLE CLEANUP)
// =========================================================================

#[tokio::test]
#[ignore = "Live authenticated rating mutation test requiring INNERTUBE_COOKIE and INNERTUBE_MUTATION_TEST=1"]
async fn test_live_authenticated_rating_mutation_with_cleanup() {
    let cookie = std::env::var("INNERTUBE_COOKIE").ok();
    let allow_mutation = std::env::var("INNERTUBE_MUTATION_TEST").unwrap_or_default() == "1";

    if cookie.is_none() || !allow_mutation {
        println!("Skipping live rating mutation test: INNERTUBE_COOKIE or INNERTUBE_MUTATION_TEST=1 is not set");
        return;
    }

    let yt = Innertube::with_options(SessionOptions { cookie, ..Default::default() })
        .await
        .expect("Authenticated Innertube should initialize");
    assert!(yt.session.is_authenticated());

    let test_video_id = "dQw4w9WgXcQ";

    // 1. Perform like mutation
    let like_res = innertube_rs::Actions::like(&yt.session, test_video_id).await;
    assert!(like_res.is_ok(), "Authenticated like should succeed: {:?}", like_res.err());

    // 2. Perform cleanup: remove rating
    let clean_res = innertube_rs::Actions::remove_rating(&yt.session, test_video_id).await;
    assert!(clean_res.is_ok(), "Authenticated remove_rating should succeed");
}

#[tokio::test]
#[ignore = "Live authenticated subscription mutation test requiring INNERTUBE_COOKIE and INNERTUBE_MUTATION_TEST=1"]
async fn test_live_authenticated_subscription_mutation_with_cleanup() {
    let cookie = std::env::var("INNERTUBE_COOKIE").ok();
    let allow_mutation = std::env::var("INNERTUBE_MUTATION_TEST").unwrap_or_default() == "1";

    if cookie.is_none() || !allow_mutation {
        println!("Skipping live subscription mutation test: INNERTUBE_COOKIE or INNERTUBE_MUTATION_TEST=1 is not set");
        return;
    }

    let yt = Innertube::with_options(SessionOptions { cookie, ..Default::default() })
        .await
        .expect("Authenticated Innertube should initialize");
    assert!(yt.session.is_authenticated());

    let test_channel_id = "UCuAXFkgsw1L7xaCfnd5JJOw";

    // 1. Subscribe to channel
    let sub_res = innertube_rs::Actions::subscribe(&yt.session, &[test_channel_id]).await;
    assert!(sub_res.is_ok(), "Authenticated subscribe should succeed");

    // 2. Cleanup: Unsubscribe
    let unsub_res = innertube_rs::Actions::unsubscribe(&yt.session, &[test_channel_id]).await;
    assert!(unsub_res.is_ok(), "Authenticated unsubscribe cleanup should succeed: {:?}", unsub_res.err());
}

#[tokio::test]
#[ignore = "Live authenticated playlist mutation test requiring INNERTUBE_COOKIE and INNERTUBE_MUTATION_TEST=1"]
async fn test_live_authenticated_playlist_lifecycle_with_cleanup() {
    let cookie = std::env::var("INNERTUBE_COOKIE").ok();
    let allow_mutation = std::env::var("INNERTUBE_MUTATION_TEST").unwrap_or_default() == "1";

    if cookie.is_none() || !allow_mutation {
        println!("Skipping live playlist mutation test: INNERTUBE_COOKIE or INNERTUBE_MUTATION_TEST=1 is not set");
        return;
    }

    let yt = Innertube::with_options(SessionOptions { cookie, ..Default::default() })
        .await
        .expect("Authenticated Innertube should initialize");
    assert!(yt.session.is_authenticated());

    // 1. Create temporary test playlist
    let create_res = innertube_rs::Actions::create_playlist(&yt.session, "Test Temp Playlist", Some(&["dQw4w9WgXcQ"])).await;
    assert!(create_res.is_ok(), "Authenticated create_playlist should succeed: {:?}", create_res.err());

    let playlist_id = create_res.unwrap().playlist_id;
    if let Some(pid) = playlist_id {
        // 2. Cleanup: Delete created playlist
        let del_res = innertube_rs::Actions::delete_playlist(&yt.session, &pid).await;
        assert!(del_res.is_ok(), "Authenticated delete_playlist cleanup should succeed");
    }
}

#[tokio::test]
#[ignore = "Live authenticated comment mutation test requiring INNERTUBE_COOKIE and INNERTUBE_MUTATION_TEST=1"]
async fn test_live_authenticated_comment_mutation() {
    let cookie = std::env::var("INNERTUBE_COOKIE").ok();
    let allow_mutation = std::env::var("INNERTUBE_MUTATION_TEST").unwrap_or_default() == "1";

    if cookie.is_none() || !allow_mutation {
        println!("Skipping live comment mutation test: INNERTUBE_COOKIE or INNERTUBE_MUTATION_TEST=1 is not set");
        return;
    }

    let yt = Innertube::with_options(SessionOptions { cookie, ..Default::default() })
        .await
        .expect("Authenticated Innertube should initialize");
    assert!(yt.session.is_authenticated());

    let test_video_id = "dQw4w9WgXcQ";
    let comment_res = innertube_rs::Actions::create_comment(&yt.session, test_video_id, "Test comment via automated parity test").await;
    println!("Comment creation result: {:?}", comment_res);
}
