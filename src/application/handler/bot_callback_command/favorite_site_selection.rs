use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;
use crate::application::dto::callback_data::CallbackData;
use crate::application::enumeration::callback_command::CallbackCommand;
use crate::application::wrapper::complete_inline_keyboard_button::complete_inline_keyboard_button;
use crate::application::wrapper::reply_markup::reply_markup;
use crate::application::wrapper::sites_to_inline_keyboards::sites_to_inline_keyboards;
use crate::domain::query::is_subscriber_has_favorite_topics::IsSubscriberHasFavoriteTopics;

pub async fn favorite_site_selection(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>) {
    let chat_id = callback_data.chat_id();
    let message_id = callback_data.message_id();

    let mut keyboards = sites_to_inline_keyboards(chat_id);
    if IsSubscriberHasFavoriteTopics::new(chat_id).execute() {
        keyboards.push(vec![complete_inline_keyboard_button(
            CallbackCommand::FinishFavoriteSelection.as_inline_button_kind()
        )]);
    }
    let markup = InlineKeyboardMarkup::new(keyboards);

    requester.delete_message(chat_id, message_id).await.ok();
    reply_markup(&requester, chat_id, "Please select a site to find your favorite manga:", markup).await.unwrap();
}