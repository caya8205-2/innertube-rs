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

#[test]
fn test_fixture_livechat_stickers_memberships_and_banners() {
    let sticker_json = json!({
        "liveChatPaidStickerRenderer": {
            "id": "stk_999",
            "authorName": { "simpleText": "Super Fan" },
            "purchaseAmountText": { "simpleText": "$10.00" },
            "sticker": { "thumbnails": [{ "url": "https://yt3.ggpht.com/sticker.png" }] }
        }
    });
    let sticker_node = YTNode::parse(&sticker_json).expect("LiveChatPaidStickerNode should parse");
    if let YTNode::LiveChatPaidSticker(ps) = sticker_node {
        assert_eq!(ps.id, "stk_999");
        assert_eq!(ps.author_name.as_deref(), Some("Super Fan"));
        assert_eq!(ps.purchase_amount_text, "$10.00");
        assert!(ps.sticker.is_some());
    } else {
        panic!("Expected YTNode::LiveChatPaidSticker");
    }

    let membership_json = json!({
        "liveChatMembershipItemRenderer": {
            "id": "mem_888",
            "authorName": { "simpleText": "Loyal Member" },
            "headerSubtext": { "runs": [{ "text": "Member for 12 months" }] },
            "message": { "runs": [{ "text": "Happy Anniversary!" }] }
        }
    });
    let mem_node = YTNode::parse(&membership_json).expect("LiveChatMembershipItemNode should parse");
    if let YTNode::LiveChatMembershipItem(mi) = mem_node {
        assert_eq!(mi.id, "mem_888");
        assert_eq!(mi.author_name.as_deref(), Some("Loyal Member"));
        assert_eq!(mi.header_subtext.as_deref(), Some("Member for 12 months"));
        assert_eq!(mi.message.as_deref(), Some("Happy Anniversary!"));
    } else {
        panic!("Expected YTNode::LiveChatMembershipItem");
    }

    let banner_json = json!({
        "liveChatBannerRenderer": {
            "header": {
                "liveChatBannerHeaderRenderer": {
                    "text": { "runs": [{ "text": "Important Announcement" }] }
                }
            }
        }
    });
    let banner_node = YTNode::parse(&banner_json).expect("LiveChatBannerNode should parse");
    if let YTNode::LiveChatBanner(b) = banner_node {
        assert_eq!(b.header.as_deref(), Some("Important Announcement"));
    } else {
        panic!("Expected YTNode::LiveChatBanner");
    }
}

#[test]
fn test_fixture_music_headers_badges_alerts_and_polls() {
    let header_json = json!({
        "musicHeaderRenderer": {
            "title": { "runs": [{ "text": "Chill Vibes Playlist" }] },
            "subtitle": { "runs": [{ "text": "Curated by YouTube Music" }] },
            "thumbnail": {
                "thumbnails": [{ "url": "https://lh3.googleusercontent.com/header.jpg", "width": 544, "height": 544 }]
            }
        }
    });
    let header_node = YTNode::parse(&header_json).expect("MusicHeaderNode should parse");
    if let YTNode::MusicHeader(mh) = header_node {
        assert_eq!(mh.title, "Chill Vibes Playlist");
        assert_eq!(mh.subtitle.as_deref(), Some("Curated by YouTube Music"));
        assert!(!mh.thumbnails.thumbnails.is_empty());
    } else {
        panic!("Expected YTNode::MusicHeader");
    }

    let badge_json = json!({
        "musicInlineBadgeRenderer": {
            "icon": { "iconType": "MUSIC_EXPLICIT_BADGE" },
            "accessibilityData": {
                "accessibilityData": { "label": "Explicit track" }
            },
            "tooltip": "Explicit"
        }
    });
    let badge_node = YTNode::parse(&badge_json).expect("MusicInlineBadgeNode should parse");
    if let YTNode::MusicInlineBadge(mib) = badge_node {
        assert_eq!(mib.icon_type.as_deref(), Some("MUSIC_EXPLICIT_BADGE"));
        assert_eq!(mib.label.as_deref(), Some("Explicit track"));
        assert_eq!(mib.tooltip.as_deref(), Some("Explicit"));
    } else {
        panic!("Expected YTNode::MusicInlineBadge");
    }

    let alert_json = json!({
        "alertRenderer": {
            "type": "WARNING",
            "text": { "runs": [{ "text": "This video is unlisted." }] }
        }
    });
    let alert_node = YTNode::parse(&alert_json).expect("AlertNode should parse");
    if let YTNode::Alert(a) = alert_node {
        assert_eq!(a.alert_type.as_deref(), Some("WARNING"));
        assert_eq!(a.text, "This video is unlisted.");
    } else {
        panic!("Expected YTNode::Alert");
    }

    let poll_json = json!({
        "pollRenderer": {
            "question": { "runs": [{ "text": "What is your favorite language?" }] },
            "choices": [
                { "text": { "runs": [{ "text": "Rust" }] } },
                { "text": { "runs": [{ "text": "TypeScript" }] } }
            ],
            "totalVotes": { "simpleText": "50,000 votes" }
        }
    });
    let poll_node = YTNode::parse(&poll_json).expect("PollNode should parse");
    if let YTNode::Poll(p) = poll_node {
        assert_eq!(p.question, "What is your favorite language?");
        assert_eq!(p.choices, vec!["Rust", "TypeScript"]);
        assert_eq!(p.total_votes.as_deref(), Some("50,000 votes"));
    } else {
        panic!("Expected YTNode::Poll");
    }
}

