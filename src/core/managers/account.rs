use crate::core::session::Session;
use crate::endpoints::account::{
    get_history, get_library, get_notifications, get_unseen_notifications_count,
};
use crate::error::{InnertubeError, Result};
use crate::models::account::{
    AccountChannelItem, AccountNotificationsResponse, HistoryFeed, LibraryFeed,
};
use crate::parser::{Parser, YTNode};

/// Account Manager for user profile and notifications (1:1 with AccountManager.ts).
pub struct AccountManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> AccountManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Fetch authenticated user watch history (`FEhistory`).
    pub async fn get_history(&self, continuation_token: Option<&str>) -> Result<HistoryFeed> {
        get_history(self.session, continuation_token).await
    }

    /// Fetch authenticated user library (`FElibrary`).
    pub async fn get_library(&self) -> Result<LibraryFeed> {
        get_library(self.session).await
    }

    /// Fetch account notifications.
    pub async fn get_notifications(&self) -> Result<AccountNotificationsResponse> {
        get_notifications(self.session).await
    }

    /// Return the number shown by YouTube's unread-notifications indicator.
    pub async fn get_unseen_notifications_count(&self) -> Result<u64> {
        get_unseen_notifications_count(self.session).await
    }

    /// List channels belonging to the signed-in account (legacy
    /// `AccountManager.getInfo(true)`): `POST /account/accounts_list` with
    /// the channel-switcher request type on the WEB client.
    pub async fn get_accounts(&self) -> Result<Vec<AccountChannelItem>> {
        self.session.ensure_authenticated()?;

        let payload = serde_json::json!({
            "requestType": "ACCOUNTS_LIST_REQUEST_TYPE_CHANNEL_SWITCHER",
            "callCircumstance": "SWITCHING_USERS_FULL"
        });
        let resp = self
            .session
            .post_innertube("/account/accounts_list", payload)
            .await?;
        let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;

        let tree = Parser::parse_tree(&raw);
        let items = tree
            .iter()
            .filter_map(|node| match node {
                YTNode::AccountItem(item) => Some(AccountChannelItem {
                    account_name: item.account_name.clone(),
                    account_photo: item
                        .account_photo
                        .thumbnails
                        .last()
                        .map(|t| t.url.clone()),
                    is_selected: item.is_selected,
                }),
                _ => None,
            })
            .collect();

        Ok(items)
    }

    /// Active channel info for the signed-in account (legacy
    /// `AccountManager.getInfo()`): `POST /account/accounts_list` on the TV
    /// client. Throws when `on_behalf_of_user` was used, matching legacy.
    ///
    /// ponytail: legacy returns a typed `AccountInfo` page wrapper; we return
    /// the parsed `AccountItemNode` list. Typed page wrappers land with the
    /// parser response-assembly batch.
    pub async fn get_active_account(&self) -> Result<Vec<AccountChannelItem>> {
        self.session.ensure_authenticated()?;

        if self
            .session
            .context
            .user
            .as_ref()
            .and_then(|u| u.on_behalf_of_user.as_ref())
            .is_some()
        {
            return Err(InnertubeError::Other(
                "Boolean argument must be true when \"on_behalf_of_user\" is specified."
                    .to_string(),
            ));
        }

        let resp = self
            .session
            .post_innertube_client("TV", "/account/accounts_list", serde_json::json!({}))
            .await?;
        let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;

        let tree = Parser::parse_tree(&raw);
        Ok(tree
            .iter()
            .filter_map(|node| match node {
                YTNode::AccountItem(item) => Some(AccountChannelItem {
                    account_name: item.account_name.clone(),
                    account_photo: item
                        .account_photo
                        .thumbnails
                        .last()
                        .map(|t| t.url.clone()),
                    is_selected: item.is_selected,
                }),
                _ => None,
            })
            .collect())
    }

    /// Fetch YouTube settings (`SPaccount_overview`).
    ///
    /// ponytail: legacy returns a typed `Settings` page wrapper; we return the
    /// parsed node tree. Typed wrappers land with the parser
    /// response-assembly batch.
    pub async fn get_settings(&self) -> Result<Vec<YTNode>> {
        self.session.ensure_authenticated()?;

        let resp = self
            .session
            .post_innertube("/browse", serde_json::json!({ "browseId": "SPaccount_overview" }))
            .await?;
        let raw: serde_json::Value = resp.json().await.map_err(InnertubeError::Network)?;
        Ok(Parser::parse_tree(&raw))
    }
}
