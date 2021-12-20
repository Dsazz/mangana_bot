use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::dsl::count_star;
use crate::domain::pool::establish_connection;
use crate::domain::schema::subscriber_topics::dsl::{subscriber_topics, chat_id as chat_clm};

pub struct IsSubscriberHasFavoriteTopics {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    chat_id: i64,
}

impl IsSubscriberHasFavoriteTopics {
    pub(crate) fn new(chat_id: i64) -> IsSubscriberHasFavoriteTopics {
        IsSubscriberHasFavoriteTopics {
            connection: establish_connection(),
            chat_id,
        }
    }

    pub fn execute(&self) -> bool {
        0 != subscriber_topics
            .select(count_star())
            .filter(chat_clm.eq(self.chat_id))
            .first::<i64>(&self.connection)
            .expect("Error loading count of subscriber topics")
    }
}
