use log::info;
use crate::application::action::find_new_history_updates::FindNewHistoryUpdates;
use crate::application::action::notify_subscribers_by_topic_updates::NotifySubscribersByTopicUpdates;

//@todo change to ParseJob class with DI (bot connection, etc.)
pub async fn job() {
    info!("|> start a parse job:\n");
    FindNewHistoryUpdates::new().execute().await.unwrap();
    NotifySubscribersByTopicUpdates::new().execute().await.ok();
}