#[test]
fn test_fixture_livechat_actions_and_moderation() {
    let add_action_json = json!({
        "addChatItemAction": {
            "clientId": "client_msg_101",
            "item": {
                "liveChatTextMessageRenderer": {
                    "id": "msg_dyn_1",
                    "message": { "runs": [{ "text": "Super live!" }] }
                }
            }
        }
    });
    let add_action_node = YTNode::parse(&add_action_json).expect("AddChatItemActionNode should parse");
    if let YTNode::AddChatItemAction(act) = add_action_node {
        assert_eq!(act.client_id.as_deref(), Some("client_msg_101"));
        assert!(act.item.is_some());
    } else {
        panic!("Expected YTNode::AddChatItemAction");
    }

    let mark_del_json = json!({
        "markChatItemAsDeletedAction": {
            "targetItemId": "bad_msg_999",
            "deletedStateMessage": { "runs": [{ "text": "[Message deleted by moderator]" }] }
        }
    });
    let mark_del_node = YTNode::parse(&mark_del_json).expect("MarkChatItemAsDeletedActionNode should parse");
    if let YTNode::MarkChatItemAsDeletedAction(del) = mark_del_node {
        assert_eq!(del.target_item_id.as_deref(), Some("bad_msg_999"));
        assert_eq!(del.deleted_state_message.as_deref(), Some("[Message deleted by moderator]"));
    } else {
        panic!("Expected YTNode::MarkChatItemAsDeletedAction");
    }

    let automod_json = json!({
        "liveChatAutoModMessageRenderer": {
            "headerText": { "runs": [{ "text": "Held for review" }] },
            "autoModeratedItem": {
                "liveChatTextMessageRenderer": { "id": "held_1" }
            }
        }
    });
    let automod_node = YTNode::parse(&automod_json).expect("LiveChatAutoModMessageNode should parse");
    if let YTNode::LiveChatAutoModMessage(am) = automod_node {
        assert_eq!(am.header_text.as_deref(), Some("Held for review"));
        assert!(am.auto_moderated_item.is_some());
    } else {
        panic!("Expected YTNode::LiveChatAutoModMessage");
    }

    let mode_change_json = json!({
        "liveChatModeChangeMessageRenderer": {
            "text": { "runs": [{ "text": "Subscribers-only mode enabled" }] },
            "icon": { "iconType": "SUBSCRIBERS_ONLY" }
        }
    });
    let mode_node = YTNode::parse(&mode_change_json).expect("LiveChatModeChangeMessageNode should parse");
    if let YTNode::LiveChatModeChangeMessage(mc) = mode_node {
        assert_eq!(mc.text, "Subscribers-only mode enabled");
        assert_eq!(mc.icon_type.as_deref(), Some("SUBSCRIBERS_ONLY"));
    } else {
        panic!("Expected YTNode::LiveChatModeChangeMessage");
    }
}

