use anyhow::Result;
use teloxide::prelude::{AutoSend, Requester};
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;

pub async fn reply_markup(requester: &AutoSend<Bot>, chat_id: i64, text: &str, markup: InlineKeyboardMarkup) -> Result<()> {
    requester.send_message(chat_id, text)
        .reply_markup(markup)
        .await?;

    Ok(())
}