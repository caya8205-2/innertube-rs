#![allow(unused_imports, dead_code)]

use innertube_rs::{
    endpoints, models, ActionResult, ApiResponse, ChannelAbout, ChannelVideosResponse,
    CommentThread, CommentsResult, CreateCommentResult, CreatePlaylistResult, DownloadOptions,
    DownloadRange, FormatFilter, FormatOptions, FormatType, GetVideoInfoOptions, GuideResponse,
    HashtagFeed, HistoryFeed, HomeFeed, Innertube, LibraryFeed, LiveChatMessage, LiveChatResponse,
    MusicAlbumView, MusicArtistPage, MusicExplore, MusicHomeFeed, MusicLikeStatus, MusicLyrics,
    MusicSearchFilter, MusicSearchResults, NavigationEndpointNode, NodeListExt,
    NotificationPreferenceType, Parser,
    PlaylistContinuation, PlaylistNode, PlaylistPanelNode, PlaylistPanelVideoNode,
    PlaylistVideoItem, PlaylistVideoNode, PlaylistView, PostCommentSort, PostNode,
    QualityPreference, ReelShelfNode, SearchFilters, SearchPrioritize, SearchResultItem,
    SearchResults, SearchSuggestionsResult, SearchTypeFilter, ShortNode, StreamingData,
    ThumbnailOverlayProgressBarNode, ThumbnailOverlayTimeStatusNode, ToggleButtonNode,
    Transcript, TranscriptSegment, TranscriptTrack, UploadDateFilter, VideoInfo, VideoNode,
    VideoPrimaryInfoNode, VideoSecondaryInfoNode, YTNode,
};
use serde_json::{json, Value};

// =========================================================================
// 1. VIDEO METADATA CONTRACTS (getInfo, getBasicInfo, getShortsVideoInfo)
// =========================================================================

#[test]
fn test_api_contract_01_get_info_composition() {
    let raw_player = json!({
        "playabilityStatus": {
            "status": "OK"
        },
        "videoDetails": {
            "videoId": "dQw4w9WgXcQ",
            "title": "Never Gonna Give You Up",
            "lengthSeconds": "213",
            "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
            "author": "Rick Astley"
        },
        "streamingData": {
            "formats": [
                {
                    "itag": 18,
                    "mimeType": "video/mp4; codecs=\"avc1.42001E, mp4a.40.2\"",
                    "bitrate": 500000,
                    "width": 640,
                    "height": 360,
                    "url": "https://googlevideo.com/videoplayback?id=18"
                }
            ],
            "adaptiveFormats": []
        }
    });

    let player_response: models::video::PlayerResponse =
        serde_json::from_value(raw_player).expect("PlayerResponse should deserialize");
    assert_eq!(player_response.video_details.as_ref().unwrap().video_id, "dQw4w9WgXcQ");

    let video_info = VideoInfo {
    player_response,
    watch_next: None,
    cpn: "CPN_RANDOM_12345".to_string(),
    po_token: None,
};

    assert_eq!(video_info.cpn.len(), 16);
    assert_eq!(video_info.title(), Some("Never Gonna Give You Up"));
    assert_eq!(video_info.author(), Some("Rick Astley"));
    assert_eq!(video_info.duration_seconds(), Some(213));
}

#[test]
fn test_api_contract_02_get_basic_info_options() {
    let options = GetVideoInfoOptions {
        client: Some("ANDROID".to_string()),
        po_token: Some("POTOKEN_TEST_XYZ".to_string()),
        playback_context: Some(json!({ "contentPlaybackContext": { "lactMilliseconds": "100" } })),
    };

    assert_eq!(options.client.as_deref(), Some("ANDROID"));
    assert_eq!(options.po_token.as_deref(), Some("POTOKEN_TEST_XYZ"));
    assert!(options.playback_context.is_some());
}

#[test]
fn test_api_contract_03_get_shorts_video_info_contract() {
    let reel_response = json!({
        "sequenceParams": "SEQ_PARAMS_ABC",
        "entries": [
            {
                "command": {
                    "reelWatchEndpoint": {
                        "videoId": "short_123",
                        "playerParams": "PARAM_XYZ"
                    }
                }
            }
        ]
    });

    let seq_params = reel_response.get("sequenceParams").and_then(Value::as_str);
    assert_eq!(seq_params, Some("SEQ_PARAMS_ABC"));
}

// =========================================================================
// 2. SEARCH & SUGGESTIONS CONTRACTS (search, getSearchSuggestions)
// =========================================================================

#[test]
fn test_api_contract_04_search_with_filters_payload() {
    let filters = SearchFilters {
        prioritize: Some(SearchPrioritize::Popularity),
        upload_date: Some(UploadDateFilter::Today),
        search_type: Some(SearchTypeFilter::Video),
        duration: None,
        features: vec![],
    };

    let params = innertube_rs::utils::proto::encode_search_filters(&filters)
        .expect("Protobuf search filter encoding must succeed");
    assert!(!params.is_empty());
}

#[test]
fn test_api_contract_05_search_suggestions_contract() {
    let raw_suggest = json!([
        "rust programming",
        [
            ["rust programming language", 0],
            ["rust tutorial", 0],
            ["rust beginner guide", 0]
        ]
    ]);

    let suggestions: Vec<String> = raw_suggest
        .get(1)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.get(0).and_then(Value::as_str).map(ToString::to_string))
                .collect()
        })
        .unwrap_or_default();

    assert_eq!(suggestions.len(), 3);
    assert_eq!(suggestions[0], "rust programming language");
}

// =========================================================================
// 3. COMMENTS CONTRACTS (getComments, getCommentReplies)
// =========================================================================

#[test]
fn test_api_contract_06_get_comments_proto_params() {
    let sort = PostCommentSort::NewestFirst;
    assert_eq!(sort.proto_value(), 1);

    let top_sort = PostCommentSort::TopComments;
    assert_eq!(top_sort.proto_value(), 0);
}

