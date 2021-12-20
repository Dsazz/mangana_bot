use strum::IntoEnumIterator;
use teloxide::types::InlineKeyboardButton;
use crate::domain::enumeration::site::Site;
use crate::domain::model::subscriber_favorite_topic::{Raw as RawSubscriberFavoriteTopic};
use crate::application::enumeration::callback_command::CallbackCommand;

pub fn sites_to_inline_keyboards(chat_id: i64) -> Vec<Vec<InlineKeyboardButton>> {
    let mut inline_keyboards: Vec<Vec<InlineKeyboardButton>> = vec![];
    for site in Site::iter() {
        inline_keyboards.push(vec![
            InlineKeyboardButton::new(
                site.name(),
                CallbackCommand::FavoriteTopicSelection(
                    RawSubscriberFavoriteTopic::new_without_topic(chat_id, site)
                ).as_inline_button_kind(),
            ),
        ]);
    }

    inline_keyboards
}
