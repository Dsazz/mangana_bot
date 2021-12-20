use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use crate::domain::model::parse_history::{ParseHistory, Raw as RawParseHistory};
use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::BigInt;
use crate::domain::model::topic::Topic;


pub struct GetAllUpdatesBySubscription {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    chat_id: i64,
}

impl GetAllUpdatesBySubscription {
    pub(crate) fn new(chat_id: i64) -> GetAllUpdatesBySubscription {
        GetAllUpdatesBySubscription {
            connection: establish_connection(),
            chat_id,
        }
    }
    pub fn execute(&self) -> Vec<ParseHistory> {
        let result: Vec<(RawParseHistory, Topic)> = sql_query("
            SELECT ph.*, t.*
            FROM parse_histories ph
            INNER JOIN topic t ON ph.topic_id = t.id
            INNER JOIN subscriber_topics st ON st.topic_id = t.id
            LEFT JOIN topic_notification_status tns ON tns.topic_id = st.topic_id
            WHERE st.chat_id = $1 AND tns.last_chapter_title IS DISTINCT FROM ph.last_chapter_title;
        ")
            .bind::<BigInt, _>(self.chat_id)
            .get_results::<(RawParseHistory, Topic)>(&self.connection)
            .expect("Error loading parse histories");

        result.into_iter().map(|(r, t)| r.to_normal(t)).collect()
    }
}