#[test]
fn test_api_contract_07_get_comment_replies_contract() {
    let reply_fixture = json!({
        "responseContext": {},
        "continuationContents": {
            "commentRepliesContinuation": {
                "contents": [
                    {
                        "commentRenderer": {
                            "commentId": "child_reply_1",
                            "authorText": { "simpleText": "Replier" },
                            "contentText": { "runs": [{ "text": "I agree!" }] }
                        }
                    }
                ]
            }
        }
    });

    let tree = Parser::parse_tree(&reply_fixture);
    let comment = tree.iter().find_map(|n| match n {
        YTNode::Comment(c) => Some(c),
        _ => None,
    }).expect("Expected YTNode::Comment");
    assert_eq!(comment.comment_id, "child_reply_1");
    assert_eq!(comment.author_name, "Replier");
    assert_eq!(comment.text, "I agree!");
}

// =========================================================================
// 4. FEEDS CONTRACTS (Home, Guide, History, Library, Notifications)
// =========================================================================

#[test]
fn test_api_contract_08_home_feed_chips_and_grid() {
    let home_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "content": {
                                "richGridRenderer": {
                                    "header": {
                                        "feedFilterChipBarRenderer": {
                                            "chips": [
                                                {
                                                    "chipCloudChipRenderer": {
                                                        "text": { "runs": [{ "text": "All" }] },
                                                        "isSelected": true
                                                    }
                                                }
                                            ]
                                        }
                                    },
                                    "contents": []
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let parsed = endpoints::feed::parse_home_feed_response(&home_payload)
        .expect("HomeFeed parse must succeed");
    assert_eq!(parsed.filter_chips.len(), 1);
    assert_eq!(parsed.filter_chips[0].text, "All");
    assert!(parsed.filter_chips[0].is_selected);
}

#[test]
fn test_api_contract_09_guide_response_contract() {
    let guide_payload = json!({
        "items": [
            {
                "guideSectionRenderer": {
                    "title": { "simpleText": "Main" },
                    "items": [
                        {
                            "guideEntryRenderer": {
                                "formattedTitle": { "simpleText": "Home" },
                                "navigationEndpoint": {
                                    "browseEndpoint": { "browseId": "FEwhat_to_watch" }
                                }
                            }
                        }
                    ]
                }
            }
        ]
    });

    let resp = endpoints::guide::parse_guide_response(&guide_payload)
        .expect("Guide response should parse");
    assert_eq!(resp.sections.len(), 1);
    assert_eq!(resp.sections[0].title.as_deref(), Some("Main"));
    assert_eq!(resp.sections[0].items[0].title, "Home");
}

#[test]
fn test_api_contract_10_history_feed_contract() {
    let history_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [
                                        {
                                            "itemSectionRenderer": {
                                                "contents": [
                                                    {
                                                        "videoRenderer": {
                                                            "videoId": "hist_vid_1",
                                                            "title": { "runs": [{ "text": "Watched Video" }] }
                                                        }
                                                    }
                                                ]
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let tree = Parser::parse_tree(&history_payload);
    let videos = tree.find_videos();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].id, "hist_vid_1");
}

#[test]
fn test_api_contract_11_library_feed_contract() {
    let lib_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [
                                        {
                                            "shelfRenderer": {
                                                "title": { "simpleText": "History" },
                                                "content": {
                                                    "horizontalListRenderer": {
                                                        "items": [
                                                            {
                                                                "videoRenderer": {
                                                                    "videoId": "lib_vid_1",
                                                                    "title": { "runs": [{ "text": "Lib Video" }] }
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let tree = Parser::parse_tree(&lib_payload);
    let videos = tree.find_videos();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].id, "lib_vid_1");
}

#[test]
fn test_api_contract_12_notifications_menu_contract() {
    let notif_payload = json!({
        "actions": [
            {
                "openPopupAction": {
                    "popup": {
                        "multiPageMenuRenderer": {
                            "sections": [
                                {
                                    "multiPageMenuNotificationSectionRenderer": {
                                        "items": [
                                            {
                                                "notificationRenderer": {
                                                    "notificationId": "notif_1",
                                                    "shortMessage": { "runs": [{ "text": "New video uploaded!" }] },
                                                    "read": false
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        ]
    });

    let notifs = notif_payload.pointer("/actions/0/openPopupAction/popup/multiPageMenuRenderer/sections/0/multiPageMenuNotificationSectionRenderer/items")
        .and_then(Value::as_array)
        .expect("Notification items array should exist");
    assert_eq!(notifs.len(), 1);
}

#[test]
fn test_api_contract_13_unseen_notifications_count_contract() {
    let payload = json!({ "unseenCount": 7 });
    let count = endpoints::account::parse_unseen_notifications_count(&payload);
    assert_eq!(count, 7);
}

// =========================================================================
// 5. CHANNEL, PLAYLIST & HASHTAG CONTRACTS
// =========================================================================

#[test]
fn test_api_contract_14_channel_about_and_tabs_contract() {
    let about = ChannelAbout {
        channel_id: "UC_channel_123".to_string(),
        title: "Test Channel".to_string(),
        description: Some("Channel description".to_string()),
        subscriber_count: Some("1.2M subscribers".to_string()),
        video_count: Some("300 videos".to_string()),
        view_count: Some("50,000,000 views".to_string()),
        joined_date: Some("Joined Jan 1, 2020".to_string()),
        country: Some("United States".to_string()),
        custom_url: Some("@testchannel".to_string()),
        avatar: None,
        banner: None,
    };

    assert_eq!(about.channel_id, "UC_channel_123");
    assert_eq!(about.custom_url.as_deref(), Some("@testchannel"));
}

#[test]
fn test_api_contract_15_playlist_and_continuation_contract() {
    let playlist = PlaylistView {
        id: "PL_test_123".to_string(),
        title: "Test Playlist".to_string(),
        author: Some("Curator".to_string()),
        author_id: Some("UC_curator".to_string()),
        description: Some("Awesome music".to_string()),
        video_count: Some(25),
        view_count: Some("1,000 views".to_string()),
        last_updated: Some("Updated yesterday".to_string()),
        thumbnail: None,
        videos: vec![PlaylistVideoItem {
            id: "vid_1".to_string(),
            title: "Track 1".to_string(),
            author: "Artist 1".to_string(),
            author_id: None,
            duration: Some("3:30".to_string()),
            duration_ms: Some(210000),
            thumbnail: None,
            index: Some(1),
            is_playable: true,
            set_video_id: Some("set_vid_1".to_string()),
        }],
        continuation_token: Some("token_playlist_next".to_string()),
        is_editable: true,
    };

    assert!(playlist.has_continuation());
    assert_eq!(playlist.videos.len(), 1);
}

#[test]
fn test_api_contract_16_hashtag_feed_contract() {
    let hashtag_param = innertube_rs::utils::proto::encode_hashtag_params("rustlang")
        .expect("Hashtag proto param encoding must succeed");
    assert!(!hashtag_param.is_empty());
}

// =========================================================================
// 6. BROWSE DESTINATIONS (Courses, Subscriptions, Channels, Playlists)
// =========================================================================

#[test]
fn test_api_contract_17_courses_destination_contract() {
    let browse_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "title": "Courses",
                            "selected": true,
                            "content": {
                                "sectionListRenderer": {
                                    "contents": []
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let feed = endpoints::feed::parse_browse_feed_response("FEcourses_destination", &browse_payload)
        .expect("Courses feed parse should succeed");
    assert_eq!(feed.browse_id, "FEcourses_destination");
}

#[test]
fn test_api_contract_18_subscriptions_feed_contract() {
    let browse_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "title": "Subscriptions",
                            "selected": true,
                            "content": {
                                "sectionListRenderer": {
                                    "contents": []
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let feed = endpoints::feed::parse_browse_feed_response("FEsubscriptions", &browse_payload)
        .expect("Subscriptions feed parse should succeed");
    assert_eq!(feed.browse_id, "FEsubscriptions");
}

#[test]
fn test_api_contract_19_channels_feed_contract() {
    let browse_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "title": "Channels",
                            "selected": true,
                            "content": {
                                "sectionListRenderer": {
                                    "contents": []
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let feed = endpoints::feed::parse_browse_feed_response("FEchannels", &browse_payload)
        .expect("Channels feed parse should succeed");
    assert_eq!(feed.browse_id, "FEchannels");
}

#[test]
fn test_api_contract_20_playlists_aggregation_contract() {
    let browse_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "title": "Playlists",
                            "selected": true,
                            "content": {
                                "sectionListRenderer": {
                                    "contents": []
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let feed = endpoints::feed::parse_browse_feed_response("FEplaylist_aggregation", &browse_payload)
        .expect("Playlists aggregation parse should succeed");
    assert_eq!(feed.browse_id, "FEplaylist_aggregation");
}

// =========================================================================
// 7. STREAMING & DOWNLOAD OPTIONS CONTRACTS
// =========================================================================

#[test]
fn test_api_contract_21_streaming_format_options() {
    let opt = FormatOptions {
        itag: Some(140),
        format_type: Some(FormatType::AudioOnly),
        quality: Some("hd1080".to_string()),
        client: None,
        po_token: None,
        format: Some("mp4".to_string()),
        codec: Some("opus".to_string()),
        language: None,
    };

    assert_eq!(opt.itag, Some(140));
    assert_eq!(opt.format_type, Some(FormatType::AudioOnly));
    assert_eq!(opt.codec.as_deref(), Some("opus"));
}

#[test]
fn test_api_contract_22_download_range_options() {
    let range = DownloadRange {
        start: 0,
        end: 1048576,
    };
    assert_eq!(range.start, 0);
    assert_eq!(range.end, 1048576);

    let dl_opt = DownloadOptions {
        format_options: FormatOptions::default(),
        range: Some(range),
    };
    assert!(dl_opt.range.is_some());
}

// =========================================================================
// 8. RESOLVE URL, COMMUNITY POST, AND ATTESTATION
// =========================================================================

#[test]
fn test_api_contract_23_resolve_url_endpoint_mapping() {
    let nav_endpoint = NavigationEndpointNode {
        endpoint_name: Some("watchEndpoint".to_string()),
        payload: json!({ "videoId": "dQw4w9WgXcQ" }),
        ..Default::default()
    };

    assert_eq!(nav_endpoint.endpoint_name.as_deref(), Some("watchEndpoint"));
    assert_eq!(nav_endpoint.payload["videoId"], "dQw4w9WgXcQ");
}

#[test]
fn test_api_contract_24_get_post_and_comments_contract() {
    let post_payload = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [
                                        {
                                            "itemSectionRenderer": {
                                                "contents": [
                                                    {
                                                        "postRenderer": {
                                                            "postId": "post_community_1",
                                                            "contentText": { "runs": [{ "text": "Live stream at 8pm!" }] }
                                                        }
                                                    }
                                                ]
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let tree = Parser::parse_tree(&post_payload);
    let posts = tree.find_posts();
    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].id, "post_community_1");
    assert_eq!(posts[0].text(), "Live stream at 8pm!");
}

#[test]
fn test_api_contract_25_attestation_challenge_contract() {
    let payload = endpoints::attestation::build_attestation_payload(
        "ENGAGEMENT_TYPE_SIGNIN",
        Some(json!([{"key": "val"}])),
    );
    assert_eq!(payload["engagementType"], "ENGAGEMENT_TYPE_SIGNIN");
    assert_eq!(payload["ids"][0]["key"], "val");
}

#[test]
fn test_api_contract_26_music_album_search_preserves_all_artists() {
    let raw = json!({
        "contents": [{
            "musicResponsiveListItemRenderer": {
                "flexColumns": [
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "Collaborative Album" }] }
                        }
                    },
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [
                                {
                                    "text": "Artist One",
                                    "navigationEndpoint": {
                                        "browseEndpoint": { "browseId": "UC_artist_one" }
                                    }
                                },
                                { "text": " • " },
                                {
                                    "text": "Artist Two",
                                    "navigationEndpoint": {
                                        "browseEndpoint": { "browseId": "UC_artist_two" }
                                    }
                                }
                            ] }
                        }
                    }
                ]
            }
        }]
    });

    let parsed = endpoints::music::parse_music_search_response(
        "collaborative album",
        Some(MusicSearchFilter::Albums),
        &raw,
    )
    .expect("multi-artist album search fixture should parse");

    assert_eq!(parsed.albums.len(), 1);
    let album = &parsed.albums[0];
    assert_eq!(album.title, "Collaborative Album");
    assert_eq!(album.artist.as_deref(), Some("Artist One"));
    assert_eq!(album.artists.len(), 2);
    assert_eq!(album.artists[0].name, "Artist One");
    assert_eq!(album.artists[0].browse_id.as_deref(), Some("UC_artist_one"));
    assert_eq!(album.artists[1].name, "Artist Two");
    assert_eq!(album.artists[1].browse_id.as_deref(), Some("UC_artist_two"));
}

#[test]
fn test_api_contract_27_music_album_responsive_header_metadata() {
    let raw = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [
                                    {
                                        "musicResponsiveHeaderRenderer": {
                                            "title": { "runs": [{ "text": "Fixture Album" }] },
                                            "subtitle": { "runs": [
                                                { "text": "EP" },
                                                { "text": " • " },
                                                { "text": "2025" }
                                            ] },
                                            "straplineTextOne": { "runs": [{
                                                "text": "Fixture Artist",
                                                "navigationEndpoint": {
                                                    "browseEndpoint": { "browseId": "UC_fixture_artist" }
                                                }
                                            }] },
                                            "secondSubtitle": { "runs": [
                                                { "text": "1 song" },
                                                { "text": " • " },
                                                { "text": "4 minutes" }
                                            ] },
                                            "thumbnail": {
                                                "musicThumbnailRenderer": {
                                                    "thumbnail": {
                                                        "thumbnails": [
                                                            { "url": "https://example.test/small.jpg", "width": 120, "height": 120 },
                                                            { "url": "https://example.test/large.jpg", "width": 544, "height": 544 }
                                                        ]
                                                    }
                                                }
                                            },
                                            "buttons": [{
                                                "musicPlayButtonRenderer": {
                                                    "playNavigationEndpoint": {
                                                        "watchEndpoint": {
                                                            "videoId": "fixture-track",
                                                            "playlistId": "OLAK5uy_fixture_album"
                                                        }
                                                    }
                                                }
                                            }],
                                            "subtitleBadge": [{
                                                "musicInlineBadgeRenderer": {
                                                    "icon": { "iconType": "MUSIC_EXPLICIT_BADGE" }
                                                }
                                            }]
                                        }
                                    },
                                    {
                                        "musicShelfRenderer": {
                                            "contents": [{
                                                "musicResponsiveListItemRenderer": {
                                                    "playlistItemData": { "videoId": "fixture-track" },
                                                    "flexColumns": [
                                                        {
                                                            "musicResponsiveListItemFlexColumnRenderer": {
                                                                "text": { "runs": [{ "text": "Fixture Track" }] }
                                                            }
                                                        },
                                                        {
                                                            "musicResponsiveListItemFlexColumnRenderer": {
                                                                "text": { "runs": [
                                                                    {
                                                                        "text": "Fixture Artist",
                                                                        "navigationEndpoint": {
                                                                            "browseEndpoint": { "browseId": "UC_fixture_artist" }
                                                                        }
                                                                    },
                                                                    { "text": " • " },
                                                                    { "text": "4:12" }
                                                                ] }
                                                            }
                                                        }
                                                    ]
                                                }
                                            }]
                                        }
                                    }
                                ]
                            }
                        }
                    }
                }]
            }
        }
    });

    let album = endpoints::music::parse_music_album_response("MPREb_fixture", &raw)
        .expect("responsive Music album fixture should parse");
    assert_eq!(album.title, "Fixture Album");
    assert_eq!(album.album_type.as_deref(), Some("EP"));
    assert_eq!(album.year.as_deref(), Some("2025"));
    assert_eq!(album.artist.as_deref(), Some("Fixture Artist"));
    assert_eq!(album.artists.len(), 1);
    assert_eq!(album.artists[0].browse_id.as_deref(), Some("UC_fixture_artist"));
    assert_eq!(album.track_count, Some(1));
    assert_eq!(album.duration.as_deref(), Some("4 minutes"));
    assert_eq!(album.duration_ms, Some(252_000));
    assert_eq!(album.audio_playlist_id.as_deref(), Some("OLAK5uy_fixture_album"));
    assert_eq!(album.thumbnail.as_deref(), Some("https://example.test/large.jpg"));
    assert!(album.is_explicit);
    assert_eq!(album.tracks.len(), 1);
    assert_eq!(album.tracks[0].video_id, "fixture-track");
}

#[test]
fn test_api_contract_28_music_artist_details_and_mpla_normalization() {
    let raw = json!({
        "header": {
            "musicImmersiveHeaderRenderer": {
                "title": { "runs": [{ "text": "Fixture Artist" }] },
                "subscriptionButton": {
                    "subscribeButtonRenderer": {
                        "channelId": "UC_subscription_fixture",
                        "subscriberCountText": { "runs": [{ "text": "12K subscribers" }] },
                        "subscribed": true
                    }
                },
                "monthlyListenerCount": { "runs": [{ "text": "2.5M monthly audience" }] },
                "playButton": {
                    "buttonRenderer": {
                        "navigationEndpoint": {
                            "watchEndpoint": { "playlistId": "RDA_fixture_shuffle" }
                        }
                    }
                },
                "startRadioButton": {
                    "buttonRenderer": {
                        "navigationEndpoint": {
                            "watchEndpoint": { "playlistId": "RDEM_fixture_radio" }
                        }
                    }
                },
                "thumbnail": {
                    "musicThumbnailRenderer": {
                        "thumbnail": {
                            "thumbnails": [
                                { "url": "https://example.test/artist-small.jpg", "width": 120, "height": 120 },
                                { "url": "https://example.test/artist-large.jpg", "width": 544, "height": 544 }
                            ]
                        }
                    }
                }
            }
        },
        "contents": {
            "singleColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [
                                    {
                                        "musicShelfRenderer": {
                                            "title": {
                                                "runs": [{
                                                    "text": "Top songs",
                                                    "navigationEndpoint": {
                                                        "browseEndpoint": { "browseId": "VL_top_songs_fixture" }
                                                    }
                                                }]
                                            },
                                            "contents": [{
                                                "musicResponsiveListItemRenderer": {
                                                    "playlistItemData": { "videoId": "artist-track-1" },
                                                    "flexColumns": [
                                                        {
                                                            "musicResponsiveListItemFlexColumnRenderer": {
                                                                "text": { "runs": [{ "text": "Artist Track" }] }
                                                            }
                                                        },
                                                        {
                                                            "musicResponsiveListItemFlexColumnRenderer": {
                                                                "text": { "runs": [
                                                                    {
                                                                        "text": "Fixture Artist",
                                                                        "navigationEndpoint": {
                                                                            "browseEndpoint": { "browseId": "UC_fixture_artist" }
                                                                        }
                                                                    },
                                                                    { "text": " • " },
                                                                    { "text": "3:05" }
                                                                ] }
                                                            }
                                                        }
                                                    ]
                                                }
                                            }]
                                        }
                                    },
                                    {
                                        "musicCarouselShelfRenderer": {
                                            "header": {
                                                "musicCarouselShelfBasicHeaderRenderer": {
                                                    "title": { "runs": [{ "text": "Albums" }] }
                                                }
                                            },
                                            "contents": [{
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Fixture Album" }] },
                                                    "subtitle": { "runs": [{ "text": "2026" }] },
                                                    "navigationEndpoint": {
                                                        "browseEndpoint": { "browseId": "MPREb_fixture_album" }
                                                    }
                                                }
                                            }]
                                        }
                                    },
                                    {
                                        "musicCarouselShelfRenderer": {
                                            "header": {
                                                "musicCarouselShelfBasicHeaderRenderer": {
                                                    "title": { "runs": [{ "text": "Singles & EPs" }] }
                                                }
                                            },
                                            "contents": [{
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Fixture Single" }] },
                                                    "subtitle": { "runs": [
                                                        { "text": "Single" },
                                                        { "text": " • " },
                                                        { "text": "2025" }
                                                    ] },
                                                    "navigationEndpoint": {
                                                        "browseEndpoint": { "browseId": "MPREb_fixture_single" }
                                                    }
                                                }
                                            }]
                                        }
                                    },
                                    {
                                        "musicCarouselShelfRenderer": {
                                            "header": {
                                                "musicCarouselShelfBasicHeaderRenderer": {
                                                    "title": { "runs": [{ "text": "Videos" }] }
                                                }
                                            },
                                            "contents": [{
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Fixture Video" }] },
                                                    "subtitle": { "runs": [
                                                        { "text": "Fixture Artist" },
                                                        { "text": " • " },
                                                        { "text": "1.2M views" }
                                                    ] },
                                                    "navigationEndpoint": {
                                                        "watchEndpoint": {
                                                            "videoId": "artist-video-1",
                                                            "playlistId": "PL_fixture_artist_videos"
                                                        }
                                                    },
                                                    "thumbnailRenderer": {
                                                        "musicThumbnailRenderer": {
                                                            "thumbnail": {
                                                                "thumbnails": [
                                                                    { "url": "https://example.test/video-small.jpg", "width": 120, "height": 68 },
                                                                    { "url": "https://example.test/video-large.jpg", "width": 544, "height": 306 }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }
                                            }]
                                        }
                                    },
                                    {
                                        "musicDescriptionShelfRenderer": {
                                            "description": { "runs": [{ "text": "Fixture biography" }] },
                                            "subheader": { "runs": [{ "text": "123M views" }] }
                                        }
                                    }
                                ]
                            }
                        }
                    }
                }]
            }
        }
    });

    let artist = endpoints::music::parse_music_artist_response("MPLAUC_fixture_artist", &raw)
        .expect("Music artist fixture should parse");
    assert_eq!(artist.id, "UC_fixture_artist");
    assert_eq!(artist.channel_id.as_deref(), Some("UC_subscription_fixture"));
    assert_eq!(artist.name, "Fixture Artist");
    assert_eq!(artist.description.as_deref(), Some("Fixture biography"));
    assert_eq!(artist.views.as_deref(), Some("123M views"));
    assert_eq!(artist.subscribers.as_deref(), Some("12K subscribers"));
    assert_eq!(artist.monthly_listeners.as_deref(), Some("2.5M"));
    assert!(artist.subscribed);
    assert_eq!(artist.shuffle_id.as_deref(), Some("RDA_fixture_shuffle"));
    assert_eq!(artist.radio_id.as_deref(), Some("RDEM_fixture_radio"));
    assert_eq!(artist.thumbnail.as_deref(), Some("https://example.test/artist-large.jpg"));
    assert_eq!(artist.top_songs.len(), 1);
    assert_eq!(artist.top_songs[0].video_id, "artist-track-1");
    assert_eq!(artist.top_songs[0].duration_ms, Some(185_000));
    assert_eq!(artist.albums.len(), 1);
    assert_eq!(artist.albums[0].year.as_deref(), Some("2026"));
    assert_eq!(artist.singles.len(), 1);
    assert_eq!(artist.singles[0].year.as_deref(), Some("2025"));
    assert_eq!(artist.videos.len(), 1);
    assert_eq!(artist.videos[0].video_id, "artist-video-1");
    assert_eq!(artist.videos[0].title, "Fixture Video");
    assert_eq!(artist.videos[0].thumbnail.as_deref(), Some("https://example.test/video-large.jpg"));
}

#[test]
fn test_api_contract_29_music_playlist_card_track_counts() {
    let playlist_endpoint = |browse_id: &str| {
        json!({
            "browseEndpoint": {
                "browseId": browse_id,
                "browseEndpointContextSupportedConfigs": {
                    "browseEndpointContextMusicConfig": {
                        "pageType": "MUSIC_PAGE_TYPE_PLAYLIST"
                    }
                }
            }
        })
    };
    let raw = json!({
        "contents": {
            "singleColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [{
                                    "musicCarouselShelfRenderer": {
                                        "header": {
                                            "musicCarouselShelfBasicHeaderRenderer": {
                                                "title": { "runs": [{ "text": "Fixture playlists" }] }
                                            }
                                        },
                                        "contents": [
                                            {
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Counted Playlist" }] },
                                                    "subtitle": { "runs": [
                                                        {
                                                            "text": "Fixture Listener",
                                                            "navigationEndpoint": {
                                                                "browseEndpoint": { "browseId": "UC_fixture_listener" }
                                                            }
                                                        },
                                                        { "text": " • " },
                                                        { "text": "123 songs" }
                                                    ] },
                                                    "navigationEndpoint": playlist_endpoint("VLfixture-counted")
                                                }
                                            },
                                            {
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "Made For Playlist" }] },
                                                    "subtitle": { "runs": [
                                                        { "text": "Made for " },
                                                        {
                                                            "text": "Fixture Listener",
                                                            "navigationEndpoint": {
                                                                "browseEndpoint": { "browseId": "UC_fixture_listener" }
                                                            }
                                                        },
                                                        { "text": " • " },
                                                        { "text": "100 songs" }
                                                    ] },
                                                    "navigationEndpoint": playlist_endpoint("VLfixture-made-for")
                                                }
                                            },
                                            {
                                                "musicTwoRowItemRenderer": {
                                                    "title": { "runs": [{ "text": "System Playlist" }] },
                                                    "subtitle": { "runs": [{ "text": "Auto playlist" }] },
                                                    "navigationEndpoint": playlist_endpoint("VLfixture-system")
                                                }
                                            }
                                        ]
                                    }
                                }]
                            }
                        }
                    }
                }]
            }
        }
    });

    let feed = endpoints::music::parse_music_home_response(&raw)
        .expect("Music playlist-card count fixture should parse");
    assert_eq!(feed.shelves.len(), 1);
    assert_eq!(feed.shelves[0].playlists.len(), 3);
    assert_eq!(feed.shelves[0].playlists[0].track_count, Some(123));
    assert_eq!(feed.shelves[0].playlists[1].track_count, None);
    assert_eq!(feed.shelves[0].playlists[2].track_count, None);
}

