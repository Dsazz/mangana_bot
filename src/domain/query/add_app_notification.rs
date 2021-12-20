use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{insert_into, RunQueryDsl};
use crate::domain::schema::app_notifications::dsl::{app_notifications};
use crate::domain::pool::establish_connection;
use crate::domain::model::app_notifications::{NewAppNotification, AppNotification};


pub struct AddAppNotification {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    entity: NewAppNotification,
}

impl AddAppNotification {
    pub(crate) fn new(entity: NewAppNotification) -> AddAppNotification {
        AddAppNotification {
            entity,
            connection: establish_connection()
        }
    }

    pub fn execute(&self) -> AppNotification {
        insert_into(app_notifications)
            .values(&self.entity)
            .get_result(&self.connection)
            .expect("Error adding an app notification")
    }
}
