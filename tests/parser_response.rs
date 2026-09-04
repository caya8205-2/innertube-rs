//! Fixture tests for `Parser::parse_response` response assembly (Batch 7),
//! mirroring legacy `Parser.parseResponse` sections and mutation handling.

use innertube_rs::parser::{
    sanitize_class_name, set_parser_error_handler, Parser, YTNode,
};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn parse_response_assembles_player_sections() {
    let raw = json!({
        "playabilityStatus": { "status": "OK", "playableInEmbed": true },
        "streamingData": {
            "expiresInSeconds": "21540",
            "formats": [],
            "adaptiveFormats": []
        },
        "videoDetails": {
            "videoId": "dQw4w9WgXcQ",
            "title": "Test",
            "lengthSeconds": "213",
            "channelId": "UCtest",
            "author": "Author"
        },
        "playbackTracking": {
            "videostatsWatchtimeUrl": { "baseUrl": "https://s.youtube.com/api/stats/watchtime" },
            "videostatsPlaybackUrl": { "baseUrl": "https://s.youtube.com/api/stats/playback" }
        },
        "captions": { "playerCaptionsTracklistRenderer": { "captionTracks": [] } },
        "playerConfig": { "audioConfig": {} },
        "estimatedResults": "1234",
        "refinements": ["refine a", "refine b"],
        "targetId": "search-results"
    });

    let parsed = Parser::parse_response(&raw);

    assert_eq!(
        parsed.playability_status.as_ref().map(|p| p.status.as_str()),
        Some("OK")
    );
    assert_eq!(
        parsed
            .streaming_data
            .as_ref()
            .and_then(|s| s.expires_in_seconds.as_deref()),
        Some("21540")
    );
    assert_eq!(
        parsed.video_details.as_ref().map(|v| v.video_id.as_str()),
        Some("dQw4w9WgXcQ")
    );
    assert!(parsed.captions.is_some());
    assert!(parsed.player_config.is_some());
    let tracking = parsed.playback_tracking.expect("playback tracking");
    assert_eq!(
        tracking.videostats_watchtime_url.as_deref(),
        Some("https://s.youtube.com/api/stats/watchtime")
    );
    assert_eq!(parsed.estimated_results, Some(1234));
    assert_eq!(
        parsed.refinements.as_deref(),
        Some(&["refine a".to_string(), "refine b".to_string()][..])
    );
    assert_eq!(parsed.target_id.as_deref(), Some("search-results"));
}

#[test]
fn parse_response_keeps_only_timed_continuation() {
    let raw = json!({
        "continuation": {
            "timedContinuationData": { "timeoutMs": 5000, "token": "timed-token" }
        }
    });
    let parsed = Parser::parse_response(&raw);
    assert_eq!(
        parsed.continuation.as_ref().and_then(|c| c.get("token")).and_then(|t| t.as_str()),
        Some("timed-token")
    );

    let reload = json!({
        "continuation": {
            "reloadContinuationData": { "token": "reload-token" }
        }
    });
    assert!(Parser::parse_response(&reload).continuation.is_none());
}

#[test]
fn parse_response_extracts_bg_challenge_fields() {
    let raw = json!({
        "bgChallenge": {
            "interpreterUrl": {
                "privateDoNotAccessOrElseTrustedResourceUrlWrappedValue": "https://example.com/bg.js"
            },
            "program": "PROGRAM",
            "globalName": "trayride",
            "clientExperimentsStateBlob": "blob"
        }
    });
    let parsed = Parser::parse_response(&raw);
    let bg = parsed.bg_challenge.expect("bg challenge");
    assert_eq!(bg.interpreter_url.as_deref(), Some("https://example.com/bg.js"));
    assert_eq!(bg.program.as_deref(), Some("PROGRAM"));
    assert_eq!(bg.global_name.as_deref(), Some("trayride"));
    assert_eq!(bg.client_experiments_state_blob.as_deref(), Some("blob"));
}

#[test]
fn parse_response_parses_contents_and_rr_sections() {
    let raw = json!({
        "contents": {
            "videoRenderer": {
                "videoId": "abc",
                "title": { "runs": [{ "text": "Content Video" }] }
            }
        },
        "onResponseReceivedActions": [
            {
                "navigateAction": {
                    "endpoint": {
                        "commandMetadata": { "webCommandMetadata": { "apiUrl": "/youtubei/v1/browse" } },
                        "browseEndpoint": { "browseId": "FEwhat_to_watch" }
                    }
                }
            }
        ]
    });

    let parsed = Parser::parse_response(&raw);
    assert!(parsed
        .contents
        .iter()
        .any(|n| matches!(n, YTNode::Video(v) if v.id == "abc")));
    assert!(!parsed.on_response_received_actions.is_empty());
}

#[test]
fn ignored_list_renderers_are_skipped_silently() {
    let raw = json!({
        "contents": [
            { "adSlotRenderer": { "foo": "bar" } },
            { "videoRenderer": { "videoId": "real", "title": { "runs": [{ "text": "V" }] } } }
        ]
    });
    let tree = Parser::parse_tree(&raw);
    assert!(tree
        .iter()
        .any(|n| matches!(n, YTNode::Video(v) if v.id == "real")));
    let ad_only = Parser::parse_tree(&json!({ "adSlotRenderer": { "foo": "bar" } }));
    assert!(ad_only.is_empty(), "ignored renderers must produce no nodes");
}

