use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::{BigInt, Integer};
use crate::domain::model::checkboxed_subscriber_topic::CheckboxedSubscriberTopic;


pub struct GetAllCheckboxedSubscriberTopics {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    chat_id: i64,
    site_id: i32,
}

impl GetAllCheckboxedSubscriberTopics {
    pub(crate) fn new(chat_id: i64, site_id: i32) -> GetAllCheckboxedSubscriberTopics {
        GetAllCheckboxedSubscriberTopics {
            connection: establish_connection(),
            chat_id,
            site_id,
        }
    }

    pub fn execute(&self) -> Vec<CheckboxedSubscriberTopic> {
        sql_query("
            SELECT t.id as topic_id, t.name AS name, CASE WHEN st.chat_id::integer IS NULL THEN FALSE ELSE TRUE END AS checked
            FROM topic t
            LEFT JOIN subscriber_topics st ON st.topic_id = t.id and st.chat_id = $1
            WHERE t.site_id = $2;
        ")
           .bind::<BigInt, _>(&self.chat_id)
           .bind::<Integer, _>(&self.site_id)
           .get_results(&self.connection)
           .expect("Error loading topics")
    }
}
