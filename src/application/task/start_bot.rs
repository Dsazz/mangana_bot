use log::{info};
use std::env;
use teloxide::prelude::*;
use teloxide::Bot;
use tokio_stream::wrappers::UnboundedReceiverStream;
use crate::application::handler::command_message::CommandMessage;
use crate::application::handler::my_chat_member_updated::MyChatMemberUpdated;
use crate::application::bot::BOT;
use crate::application::handler::bot_callback::handle;
use crate::application::enumeration::command::Command;

type DispatcherHandler<T> = DispatcherHandlerRx<AutoSend<Bot>, T>;

pub struct StartBot {
    /* bot_connection -> BOT.clone() */
}

impl StartBot {
    pub(crate) fn new(/*bot_connection*/) -> StartBot { StartBot {} }

    /// # Panics
    ///
    /// Will panic if something went wrong in handlers
    pub async fn execute(&self) {
        info!("|> run the bot dispatcher!\n");

        let bot_name = env::var("BOT_NAME").expect("BOT_NAME must be set");

        Dispatcher::new(BOT.clone())
            .messages_handler(move |rx: DispatcherHandler<Message>| {
                UnboundedReceiverStream::new(rx)
                    .commands::<Command, &str>(&bot_name)
                    //@todo try to remove 'move'
                    .for_each_concurrent(8, |cx| async move {
                        CommandMessage::new(cx).handle().await.unwrap();
                    })
            })
            .my_chat_members_handler(|rx: DispatcherHandler<ChatMemberUpdated>| {
                UnboundedReceiverStream::new(rx)
                    .for_each_concurrent(8, |message| async move {
                        MyChatMemberUpdated::new(message).handle().await;
                    })
            })
            .callback_queries_handler(|rx: DispatcherHandler<CallbackQuery>| {
                UnboundedReceiverStream::new(rx).for_each_concurrent(8, |message| async {
                    handle(message).await;
                })
            })
            .dispatch()
            .await

    }
}
