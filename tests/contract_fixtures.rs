#![allow(unused_imports, dead_code)]

use innertube_rs::{
    ActionResult, ApiResponse, CreateCommentResult, CreatePlaylistResult, FormatFilter,
    FormatType, NavigationEndpointNode, NodeListExt, NotificationPreferenceType, Parser,
    PlaylistNode, PlaylistVideoNode, PostCommentSort, PostNode, QualityPreference, SearchFilters,
    SearchPrioritize, ShortNode, VideoNode, YouTubePlaylistView, YTNode,
};
use serde_json::{json, Value};

#[test]
fn test_fixture_search_response_parsing() {
    let fixture = json!({
        "contents": {
            "twoColumnSearchResultsRenderer": {
                "primaryContents": {
                    "sectionListRenderer": {
                        "contents": [
                            {
                                "itemSectionRenderer": {
                                    "contents": [
                                        {
                                            "videoRenderer": {
                                                "videoId": "dQw4w9WgXcQ",
                                                "title": { "runs": [{ "text": "Never Gonna Give You Up" }] },
                                                "ownerText": { "runs": [{ "text": "Rick Astley" }] },
                                                "lengthText": { "simpleText": "3:33" },
                                                "viewCountText": { "simpleText": "1,500,000,000 views" }
                                            }
                                        },
                                        {
                                            "reelItemRenderer": {
                                                "videoId": "short_abc",
                                                "headline": { "simpleText": "Rickroll Short" },
                                                "viewCountText": { "simpleText": "10M views" }
                                            }
                                        },
                                        {
                                            "channelRenderer": {
                                                "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                                                "title": { "simpleText": "Rick Astley" },
                                                "videoCountText": { "simpleText": "150 videos" }
                                            }
                                        },
                                        {
                                            "playlistRenderer": {
                                                "playlistId": "PL_rick_greatest_hits",
                                                "title": { "simpleText": "Rick Astley - Greatest Hits" },
                                                "videoCount": "25"
                                            }
                                        }
                                    ]
                                }
                            },
                            {
                                "continuationItemRenderer": {
                                    "continuationEndpoint": {
                                        "continuationCommand": {
                                            "token": "search_continuation_token_xyz"
                                        }
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }
    });

    let nodes = Parser::parse_tree(&fixture);
    assert_eq!(nodes.find_videos().len(), 1);
    assert_eq!(nodes.find_videos()[0].id, "dQw4w9WgXcQ");
    assert_eq!(nodes.find_videos()[0].title, "Never Gonna Give You Up");

    assert_eq!(nodes.find_shorts().len(), 1);
    assert_eq!(nodes.find_shorts()[0].id, "short_abc");

    assert_eq!(nodes.find_channels().len(), 1);
    assert_eq!(nodes.find_channels()[0].id, "UCuAXFkgsw1L7xaCfnd5JJOw");

    assert_eq!(nodes.find_playlists().len(), 1);
    assert_eq!(nodes.find_playlists()[0].id, "PL_rick_greatest_hits");

    assert_eq!(nodes.find_continuation_token().as_deref(), Some("search_continuation_token_xyz"));
}

#[test]
fn test_fixture_channel_tabs_and_header_parsing() {
    let fixture = json!({
        "header": {
            "c4TabbedHeaderRenderer": {
                "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                "title": "Rick Astley",
                "subscriberCountText": { "simpleText": "4.5M subscribers" },
                "videosCountText": { "simpleText": "150 videos" }
            }
        },
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "title": "Home",
                            "selected": true,
                            "content": {
                                "sectionListRenderer": {
                                    "contents": []
                                }
                            }
                        }
                    },
                    {
                        "tabRenderer": {
                            "title": "Videos",
                            "selected": false,
                            "endpoint": {
                                "browseEndpoint": {
                                    "browseId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                                    "params": "EgZ2aWRlb3M%3D"
                                }
                            }
                        }
                    },
                    {
                        "tabRenderer": {
                            "title": "Community",
                            "selected": false,
                            "endpoint": {
                                "browseEndpoint": {
                                    "browseId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                                    "params": "Egljb21tdW5pdHk%3D"
                                }
                            }
                        }
                    }
                ]
            }
        }
    });

    let nodes = Parser::parse_tree(&fixture);
    assert_eq!(nodes.find_tabs().len(), 3);
    assert_eq!(nodes.find_tabs()[0].title, "Home");
    assert!(nodes.find_tabs()[0].selected);
    assert_eq!(nodes.find_tabs()[1].title, "Videos");
    assert_eq!(nodes.find_tabs()[2].title, "Community");

    let header = nodes.iter().find_map(|n| match n {
        YTNode::ChannelHeader(h) => Some(h),
        _ => None,
    }).expect("ChannelHeader should be parsed");

    assert_eq!(header.id, "UCuAXFkgsw1L7xaCfnd5JJOw");
    assert_eq!(header.title, "Rick Astley");
    assert_eq!(header.subscriber_count.as_deref(), Some("4.5M subscribers"));
}

#[test]
fn test_fixture_playlist_response_parsing() {
    let fixture = json!({
        "header": {
            "playlistHeaderRenderer": {
                "playlistId": "PL_test_123",
                "title": { "simpleText": "Chill Music 2026" },
                "numVideosText": { "runs": [{ "text": "50 videos" }] },
                "viewCountText": { "simpleText": "100,000 views" }
            }
        },
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
                                                        "playlistVideoListRenderer": {
                                                            "contents": [
                                                                {
                                                                    "playlistVideoRenderer": {
                                                                        "videoId": "vid_1",
                                                                        "title": { "runs": [{ "text": "Track 1" }] },
                                                                        "lengthSeconds": "180"
                                                                    }
                                                                },
                                                                {
                                                                    "playlistVideoRenderer": {
                                                                        "videoId": "vid_2",
                                                                        "title": { "runs": [{ "text": "Track 2" }] },
                                                                        "lengthSeconds": "240"
                                                                    }
                                                                }
                                                            ]
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

    let nodes = Parser::parse_tree(&fixture);
    assert_eq!(nodes.find_playlists().len(), 1);
    assert_eq!(nodes.find_playlists()[0].title, "Chill Music 2026");
    assert_eq!(nodes.find_playlists()[0].video_count, Some(50));

    assert_eq!(nodes.find_playlist_videos().len(), 2);
    assert_eq!(nodes.find_playlist_videos()[0].id, "vid_1");
    assert_eq!(nodes.find_playlist_videos()[0].duration_ms, Some(180_000));
}

#[test]
fn test_fixture_music_responsive_list_and_shelf() {
    let fixture = json!({
        "contents": {
            "singleColumnBrowseResultsRenderer": {
                "tabs": [
                    {
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [
                                        {
                                            "musicDescriptionShelfRenderer": {
                                                "header": { "runs": [{ "text": "Lyrics" }] },
                                                "description": { "runs": [{ "text": "We're no strangers to love..." }] },
                                                "footer": { "runs": [{ "text": "Source: LyricFind" }] }
                                            }
                                        },
                                        {
                                            "musicResponsiveListItemRenderer": {
                                                "playlistItemData": { "videoId": "dQw4w9WgXcQ" },
                                                "flexColumns": [
                                                    {
                                                        "musicResponsiveListItemFlexColumnRenderer": {
                                                            "text": { "runs": [{ "text": "Never Gonna Give You Up" }] }
                                                        }
                                                    },
                                                    {
                                                        "musicResponsiveListItemFlexColumnRenderer": {
                                                            "text": {
                                                                "runs": [
                                                                    {
                                                                        "text": "Rick Astley",
                                                                        "navigationEndpoint": {
                                                                            "browseEndpoint": { "browseId": "UCuAXFkgsw1L7xaCfnd5JJOw" }
                                                                        }
                                                                    },
                                                                    { "text": " • " },
                                                                    {
                                                                        "text": "Whenever You Need Somebody",
                                                                        "navigationEndpoint": {
                                                                            "browseEndpoint": { "browseId": "MPREb_album123" }
                                                                        }
                                                                    },
                                                                    { "text": " • " },
                                                                    { "text": "3:33" }
                                                                ]
                                                            }
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

    let nodes = Parser::parse_tree(&fixture);
    assert_eq!(nodes.find_music_items().len(), 1);
    let item = nodes.find_music_items()[0];
    assert_eq!(item.id.as_deref(), Some("dQw4w9WgXcQ"));
    assert_eq!(item.title, "Never Gonna Give You Up");
    assert_eq!(item.artists.len(), 1);
    assert_eq!(item.artists[0].name, "Rick Astley");
    assert_eq!(item.album.as_deref(), Some("Whenever You Need Somebody"));
    assert_eq!(item.album_id.as_deref(), Some("MPREb_album123"));
    assert_eq!(item.duration.as_deref(), Some("3:33"));

    let shelf = nodes.iter().find_map(|n| match n {
        YTNode::MusicDescriptionShelf(s) => Some(s),
        _ => None,
    }).expect("MusicDescriptionShelf should be parsed");
    assert_eq!(shelf.header.as_deref(), Some("Lyrics"));
    assert!(shelf.description.contains("no strangers to love"));
    assert_eq!(shelf.footer.as_deref(), Some("Source: LyricFind"));
}

#[test]
fn test_fixture_comments_and_posts_parsing() {
    let fixture = json!({
        "contents": {
            "itemSectionRenderer": {
                "contents": [
                    {
                        "commentThreadRenderer": {
                            "comment": {
                                "commentRenderer": {
                                    "commentId": "Ugx_comment_123",
                                    "authorText": { "simpleText": "Viewer 1" },
                                    "contentText": { "runs": [{ "text": "Awesome video!" }] },
                                    "voteCount": { "simpleText": "42" },
                                    "authorIsChannelOwner": true
                                }
                            }
                        }
                    },
                    {
                        "backstagePostRenderer": {
                            "postId": "post_789",
                            "authorText": { "simpleText": "Creator" },
                            "contentText": { "runs": [{ "text": "New release coming tomorrow!" }] },
                            "voteCount": { "simpleText": "500" }
                        }
                    }
                ]
            }
        }
    });

    let nodes = Parser::parse_tree(&fixture);
    assert_eq!(nodes.find_comments().len(), 1);
    assert_eq!(nodes.find_comments()[0].comment.comment_id, "Ugx_comment_123");
    assert_eq!(nodes.find_comments()[0].comment.author_name, "Viewer 1");
    assert_eq!(nodes.find_comments()[0].comment.text, "Awesome video!");
    assert_eq!(nodes.find_comments()[0].comment.like_count.as_deref(), Some("42"));
    assert!(nodes.find_comments()[0].comment.is_author_channel_owner);

    assert_eq!(nodes.find_posts().len(), 1);
    assert_eq!(nodes.find_posts()[0].post.id, "post_789");
    assert_eq!(nodes.find_posts()[0].post.author.as_ref().map(|a| a.name.as_str()), Some("Creator"));
    assert_eq!(nodes.find_posts()[0].post.content_text, "New release coming tomorrow!");
}

#[test]
fn test_fixture_actions_execute_response() {
    let raw_success = json!({
        "responseContext": {
            "serviceTrackingParams": []
        },
        "actions": [
            {
                "clickTrackingParams": "CAEQ...",
                "addToPlaylistAction": {
                    "playlistId": "PL_test"
                }
            }
        ]
    });

    let api_response = ApiResponse {
        success: true,
        status_code: 200,
        data: raw_success.clone(),
    };

    assert!(api_response.success);
    assert_eq!(api_response.status_code, 200);
    assert!(api_response.data.get("actions").is_some());
}

#[test]
fn test_fixture_courses_and_subscriptions_feeds() {
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
                                    "contents": [
                                        {
                                            "itemSectionRenderer": {
                                                "contents": [
                                                    {
                                                        "playlistRenderer": {
                                                            "playlistId": "PL_course_1",
                                                            "title": { "simpleText": "Rust Deep Dive" },
                                                            "videoCount": "40"
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

    let feed = innertube_rs::endpoints::feed::parse_browse_feed_response("FEcourses", &browse_payload)
        .expect("Should parse courses feed");
    assert_eq!(feed.browse_id, "FEcourses");
    assert_eq!(feed.playlists.len(), 1);
    assert_eq!(feed.playlists[0].title, "Rust Deep Dive");
}

#[test]
fn test_fixture_unseen_notifications_count() {
    let top_level = json!({ "unseenCount": 12 });
    assert_eq!(
        innertube_rs::endpoints::account::parse_unseen_notifications_count(&top_level),
        12
    );

    let action_wrapped = json!({
        "actions": [
            {
                "updateNotificationsUnseenCountAction": {
                    "unseenCount": "5"
                }
            }
        ]
    });
    assert_eq!(
        innertube_rs::endpoints::account::parse_unseen_notifications_count(&action_wrapped),
        5
    );
}

#[test]
fn test_fixture_attestation_challenge() {
    let payload = innertube_rs::endpoints::attestation::build_attestation_payload(
        "ENGAGEMENT_TYPE_VIDEO",
        Some(json!([{"videoId": "dQw4w9WgXcQ"}])),
    );
    assert_eq!(payload["engagementType"], "ENGAGEMENT_TYPE_VIDEO");
    assert_eq!(payload["ids"][0]["videoId"], "dQw4w9WgXcQ");
}

#[test]
fn test_fixture_feed_mixin_continuations() {
    let search_res = innertube_rs::SearchResults {
        query: "rust".to_string(),
        items: vec![],
        continuation_token: Some("token_search".to_string()),
    };
    assert!(search_res.has_continuation());

    let comments_res = innertube_rs::CommentsResult {
        total_comments_text: None,
        comments: vec![],
        continuation_token: Some("token_comments".to_string()),
    };
    assert!(comments_res.has_continuation());

    let playlist_view = innertube_rs::PlaylistView {
        id: "PL_123".to_string(),
        title: "Test".to_string(),
        author: None,
        author_id: None,
        description: None,
        video_count: None,
        view_count: None,
        last_updated: None,
        thumbnail: None,
        videos: vec![],
        continuation_token: Some("token_playlist".to_string()),
    };
    assert!(playlist_view.has_continuation());

    let channel_videos = innertube_rs::ChannelVideosResponse {
        channel_id: "UC_123".to_string(),
        videos: vec![],
        continuation_token: Some("token_channel".to_string()),
    };
    assert!(channel_videos.has_continuation());
}

#[test]
fn test_fixture_search_modifiers_and_endscreen_nodes() {
    let did_you_mean_json = json!({
        "didYouMeanRenderer": {
            "correctedQuery": { "runs": [{ "text": "rust programming language" }] },
            "navigationEndpoint": {
                "searchEndpoint": { "query": "rust programming language" }
            }
        }
    });
    let dym_node = YTNode::parse(&did_you_mean_json).expect("DidYouMeanNode should parse");
    if let YTNode::DidYouMean(dym) = dym_node {
        assert_eq!(dym.corrected_query, "rust programming language");
        assert!(dym.endpoint.is_some());
    } else {
        panic!("Expected YTNode::DidYouMean");
    }

    let showing_results_json = json!({
        "showingResultsForRenderer": {
            "correctedQuery": { "simpleText": "rust lang" },
            "originalQueryEndpoint": {
                "searchEndpoint": { "query": "rust lage" }
            }
        }
    });
    let srf_node = YTNode::parse(&showing_results_json).expect("ShowingResultsForNode should parse");
    if let YTNode::ShowingResultsFor(srf) = srf_node {
        assert_eq!(srf.corrected_query, "rust lang");
        assert!(srf.original_query_endpoint.is_some());
    } else {
        panic!("Expected YTNode::ShowingResultsFor");
    }

    let endscreen_json = json!({
        "endscreenRenderer": {
            "startMs": "180000",
            "elements": [
                {
                    "endscreenElementRenderer": {
                        "style": "VIDEO",
                        "title": { "runs": [{ "text": "Next Suggested Video" }] },
                        "endpoint": { "watchEndpoint": { "videoId": "next_vid_123" } }
                    }
                }
            ]
        }
    });
    let es_node = YTNode::parse(&endscreen_json).expect("EndscreenNode should parse");
    if let YTNode::Endscreen(es) = es_node {
        assert_eq!(es.start_ms, Some(180000));
        assert_eq!(es.elements.len(), 1);
        assert_eq!(es.elements[0].style, "VIDEO");
        assert_eq!(es.elements[0].title.as_deref(), Some("Next Suggested Video"));
    } else {
        panic!("Expected YTNode::Endscreen");
    }
}

#[test]
fn test_fixture_metadata_badges_and_channel_metadata_nodes() {
    let badge_json = json!({
        "metadataBadgeRenderer": {
            "style": "BADGE_STYLE_TYPE_VERIFIED",
            "label": "Verified Artist",
            "tooltip": "Official Artist Channel",
            "icon": { "iconType": "OFFICIAL_ARTIST_BADGE" }
        }
    });
    let badge_node = YTNode::parse(&badge_json).expect("MetadataBadgeNode should parse");
    if let YTNode::MetadataBadge(mb) = badge_node {
        assert_eq!(mb.style.as_deref(), Some("BADGE_STYLE_TYPE_VERIFIED"));
        assert_eq!(mb.label, "Verified Artist");
        assert_eq!(mb.tooltip.as_deref(), Some("Official Artist Channel"));
        assert_eq!(mb.icon_type.as_deref(), Some("OFFICIAL_ARTIST_BADGE"));
    } else {
        panic!("Expected YTNode::MetadataBadge");
    }

    let channel_about_json = json!({
        "channelAboutFullMetadataRenderer": {
            "description": { "runs": [{ "text": "Official channel description and bio." }] },
            "viewCountText": { "simpleText": "1,234,567,890 views" },
            "joinedDateText": { "simpleText": "Joined Jan 1, 2010" },
            "country": { "simpleText": "United States" },
            "canonicalChannelUrl": "https://www.youtube.com/c/example"
        }
    });
    let about_node = YTNode::parse(&channel_about_json).expect("ChannelAboutFullMetadataNode should parse");
    if let YTNode::ChannelAboutFullMetadata(cafm) = about_node {
        assert_eq!(cafm.description.as_deref(), Some("Official channel description and bio."));
        assert_eq!(cafm.view_count.as_deref(), Some("1,234,567,890 views"));
        assert_eq!(cafm.joined_date.as_deref(), Some("Joined Jan 1, 2010"));
        assert_eq!(cafm.country.as_deref(), Some("United States"));
        assert_eq!(cafm.canonical_channel_url.as_deref(), Some("https://www.youtube.com/c/example"));
    } else {
        panic!("Expected YTNode::ChannelAboutFullMetadata");
    }
}

