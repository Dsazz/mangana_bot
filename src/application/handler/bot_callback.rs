use log::info;
use teloxide::prelude::*;
use crate::application::dto::callback_data::CallbackData;
use crate::application::enumeration::callback_command::CallbackCommand;
use crate::application::handler::bot_callback_command::cancel_app_notification::cancel_app_notification;
use crate::application::handler::bot_callback_command::favorite_site_selection::favorite_site_selection;
use crate::application::handler::bot_callback_command::favorite_topic_selection::favorite_topic_selection;
use crate::application::handler::bot_callback_command::finish_favorite_selection::finish_favorite_selection;
use crate::application::handler::bot_callback_command::send_app_notification::send_app_notification;
use crate::application::handler::bot_callback_command::subscribe::subscribe;
use crate::application::handler::bot_callback_command::unknown::unknown;
use crate::application::handler::types::BotContext;


pub async fn handle(callback: BotContext<CallbackQuery>) {
    let callback_data = CallbackData::new(&callback.update);
    if callback_data.is_data_empty() {
        info!("  |>  CallbackData empty");
        return;
    }

    let requester = callback.requester;
    let command = callback_data.command();
    info!("  |> RUN command: {:?}", command);

    match command {
        CallbackCommand::SendAppNotification(notification_id) =>
            send_app_notification(&requester, callback_data, notification_id).await,

        CallbackCommand::CancelAppNotification(notification_id) =>
            cancel_app_notification(&requester, callback_data, notification_id).await,

        CallbackCommand::Subscribe => subscribe(&requester, callback_data).await,

        CallbackCommand::FavoriteSiteSelection =>
            favorite_site_selection(&requester, callback_data).await,

        CallbackCommand::FinishFavoriteSelection =>
            finish_favorite_selection(&requester, callback_data).await,

        CallbackCommand::FavoriteTopicSelection(raw_favorite_topic) =>
            favorite_topic_selection(&requester, callback_data, raw_favorite_topic).await,

        CallbackCommand::Unknown => unknown(&requester, callback_data).await,
    }
}
