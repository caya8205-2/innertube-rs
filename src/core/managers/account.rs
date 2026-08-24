use crate::core::session::Session;
use crate::endpoints::account::{
    get_history, get_library, get_notifications, get_unseen_notifications_count,
};
use crate::error::Result;
use crate::models::account::{AccountNotificationsResponse, HistoryFeed, LibraryFeed};

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
}