#[test]
fn test_api_contract_30_music_track_like_status() {
    let raw = json!({
        "contents": [
            {
                "musicResponsiveListItemRenderer": {
                    "playlistItemData": { "videoId": "liked-track" },
                    "flexColumns": [{
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "Liked Track" }] }
                        }
                    }],
                    "menu": {
                        "menuRenderer": {
                            "topLevelButtons": [{
                                "likeButtonRenderer": { "likeStatus": "LIKE" }
                            }]
                        }
                    }
                }
            },
            {
                "musicResponsiveListItemRenderer": {
                    "playlistItemData": { "videoId": "disliked-track" },
                    "flexColumns": [{
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "Disliked Track" }] }
                        }
                    }],
                    "menu": {
                        "menuRenderer": {
                            "topLevelButtons": [{
                                "likeButtonRenderer": { "likeStatus": "DISLIKE" }
                            }]
                        }
                    }
                }
            },
            {
                "musicResponsiveListItemRenderer": {
                    "playlistItemData": { "videoId": "neutral-track" },
                    "flexColumns": [{
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "Neutral Track" }] }
                        }
                    }]
                }
            }
        ]
    });

    let parsed = endpoints::music::parse_music_search_response(
        "fixture",
        Some(MusicSearchFilter::Songs),
        &raw,
    )
    .expect("Music track-rating fixture should parse");
    assert_eq!(parsed.songs.len(), 3);
    assert_eq!(parsed.songs[0].like_status, MusicLikeStatus::Like);
    assert_eq!(parsed.songs[1].like_status, MusicLikeStatus::Dislike);
    assert_eq!(parsed.songs[2].like_status, MusicLikeStatus::Indifferent);

    let currently_liked = json!({
        "playlistPanelVideoRenderer": {
            "videoId": "watch-liked",
            "title": { "runs": [{ "text": "Watch Liked" }] },
            "menu": {
                "menuRenderer": {
                    "items": [{
                        "toggleMenuServiceItemRenderer": {
                            "defaultServiceEndpoint": {
                                "likeEndpoint": { "status": "INDIFFERENT" }
                            },
                            "toggledServiceEndpoint": {
                                "likeEndpoint": { "status": "LIKE" }
                            }
                        }
                    }]
                }
            }
        }
    });
    let watch = PlaylistPanelVideoNode::from_value(&currently_liked)
        .expect("watch playlist rating fixture should parse");
    assert_eq!(watch.like_status.as_deref(), Some("LIKE"));

    let currently_indifferent = json!({
        "playlistPanelVideoRenderer": {
            "videoId": "watch-neutral",
            "title": { "runs": [{ "text": "Watch Neutral" }] },
            "menu": {
                "menuRenderer": {
                    "items": [{
                        "toggleMenuServiceItemRenderer": {
                            "defaultServiceEndpoint": {
                                "likeEndpoint": { "status": "LIKE" }
                            },
                            "toggledServiceEndpoint": {
                                "likeEndpoint": { "status": "INDIFFERENT" }
                            }
                        }
                    }]
                }
            }
        }
    });
    let watch_neutral = PlaylistPanelVideoNode::from_value(&currently_indifferent)
        .expect("watch neutral fixture should parse");
    assert_eq!(watch_neutral.like_status.as_deref(), Some("INDIFFERENT"));
}

