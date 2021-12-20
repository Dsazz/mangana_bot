use teloxide::prelude::*;
use crate::application::action::notify_subscribers::NotifySubscribers;
use crate::application::dto::callback_data::CallbackData;
use crate::application::wrapper::chat_process_preloader::{chat_async_process_preloader};
use crate::application::wrapper::show_alert::show_alert;
use crate::domain::query::find_app_notification::FindAppNotification;

pub async fn send_app_notification(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>, notification_id: i32) {
    let chat_id = callback_data.chat_id();
    chat_async_process_preloader(&requester, chat_id, preloader_process(requester, callback_data, notification_id)).await.unwrap();
}

//@todo need to handle correctly
async fn preloader_process(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>, notification_id: i32) -> Result<(), ()> {
    let chat_id = callback_data.chat_id();
    let callback_id = callback_data.callback_id();
    let message_id = callback_data.message_id();

    match FindAppNotification::new(notification_id).execute() {
        None => {
            show_alert(&requester, callback_id, &format!("Уведомление c ID {} не найдено", notification_id)).await.ok();
            Ok(())
        }
        Some(data) => {
            requester.delete_message(chat_id, message_id).await.ok();
            NotifySubscribers::new(data.text).execute().await.ok();

            show_alert(&requester, callback_id, "Все подписчики были уведомлены").await.ok();
            Ok(())
        },
    }
}