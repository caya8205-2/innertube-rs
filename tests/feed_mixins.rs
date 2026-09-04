//! Offline contract tests for Feed/FilterableFeed/TabbedFeed behavior
//! (Batch 10), mirroring legacy mixins.

use innertube_rs::models::feed::{FilterableFeed, TrendingFeed, TrendingTab};
use innertube_rs::parser::{Parser, YTNode};
use serde_json::json;

fn chipbar_page() -> serde_json::Value {
    json!({
        "contents": {
            "feedFilterChipBarRenderer": {
                "contents": [
                    {
                        "chipCloudChipRenderer": {
                            "text": { "runs": [{ "text": "All" }] },
                            "isSelected": true
                        }
                    },
                    {
                        "chipCloudChipRenderer": {
                            "text": { "runs": [{ "text": "Music" }] },
                            "isSelected": false,
                            "navigationEndpoint": {
                                "commandMetadata": { "webCommandMetadata": { "apiUrl": "/youtubei/v1/browse" } },
                                "browseEndpoint": { "browseId": "FEwhat_to_watch", "params": "music-params" }
                            }
                        }
                    }
                ]
            }
        }
    })
}

#[test]
fn filterable_feed_chipbar_path_and_labels() {
    let nodes = Parser::parse_tree(&chipbar_page());
    let feed = FilterableFeed::from_nodes(&nodes).unwrap();
    assert_eq!(feed.filters(), vec!["All".to_string(), "Music".to_string()]);
    assert!(feed.secondary_filter_labels().is_empty());

    // Endpoint resolution from the chip's navigationEndpoint.
    let music = feed
        .primary_filters
        .iter()
        .find(|n| n.label() == "Music")
        .unwrap();
    let endpoint = match music {
        innertube_rs::models::feed::FilterNode::Chip(c) => c.endpoint.clone(),
        _ => None,
    }
    .expect("chip endpoint");
    assert_eq!(endpoint.api_path.as_deref(), Some("/browse"));
    assert_eq!(endpoint.payload["browseId"], json!("FEwhat_to_watch"));
    assert_eq!(endpoint.payload["params"], json!("music-params"));
}

#[test]
fn filterable_feed_unknown_filter_lists_available() {
    let nodes = Parser::parse_tree(&chipbar_page());
    let feed = FilterableFeed::from_nodes(&nodes).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let session = rt.block_on(async {
        innertube_rs::Session::create(innertube_rs::SessionOptions {
            generate_session_locally: Some(true),
            retrieve_innertube_config: Some(false),
            ..Default::default()
        })
        .await
        .unwrap()
    });

    let err = rt
        .block_on(feed.get_filtered_feed(&session, "Nonexistent", None))
        .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Nonexistent"), "{msg}");
    assert!(msg.contains("All"), "{msg}");
    assert!(msg.contains("Music"), "{msg}");
}

#[test]
fn filterable_feed_too_many_chipbars_errors() {
    let raw = json!({
        "a": { "feedFilterChipBarRenderer": { "contents": [] } },
        "b": { "feedFilterChipBarRenderer": { "contents": [] } }
    });
    let nodes = Parser::parse_tree(&raw);
    assert!(FilterableFeed::from_nodes(&nodes).is_err());
}

