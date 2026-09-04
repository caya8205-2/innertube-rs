use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{OnceLock, RwLock};

use crate::models::video::{PlayabilityStatus, StreamingData, VideoDetails};
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::nodes::YTNode;

/// Renderer names skipped silently, mirroring `IGNORED_LIST` in legacy
/// `parser.ts`.
pub const IGNORED_LIST: [&str; 14] = [
    "AdSlot",
    "DisplayAd",
    "SearchPyv",
    "MealbarPromo",
    "PrimetimePromo",
    "PromotedSparklesWeb",
    "CompactPromotedVideo",
    "BrandVideoShelf",
    "BrandVideoSingleton",
    "StatementBanner",
    "GuideSigninPromo",
    "AdsEngagementPanelContent",
    "MiniGameCardView",
    "GenAiFeedbackFormView",
];

/// Legacy `sanitizeClassName`: strip `Renderer`/`Model` suffixes (and the
/// `Radio` -> `Mix` alias).
pub fn sanitize_class_name(input: &str) -> String {
    let trimmed = input
        .strip_suffix("Renderer")
        .or_else(|| input.strip_suffix("Model"))
        .unwrap_or(input);
    if trimmed == "Radio" {
        "Mix".to_string()
    } else {
        trimmed.to_string()
    }
}

/// A recoverable parser error, mirroring legacy `setParserErrorHandler`
/// records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub error_type: String,
    pub class_name: String,
    pub detail: Option<String>,
}

type ErrorHandler = Box<dyn Fn(ParserError) + Send + Sync>;

static ERROR_HANDLER: OnceLock<RwLock<Option<ErrorHandler>>> = OnceLock::new();

fn error_handler_slot() -> &'static RwLock<Option<ErrorHandler>> {
    ERROR_HANDLER.get_or_init(|| RwLock::new(None))
}

/// Install a global parser error handler (legacy `setParserErrorHandler`).
pub fn set_parser_error_handler(handler: impl Fn(ParserError) + Send + Sync + 'static) {
    if let Ok(mut slot) = error_handler_slot().write() {
        *slot = Some(Box::new(handler));
    }
}

pub(crate) fn report_parser_error(error: ParserError) {
    if let Ok(slot) = error_handler_slot().read() {
        if let Some(handler) = slot.as_ref() {
            handler(error);
        }
    }
}

/// Playback tracking base URLs (legacy `IPlaybackTracking`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackTracking {
    pub videostats_watchtime_url: Option<String>,
    pub videostats_playback_url: Option<String>,
}

/// BotGuard challenge data (legacy `IBotguardChallenge`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotguardChallenge {
    pub interpreter_url: Option<String>,
    pub program: Option<String>,
    pub global_name: Option<String>,
    pub client_experiments_state_blob: Option<String>,
}

/// Structured InnerTube response assembly, mirroring legacy
/// `Parser.parseResponse` (`IParsedResponse`).
#[derive(Debug, Clone, Default)]
pub struct ParsedResponse {
    pub contents: Vec<YTNode>,
    pub header: Vec<YTNode>,
    pub sidebar: Vec<YTNode>,
    pub items: Vec<YTNode>,
    pub actions: Vec<YTNode>,
    pub alerts: Vec<YTNode>,
    pub on_response_received_actions: Vec<YTNode>,
    pub on_response_received_endpoints: Vec<YTNode>,
    pub on_response_received_commands: Vec<YTNode>,
    pub continuation_contents: Vec<YTNode>,
    pub engagement_panels: Vec<YTNode>,
    pub cards: Vec<YTNode>,
    pub annotations: Vec<YTNode>,
    pub player_overlays: Vec<YTNode>,
    pub endscreen: Vec<YTNode>,
    pub storyboards: Vec<YTNode>,
    pub entries: Vec<NavigationEndpointNode>,
    pub overlay: Option<YTNode>,
    pub microformat: Option<YTNode>,
    pub metadata: Option<YTNode>,
    pub background: Option<YTNode>,
    pub current_video_endpoint: Option<NavigationEndpointNode>,
    pub endpoint: Option<NavigationEndpointNode>,
    /// `timedContinuationData` continuation only (legacy `parseC`).
    pub continuation: Option<Value>,
    pub continuation_endpoint: Option<NavigationEndpointNode>,
    pub refinements: Option<Vec<String>>,
    pub estimated_results: Option<u64>,
    pub playability_status: Option<PlayabilityStatus>,
    pub streaming_data: Option<StreamingData>,
    pub video_details: Option<VideoDetails>,
    pub captions: Option<Value>,
    pub playback_tracking: Option<PlaybackTracking>,
    pub player_config: Option<Value>,
    pub bg_challenge: Option<BotguardChallenge>,
    pub target_id: Option<String>,
    pub framework_updates: Option<Value>,
}

