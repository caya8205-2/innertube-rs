use crate::core::actions::Actions;
use crate::core::session::Session;
use crate::error::Result;
use crate::models::actions::{ActionResult, CreateCommentResult, NotificationPreferenceType};

/// Interaction Manager for ratings, subscriptions, comments, and preferences (1:1 with InteractionManager.ts).
pub struct InteractionManager<'a> {
    pub(crate) session: &'a Session,
}

impl<'a> InteractionManager<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }

    /// Like a video.
    pub async fn like(&self, video_id: &str) -> Result<ActionResult> {
        Actions::like(self.session, video_id).await
    }

    /// Dislike a video.
    pub async fn dislike(&self, video_id: &str) -> Result<ActionResult> {
        Actions::dislike(self.session, video_id).await
    }

    /// Remove rating from a video.
    pub async fn remove_rating(&self, video_id: &str) -> Result<ActionResult> {
        Actions::remove_rating(self.session, video_id).await
    }

    /// Subscribe to channel(s).
    pub async fn subscribe(&self, channel_ids: &[&str]) -> Result<ActionResult> {
        Actions::subscribe(self.session, channel_ids).await
    }

    /// Unsubscribe from channel(s).
    pub async fn unsubscribe(&self, channel_ids: &[&str]) -> Result<ActionResult> {
        Actions::unsubscribe(self.session, channel_ids).await
    }

    /// Post a comment on a video.
    pub async fn comment(&self, video_id: &str, text: &str) -> Result<CreateCommentResult> {
        Actions::create_comment(self.session, video_id, text).await
    }

    /// Set channel notification preference.
    pub async fn set_notification_preferences(
        &self,
        channel_id: &str,
        pref_type: NotificationPreferenceType,
    ) -> Result<ActionResult> {
        Actions::set_notification_preferences(self.session, channel_id, pref_type).await
    }

    /// Dispatch raw API call matching `Actions.execute`.
    pub async fn execute(&self, endpoint: &str, payload: serde_json::Value) -> Result<crate::core::actions::ApiResponse> {
        Actions::execute(self.session, endpoint, payload).await
    }
}