#[test]
fn filterable_feed_dropdown_path_uses_list_items() {
    let raw = json!({
        "contents": [
            {
                "chipView": {
                    "text": "Type",
                    "displayType": "CHIP_VIEW_MODEL_DISPLAY_TYPE_DROP_DOWN",
                    "selected": false,
                    "tapCommand": {
                        "innertubeCommand": {
                            "showSheetCommand": {
                                "inlineContent": {
                                    "sheetView": {
                                        "content": {
                                            "listView": {
                                                "items": [
                                                    { "listItemView": { "title": { "content": "Videos" }, "isSelected": true } },
                                                    { "listItemView": { "title": { "content": "Shorts" } } }
                                                ]
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            { "chipView": { "text": "Recent", "selected": true } }
        ]
    });
    let nodes = Parser::parse_tree(&raw);
    let feed = FilterableFeed::from_nodes(&nodes).unwrap();
    assert_eq!(feed.filters(), vec!["Videos".to_string(), "Shorts".to_string()]);
    assert_eq!(feed.secondary_filter_labels(), vec!["Recent".to_string()]);
}

#[test]
fn tabbed_feed_lookup_rules() {
    let feed = TrendingFeed {
        current_tab: "Now".to_string(),
        tabs: vec![
            TrendingTab {
                title: "Now".to_string(),
                params: None,
                is_selected: true,
                url: Some("/feed/trending".to_string()),
            },
            TrendingTab {
                title: "Music".to_string(),
                params: Some("music-params".to_string()),
                is_selected: false,
                url: Some("/feed/music".to_string()),
            },
        ],
        videos: vec![],
    };

    assert_eq!(feed.title(), Some("Now"));
    // Case-insensitive; selected tab short-circuits to None (legacy `this`).
    assert!(feed.resolve_tab_by_name("now").unwrap().is_none());
    let music = feed.resolve_tab_by_name("music").unwrap().unwrap();
    assert_eq!(music.params.as_deref(), Some("music-params"));
    assert!(feed.resolve_tab_by_name("nope").is_err());

    assert!(feed.has_tab_with_url("music"));
    assert!(!feed.has_tab_with_url("gaming"));
    assert!(feed.resolve_tab_by_url("music").unwrap().is_some());
    // Selected tab short-circuits.
    assert!(feed.resolve_tab_by_url("trending").unwrap().is_none());
}

#[test]
fn body_continuation_excludes_header_chips() {
    let raw = json!({
        "header": {
            "feedTabbedHeaderRenderer": {
                "chipCloudRenderer": {
                    "chips": [{
                        "chipCloudChipRenderer": {
                            "text": { "runs": [{ "text": "All" }] },
                            "navigationEndpoint": {
                                "continuationEndpoint": {
                                    "continuationCommand": { "token": "HEADER_CHIP_TOKEN" }
                                }
                            }
                        }
                    }]
                }
            }
        },
        "contents": [
            { "videoRenderer": { "videoId": "v1", "title": { "simpleText": "V" } } },
            {
                "continuationItemRenderer": {
                    "continuationEndpoint": {
                        "continuationCommand": { "token": "BODY_TOKEN" }
                    }
                }
            }
        ]
    });

    let feed = innertube_rs::endpoints::feed::parse_home_feed_response(&raw).unwrap();
    assert_eq!(feed.continuation_token.as_deref(), Some("BODY_TOKEN"));
    assert!(!feed.videos.is_empty());
}

#[test]
fn library_groups_videos_by_shelf_icon() {
    let raw = json!({
        "contents": {
            "twoColumnBrowseResultsRenderer": {
                "tabs": [{
                    "tabRenderer": {
                        "content": {
                            "sectionListRenderer": {
                                "contents": [
                                    {
                                        "shelfRenderer": {
                                            "title": { "runs": [{ "text": "Watch Later" }] },
                                            "icon": { "iconType": "WATCH_LATER" },
                                            "content": {
                                                "horizontalListRenderer": {
                                                    "items": [
                                                        { "videoRenderer": { "videoId": "wl1", "title": { "simpleText": "Later" } } }
                                                    ]
                                                }
                                            }
                                        }
                                    },
                                    {
                                        "shelfRenderer": {
                                            "title": { "runs": [{ "text": "History" }] },
                                            "icon": { "iconType": "HISTORY" },
                                            "content": {
                                                "horizontalListRenderer": {
                                                    "items": [
                                                        { "videoRenderer": { "videoId": "h1", "title": { "simpleText": "Hist" } } },
                                                        { "videoRenderer": { "videoId": "h2", "title": { "simpleText": "Hist2" } } }
                                                    ]
                                                }
                                            }
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

    let sections = innertube_rs::parser::Parser::parse_tree(&raw)
        .iter()
        .filter_map(|n| match n {
            YTNode::Shelf(s) => Some(s),
            _ => None,
        })
        .count();
    assert_eq!(sections, 2);
}

#[test]
fn browse_feed_collects_posts() {
    let raw = json!({
        "contents": [{
            "backstagePostRenderer": {
                "postId": "UgkxTest",
                "contentText": { "runs": [{ "text": "Hello" }] }
            }
        }]
    });
    let feed = innertube_rs::endpoints::feed::parse_browse_feed_response("FEchannels", &raw).unwrap();
    assert_eq!(feed.posts.len(), 1);
}
