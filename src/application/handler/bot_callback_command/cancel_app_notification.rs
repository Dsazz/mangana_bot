use teloxide::prelude::*;
use crate::application::dto::callback_data::CallbackData;
use crate::application::wrapper::show_alert::show_alert;
use crate::domain::query::delete_app_notificaion::DeleteAppNotification;

pub async fn cancel_app_notification(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>, notification_id: i32) {
    let chat_id = callback_data.chat_id();
    let callback_id = callback_data.callback_id();
    let message_id = callback_data.message_id();

    DeleteAppNotification::new(notification_id).execute();
    show_alert(&requester, callback_id, "Notification has been canceled").await.unwrap();
    requester.delete_message(chat_id, message_id).await.ok();
}