fn music_home_track(video_id: &str, title: &str) -> Value {
    json!({
        "musicResponsiveListItemRenderer": {
            "playlistItemData": { "videoId": video_id },
            "flexColumns": [
                {
                    "musicResponsiveListItemFlexColumnRenderer": {
                        "text": { "runs": [{ "text": title }] }
                    }
                }
            ]
        }
    })
}

fn music_home_shelf(title: &str, video_id: &str) -> Value {
    json!({
        "musicCarouselShelfRenderer": {
            "header": {
                "musicCarouselShelfBasicHeaderRenderer": {
                    "title": { "runs": [{ "text": title }] }
                }
            },
            "contents": [music_home_track(video_id, title)]
        }
    })
}

#[test]
fn test_api_contract_31_music_home_initial_page_retains_continuation() {
    let raw = json!({
        "contents": {
            "singleColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [music_home_shelf("Quick picks", "track-page-1")],
                                "continuations": [{
                                    "nextContinuationData": {
                                        "continuation": "music-home-page-2"
                                    }
                                }]
                            }
                        }
                    }
                }]
            }
        }
    });

    let feed = endpoints::music::parse_music_home_response(&raw)
        .expect("initial Music home fixture should parse");
    assert_eq!(feed.shelves.len(), 1);
    assert_eq!(feed.shelves[0].title, "Quick picks");
    assert_eq!(feed.shelves[0].tracks.len(), 1);
    assert_eq!(feed.continuation_token.as_deref(), Some("music-home-page-2"));
}

