use log::info;
use teloxide::prelude::*;
use crate::application::dto::callback_data::CallbackData;
use crate::application::wrapper::chat_process_preloader::chat_process_preloader;
use crate::application::wrapper::send_message::send_message;
use crate::domain::query::get_current_state_of_subscriptions::GetCurrentStateOfSubscriptions;

pub async fn finish_favorite_selection(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>) {
    let chat_id = callback_data.chat_id();
    let message_id = callback_data.message_id();

    let preloader_process = || -> Result<String, ()> {
        let mut answer = "".to_string();
        for history in GetCurrentStateOfSubscriptions::new(chat_id).execute() {
            info!("   {}", history.to_log());
            answer.push_str(&format!("\n\n{}", history.formatted()));
        }

        Ok(answer)
    };

    let message = chat_process_preloader(&requester, chat_id, preloader_process)
        .await.unwrap();

    requester.delete_message(chat_id, message_id).await.ok();
    send_message(&requester, chat_id, &message).await.unwrap();
}