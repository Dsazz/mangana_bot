use log::info;
use crate::application::bot::BOT;
use crate::domain::query::get_all_subscribers::GetAllSubscribers;
use std::error::Error;
use crate::application::wrapper::send_message::send_escaped_message;


pub struct NotifySubscribers {
    message: String,
}

impl NotifySubscribers {
    pub(crate) fn new(message: String) -> NotifySubscribers {
        NotifySubscribers { message }
    }

    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {

        if self.message.is_empty() {
            info!("  |> there is nothing to notify");
            return Err("there is nothing to notify".into());
        }

        info!("  |> notify all subscribers with message:\n {} \n", self.message.as_str());
        let subscribers = GetAllSubscribers::new().execute();

        //@todo parallelize this
        for subscriber in subscribers {
            info!("  |> notify subscriber: {}", subscriber.chat_id);
            //@todo need to handle exceptions correctly
            send_escaped_message(&BOT, subscriber.chat_id, self.message.as_str())
                .await.unwrap_or(());
        }

        info!("  |> all subscribers has been notified \n");
        Ok(())
    }
}