#[test]
fn test_api_contract_32_music_home_continuation_parses_shelves_and_next_token() {
    let raw = json!({
        "continuationContents": {
            "sectionListContinuation": {
                "contents": [music_home_shelf("Listen again", "track-page-2")],
                "continuations": [{
                    "nextContinuationData": {
                        "continuation": "music-home-page-3"
                    }
                }]
            }
        }
    });

    let feed = endpoints::music::parse_music_home_response(&raw)
        .expect("Music home continuation fixture should parse");
    assert_eq!(feed.shelves.len(), 1);
    assert_eq!(feed.shelves[0].title, "Listen again");
    assert_eq!(feed.shelves[0].tracks.len(), 1);
    assert_eq!(feed.continuation_token.as_deref(), Some("music-home-page-3"));
}

#[test]
fn test_api_contract_33_music_playlist_details_metadata() {
    let followed = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [{
                                    "musicResponsiveHeaderRenderer": {
                                        "title": { "runs": [{ "text": "Fixture Playlist" }] },
                                        "description": {
                                            "musicDescriptionShelfRenderer": {
                                                "description": { "runs": [
                                                    { "text": "First line. " },
                                                    { "text": "Second line." }
                                                ] }
                                            }
                                        },
                                        "facepile": {
                                            "avatarStackViewModel": {
                                                "text": { "content": "Fixture Curator" },
                                                "rendererContext": {
                                                    "commandContext": {
                                                        "onTap": {
                                                            "innertubeCommand": {
                                                                "browseEndpoint": { "browseId": "UC_fixture_curator" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        "subtitle": { "runs": [
                                            { "text": "Playlist" },
                                            { "text": " • " },
                                            { "text": "2026" }
                                        ] },
                                        "secondSubtitle": { "runs": [
                                            { "text": "1.2K views" },
                                            { "text": " • " },
                                            { "text": "321 songs" },
                                            { "text": " • " },
                                            { "text": "18 hours" }
                                        ] },
                                        "thumbnail": {
                                            "musicThumbnailRenderer": {
                                                "thumbnail": {
                                                    "thumbnails": [
                                                        { "url": "https://example.test/playlist-small.jpg", "width": 120, "height": 120 },
                                                        { "url": "https://example.test/playlist-large.jpg", "width": 544, "height": 544 }
                                                    ]
                                                }
                                            }
                                        }
                                    }
                                }],
                                "secondaryContents": {
                                    "sectionListRenderer": {
                                        "contents": [{
                                            "musicPlaylistShelfRenderer": {
                                                "contents": [{
                                                    "musicResponsiveListItemRenderer": {
                                                        "playlistItemData": { "videoId": "fixture-playlist-track" },
                                                        "flexColumns": [
                                                            {
                                                                "musicResponsiveListItemFlexColumnRenderer": {
                                                                    "text": { "runs": [{ "text": "Fixture Track" }] }
                                                                }
                                                            },
                                                            {
                                                                "musicResponsiveListItemFlexColumnRenderer": {
                                                                    "text": { "runs": [
                                                                        {
                                                                            "text": "Fixture Artist",
                                                                            "navigationEndpoint": {
                                                                                "browseEndpoint": { "browseId": "UC_fixture_artist" }
                                                                            }
                                                                        },
                                                                        { "text": " • " },
                                                                        {
                                                                            "text": "Fixture Album",
                                                                            "navigationEndpoint": {
                                                                                "browseEndpoint": { "browseId": "MPREb_fixture_album" }
                                                                            }
                                                                        },
                                                                        { "text": " • " },
                                                                        { "text": "4:02" }
                                                                    ] }
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }]
                                            }
                                        }]
                                    }
                                }
                            }
                        }
                    }
                }]
            }
        }
    });

    let playlist = endpoints::music::parse_music_playlist_details_response(
        "VLfixture-followed",
        &followed,
    )
    .expect("followed Music playlist fixture should parse");
    assert_eq!(playlist.id, "fixture-followed");
    assert_eq!(playlist.title, "Fixture Playlist");
    assert_eq!(playlist.description.as_deref(), Some("First line. Second line."));
    assert_eq!(playlist.author.as_ref().map(|author| author.name.as_str()), Some("Fixture Curator"));
    assert_eq!(
        playlist.author.as_ref().and_then(|author| author.browse_id.as_deref()),
        Some("UC_fixture_curator")
    );
    assert_eq!(playlist.track_count, Some(321));
    assert_eq!(playlist.duration.as_deref(), Some("18 hours"));
    assert_eq!(playlist.year.as_deref(), Some("2026"));
    assert_eq!(playlist.privacy.as_deref(), Some("PUBLIC"));
    assert!(!playlist.owned);
    assert!(!playlist.is_collaborative);
    assert_eq!(playlist.thumbnail.as_deref(), Some("https://example.test/playlist-large.jpg"));
    assert_eq!(playlist.tracks.len(), 1);
    assert_eq!(playlist.tracks[0].video_id, "fixture-playlist-track");
    assert_eq!(playlist.tracks[0].duration_ms, Some(242_000));

    let owned = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [{
                                    "musicEditablePlaylistDetailHeaderRenderer": {
                                        "playlistId": "fixture-owned",
                                        "editHeader": {
                                            "musicPlaylistEditHeaderRenderer": { "privacy": "PRIVATE" }
                                        },
                                        "header": {
                                            "musicResponsiveHeaderRenderer": {
                                                "title": { "runs": [{ "text": "Owned Fixture" }] },
                                                "facepile": {
                                                    "avatarStackViewModel": {
                                                        "text": { "content": "Fixture Owner" },
                                                        "rendererContext": {
                                                            "commandContext": {
                                                                "onTap": {
                                                                    "innertubeCommand": {
                                                                        "browseEndpoint": { "browseId": "UC_fixture_owner" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                "subtitle": { "runs": [
                                                    { "text": "Playlist" },
                                                    { "text": " • " },
                                                    { "text": "Fixture Owner" },
                                                    { "text": " • " },
                                                    { "text": "2025" }
                                                ] },
                                                "secondSubtitle": { "runs": [
                                                    { "text": "62 songs" },
                                                    { "text": " • " },
                                                    { "text": "3 hours" }
                                                ] }
                                            }
                                        }
                                    }
                                }]
                            }
                        }
                    }
                }]
            }
        }
    });

    let playlist = endpoints::music::parse_music_playlist_details_response("fixture-owned", &owned)
        .expect("owned Music playlist fixture should parse");
    assert_eq!(playlist.id, "fixture-owned");
    assert_eq!(playlist.title, "Owned Fixture");
    assert!(playlist.owned);
    assert_eq!(playlist.privacy.as_deref(), Some("PRIVATE"));
    assert_eq!(playlist.track_count, Some(62));
    assert_eq!(playlist.duration.as_deref(), Some("3 hours"));
    assert_eq!(playlist.year.as_deref(), Some("2025"));
    assert_eq!(playlist.author.as_ref().map(|author| author.name.as_str()), Some("Fixture Owner"));
    assert!(playlist.tracks.is_empty());
}

