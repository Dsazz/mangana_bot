use crate::domain::model::parse_history::{Raw as RawParseHistory};
use crate::domain::pool::establish_connection;
use crate::domain::schema::parse_histories::dsl::{parse_histories as add_parse_history, topic_id};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{RunQueryDsl, insert_into};

pub struct UpsertParseHistory {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    entity: RawParseHistory
}

impl<'a> UpsertParseHistory {
    pub(crate) fn new(entity: RawParseHistory) -> UpsertParseHistory {
        UpsertParseHistory {
            entity,
            connection: establish_connection()
        }
    }

    pub fn execute(&self) {
        insert_into(add_parse_history)
            .values(&self.entity)
            .on_conflict(topic_id)
            .do_update()
            .set(&self.entity)
            .execute(&self.connection)
            .expect("Error upserting a parse history");
    }
}