#[test]
fn test_fixture_player_overlays_and_profile_columns() {
    let overlay_json = json!({
        "playerOverlayRenderer": {
            "actions": [{ "likeButtonRenderer": { "likeStatus": "LIKE" } }],
            "autonavToggle": { "toggleButtonRenderer": { "defaultText": { "simpleText": "Autoplay" } } }
        }
    });
    let overlay_node = YTNode::parse(&overlay_json).expect("PlayerOverlayNode should parse");
    if let YTNode::PlayerOverlay(po) = overlay_node {
        assert_eq!(po.actions.len(), 1);
        assert!(po.autonav_toggle.is_some());
    } else {
        panic!("Expected YTNode::PlayerOverlay");
    }

    let storyboard_json = json!({
        "playerStoryboardSpecRenderer": {
            "spec": "https://i.ytimg.com/sb/xyz/storyboard.jpg#48#27#10#10"
        }
    });
    let sb_node = YTNode::parse(&storyboard_json).expect("PlayerStoryboardSpecNode should parse");
    if let YTNode::PlayerStoryboardSpec(sb) = sb_node {
        assert_eq!(sb.spec.as_deref(), Some("https://i.ytimg.com/sb/xyz/storyboard.jpg#48#27#10#10"));
    } else {
        panic!("Expected YTNode::PlayerStoryboardSpec");
    }

    let marker_json = json!({
        "timedMarkerDecorationRenderer": {
            "visibleTimeRangeStartMillis": 5000,
            "visibleTimeRangeEndMillis": 15000,
            "decorationTimeMillis": 10000,
            "label": { "runs": [{ "text": "Key Moment" }] },
            "icon": { "iconType": "CHAPTER_MARKER" }
        }
    });
    let marker_node = YTNode::parse(&marker_json).expect("TimedMarkerDecorationNode should parse");
    if let YTNode::TimedMarkerDecoration(tmd) = marker_node {
        assert_eq!(tmd.visible_time_range_start_millis, Some(5000));
        assert_eq!(tmd.visible_time_range_end_millis, Some(15000));
        assert_eq!(tmd.decoration_time_millis, Some(10000));
        assert_eq!(tmd.label.as_deref(), Some("Key Moment"));
        assert_eq!(tmd.icon_type.as_deref(), Some("CHAPTER_MARKER"));
    } else {
        panic!("Expected YTNode::TimedMarkerDecoration");
    }

    let profile_user_json = json!({
        "profileColumnUserInfoRenderer": {
            "title": { "runs": [{ "text": "TechExplorer" }] },
            "description": { "runs": [{ "text": "Exploring systems and rust." }] },
            "thumbnail": { "thumbnails": [{ "url": "https://yt3.ggpht.com/avatar.jpg" }] }
        }
    });
    let profile_node = YTNode::parse(&profile_user_json).expect("ProfileColumnUserInfoNode should parse");
    if let YTNode::ProfileColumnUserInfo(pcu) = profile_node {
        assert_eq!(pcu.title, "TechExplorer");
        assert_eq!(pcu.description.as_deref(), Some("Exploring systems and rust."));
        assert!(!pcu.thumbnails.thumbnails.is_empty());
    } else {
        panic!("Expected YTNode::ProfileColumnUserInfo");
    }

    let vertical_list_json = json!({
        "verticalListRenderer": {
            "items": [{ "videoRenderer": { "videoId": "v_vert_1" } }],
            "collapsedItemCount": 5
        }
    });
    let vl_node = YTNode::parse(&vertical_list_json).expect("VerticalListNode should parse");
    if let YTNode::VerticalList(vl) = vl_node {
        assert_eq!(vl.items.len(), 1);
        assert_eq!(vl.collapsed_item_count, Some(5));
    } else {
        panic!("Expected YTNode::VerticalList");
    }
}

