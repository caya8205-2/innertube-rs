use crate::core::actions::{Actions, ApiResponse};
use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::video::{KidsVideoInfo, PlayerResponse};
use crate::parser::nodes::misc::navigation::NavigationEndpointNode;
use crate::parser::{Parser, YTNode};
use serde_json::{json, Value};

/// YouTube Kids Manager (1:1 with Kids.ts).
pub struct KidsManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> KidsManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Fetch YouTube Kids Home Feed (`kidsHomeScreenRenderer`), parsed into
    /// typed nodes.
    pub async fn get_home(&self) -> Result<Vec<YTNode>> {
        let resp = self
            .session
            .post_innertube_client("YTKIDS", "/browse", json!({ "browseId": "FEkids_home" }))
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
        Ok(Parser::parse_tree(&raw))
    }

    /// Fetch YouTube Kids Home Feed (alias matching Kids.ts getHomeFeed).
    pub async fn get_home_feed(&self) -> Result<Vec<YTNode>> {
        self.get_home().await
    }

    /// Search YouTube Kids, parsed into typed nodes.
    pub async fn search(&self, query: &str) -> Result<Vec<YTNode>> {
        let resp = self
            .session
            .post_innertube_client("YTKIDS", "/search", json!({ "query": query }))
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
        Ok(Parser::parse_tree(&raw))
    }

    /// Fetch Kids video info via parallel `/player` + `/next` on the YTKIDS
    /// client (legacy `Kids.getInfo`).
    ///
    /// ponytail: legacy attaches `session.player?.signatureTimestamp`; the
    /// manager has no player handle, so the field is omitted (same as legacy
    /// when the player was not retrieved).
    pub async fn get_info(&self, video_id: &str) -> Result<KidsVideoInfo> {
        let mut player_payload = json!({
            "videoId": video_id,
            "racyCheckOk": true,
            "contentCheckOk": true,
            "playbackContext": {
                "contentPlaybackContext": {
                    "vis": 0,
                    "splay": false,
                    "lactMilliseconds": "-1"
                }
            }
        });

        if let Some(ref pot) = self.session.po_token {
            player_payload["serviceIntegrityDimensions"] = json!({ "poToken": pot });
        }

        let player_future = self
            .session
            .post_innertube_client("YTKIDS", "/player", player_payload);
        let next_future = self
            .session
            .post_innertube_client("YTKIDS", "/next", json!({ "videoId": video_id }));

        let (player_resp, next_resp) = tokio::join!(player_future, next_future);

        let player_response: PlayerResponse = player_resp?
            .json()
            .await
            .map_err(InnertubeError::Network)?;
        let watch_next: Option<Value> = match next_resp {
            Ok(resp) => resp.json().await.ok(),
            Err(_) => None,
        };

        Ok(KidsVideoInfo {
            player_response,
            watch_next,
            cpn: crate::utils::proto::generate_random_string(16),
        })
    }

    /// Fetch a Kids channel page (legacy `Kids.getChannel`), parsed into
    /// typed nodes.
    pub async fn get_channel(&self, channel_id: &str) -> Result<Vec<YTNode>> {
        let resp = self
            .session
            .post_innertube_client("YTKIDS", "/browse", json!({ "browseId": channel_id }))
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;
        Ok(Parser::parse_tree(&raw))
    }

    /// Block a channel for every supervised kids profile the signed-in user
    /// manages (legacy `Kids.blockChannel`). Requires authentication.
    pub async fn block_channel(&self, channel_id: &str) -> Result<Vec<ApiResponse>> {
        self.session.ensure_authenticated()?;

        let resp = self
            .session
            .post_innertube_client(
                "YTKIDS",
                "/kids/get_kids_blocklist_picker",
                json!({
                    "blockedForKidsContent": {
                        "external_channel_id": channel_id
                    }
                }),
            )
            .await?;
        let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

        // The picker dialog lives under command.confirmDialogEndpoint.content.
        let content = raw
            .pointer("/command/confirmDialogEndpoint/content")
            .cloned()
            .ok_or_else(|| {
                InnertubeError::Other(
                    "Could not find any kids profiles or supervised accounts.".to_string(),
                )
            })?;

        let fragment = json!({ "contents": content, "engagementPanels": [] });
        let tree = Parser::parse_tree(&fragment);

        let mut responses = Vec::new();
        for node in &tree {
            let YTNode::KidsBlocklistPickerItem(item) = node else {
                continue;
            };
            let Some(button) = item.block_button.as_ref() else {
                continue;
            };
            let toggle = button.get("toggleButtonRenderer").unwrap_or(button);
            let is_toggled = toggle
                .get("isToggled")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            if is_toggled {
                continue;
            }

            let endpoint_value = toggle
                .get("defaultServiceEndpoint")
                .or_else(|| toggle.get("defaultEndpoint"))
                .cloned();
            let Some(endpoint_value) = endpoint_value else {
                continue;
            };

            let endpoint = NavigationEndpointNode::from_value(&endpoint_value).ok_or_else(|| {
                InnertubeError::Format("Kids blocklist item endpoint is not navigable".to_string())
            })?;
            let path = endpoint.api_path.clone().ok_or_else(|| {
                InnertubeError::NotFound(
                    "Kids blocklist item endpoint has no InnerTube API path".to_string(),
                )
            })?;

            responses.push(
                Actions::execute(self.session, &path, endpoint.payload.clone()).await?,
            );
        }

        Ok(responses)
    }
}
