use teloxide::utils::command::BotCommand;

#[derive(BotCommand, PartialEq)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "start command.")]
    Start,
    #[command(description = "subscribe to parser")]
    Subscribe,
    #[command(description = "unsubscribe from parser")]
    Unsubscribe,
    Admin,
}