#[test]
fn test_api_contract_34_music_playlist_pagination() {
    let initial = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [{
                                    "musicResponsiveHeaderRenderer": {
                                        "facepile": {
                                            "avatarStackViewModel": {
                                                "rendererContext": {
                                                    "commandContext": {
                                                        "onTap": {
                                                            "innertubeCommand": {
                                                                "showEngagementPanelEndpoint": {
                                                                    "identifier": { "tag": "PAplaylist_collaborate" }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }]
                            }
                        }
                    }
                }],
                "secondaryContents": {
                    "sectionListRenderer": {
                        "contents": [{
                            "musicPlaylistShelfRenderer": {
                                "contents": [
                                    music_home_track("playlist-song-1", "Playlist Song 1"),
                                    {
                                        "continuationItemRenderer": {
                                            "continuationEndpoint": {
                                                "continuationCommand": {
                                                    "token": "playlist-page-2"
                                                }
                                            }
                                        }
                                    }
                                ]
                            }
                        }]
                    }
                }
            }
        }
    });

    let first = endpoints::music::parse_music_playlist_response(&initial, false)
        .expect("initial Music playlist fixture should parse");
    assert_eq!(first.tracks.len(), 1);
    assert_eq!(first.tracks[0].video_id, "playlist-song-1");
    assert_eq!(first.continuation_token.as_deref(), Some("playlist-page-2"));
    assert!(first.is_collaborative);

    let continuation = json!({
        "onResponseReceivedActions": [{
            "appendContinuationItemsAction": {
                "targetId": "PL_fixture",
                "continuationItems": [
                    music_home_track("playlist-song-2", "Playlist Song 2"),
                    {
                        "continuationItemRenderer": {
                            "continuationEndpoint": {
                                "continuationCommand": {
                                    "token": "playlist-page-3"
                                }
                            }
                        }
                    }
                ]
            }
        }]
    });

    let second = endpoints::music::parse_music_playlist_response(&continuation, first.is_collaborative)
        .expect("Music playlist continuation fixture should parse");
    assert_eq!(second.tracks.len(), 1);
    assert_eq!(second.tracks[0].video_id, "playlist-song-2");
    assert_eq!(second.continuation_token.as_deref(), Some("playlist-page-3"));
    assert!(second.is_collaborative);
}
