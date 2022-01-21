use log::info;
use crate::application::action::search_for_topic_updates::SearchForTopicUpdates;
use crate::application::action::notify_subscribers_by_topic_updates::NotifySubscribersByTopicUpdates;

//@todo change to ParseJob class with DI (bot connection, etc.)
pub async fn job() {
    info!("|> start a parse job:\n");
    SearchForTopicUpdates::new().execute().await.unwrap();
    NotifySubscribersByTopicUpdates::new().execute().await.ok();
}