#[test]
fn test_fixture_video_chapters_markers_and_heatmaps() {
    let chapter_json = json!({
        "chapterRenderer": {
            "title": { "runs": [{ "text": "Getting Started with Rust" }] },
            "timeRangeStartMillis": 120000,
            "thumbnail": {
                "thumbnails": [{ "url": "https://img.youtube.com/chap1.jpg" }]
            }
        }
    });
    let chapter_node = YTNode::parse(&chapter_json).expect("ChapterNode should parse");
    if let YTNode::Chapter(ch) = chapter_node {
        assert_eq!(ch.title, "Getting Started with Rust");
        assert_eq!(ch.time_range_start_millis, 120000);
        assert!(!ch.thumbnails.thumbnails.is_empty());
    } else {
        panic!("Expected YTNode::Chapter");
    }

    let heatmap_json = json!({
        "heatmapRenderer": {
            "maxHeightDp": 50.0,
            "minHeightDp": 10.0,
            "showHeatmapOnSeek": true
        }
    });
    let heatmap_node = YTNode::parse(&heatmap_json).expect("HeatmapNode should parse");
    if let YTNode::Heatmap(hm) = heatmap_node {
        assert_eq!(hm.max_height_dp, Some(50.0));
        assert_eq!(hm.min_height_dp, Some(10.0));
        assert_eq!(hm.show_heatmap_on_seek, Some(true));
    } else {
        panic!("Expected YTNode::Heatmap");
    }

    let macro_markers_json = json!({
        "macroMarkersListRenderer": {
            "title": { "runs": [{ "text": "Chapters" }] },
            "contents": [{
                "macroMarkersListItemRenderer": {
                    "title": { "runs": [{ "text": "Section 1" }] },
                    "timeDescription": { "runs": [{ "text": "0:00" }] },
                    "thumbnail": { "thumbnails": [{ "url": "https://img.youtube.com/m1.jpg" }] }
                }
            }]
        }
    });
    let macro_node = YTNode::parse(&macro_markers_json).expect("MacroMarkersListNode should parse");
    if let YTNode::MacroMarkersList(mml) = macro_node {
        assert_eq!(mml.title.as_deref(), Some("Chapters"));
        assert_eq!(mml.contents.len(), 1);
    } else {
        panic!("Expected YTNode::MacroMarkersList");
    }
}

