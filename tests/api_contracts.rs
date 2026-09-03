#![allow(unused_imports, dead_code)]

use innertube_rs::{
    endpoints, models, ActionResult, ApiResponse, ChannelAbout, ChannelVideosResponse,
    CommentThread, CommentsResult, CreateCommentResult, CreatePlaylistResult, DownloadOptions,
    DownloadRange, FormatFilter, FormatOptions, FormatType, GetVideoInfoOptions, GuideResponse,
    HashtagFeed, HistoryFeed, HomeFeed, Innertube, LibraryFeed, LiveChatMessage, LiveChatResponse,
    MusicAlbumView, MusicArtistPage, MusicExplore, MusicHomeFeed, MusicLyrics, MusicSearchFilter,
    MusicSearchResults, NavigationEndpointNode, NodeListExt, NotificationPreferenceType, Parser,
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
        }],
        continuation_token: Some("token_playlist_next".to_string()),
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
fn test_api_contract_26_music_artist_details_and_mpla_normalization() {
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
