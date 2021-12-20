use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{insert_into, RunQueryDsl};
use crate::domain::model::subscriber::Subscriber;
use crate::domain::pool::establish_connection;
use crate::domain::schema::subscribers::dsl::{subscribers as add_subscriber, chat_id};


pub struct AddSubscriber {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    entity: Subscriber,
}

impl AddSubscriber {
    pub(crate) fn new(entity: Subscriber) -> AddSubscriber {
        AddSubscriber {
            entity,
            connection: establish_connection()
        }
    }

    pub fn execute(&self) -> usize {

        insert_into(add_subscriber)
            .values(&self.entity)
            .on_conflict(chat_id)
            .do_nothing()
            .execute(&self.connection)
            .expect("Error loading a subscriber")
    }
}
