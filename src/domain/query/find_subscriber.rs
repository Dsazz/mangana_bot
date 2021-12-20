use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::model::subscriber::Subscriber;
use crate::domain::pool::establish_connection;
use crate::domain::schema::subscribers::dsl::{
    subscribers as find_subscriber,
    chat_id as chat_id_column
};

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};


pub struct FindSubscriber {
    chat_id: i64,
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FindSubscriber {
    pub(crate) fn new(chat_id: i64) -> FindSubscriber {
        FindSubscriber {
            chat_id,
            connection: establish_connection()
        }
    }

    pub fn execute(&self) -> Option<Subscriber> {
        find_subscriber
            .filter(chat_id_column.eq(&self.chat_id))
            .first(&self.connection)
            .optional()
            .expect("Error loading a subscriber")
    }
}
