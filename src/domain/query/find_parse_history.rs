use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::model::parse_history::{ParseHistory, Raw as RawParseHistory};
use crate::domain::pool::establish_connection;
use crate::domain::schema::parse_histories::dsl::{
    parse_histories as get_parse_history,
    topic_id as topic_column,
    last_chapter_title as lct_column,
};
use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};


pub struct FindParseHistory<'a> {
    topic_id: i32,
    last_chapter_title: &'a str,
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl<'a> FindParseHistory<'a> {
    pub(crate) fn new(parse_history: &'a ParseHistory) -> FindParseHistory<'a> {
        FindParseHistory {
            topic_id: parse_history.topic.id,
            last_chapter_title: parse_history.last_chapter_title.as_str(),
            connection: establish_connection()
        }
    }

    pub fn execute(&self) -> Option<RawParseHistory> {
        get_parse_history
            .filter(
                topic_column.eq(&self.topic_id)
                    .and(lct_column.eq(&self.last_chapter_title))
            )
            .first(&self.connection)
            .optional()
            .expect("Error loading a parse history")
    }
}
