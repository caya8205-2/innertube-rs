use innertube_rs::{
    ApiResponse, NodeListExt, Parser, YTNode,
};
use serde_json::json;

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
