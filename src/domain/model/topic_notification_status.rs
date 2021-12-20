use crate::domain::schema::topic_notification_status;
use crate::domain::model::topic::Topic;

#[derive(Debug, Clone, Queryable, Associations)]
#[table_name = "topic_notification_status"]
#[belongs_to(Topic, foreign_key = "topic_id")]
pub struct TopicNotificationStatus {
    pub topic_id: i32,
    pub last_chapter_title: String,
    pub updated_at: chrono::NaiveDateTime
}

#[derive(Debug, Insertable, AsChangeset, Hash, Eq, PartialEq)]
#[table_name = "topic_notification_status"]
pub struct New {
    pub topic_id: i32,
    pub last_chapter_title: String,
}

impl New {
    pub(crate) fn new(topic_id: i32, last_chapter_title: String) -> New {
        New { topic_id, last_chapter_title }
    }
}