#[test]
fn test_fixture_search_refinements_post_media_and_channel_submenus() {
    let refinement_json = json!({
        "searchRefinementCardRenderer": {
            "query": { "runs": [{ "text": "rust async tutorial" }] },
            "thumbnail": { "thumbnails": [{ "url": "https://img.youtube.com/ref.jpg" }] },
            "searchEndpoint": { "searchEndpoint": { "query": "rust async tutorial" } }
        }
    });
    let refinement_node = YTNode::parse(&refinement_json).expect("SearchRefinementCardNode should parse");
    if let YTNode::SearchRefinementCard(src) = refinement_node {
        assert_eq!(src.query, "rust async tutorial");
        assert!(!src.thumbnails.thumbnails.is_empty());
        assert!(src.endpoint.is_some());
    } else {
        panic!("Expected YTNode::SearchRefinementCard");
    }

    let horiz_cards_json = json!({
        "horizontalCardListRenderer": {
            "cards": [{ "searchRefinementCardRenderer": { "query": { "simpleText": "tokio" } } }],
            "header": { "richListHeaderRenderer": { "title": { "simpleText": "Related" } } }
        }
    });
    let horiz_node = YTNode::parse(&horiz_cards_json).expect("HorizontalCardListNode should parse");
    if let YTNode::HorizontalCardList(hcl) = horiz_node {
        assert_eq!(hcl.cards.len(), 1);
        assert!(hcl.header.is_some());
    } else {
        panic!("Expected YTNode::HorizontalCardList");
    }

    let expandable_tab_json = json!({
        "expandableTabRenderer": {
            "title": { "runs": [{ "text": "Live streams" }] },
            "selected": true
        }
    });
    let tab_node = YTNode::parse(&expandable_tab_json).expect("ExpandableTabNode should parse");
    if let YTNode::ExpandableTab(et) = tab_node {
        assert_eq!(et.title, "Live streams");
        assert!(et.selected);
    } else {
        panic!("Expected YTNode::ExpandableTab");
    }

    let backstage_img_json = json!({
        "backstageImageRenderer": {
            "image": { "thumbnails": [{ "url": "https://yt3.ggpht.com/post_img.jpg" }] }
        }
    });
    let img_node = YTNode::parse(&backstage_img_json).expect("BackstageImageNode should parse");
    if let YTNode::BackstageImage(bi) = img_node {
        assert!(!bi.image.thumbnails.is_empty());
    } else {
        panic!("Expected YTNode::BackstageImage");
    }

    let post_multi_img_json = json!({
        "postMultiImageRenderer": {
            "images": [
                { "backstageImageRenderer": { "image": { "thumbnails": [{ "url": "https://yt3.ggpht.com/img1.jpg" }] } } },
                { "backstageImageRenderer": { "image": { "thumbnails": [{ "url": "https://yt3.ggpht.com/img2.jpg" }] } } }
            ]
        }
    });
    let multi_node = YTNode::parse(&post_multi_img_json).expect("PostMultiImageNode should parse");
    if let YTNode::PostMultiImage(pmi) = multi_node {
        assert_eq!(pmi.images.len(), 2);
    } else {
        panic!("Expected YTNode::PostMultiImage");
    }

    let channel_sub_menu_json = json!({
        "channelSubMenuRenderer": {
            "contentTypeSubMenuItems": [{ "title": "Latest" }, { "title": "Popular" }],
            "sortFilterSubMenu": { "title": "Sort by" }
        }
    });
    let sub_menu_node = YTNode::parse(&channel_sub_menu_json).expect("ChannelSubMenuNode should parse");
    if let YTNode::ChannelSubMenu(csm) = sub_menu_node {
        assert_eq!(csm.content_type_sub_menu_items.len(), 2);
        assert!(csm.sort_filter_sub_menu.is_some());
    } else {
        panic!("Expected YTNode::ChannelSubMenu");
    }
}

#[test]
fn test_fixture_engagement_panel_and_navigation_actions() {
    let show_panel_json = json!({
        "showEngagementPanelEndpoint": {
            "panelIdentifier": "engagement-panel-structured-description",
            "engagementPanel": { "title": "Description" }
        }
    });
    let show_panel_node = YTNode::parse(&show_panel_json).expect("ShowEngagementPanelActionNode should parse");
    if let YTNode::ShowEngagementPanelAction(sp) = show_panel_node {
        assert_eq!(sp.panel_identifier.as_deref(), Some("engagement-panel-structured-description"));
        assert!(sp.content.is_some());
    } else {
        panic!("Expected YTNode::ShowEngagementPanelAction");
    }

    let update_panel_json = json!({
        "updateEngagementPanelAction": {
            "panelIdentifier": "engagement-panel-comments",
            "content": { "commentsCount": 120 }
        }
    });
    let update_panel_node = YTNode::parse(&update_panel_json).expect("UpdateEngagementPanelActionNode should parse");
    if let YTNode::UpdateEngagementPanelAction(up) = update_panel_node {
        assert_eq!(up.panel_identifier.as_deref(), Some("engagement-panel-comments"));
        assert!(up.content.is_some());
    } else {
        panic!("Expected YTNode::UpdateEngagementPanelAction");
    }

    let navigate_json = json!({
        "navigateAction": {
            "endpoint": { "browseEndpoint": { "browseId": "FEwhat_to_watch" } }
        }
    });
    let nav_node = YTNode::parse(&navigate_json).expect("NavigateActionNode should parse");
    if let YTNode::NavigateAction(na) = nav_node {
        assert!(na.endpoint.is_some());
    } else {
        panic!("Expected YTNode::NavigateAction");
    }

    let show_live_json = json!({
        "showLiveChatAction": {
            "clientId": "live_msg_sync",
            "chatItem": { "liveChatTextMessageRenderer": { "id": "l1" } }
        }
    });
    let live_node = YTNode::parse(&show_live_json).expect("ShowLiveChatActionNode should parse");
    if let YTNode::ShowLiveChatAction(sla) = live_node {
        assert_eq!(sla.client_id.as_deref(), Some("live_msg_sync"));
        assert!(sla.chat_item.is_some());
    } else {
        panic!("Expected YTNode::ShowLiveChatAction");
    }
}

