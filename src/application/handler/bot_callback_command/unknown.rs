use log::warn;
use teloxide::prelude::*;
use crate::application::dto::callback_data::CallbackData;
use crate::application::wrapper::show_alert::show_alert;

pub async fn unknown(requester: &AutoSend<Bot>, callback_data: CallbackData<'_>) {
    let callback_id = callback_data.callback_id();

    warn!("  |> unexpected callback command");
    show_alert(&requester, callback_id, "unexpected callback command").await.unwrap();
}