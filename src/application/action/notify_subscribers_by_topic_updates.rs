use log::info;
use std::error::Error;
use std::collections::HashSet;
use crate::application::bot::BOT;
use crate::domain::query::get_all_subscribers::GetAllSubscribers;
use crate::application::wrapper::send_message::{send_message};
use crate::domain::model::topic_notification_status::New as NewTopicNotificationStatus;
use crate::domain::query::get_all_updates_by_subscription::GetAllUpdatesBySubscription;
use crate::domain::query::upsert_topic_notification_status::UpsertTopicNotificationStatus;


pub struct NotifySubscribersByTopicUpdates {}

impl NotifySubscribersByTopicUpdates {
    pub(crate) fn new() -> NotifySubscribersByTopicUpdates {
        NotifySubscribersByTopicUpdates {}
    }

    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut topic_notification_statuses: HashSet<NewTopicNotificationStatus> = HashSet::new();

        //@todo parallelize this
        for subscriber in GetAllSubscribers::new().execute() {
            info!("  |> notify subscriber: {}", subscriber.chat_id);

            let mut message = "".to_string();
            for history in GetAllUpdatesBySubscription::new(subscriber.chat_id).execute() {
                info!("  {}", history.to_log());
                message.push_str(&format!("\n\n{}", history.formatted()));
                topic_notification_statuses.insert(
                    NewTopicNotificationStatus::new(history.topic.id, history.last_chapter_title),
                );
            }
            if message.is_empty() {
                info!("  |> there is nothing to notify");
                continue;
            }

            send_message(&BOT, subscriber.chat_id, &message).await.unwrap();
        }

        if !topic_notification_statuses.is_empty() {
            info!("  |> update topic notification statuses \n");
            UpsertTopicNotificationStatus::new(topic_notification_statuses.into_iter().collect()).execute();
        }

        info!("  |> all subscribers has been notified \n");
        Ok(())
    }
}