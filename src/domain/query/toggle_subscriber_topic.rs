use log::{info};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, select, insert_into, delete};
use diesel::dsl::{exists};
use tokio::sync::Mutex;
use crate::domain::model::subscriber_favorite_topic::SubscriberFavoriteTopic;
use crate::domain::schema::subscriber_topics::dsl::{subscriber_topics, chat_id as chat_column, topic_id as topic_column};


pub struct ToggleSubscriberTopic {
    entity: SubscriberFavoriteTopic,
    connection: Mutex<PooledConnection<ConnectionManager<PgConnection>>>,
}

impl ToggleSubscriberTopic {
    pub(crate) fn new(entity: SubscriberFavoriteTopic) -> ToggleSubscriberTopic {
        ToggleSubscriberTopic { entity, connection: Mutex::new(establish_connection()) }
    }

    pub fn execute(&mut self) {
        if self.subscription_exists() {
            info!("  |> Unsubscribe user ID {:?} from topic {:?}", &self.entity.chat_id, &self.entity.topic_id);
            self.unsubscribe_from_topic();
        } else {
            info!("  |> Subscribe user ID {:?} to topic {:?}", &self.entity.chat_id, &self.entity.topic_id);
            self.subscribe_to_topic();
        }
    }

    fn subscribe_to_topic(&mut self) -> usize {
        insert_into(subscriber_topics)
            .values(&self.entity)
            .execute(self.connection.get_mut())
            .expect("Error inserting a new subscription")
    }

    fn unsubscribe_from_topic(&mut self) -> usize {
        delete(subscriber_topics.filter(
            chat_column.eq(&self.entity.chat_id).and(topic_column.eq(&self.entity.topic_id))
        ))
            .execute(self.connection.get_mut())
            .expect("Error deleting a subscription")
    }

    fn subscription_exists(&mut self) -> bool {
        select(exists(subscriber_topics
            .filter(
                chat_column.eq(&self.entity.chat_id).and(topic_column.eq(&self.entity.topic_id))
            )
        ))
            .get_result(self.connection.get_mut())
            .expect("Error loading is subscriber exists")
    }
}
