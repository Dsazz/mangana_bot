use log::info;
use teloxide::prelude::*;
use crate::domain::model::subscriber::Subscriber;
use crate::domain::query::delete_subscriber::DeleteSubscriber;
use crate::application::handler::types::BotContext;

pub struct MyChatMemberUpdated {
    message: BotContext<ChatMemberUpdated>
}
impl MyChatMemberUpdated {
    pub(crate) fn new(message: BotContext<ChatMemberUpdated>) -> MyChatMemberUpdated {
        MyChatMemberUpdated { message }
    }

    pub async fn handle(&self) {
        let member_update = &self.message.update;
        info!("|> chat member @{:?} has updates", member_update.from.username);
        info!("   * member status: {:?}", member_update.new_chat_member.status());

        if member_update.new_chat_member.is_left() || member_update.new_chat_member.is_banned() {
            let chat_id = member_update.chat.id;
            info!("  |> member {} left the chat", chat_id);
            info!("  |> start unsubscribing chat id: {}", chat_id);

            DeleteSubscriber::new(Subscriber { chat_id }).execute();
        }
    }
}