use crate::domain::enumeration::site::Site;
use crate::domain::schema::subscriber_topics;
use crate::domain::model::topic::Topic;
use crate::domain::model::subscriber::Subscriber;

const EMPTY_TOPIC_ID: i32 = 0;

#[derive(Identifiable, Clone, Queryable, Debug, Insertable, Associations)]
#[primary_key(chat_id, topic_id)]
#[belongs_to(Subscriber, foreign_key = "chat_id")]
#[belongs_to(Topic, foreign_key = "topic_id")]
#[table_name = "subscriber_topics"]
pub struct SubscriberFavoriteTopic {
    pub chat_id: i64,
    pub topic_id: i32,
}
impl SubscriberFavoriteTopic {
    pub(crate) fn new(chat_id: i64, topic_id: i32) -> SubscriberFavoriteTopic {
        SubscriberFavoriteTopic { chat_id, topic_id }
    }
}

#[derive(Identifiable, Clone, Queryable, Debug, Associations, PartialEq)]
#[primary_key(chat_id, topic_id)]
#[belongs_to(Subscriber, foreign_key = "chat_id")]
#[belongs_to(Topic, foreign_key = "topic_id")]
#[table_name = "subscriber_topics"]
pub struct Raw {
    pub chat_id: i64,
    pub topic_id: i32,
    pub site: Site,
}
impl Raw {
    pub(crate) fn new(chat_id: i64, topic_id: i32, site: Site) -> Raw {
        Raw { chat_id, topic_id, site }
    }
    pub(crate) fn new_without_topic(chat_id: i64, site: Site) -> Raw {
        Raw { chat_id, topic_id: EMPTY_TOPIC_ID, site }
    }

    pub fn has_not_empty_topic(&self) -> bool {
        !self.has_empty_topic()
    }
    pub fn has_empty_topic(&self) -> bool {
        self.topic_id == EMPTY_TOPIC_ID
    }
    pub fn to_normal(&self) -> SubscriberFavoriteTopic {
        SubscriberFavoriteTopic::new(self.chat_id, self.topic_id)
    }
}