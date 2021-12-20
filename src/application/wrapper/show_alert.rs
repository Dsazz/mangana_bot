use anyhow::Result;
use teloxide::prelude::{AutoSend, Requester};
use teloxide::Bot;
use teloxide::prelude::*;

pub async fn show_alert(requester: &AutoSend<Bot>, callback_id: &str, text: &str) -> Result<()> {
    requester.answer_callback_query(callback_id)
        .show_alert(true)
        .text(text)
        .await?;
    
    Ok(())
}