#[test]
fn test_fixture_player_media_and_playlist_sidebars() {
    let captions_json = json!({
        "playerCaptionsTracklistRenderer": {
            "captionTracks": [{ "baseUrl": "https://youtube.com/api/timedtext", "languageCode": "en" }],
            "audioTracks": [{ "audioTrackId": "1" }],
            "translationLanguages": [{ "languageCode": "id", "languageName": "Indonesian" }]
        }
    });
    let cap_node = YTNode::parse(&captions_json).expect("PlayerCaptionsTracklistNode should parse");
    if let YTNode::PlayerCaptionsTracklist(pct) = cap_node {
        assert_eq!(pct.caption_tracks.len(), 1);
        assert_eq!(pct.audio_tracks.len(), 1);
        assert_eq!(pct.translation_languages.len(), 1);
    } else {
        panic!("Expected YTNode::PlayerCaptionsTracklist");
    }

    let error_json = json!({
        "playerErrorMessageRenderer": {
            "reason": { "runs": [{ "text": "Video unavailable" }] },
            "subreason": { "runs": [{ "text": "This video is private." }] },
            "icon": { "iconType": "ERROR" }
        }
    });
    let err_node = YTNode::parse(&error_json).expect("PlayerErrorMessageNode should parse");
    if let YTNode::PlayerErrorMessage(pem) = err_node {
        assert_eq!(pem.reason, "Video unavailable");
        assert_eq!(pem.subreason.as_deref(), Some("This video is private."));
        assert_eq!(pem.icon_type.as_deref(), Some("ERROR"));
    } else {
        panic!("Expected YTNode::PlayerErrorMessage");
    }

    let trailer_json = json!({
        "playerLegacyDesktopYpcTrailerRenderer": {
            "videoId": "trailer_abc",
            "ypcMessage": { "runs": [{ "text": "Rent this movie to watch" }] }
        }
    });
    let tr_node = YTNode::parse(&trailer_json).expect("PlayerLegacyDesktopYpcTrailerNode should parse");
    if let YTNode::PlayerLegacyDesktopYpcTrailer(tr) = tr_node {
        assert_eq!(tr.video_id.as_deref(), Some("trailer_abc"));
        assert_eq!(tr.ypc_message.as_deref(), Some("Rent this movie to watch"));
    } else {
        panic!("Expected YTNode::PlayerLegacyDesktopYpcTrailer");
    }

    let pl_meta_json = json!({
        "playlistMetadataRenderer": {
            "title": "Favorite Coding Music",
            "description": "Selected tracks for focus",
            "privacy": "PUBLIC"
        }
    });
    let meta_node = YTNode::parse(&pl_meta_json).expect("PlaylistMetadataNode should parse");
    if let YTNode::PlaylistMetadata(pm) = meta_node {
        assert_eq!(pm.title.as_deref(), Some("Favorite Coding Music"));
        assert_eq!(pm.description.as_deref(), Some("Selected tracks for focus"));
        assert_eq!(pm.privacy.as_deref(), Some("PUBLIC"));
    } else {
        panic!("Expected YTNode::PlaylistMetadata");
    }

    let pl_sidebar_primary_json = json!({
        "playlistSidebarPrimaryInfoRenderer": {
            "title": { "runs": [{ "text": "Rust Learning Track" }] },
            "stats": [
                { "runs": [{ "text": "15 videos" }] },
                { "runs": [{ "text": "100,000 views" }] }
            ],
            "thumbnailRenderer": {
                "thumbnails": [{ "url": "https://img.youtube.com/pl_thumb.jpg" }]
            }
        }
    });
    let prim_node = YTNode::parse(&pl_sidebar_primary_json).expect("PlaylistSidebarPrimaryInfoNode should parse");
    if let YTNode::PlaylistSidebarPrimaryInfo(spi) = prim_node {
        assert_eq!(spi.title.as_deref(), Some("Rust Learning Track"));
        assert_eq!(spi.stats.len(), 2);
        assert!(!spi.thumbnails.thumbnails.is_empty());
    } else {
        panic!("Expected YTNode::PlaylistSidebarPrimaryInfo");
    }

    let pl_sidebar_secondary_json = json!({
        "playlistSidebarSecondaryInfoRenderer": {
            "videoOwner": {
                "videoOwnerRenderer": { "title": { "runs": [{ "text": "Rust Foundation" }] } }
            },
            "button": { "buttonRenderer": { "text": { "runs": [{ "text": "Play All" }] } } }
        }
    });
    let sec_node = YTNode::parse(&pl_sidebar_secondary_json).expect("PlaylistSidebarSecondaryInfoNode should parse");
    if let YTNode::PlaylistSidebarSecondaryInfo(ssi) = sec_node {
        assert!(ssi.owner.is_some());
        assert!(ssi.button.is_some());
    } else {
        panic!("Expected YTNode::PlaylistSidebarSecondaryInfo");
    }
}

