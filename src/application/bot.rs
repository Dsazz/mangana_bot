use teloxide::Bot;
use teloxide::prelude::*;

lazy_static! {
    pub static ref BOT: AutoSend<Bot> = {
        teloxide::Bot::from_env().auto_send()
    };
}