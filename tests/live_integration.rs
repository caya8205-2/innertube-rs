use innertube_rs::{
    FormatFilter, FormatType, GetVideoInfoOptions, Innertube, MusicSearchFilter,
    PostCommentSort, QualityPreference, SearchFilters, SearchPrioritize,
};

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_video_info_and_stream_url() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let video_id = "dQw4w9WgXcQ";

    // Test get_video_info
    let info = client
        .get_video_info(video_id)
        .await
        .expect("Failed to fetch video info");
    assert_eq!(
        info.video_details.as_ref().map(|v| v.video_id.as_str()),
        Some(video_id)
    );
    let stream_data = info.streaming_data.as_ref().expect("Streaming data missing");
    assert!(!stream_data.formats.is_empty() || !stream_data.adaptive_formats.is_empty());

    // Test get_basic_info
    let basic = client
        .get_basic_info(video_id, Some(&GetVideoInfoOptions::default()))
        .await
        .expect("Failed to fetch basic info");
    assert_eq!(
        basic.player_response.video_details.as_ref().map(|v| v.video_id.as_str()),
        Some(video_id)
    );
    assert!(!basic.cpn.is_empty());

    // Test get_info (concurrent player + watch_next)
    let full_info = client
        .get_info(video_id, None)
        .await
        .expect("Failed to fetch full video info");
    assert_eq!(full_info.id(), Some(video_id));
    assert!(full_info.watch_next.is_some());
    assert!(!full_info.cpn.is_empty());

    // Test get_stream_url
    let filter = FormatFilter {
        format_type: FormatType::AudioOnly,
        quality: QualityPreference::Highest,
        container: None,
    };
    let stream_url = client
        .get_stream_url(video_id, &filter)
        .await
        .expect("Failed to resolve stream URL");
    assert!(stream_url.starts_with("https://"));
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_search_and_filters() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    // Standard search
    let results = client
        .search("Never Gonna Give You Up", None)
        .await
        .expect("Failed to execute search");
    assert!(!results.items.is_empty());

    // Search with filters
    let filters = SearchFilters {
        prioritize: Some(SearchPrioritize::Relevance),
        ..Default::default()
    };
    let filtered_results = client
        .search_with_filters("Never Gonna Give You Up", Some(&filters), None)
        .await
        .expect("Failed to execute filtered search");
    assert!(!filtered_results.items.is_empty());
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_search_suggestions() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let suggestions = client
        .get_search_suggestions("rust programming", false)
        .await
        .expect("Failed to fetch suggestions");
    assert!(!suggestions.suggestions.is_empty());

    let suggestions_with_opt = client
        .get_search_suggestions_with_options("rust lang", Some("rust"), false)
        .await
        .expect("Failed to fetch suggestions with previous query");
    assert!(!suggestions_with_opt.suggestions.is_empty());
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_comments() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let video_id = "dQw4w9WgXcQ";
    let comments = client
        .get_comments_with_options(video_id, Some(PostCommentSort::TopComments), None, None)
        .await
        .expect("Failed to fetch comments");
    assert!(!comments.comments.is_empty());
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_channel_and_about() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let channel = client
        .get_channel("@RickAstleyYT")
        .await
        .expect("Failed to fetch channel");
    assert!(!channel.name.is_empty());
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_music_explore_and_search() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let explore = client
        .get_music_explore()
        .await
        .expect("Failed to fetch music explore");
    assert!(!explore.top_songs.is_empty() || !explore.top_videos.is_empty());

    let music_search = client
        .search_music("Rick Astley", Some(MusicSearchFilter::Songs))
        .await
        .expect("Failed to search YouTube Music");
    assert!(!music_search.songs.is_empty());
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_guide_and_feed() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let guide = client.get_guide().await.expect("Failed to fetch guide");
    assert!(!guide.sections.is_empty());

    let hashtag = client
        .get_hashtag_feed("rust")
        .await
        .expect("Failed to fetch hashtag feed");
    assert_eq!(hashtag.hashtag, "rust");
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_transcript() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let tracks = client
        .get_transcript_tracks("dQw4w9WgXcQ")
        .await
        .expect("Failed to fetch transcript tracks");
    assert!(!tracks.is_empty());
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_resolve_url_and_home_feed() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    let resolved = client
        .resolve_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
        .await
        .expect("Failed to resolve URL");
    assert!(resolved.watch.is_some() || resolved.endpoint_name.is_some());

    let _home = client
        .get_home_feed(None)
        .await
        .expect("Failed to fetch home feed");
}

#[tokio::test]
#[ignore = "requires network access; run with cargo test --test live_integration -- --ignored"]
async fn test_live_music_and_kids_managers() {
    let client = Innertube::new()
        .await
        .expect("Failed to initialize Innertube client");

    // YouTube Music Manager
    let artist = client
        .music()
        .get_artist("UC52ZqHVQz5OoGhvbWiRal6g")
        .await
        .expect("Failed to fetch music artist");
    assert!(!artist.name.is_empty());

    // YouTube Kids Manager
    let kids_home = client
        .kids()
        .get_home_feed()
        .await
        .expect("Failed to fetch kids home feed");
    assert!(kids_home.get("contents").is_some() || kids_home.get("responseContext").is_some());
}