#[test]
fn test_fixture_notifications_and_account_components() {
    let notif_json = json!({
        "notificationRenderer": {
            "notificationId": "notif_xyz123",
            "primaryText": { "runs": [{ "text": "New release available" }] },
            "thumbnail": { "thumbnails": [{ "url": "https://img.youtube.com/notif.jpg" }] },
            "navigationEndpoint": { "watchEndpoint": { "videoId": "v_new" } },
            "unread": true
        }
    });
    let notif_node = YTNode::parse(&notif_json).expect("NotificationNode should parse");
    if let YTNode::Notification(n) = notif_node {
        assert_eq!(n.notification_id.as_deref(), Some("notif_xyz123"));
        assert_eq!(n.primary_text, "New release available");
        assert!(!n.thumbnails.thumbnails.is_empty());
        assert!(n.endpoint.is_some());
        assert!(n.unread);
    } else {
        panic!("Expected YTNode::Notification");
    }

    let history_sugg_json = json!({
        "historySuggestionRenderer": {
            "suggestion": { "runs": [{ "text": "rust async tokio stream" }] },
            "navigationEndpoint": { "searchEndpoint": { "query": "rust async tokio stream" } }
        }
    });
    let sugg_node = YTNode::parse(&history_sugg_json).expect("HistorySuggestionNode should parse");
    if let YTNode::HistorySuggestion(hs) = sugg_node {
        assert_eq!(hs.suggestion, "rust async tokio stream");
        assert!(hs.endpoint.is_some());
    } else {
        panic!("Expected YTNode::HistorySuggestion");
    }

    let account_sec_json = json!({
        "accountSectionListRenderer": {
            "contents": [{ "accountItemRenderer": { "accountName": { "simpleText": "Caya" } } }],
            "header": { "title": "Accounts" }
        }
    });
    let sec_node = YTNode::parse(&account_sec_json).expect("AccountSectionListNode should parse");
    if let YTNode::AccountSectionList(asl) = sec_node {
        assert_eq!(asl.contents.len(), 1);
        assert!(asl.header.is_some());
    } else {
        panic!("Expected YTNode::AccountSectionList");
    }

    let account_item_json = json!({
        "accountItemRenderer": {
            "accountName": { "runs": [{ "text": "Caya Rustacean" }] },
            "accountPhoto": { "thumbnails": [{ "url": "https://yt3.ggpht.com/avatar.jpg" }] },
            "isSelected": true
        }
    });
    let item_node = YTNode::parse(&account_item_json).expect("AccountItemNode should parse");
    if let YTNode::AccountItem(ai) = item_node {
        assert_eq!(ai.account_name, "Caya Rustacean");
        assert!(!ai.account_photo.thumbnails.is_empty());
        assert!(ai.is_selected);
    } else {
        panic!("Expected YTNode::AccountItem");
    }
}

