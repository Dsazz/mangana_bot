use anyhow::{Result, Error};
use std::cmp::PartialEq;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind};
use crate::domain::enumeration::site::Site;
use crate::domain::model::subscriber_favorite_topic::Raw as SubscriberFavoriteTopic;

const SEND_APP_NOTIFICATION_TYPE: &str = "/send_app_notification";
const CANCEL_APP_NOTIFICATION_TYPE: &str = "/cancel_app_notification";
const SUBSCRIBE_TYPE: &str = "/subscribe";
const FAVORITE_SITE_SELECTION_TYPE: &str = "/favorite_site_selection";
const FAVORITE_TOPIC_SELECTION_TYPE: &str = "/favorite_topic_selection";
const FINISH_FAVORITE_SELECTION_TYPE: &str = "/finish_favorite_selection";
const UNKNOWN_TYPE: &str = "/unknown";

#[derive(Debug, PartialEq, Clone)]
pub enum CallbackCommand {
    SendAppNotification(i32),
    CancelAppNotification(i32),
    Subscribe,
    FavoriteTopicSelection(SubscriberFavoriteTopic),
    FavoriteSiteSelection,
    FinishFavoriteSelection,
    Unknown,
}

impl CallbackCommand {
    pub fn as_string(&self) -> &'static str {
        match self {
            CallbackCommand::SendAppNotification(_) => SEND_APP_NOTIFICATION_TYPE,
            CallbackCommand::CancelAppNotification(_) => CANCEL_APP_NOTIFICATION_TYPE,
            CallbackCommand::Subscribe => SUBSCRIBE_TYPE,
            CallbackCommand::FavoriteSiteSelection => FAVORITE_SITE_SELECTION_TYPE,
            CallbackCommand::FavoriteTopicSelection(_) => FAVORITE_TOPIC_SELECTION_TYPE,
            CallbackCommand::FinishFavoriteSelection => FINISH_FAVORITE_SELECTION_TYPE,
            CallbackCommand::Unknown => UNKNOWN_TYPE,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            CallbackCommand::SendAppNotification(_) => "Send",
            CallbackCommand::CancelAppNotification(_) => "Cancel",
            CallbackCommand::Subscribe => "Subscribe",
            CallbackCommand::FavoriteSiteSelection => "Select a favorite site",
            CallbackCommand::FavoriteTopicSelection(_) => "Select favorite topics",
            CallbackCommand::FinishFavoriteSelection => "Complete",
            CallbackCommand::Unknown => "",
        }
    }

    pub fn as_inline_button(&self) -> InlineKeyboardButton {
        InlineKeyboardButton::new(self.get_name(), self.as_inline_button_kind())
    }

    pub fn as_inline_button_kind(&self) -> InlineKeyboardButtonKind {
        InlineKeyboardButtonKind::CallbackData(self.as_args_string())
    }

    pub fn as_args_string(&self) -> String {
        match self {
            CallbackCommand::SendAppNotification(id) | CallbackCommand::CancelAppNotification(id) => {
                format!("{} {}", self.as_string(), id)
            },
            CallbackCommand::FavoriteTopicSelection(subscriber_favorite_topic) => {
                format!("{} {} {} {}", self.as_string(), subscriber_favorite_topic.chat_id, subscriber_favorite_topic.site.id(), subscriber_favorite_topic.topic_id)
            }
            _ => {
                self.as_string().to_string()
            },
        }
    }

    pub(crate) fn from_string(variant: &str, args: &[&str]) -> Result<CallbackCommand, Error> {
        match variant {
            SEND_APP_NOTIFICATION_TYPE => {
                let notification_id = args[0].parse::<i32>().map_err(|_| anyhow!("Failed to parse notification ID from {}", args[0]))?;
                Ok(CallbackCommand::SendAppNotification(notification_id))
            },
            CANCEL_APP_NOTIFICATION_TYPE => {
                let notification_id = args[0].parse::<i32>().map_err(|_| anyhow!("Failed to parse notification ID from {}", args[0]))?;
                Ok(CallbackCommand::CancelAppNotification(notification_id))
            },
            FAVORITE_TOPIC_SELECTION_TYPE => {
                let chat_id = args[0].parse::<i64>().map_err(|_| anyhow!("Failed to parse chat ID from {}", args[0]))?;
                let site_id = args[1].parse::<i32>().map_err(|_| anyhow!("Failed to parse site ID from {}", args[1]))?;
                let site = match Site::new(site_id) {
                    Ok(id) => id,
                    //@todo need to handle correctly
                    Err(error) => panic!("Failed to convert site ID to Site enum: {:?}", error),
                };
                let topic_id = args[2].parse::<i32>().map_err(|_| anyhow!("Failed to parse topic ID from {}", args[2]))?;

                Ok(CallbackCommand::FavoriteTopicSelection(SubscriberFavoriteTopic::new(chat_id, topic_id, site)))
            },
            FAVORITE_SITE_SELECTION_TYPE => Ok(CallbackCommand::FavoriteSiteSelection),
            FINISH_FAVORITE_SELECTION_TYPE => Ok(CallbackCommand::FinishFavoriteSelection),
            SUBSCRIBE_TYPE => Ok(CallbackCommand::Subscribe),

            _ => Ok(CallbackCommand::Unknown),
        }
    }
}
