use anyhow::Result;
use log::info;

use teloxide::types::{InlineKeyboardMarkup};
use teloxide::prelude::*;

use crate::domain::query::add_app_notification::AddAppNotification;
use crate::domain::model::app_notifications::NewAppNotification;
use crate::domain::query::find_subscriber::FindSubscriber;
use crate::domain::query::delete_subscriber::DeleteSubscriber;
use crate::application::handler::types::BotContext;
use crate::application::dto::config::get_config;
use crate::application::enumeration::command::Command;
use crate::application::wrapper::reply_markup::reply_markup;
use crate::application::wrapper::send_message::send_escaped_message;
use crate::application::enumeration::callback_command::CallbackCommand;


pub struct CommandMessage {
    cx: BotContext<Message>,
    command: Command,
}

impl CommandMessage {
    pub(crate) fn new(data: (BotContext<Message>, Command)) -> CommandMessage {
        CommandMessage { cx: data.0, command: data.1 }
    }

    pub async fn handle(&self) -> Result<()> {
        match self.command {
            Command::Start => self.start().await,
            Command::Unsubscribe => self.unsubscribe().await,
            Command::Subscribe => self.subscribe().await,
            Command::Admin => self.admin().await,
        }
    }

    async fn admin(&self) -> Result<()> {
        let requester = &self.cx.requester;
        let chat_id = self.cx.update.chat.id;

        if !get_config().is_user_admin(self.cx.update.chat.id) {
            return send_escaped_message(requester, chat_id, "You do not have administrator rights!").await;
        }

        let command_arg = self.cx.update.text().unwrap().replace("/admin ", "");
        let app_notification = AddAppNotification::new(NewAppNotification {
            text: command_arg, notification_type: "admin".to_owned()
        }).execute();

        let markup = InlineKeyboardMarkup::new(vec![
            vec![
                CallbackCommand::SendAppNotification(app_notification.id).as_inline_button(),
                CallbackCommand::CancelAppNotification(app_notification.id).as_inline_button(),
            ]
        ]);

        reply_markup(requester, chat_id, "Please confirm send notification", markup).await
    }

    async fn start(&self) -> Result<()> {
        let markup = InlineKeyboardMarkup::new(vec![
            vec![CallbackCommand::Subscribe.as_inline_button()]
        ]);

        reply_markup(&self.cx.requester, self.cx.update.chat.id,
        "Hi there! Welcome to Manger parser! Please subscribe to be on top of the latest updates",
            markup
        ).await
    }

    async fn subscribe(&self) -> Result<()> {
        let markup = InlineKeyboardMarkup::new(vec![
            vec![CallbackCommand::Subscribe.as_inline_button()]
        ]);

        reply_markup(&self.cx.requester, self.cx.update.chat.id,
            "Please click the button to start the subscription process:",
            markup
        ).await
    }

    async fn unsubscribe(&self) -> Result<()> {
        let requester = &self.cx.requester;
        let chat_id = self.cx.update.chat.id;
        info!("|> start unsubscribing chat id: {}", chat_id);

        let subscriber = FindSubscriber::new(chat_id).execute();
        match subscriber {
            None => send_escaped_message(requester, chat_id, "You are not unsubscribed!").await,

            Some(subscriber) => {
                info!("  * removing subscriber: {}", subscriber.chat_id);
                DeleteSubscriber::new(subscriber).execute();
                send_escaped_message(requester, chat_id, "You have been unsubscribed!").await
            }
        }
    }
}