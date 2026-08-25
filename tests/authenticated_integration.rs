#![allow(unused_imports, dead_code)]

use innertube_rs::{
    ActionResult, CreateCommentResult, CreatePlaylistResult, Innertube, InnertubeError,
    NotificationPreferenceType, Session, SessionOptions,
};
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
// 2. AUTHENTICATED SESSION PAYLOAD FORMATTING CONTRACTS
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
}

// =========================================================================
// 3. OPT-IN LIVE AUTHENTICATED MUTATION TEST
// =========================================================================

#[tokio::test]
#[ignore = "Live authenticated network test requiring INNERTUBE_COOKIE or INNERTUBE_OAUTH_TOKEN"]
async fn test_live_authenticated_account_and_notifications() {
    let cookie = std::env::var("INNERTUBE_COOKIE").ok();
    if cookie.is_none() {
        println!("Skipping live authenticated test: INNERTUBE_COOKIE is not set");
        return;
    }

    let options = SessionOptions {
        cookie,
        ..Default::default()
    };

    let yt = Innertube::with_options(options).await.expect("Authenticated Innertube should initialize");
    assert!(yt.session.is_authenticated());

    let unseen = yt.get_unseen_notifications_count().await;
    println!("Live unseen notifications count: {:?}", unseen);
    assert!(unseen.is_ok());
}