#[test]
fn unknown_renderer_reports_to_error_handler() {
    static HITS: AtomicUsize = AtomicUsize::new(0);
    set_parser_error_handler(move |err| {
        if err.error_type == "class_not_found" && err.class_name == "totallyUnknownWidget" {
            HITS.fetch_add(1, Ordering::SeqCst);
        }
    });

    let raw = json!({ "contents": [{ "totallyUnknownWidgetRenderer": { "x": 1 } }] });
    let _ = Parser::parse_tree(&raw);
    assert_eq!(sanitize_class_name("totallyUnknownWidgetRenderer"), "totallyUnknownWidget");
    assert!(HITS.load(Ordering::SeqCst) >= 1);
}

#[test]
fn music_multi_select_mutation_sets_selected_with_opaque_token() {
    let raw = json!({
        "contents": {
            "musicMultiSelectMenuItemRenderer": {
                "title": { "runs": [{ "text": "Albums" }] },
                "formItemEntityKey": "key-albums"
            }
        },
        "frameworkUpdates": {
            "entityBatchUpdate": {
                "mutations": [
                    {
                        "payload": {
                            "musicFormBooleanChoice": {
                                "id": "key-albums",
                                "selected": true,
                                "opaqueToken": "tok"
                            }
                        }
                    }
                ]
            }
        }
    });

    let parsed = Parser::parse_response(&raw);
    let item = parsed
        .contents
        .iter()
        .find_map(|n| match n {
            YTNode::MusicMultiSelectMenuItem(item) => Some(item),
            _ => None,
        })
        .expect("multi-select item");
    assert!(item.selected, "mutation with opaqueToken must set selected");
}

#[test]
fn playlist_panel_continuation_falls_back_to_radio_token() {
    let raw = json!({
        "continuationContents": {
            "playlistPanelContinuation": {
                "contents": [],
                "continuations": [
                    { "nextRadioContinuationData": { "continuation": "radio-token" } }
                ]
            }
        }
    });
    let parsed = Parser::parse_response(&raw);
    let panel = parsed
        .continuation_contents
        .iter()
        .find_map(|n| match n {
            YTNode::PlaylistPanelContinuation(p) => Some(p),
            _ => None,
        })
        .expect("playlist panel continuation");
    assert_eq!(panel.continuation.as_deref(), Some("radio-token"));
    assert!(panel.is_radio);
}

#[test]
fn live_chat_continuation_reads_all_token_variants() {
    for (key, token) in [
        ("timedContinuationData", "timed-tok"),
        ("invalidationContinuationData", "invalidation-tok"),
        ("liveChatReplayContinuationData", "replay-tok"),
    ] {
        let raw = json!({
            "continuationContents": {
                "liveChatContinuation": {
                    "actions": [],
                    "continuations": [{ (key): { "continuation": token } }]
                }
            }
        });
        let parsed = Parser::parse_response(&raw);
        let chat = parsed
            .continuation_contents
            .iter()
            .find_map(|n| match n {
                YTNode::LiveChatContinuation(c) => Some(c),
                _ => None,
            })
            .expect("live chat continuation");
        assert_eq!(chat.continuation.as_deref(), Some(token), "{key}");
    }
}

#[test]
fn section_and_item_continuations_parse_tokens() {
    let raw = json!({
        "continuationContents": {
            "sectionListContinuation": {
                "contents": [],
                "continuations": [{ "nextContinuationData": { "continuation": "sec-tok" } }]
            }
        }
    });
    let parsed = Parser::parse_response(&raw);
    let section = parsed
        .continuation_contents
        .iter()
        .find_map(|n| match n {
            YTNode::SectionListContinuation(s) => Some(s),
            _ => None,
        })
        .expect("section list continuation");
    assert_eq!(section.continuation.as_deref(), Some("sec-tok"));
}

#[test]
fn comment_view_mutations_enrich_content_and_toolbar_state() {
    let raw = json!({
        "onResponseReceivedEndpoints": [
            {
                "commentViewModel": {
                    "commentId": "cid1",
                    "commentKey": "comment-key-1",
                    "toolbarStateKey": "toolbar-key-1"
                }
            }
        ],
        "frameworkUpdates": {
            "entityBatchUpdate": {
                "mutations": [
                    {
                        "payload": {
                            "commentEntityPayload": {
                                "key": "comment-key-1",
                                "properties": {
                                    "content": { "content": "Enriched text" },
                                    "publishedTime": "1 day ago"
                                },
                                "author": { "displayName": "Enriched Author", "isCreator": true },
                                "toolbar": { "likeCountNotliked": "42", "replyCount": "7" }
                            }
                        }
                    },
                    {
                        "payload": {
                            "engagementToolbarStateEntityPayload": {
                                "key": "toolbar-key-1",
                                "likeState": "TOOLBAR_LIKE_STATE_LIKED",
                                "heartState": "TOOLBAR_HEART_STATE_HEARTED"
                            }
                        }
                    }
                ]
            }
        }
    });

    let parsed = Parser::parse_response(&raw);
    let comment = parsed
        .on_response_received_endpoints
        .iter()
        .find_map(|n| match n {
            YTNode::CommentView(c) => Some(c),
            _ => None,
        })
        .expect("comment view");

    assert_eq!(comment.content.as_deref(), Some("Enriched text"));
    assert_eq!(comment.published_time.as_deref(), Some("1 day ago"));
    assert_eq!(comment.like_count.as_deref(), Some("42"));
    assert_eq!(comment.reply_count.as_deref(), Some("7"));
    assert!(comment.author_is_channel_owner);
    assert!(comment.is_liked);
    assert!(comment.is_hearted);
    assert!(!comment.is_disliked);
}
