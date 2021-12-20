use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::application::dto::callback_data::CallbackData;
use crate::application::enumeration::callback_command::CallbackCommand;
use crate::application::wrapper::back_inline_keyboard_button::back_inline_keyboard_button;
use crate::application::wrapper::chat_process_preloader::chat_process_preloader;
use crate::application::wrapper::checkbox_inline_keyboard_button::checkbox_inline_keyboard_button;
use crate::application::wrapper::complete_inline_keyboard_button::complete_inline_keyboard_button;
use crate::application::wrapper::reply_markup::reply_markup;
use crate::domain::model::subscriber_favorite_topic::Raw as RawSubscriberFavoriteTopic;
use crate::domain::query::get_all_checkboxed_subscriber_topics::GetAllCheckboxedSubscriberTopics;
use crate::domain::query::is_subscriber_has_favorite_topics::IsSubscriberHasFavoriteTopics;
use crate::domain::query::toggle_subscriber_topic::ToggleSubscriberTopic;

pub async fn favorite_topic_selection(
    requester: &AutoSend<Bot>,
    callback_data: CallbackData<'_>,
    raw_favorite_topic: RawSubscriberFavoriteTopic,
) {
    let chat_id = callback_data.chat_id();
    let message_id = callback_data.message_id();

    let preloader_process = || -> Result<InlineKeyboardMarkup, ()> {
        if raw_favorite_topic.has_not_empty_topic() {
            ToggleSubscriberTopic::new(raw_favorite_topic.to_normal()).execute();
        }

        let mut markup_data: Vec<Vec<InlineKeyboardButton>> = vec![];
        let checkboxed_topics = GetAllCheckboxedSubscriberTopics::new(chat_id, raw_favorite_topic.site.id()).execute();
        for input in checkboxed_topics {
            markup_data.push(vec![
                checkbox_inline_keyboard_button(
                    &input.name,
                    input.checked,
                    CallbackCommand::FavoriteTopicSelection(
                        RawSubscriberFavoriteTopic::new(chat_id, input.topic_id, raw_favorite_topic.site),
                    ).as_inline_button_kind(),
                ),
            ]);
        }

        let mut extra_buttons = vec![
            back_inline_keyboard_button(CallbackCommand::FavoriteSiteSelection.as_inline_button_kind()),
        ];
        if IsSubscriberHasFavoriteTopics::new(chat_id).execute() {
            extra_buttons.push(complete_inline_keyboard_button(
                CallbackCommand::FinishFavoriteSelection.as_inline_button_kind()
            ));
        }
        markup_data.push(extra_buttons);

        Ok(InlineKeyboardMarkup::new(markup_data))
    };

    requester.delete_message(chat_id, message_id).await.ok();
    let markup = chat_process_preloader(&requester, chat_id, preloader_process).await.unwrap();
    reply_markup(&requester, chat_id, "Please select manga to subscribe:", markup).await.unwrap();
}