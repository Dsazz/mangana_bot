use log::info;
use teloxide::types::Message;
use teloxide::prelude::CallbackQuery;
use crate::application::enumeration::callback_command::CallbackCommand;

#[derive(Debug)]
pub struct CallbackData<'a> {
    callback_id: &'a str,
    message: &'a Message,
    data: Vec<&'a str>
}

impl<'a> CallbackData<'a> {
    pub(crate) fn new(query: &'a CallbackQuery) -> CallbackData<'a> {
        CallbackData {
            callback_id: &query.id,
            message: query.message.as_ref().unwrap(),
            data: CallbackData::parse_data(&query)
        }
    }

    pub fn is_data_empty(&self) -> bool {
        info!("|> callback data is empty!");
        self.data.is_empty()
    }

    pub fn callback_id(&self) -> &str {
        self.callback_id
    }

    pub fn chat_id(&self) -> i64 {
        self.message.chat_id()
    }

    pub fn message_id(&self) -> i32 {
        self.message.id
    }

    pub fn command(&self) -> CallbackCommand {
        if self.is_data_empty() { return CallbackCommand::Unknown; }

        CallbackCommand::from_string(self.data[0], &self.command_args())
            .unwrap_or(CallbackCommand::Unknown)
    }

    pub fn command_args(&self) -> Vec<&str> {
        if self.data.is_empty() {
            return vec![];
        }

        self.data[1..].into()
    }

    fn parse_data(query: &CallbackQuery) -> Vec<&str> {
        info!("|> callback query data: {:?}", query.data);
        query.data.as_ref().unwrap().split_whitespace().collect()
    }
}
