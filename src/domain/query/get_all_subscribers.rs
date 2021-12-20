use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use crate::domain::schema::subscribers::dsl::subscribers as get_subscribers;
use crate::domain::model::subscriber::Subscriber;

use diesel::RunQueryDsl;


pub struct GetAllSubscribers {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl GetAllSubscribers {
    pub(crate) fn new() -> GetAllSubscribers {
        GetAllSubscribers {
            connection: establish_connection()
        }
    }

    pub fn execute(&self) -> Vec<Subscriber> {
        get_subscribers
            .load::<Subscriber>(&self.connection)
            .expect("Error loading posts")
    }
}
