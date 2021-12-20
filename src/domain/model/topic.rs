use anyhow::Error;
use crate::domain::enumeration::site::Site;
use diesel::sql_types::{Integer, Text};

#[derive(Debug, Clone, Queryable, QueryableByName)]
pub struct Topic {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub site_id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Text"]
    pub url_name: String,
}

impl Topic {
    pub fn site(&self) -> Result<Site, Error> {
        Site::new(self.site_id)
    }
    pub fn url(&self) -> String {
        self.site().unwrap().topic_url_prefix(&self)
    }
}
