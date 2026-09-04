use serde_json::{json, Value};

use crate::core::session::Session;
use crate::error::{InnertubeError, Result};
use crate::models::account::{
    AccountNotification, AccountNotificationsResponse, HistoryFeed, LibraryFeed,
};
use crate::parser::nodes::misc::text::TextNode;
use crate::parser::nodes::misc::thumbnail::ThumbnailListNode;
use crate::parser::nodes::video::VideoNode;
use crate::parser::{NodeListExt, Parser};

/// Fetch authenticated user watch history (`FEhistory`).
pub async fn get_history(session: &Session, continuation_token: Option<&str>) -> Result<HistoryFeed> {
    let payload = if let Some(token) = continuation_token {
        json!({
            "continuation": token,
        })
    } else {
        json!({
            "browseId": "FEhistory",
        })
    };

    let resp = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let parsed_tree = Parser::parse_tree(&raw);
    let videos = parsed_tree.find_videos().into_iter().cloned().collect();
    let continuation = parsed_tree.find_continuation_token();

    Ok(HistoryFeed {
        videos,
        continuation_token: continuation,
    })
}

/// Fetch authenticated user library (`FElibrary`).
///
/// Videos are grouped by their shelf (legacy `Library` sections keyed by
/// shelf icon type) instead of positional splits.
pub async fn get_library(session: &Session) -> Result<LibraryFeed> {
    let payload = json!({
        "browseId": "FElibrary",
    });

    let resp = session.post_innertube("/browse", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let sections = parse_library_sections(&raw);

    let by_icon = |needle: &str| -> Vec<VideoNode> {
        sections
            .iter()
            .find(|s| {
                s.icon_type
                    .as_deref()
                    .is_some_and(|i| i.contains(needle))
                    || s.title.contains(needle)
            })
            .map(|s| s.videos.clone())
            .unwrap_or_default()
    };

    Ok(LibraryFeed {
        history_videos: by_icon("HISTORY"),
        watch_later_videos: by_icon("WATCH_LATER"),
        liked_videos: by_icon("LIKE"),
        playlists_count: Parser::parse_tree(&raw).find_playlists().len(),
        sections,
    })
}

/// Parse library shelves preserving grouping (legacy `Library.sections`).
fn parse_library_sections(raw: &Value) -> Vec<crate::models::account::LibrarySection> {
    let mut sections = Vec::new();

    fn walk(v: &Value, out: &mut Vec<crate::models::account::LibrarySection>) {
        if let Some(shelf) = v.get("shelfRenderer") {
            let title = TextNode::from_value(shelf.get("title").unwrap_or(&Value::Null))
                .map(|t| t.text)
                .unwrap_or_default();
            let icon_type = shelf
                .pointer("/icon/iconType")
                .and_then(Value::as_str)
                .map(ToString::to_string);
            let items = shelf
                .pointer("/content/horizontalListRenderer/items")
                .or_else(|| shelf.pointer("/content/verticalListRenderer/items"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            let mut videos = Vec::new();
            for item in &items {
                if let Some(video) = crate::parser::nodes::video::VideoNode::from_value(item) {
                    videos.push(video);
                }
            }

            out.push(crate::models::account::LibrarySection {
                title,
                icon_type,
                videos,
            });
            return;
        }
        match v {
            Value::Object(map) => map.values().for_each(|x| walk(x, out)),
            Value::Array(items) => items.iter().for_each(|x| walk(x, out)),
            _ => {}
        }
    }

    walk(raw, &mut sections);
    sections
}

/// Fetch account notifications (`POST /notification/get_notification_menu`).
pub async fn get_notifications(session: &Session) -> Result<AccountNotificationsResponse> {
    let payload = json!({
        "notificationsMenuRequestType": "NOTIFICATIONS_MENU_REQUEST_TYPE_INBOX"
    });

    let resp = session.post_innertube("/notification/get_notification_menu", payload).await?;
    let raw: Value = resp.json().await.map_err(InnertubeError::Network)?;

    let mut notifications = Vec::new();

    if let Some(items) = raw.pointer("/actions/0/openPopupAction/popup/multiPageMenuRenderer/sections/0/multiPageMenuNotificationSectionRenderer/items").and_then(|i| i.as_array()) {
        for item in items {
            if let Some(nr) = item.get("notificationRenderer") {
                let id = nr.get("notificationId").and_then(Value::as_str).unwrap_or("").to_string();
                let title = TextNode::from_value(nr.get("shortMessage").unwrap_or(&Value::Null))
                    .map(|t| t.text)
                    .unwrap_or_default();
                let sent_time = TextNode::from_value(nr.get("sentTimeText").unwrap_or(&Value::Null))
                    .map(|t| t.text);
                let thumbnail = ThumbnailListNode::from_value(nr.get("thumbnail").unwrap_or(nr)).best_url().map(|u| u.to_string());
                let video_id = nr.pointer("/navigationEndpoint/watchEndpoint/videoId")
                    .or_else(|| nr.pointer("/navigationEndpoint/commandExecutorCommand/commands/0/watchEndpoint/videoId"))
                    .and_then(Value::as_str)
                    .map(|s| s.to_string());
                let is_read = nr.get("read").and_then(Value::as_bool).unwrap_or(false);

                if !title.is_empty() {
                    notifications.push(AccountNotification {
                        id,
                        title,
                        sent_time,
                        thumbnail,
                        video_id,
                        is_read,
                    });
                }
            }
        }
    }

    let continuation_token = raw
        .pointer("/actions/0/openPopupAction/popup/multiPageMenuRenderer/sections/0/multiPageMenuNotificationSectionRenderer/items")
        .and_then(|items| Parser::parse_tree(items).find_continuation_token());

    Ok(AccountNotificationsResponse {
        notifications,
        continuation_token,
    })
}

/// Fetch the number shown by YouTube's unread-notifications indicator.
pub async fn get_unseen_notifications_count(session: &Session) -> Result<u64> {
    let response = session
        .post_innertube("/notification/get_unseen_count", json!({}))
        .await?;
    let raw: Value = response.json().await.map_err(InnertubeError::Network)?;
    Ok(parse_unseen_notifications_count(&raw))
}

/// Parse both response layouts accepted by the legacy implementation.
pub fn parse_unseen_notifications_count(raw: &Value) -> u64 {
    raw.get("unseenCount")
        .or_else(|| raw.pointer("/actions/0/updateNotificationsUnseenCountAction/unseenCount"))
        .and_then(value_as_u64)
        .unwrap_or(0)
}

fn value_as_u64(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|count| count.parse().ok()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_top_level_unseen_count() {
        assert_eq!(parse_unseen_notifications_count(&json!({ "unseenCount": 4 })), 4);
    }

    #[test]
    fn parses_action_unseen_count_and_defaults_to_zero() {
        let action_response = json!({
            "actions": [{
                "updateNotificationsUnseenCountAction": { "unseenCount": "7" }
            }]
        });

        assert_eq!(parse_unseen_notifications_count(&action_response), 7);
        assert_eq!(parse_unseen_notifications_count(&json!({})), 0);
    }
}