impl ParsedResponse {
    /// Apply `frameworkUpdates.entityBatchUpdate` mutations, mirroring
    /// legacy `applyMutations` (music multi-select items + heatmap markers).
    pub fn apply_mutations(&mut self) {
        let Some(mutations) = self
            .framework_updates
            .as_ref()
            .and_then(|fu| fu.pointer("/entityBatchUpdate/mutations"))
            .and_then(Value::as_array)
            .cloned()
        else {
            return;
        };

        apply_music_multi_select_mutations(&mut self.contents, &mut self.items, &mutations);

        // Heatmap marker entities are appended as MacroMarkersListEntity
        // nodes (legacy pushes them onto the memo).
        for mutation in &mutations {
            let entity = mutation
                .pointer("/payload/macroMarkersListEntity")
                .filter(|e| {
                    e.pointer("/markersList/markerType").and_then(Value::as_str)
                        == Some("MARKER_TYPE_HEATMAP")
                });
            if let Some(entity) = entity {
                if let Some(node) =
                    crate::parser::nodes::misc::panels::MacroMarkersListEntityNode::from_value(
                        &json_wrapped("macroMarkersListEntity", entity),
                    )
                {
                    self.contents.push(YTNode::MacroMarkersListEntity(node));
                }
            }
        }
    }

    /// Apply comment mutations to `on_response_received_endpoints`,
    /// mirroring legacy `applyCommentsMutations`.
    pub fn apply_comments_mutations(&mut self) {
        let Some(mutations) = self
            .framework_updates
            .as_ref()
            .and_then(|fu| fu.pointer("/entityBatchUpdate/mutations"))
            .and_then(Value::as_array)
            .cloned()
        else {
            return;
        };

        for node in &mut self.on_response_received_endpoints {
            if let YTNode::CommentView(comment_view) = node {
                comment_view.apply_mutations(&mutations);
            }
        }
        for node in &mut self.contents {
            if let YTNode::CommentView(comment_view) = node {
                comment_view.apply_mutations(&mutations);
            }
        }
    }
}

fn json_wrapped(key: &str, value: &Value) -> Value {
    let mut map = serde_json::Map::new();
    map.insert(key.to_string(), value.clone());
    Value::Object(map)
}

/// Legacy multi-select mutation rule: `selected` applies only when the
/// mutation carries both `selected` and `opaqueToken`.
fn apply_music_multi_select_mutations(
    contents: &mut [YTNode],
    items: &mut [YTNode],
    mutations: &[Value],
) {
    let mut missing = 0usize;
    let mut total = 0usize;

    for nodes in [contents, items] {
        for node in nodes.iter_mut() {
            let YTNode::MusicMultiSelectMenuItem(item) = node else {
                continue;
            };
            total += 1;

            let choice = mutations.iter().find_map(|m| {
                let c = m.pointer("/payload/musicFormBooleanChoice")?;
                (c.get("id").and_then(Value::as_str) == item.form_item_entity_key.as_deref())
                    .then_some(c)
            });

            let applied = choice.is_some_and(|c| {
                c.get("selected").is_some() && c.get("opaqueToken").is_some()
            });

            if let (Some(choice), true) = (choice, applied) {
                item.selected = choice
                    .get("selected")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
            } else {
                missing += 1;
            }
        }
    }

    if total > 0 && missing > 0 {
        report_parser_error(ParserError {
            error_type: "mutation_data_invalid".to_string(),
            class_name: "MusicMultiSelectMenuItem".to_string(),
            detail: Some(format!("{missing}/{total} items missing valid mutations")),
        });
    }
}

