use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use crate::domain::schema::app_notifications::dsl::{app_notifications};
use diesel::{delete, RunQueryDsl, QueryDsl};


pub struct DeleteAppNotification {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    id: i32,
}

impl DeleteAppNotification {
    pub(crate) fn new(id: i32) -> DeleteAppNotification {
        DeleteAppNotification { id, connection: establish_connection() }
    }

    pub fn execute(&self) -> usize {
        delete(app_notifications.find(&self.id))
            .execute(&self.connection)
            .expect("Cant delete a subscriber from DB")
    }
}
