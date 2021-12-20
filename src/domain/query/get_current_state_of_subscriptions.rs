use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use crate::domain::model::parse_history::{ParseHistory, Raw as RawParseHistory};
use crate::domain::model::subscriber_favorite_topic::SubscriberFavoriteTopic;
use crate::domain::model::topic::Topic;
use crate::domain::schema::parse_histories::dsl::parse_histories;
use crate::domain::schema::topic::dsl::{topic as topic_table};
use crate::domain::schema::subscriber_topics::dsl::{subscriber_topics, chat_id as chat_clm};
use crate::diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};


pub struct GetCurrentStateOfSubscriptions {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    chat_id: i64,
}

impl GetCurrentStateOfSubscriptions {
    pub(crate) fn new(chat_id: i64) -> GetCurrentStateOfSubscriptions {
        GetCurrentStateOfSubscriptions {
            connection: establish_connection(),
            chat_id,
        }
    }
    pub fn execute(&self) -> Vec<ParseHistory> {
        let result: Vec<(RawParseHistory, Topic, SubscriberFavoriteTopic)> = parse_histories
            .inner_join(topic_table)
            .inner_join(subscriber_topics)
            .filter(chat_clm.eq(self.chat_id))
            .load::<(RawParseHistory, Topic, SubscriberFavoriteTopic)>(&self.connection)
            .expect("Error loading parse histories");

        result.into_iter().map(|(r, t, _s)| r.to_normal(t)).collect()
    }
}
