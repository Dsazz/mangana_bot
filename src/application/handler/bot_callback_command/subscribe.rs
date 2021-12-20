use log::info;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;
use crate::application::dto::callback_data::CallbackData;
use crate::application::wrapper::reply_markup::reply_markup;
use crate::application::wrapper::send_message::send_escaped_message;
use crate::application::wrapper::sites_to_inline_keyboards::sites_to_inline_keyboards;
use crate::domain::model::subscriber::Subscriber;
use crate::domain::query::add_subscriber::AddSubscriber;
use crate::domain::query::find_subscriber::FindSubscriber;

pub async fn subscribe(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>) {
    let chat_id = callback_data.chat_id();
    let message_id = callback_data.message_id();

    info!("  |> start subscribing chat id: {}", chat_id);
    let subscriber = FindSubscriber::new(chat_id).execute();
    match subscriber {
        None => {
            AddSubscriber::new(Subscriber { chat_id }).execute();
            requester.delete_message(chat_id, message_id).await.ok();
            send_escaped_message(&requester, chat_id, "You have been subscribed!").await.unwrap();

            let markup = InlineKeyboardMarkup::new(sites_to_inline_keyboards(chat_id));
            reply_markup(&requester, chat_id, "Please select a site to find your favorite manga:", markup).await.unwrap();
        },
        Some(_) => {
            send_escaped_message(&requester, chat_id, "You are already subscribed!").await.unwrap();
            requester.delete_message(chat_id, message_id).await.ok();
        }
    }
}