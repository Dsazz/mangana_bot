use teloxide::prelude::*;
use teloxide::Bot;

pub type BotContext<T> = UpdateWithCx<AutoSend<Bot>, T>;