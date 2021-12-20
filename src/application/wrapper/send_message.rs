use anyhow::Result;
use teloxide::prelude::{AutoSend, Requester};
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::utils::markdown::escape;

pub async fn send_escaped_message(requester: &AutoSend<Bot>, chat_id: i64, text: &str) -> Result<()> {
    send_message(requester, chat_id, &escape(text)).await
}
pub async fn send_message(requester: &AutoSend<Bot>, chat_id: i64, text: &str) -> Result<()> {
    requester.send_message(chat_id, text)
        .parse_mode(MarkdownV2)
        .disable_web_page_preview(true)
        .send()
        .await?;

    Ok(())
}