impl crate::parser::nodes::misc::channels_comments::CommentViewNode {
    /// Enrich the comment from entity-batch mutations (legacy
    /// `CommentView.applyMutations`). Toolbar-surface/comment-surface
    /// command wiring is not ported (no command fields on the node).
    pub fn apply_mutations(&mut self, mutations: &[Value]) {
        let comment = mutations.iter().find_map(|m| {
            let c = m.pointer("/payload/commentEntityPayload")?;
            (c.get("key").and_then(Value::as_str) == self.keys.comment.as_deref()).then_some(c)
        });

        if let Some(comment) = comment {
            self.content = comment
                .pointer("/properties/content/content")
                .or_else(|| comment.pointer("/properties/content"))
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.content.take());
            self.published_time = comment
                .pointer("/properties/publishedTime")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.published_time.take());
            self.reply_level = comment
                .pointer("/properties/replyLevel")
                .and_then(Value::as_u64)
                .or(Some(0));
            self.author_button_a11y = comment
                .pointer("/properties/authorButtonA11y")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.author_button_a11y.take());
            self.author_is_channel_owner = comment
                .pointer("/author/isCreator")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            self.creator_thumbnail_url = comment
                .pointer("/toolbar/creatorThumbnailUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.creator_thumbnail_url.take());
            self.like_count = Some(
                comment
                    .pointer("/toolbar/likeCountNotliked")
                    .and_then(Value::as_str)
                    .unwrap_or("0")
                    .to_string(),
            );
            self.like_count_liked = Some(
                comment
                    .pointer("/toolbar/likeCountLiked")
                    .and_then(Value::as_str)
                    .unwrap_or("0")
                    .to_string(),
            );
            self.like_count_a11y = comment
                .pointer("/toolbar/likeCountA11y")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.like_count_a11y.take());
            self.like_active_tooltip = comment
                .pointer("/toolbar/likeActiveTooltip")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.like_active_tooltip.take());
            self.like_inactive_tooltip = comment
                .pointer("/toolbar/likeInactiveTooltip")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.like_inactive_tooltip.take());
            self.dislike_active_tooltip = comment
                .pointer("/toolbar/dislikeActiveTooltip")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.dislike_active_tooltip.take());
            self.dislike_inactive_tooltip = comment
                .pointer("/toolbar/dislikeInactiveTooltip")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.dislike_inactive_tooltip.take());
            self.like_button_a11y = comment
                .pointer("/toolbar/likeButtonA11y")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.like_button_a11y.take());
            self.heart_active_tooltip = comment
                .pointer("/toolbar/heartActiveTooltip")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.heart_active_tooltip.take());
            self.reply_count_a11y = comment
                .pointer("/toolbar/replyCountA11y")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.reply_count_a11y.take());
            self.reply_count = Some(
                comment
                    .pointer("/toolbar/replyCount")
                    .and_then(Value::as_str)
                    .unwrap_or("0")
                    .to_string(),
            );
            self.is_member = comment.pointer("/author/sponsorBadgeUrl").is_some();
            if let Some(url) = comment
                .pointer("/author/sponsorBadgeUrl")
                .and_then(Value::as_str)
            {
                self.member_badge = Some(
                    crate::parser::nodes::misc::channels_comments::MemberBadgeNode {
                        url: url.to_string(),
                        a11y: comment
                            .pointer("/author/sponsorBadgeA11y")
                            .and_then(Value::as_str)
                            .map(ToString::to_string),
                    },
                );
            }
            self.author_name = comment
                .pointer("/author/displayName")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.author_name.take());
            self.author_channel_id = comment
                .pointer("/author/channelId")
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .or(self.author_channel_id.take());
        }

        let toolbar_state = mutations.iter().find_map(|m| {
            let s = m.pointer("/payload/engagementToolbarStateEntityPayload")?;
            (s.get("key").and_then(Value::as_str) == self.keys.toolbar_state.as_deref())
                .then_some(s)
        });

        if let Some(state) = toolbar_state {
            self.is_hearted = state.get("heartState").and_then(Value::as_str)
                == Some("TOOLBAR_HEART_STATE_HEARTED");
            self.is_liked = state.get("likeState").and_then(Value::as_str)
                == Some("TOOLBAR_LIKE_STATE_LIKED");
            self.is_disliked = state.get("likeState").and_then(Value::as_str)
                == Some("TOOLBAR_LIKE_STATE_DISLIKED");
        }
    }
}

