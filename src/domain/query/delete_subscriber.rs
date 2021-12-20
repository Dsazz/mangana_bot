use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::model::subscriber::Subscriber;
use crate::domain::pool::establish_connection;
use crate::domain::schema::subscribers::dsl::{subscribers as delete_subscriber, chat_id as chat_id_column};
use diesel::{delete, QueryDsl, ExpressionMethods, RunQueryDsl};


pub struct DeleteSubscriber {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    entity: Subscriber,
}

impl DeleteSubscriber {
    pub(crate) fn new(entity: Subscriber) -> DeleteSubscriber {
        DeleteSubscriber {
            entity,
            connection: establish_connection()
        }
    }

    pub fn execute(&self) -> usize {
        delete(delete_subscriber.filter(chat_id_column.eq(&self.entity.chat_id)))
            .execute(&self.connection)
            .expect("Cant delete a subscriber from DB")
    }
}
