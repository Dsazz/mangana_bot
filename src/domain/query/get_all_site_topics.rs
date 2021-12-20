use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::domain::pool::establish_connection;
use crate::domain::schema::topic::dsl::{
    topic, site_id as site_id_clm,
};
use crate::domain::model::topic::Topic;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::domain::enumeration::site::Site;


pub struct GetAllSiteTopics {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    site: Site,
}

impl GetAllSiteTopics {
    pub(crate) fn new(site: Site) -> GetAllSiteTopics {
        GetAllSiteTopics {
            connection: establish_connection(),
            site,
        }
    }

    pub fn execute(&self) -> Vec<Topic> {
       topic
           .filter(site_id_clm.eq(&self.site.id()))
           .load::<Topic>(&self.connection)
           .expect("Error loading topics")
    }
}