impl crate::parser::Parser {
    /// Assemble a structured response from a raw InnerTube JSON payload,
    /// mirroring legacy `Parser.parseResponse`.
    pub fn parse_response(data: &Value) -> ParsedResponse {
        let mut parsed = ParsedResponse {
            framework_updates: data.get("frameworkUpdates").cloned(),
            ..Default::default()
        };

        let section = |key: &str| -> Vec<YTNode> {
            data.get(key).map(crate::parser::Parser::parse_tree_reporting).unwrap_or_default()
        };

        parsed.contents = section("contents");
        parsed.header = section("header");
        parsed.sidebar = section("sidebar");
        parsed.items = section("items");
        parsed.actions = section("actions");
        parsed.alerts = section("alerts");
        parsed.on_response_received_actions = section("onResponseReceivedActions");
        parsed.on_response_received_endpoints = section("onResponseReceivedEndpoints");
        parsed.on_response_received_commands = section("onResponseReceivedCommands");
        parsed.continuation_contents = section("continuationContents");
        parsed.engagement_panels = section("engagementPanels");
        parsed.cards = section("cards");
        parsed.annotations = section("annotations");
        parsed.player_overlays = section("playerOverlays");
        parsed.endscreen = section("endscreen");
        parsed.storyboards = section("storyboards");

        parsed.overlay = data.get("overlay").and_then(YTNode::parse);
        parsed.microformat = data.get("microformat").and_then(YTNode::parse);
        parsed.metadata = data.get("metadata").and_then(YTNode::parse);
        parsed.background = data.get("background").and_then(YTNode::parse);

        parsed.current_video_endpoint = data
            .get("currentVideoEndpoint")
            .and_then(NavigationEndpointNode::from_value);
        parsed.endpoint = data.get("endpoint").and_then(NavigationEndpointNode::from_value);
        parsed.continuation_endpoint = data
            .get("continuationEndpoint")
            .and_then(NavigationEndpointNode::from_value);

        if let Some(entries) = data.get("entries").and_then(Value::as_array) {
            parsed.entries = entries
                .iter()
                .filter_map(NavigationEndpointNode::from_value)
                .collect();
        }

        // Legacy parseC: timedContinuationData only.
        parsed.continuation = data
            .get("continuation")
            .and_then(|c| c.get("timedContinuationData"))
            .cloned();

        parsed.refinements = data.get("refinements").and_then(Value::as_array).map(|arr| {
            arr.iter()
                .filter_map(|r| r.as_str().map(ToString::to_string))
                .collect()
        });
        parsed.estimated_results = data
            .get("estimatedResults")
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<u64>().ok());
        parsed.target_id = data
            .get("targetId")
            .and_then(Value::as_str)
            .map(ToString::to_string);

        parsed.playability_status = data
            .get("playabilityStatus")
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok());
        parsed.streaming_data = data
            .get("streamingData")
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok());
        parsed.video_details = data
            .get("videoDetails")
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok());
        parsed.captions = data.get("captions").cloned();
        parsed.player_config = data.get("playerConfig").cloned();

        parsed.playback_tracking = data.get("playbackTracking").map(|pt| PlaybackTracking {
            videostats_watchtime_url: pt
                .pointer("/videostatsWatchtimeUrl/baseUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            videostats_playback_url: pt
                .pointer("/videostatsPlaybackUrl/baseUrl")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        });

        parsed.bg_challenge = data.get("bgChallenge").map(|bg| BotguardChallenge {
            interpreter_url: bg
                .pointer("/interpreterUrl/privateDoNotAccessOrElseTrustedResourceUrlWrappedValue")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            program: bg
                .get("program")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            global_name: bg
                .get("globalName")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            client_experiments_state_blob: bg
                .get("clientExperimentsStateBlob")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        });

        parsed.apply_mutations();
        parsed.apply_comments_mutations();

        parsed
    }
}
