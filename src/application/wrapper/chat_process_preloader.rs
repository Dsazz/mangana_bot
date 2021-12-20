use std::future::Future;
use teloxide::Bot;
use teloxide::prelude::{AutoSend, Requester};
use crate::application::dto::config::get_config;

pub async fn chat_process_preloader<F, T, E>(requester: &AutoSend<Bot>, chat_id: i64, process: F) -> Result<T, E>
    where F: FnOnce() -> Result<T, E>,
{
    // run animation
    let animation = requester
        .send_animation(chat_id, get_config().preloader_animation()).await.unwrap();

    let result = process();

    // stop animation
    requester.delete_message(chat_id, animation.id).await.unwrap();

    result
}

pub async fn chat_async_process_preloader<Fut, T, E>(requester: &AutoSend<Bot>, chat_id: i64, process: Fut) -> Result<T, E>
    where Fut: Future<Output = Result<T, E>>,
{
    // run animation
    let animation = requester.send_animation(chat_id, get_config().preloader_animation()).await.unwrap();
    let result = process.await;
    // stop animation
    requester.delete_message(chat_id, animation.id).await.unwrap();

    result
}