#[test]
fn test_fixture_search_filters_kids_and_music_queue() {
    let sfg_json = json!({
        "searchFilterGroupRenderer": {
            "title": { "runs": [{ "text": "Upload Date" }] },
            "filters": [
                { "searchFilterRenderer": { "label": { "runs": [{ "text": "Last hour" }] }, "status": "FILTER_STATUS_SELECTED" } },
                { "searchFilterRenderer": { "label": { "runs": [{ "text": "Today" }] }, "status": "FILTER_STATUS_UNSELECTED" } }
            ]
        }
    });
    let sfg_node = YTNode::parse(&sfg_json).expect("SearchFilterGroupNode should parse");
    if let YTNode::SearchFilterGroup(sfg) = sfg_node {
        assert_eq!(sfg.title.as_deref(), Some("Upload Date"));
        assert_eq!(sfg.filters.len(), 2);
    } else {
        panic!("Expected YTNode::SearchFilterGroup");
    }

    let sf_json = json!({
        "searchFilterRenderer": {
            "label": { "runs": [{ "text": "4K Video" }] },
            "status": "FILTER_STATUS_SELECTED",
            "tooltip": "Search for 4K quality",
            "navigationEndpoint": { "searchEndpoint": { "params": "CA4%3D" } }
        }
    });
    let sf_node = YTNode::parse(&sf_json).expect("SearchFilterNode should parse");
    if let YTNode::SearchFilter(sf) = sf_node {
        assert_eq!(sf.label, "4K Video");
        assert!(sf.selected);
        assert_eq!(sf.tooltip.as_deref(), Some("Search for 4K quality"));
        assert!(sf.endpoint.is_some());
    } else {
        panic!("Expected YTNode::SearchFilter");
    }

    let kids_hdr_json = json!({
        "kidsCategoriesHeaderRenderer": {
            "categoryTabs": [
                { "title": "Shows" },
                { "title": "Music" },
                { "title": "Gaming" }
            ]
        }
    });
    let k_node = YTNode::parse(&kids_hdr_json).expect("KidsCategoriesHeaderNode should parse");
    if let YTNode::KidsCategoriesHeader(kh) = k_node {
        assert_eq!(kh.category_tabs.len(), 3);
    } else {
        panic!("Expected YTNode::KidsCategoriesHeader");
    }

    let kids_home_json = json!({
        "kidsHomeScreenRenderer": {
            "anchors": [{ "anchor": { "title": "Popular" } }]
        }
    });
    let kh_node = YTNode::parse(&kids_home_json).expect("KidsHomeScreenNode should parse");
    if let YTNode::KidsHomeScreen(khs) = kh_node {
        assert_eq!(khs.anchors.len(), 1);
    } else {
        panic!("Expected YTNode::KidsHomeScreen");
    }

    let music_queue_json = json!({
        "musicQueueRenderer": {
            "content": { "playlistPanelRenderer": { "title": { "simpleText": "Queue" } } }
        }
    });
    let mq_node = YTNode::parse(&music_queue_json).expect("MusicQueueNode should parse");
    if let YTNode::MusicQueue(mq) = mq_node {
        assert!(mq.content.is_some());
    } else {
        panic!("Expected YTNode::MusicQueue");
    }

    let music_play_btn_json = json!({
        "musicPlayButtonRenderer": {
            "playNavigationEndpoint": { "watchEndpoint": { "videoId": "music_track_1" } },
            "icon": { "iconType": "MUSIC_PLAY" }
        }
    });
    let mpb_node = YTNode::parse(&music_play_btn_json).expect("MusicPlayButtonNode should parse");
    if let YTNode::MusicPlayButton(mpb) = mpb_node {
        assert!(mpb.play_navigation_endpoint.is_some());
        assert_eq!(mpb.icon_type.as_deref(), Some("MUSIC_PLAY"));
    } else {
        panic!("Expected YTNode::MusicPlayButton");
    }
}






