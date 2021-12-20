use crate::domain::model::topic_notification_status::{New as NewTopicNotificationStatus};
use crate::domain::pool::establish_connection;
use crate::domain::schema::topic_notification_status::dsl::{topic_notification_status, topic_id, last_chapter_title};

use diesel::pg::upsert::excluded;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{RunQueryDsl, insert_into, ExpressionMethods};

pub struct UpsertTopicNotificationStatus {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    entities: Vec<NewTopicNotificationStatus>
}

impl<'a> UpsertTopicNotificationStatus {
    pub(crate) fn new(entities: Vec<NewTopicNotificationStatus>) -> UpsertTopicNotificationStatus {
        UpsertTopicNotificationStatus {
            entities,
            connection: establish_connection()
        }
    }

    pub fn execute(&self) {
        insert_into(topic_notification_status)
            .values(&self.entities)
            .on_conflict(topic_id)
            .do_update()
            .set(last_chapter_title.eq(excluded(last_chapter_title)))
            .execute(&self.connection)
            .expect("Error upserting a topic notification status");

    }
}
