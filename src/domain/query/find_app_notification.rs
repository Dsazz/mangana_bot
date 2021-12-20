use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::model::app_notifications::AppNotification;
use crate::domain::pool::establish_connection;
use crate::domain::schema::app_notifications::dsl::{
    app_notifications,
};
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use tokio::sync::Mutex;


pub struct FindAppNotification {
    id: i32,
    connection: Mutex<PooledConnection<ConnectionManager<PgConnection>>>,
}

impl FindAppNotification {
    pub(crate) fn new(id: i32) -> FindAppNotification {
        FindAppNotification { id, connection: Mutex::new(establish_connection()) }
    }

    pub fn execute(&mut self) -> Option<AppNotification> {
       app_notifications
            .find(&self.id)
            .first(self.connection.get_mut())
            .optional()
            .expect("Error loading a subscriber")
    }
}
