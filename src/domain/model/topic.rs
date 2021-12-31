use anyhow::Error;
use crate::domain::enumeration::site::Site;
use diesel::sql_types::{Integer, Text};
use crate::application::dto::extracted_chapter::ExtractedChapter;
use crate::domain::model::parse_history::ParseHistory;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Queryable, QueryableByName, PartialEq)]
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
    pub fn to_parse_history(&self, chapter: &ExtractedChapter) -> ParseHistory {
        ParseHistory {
            topic: self.clone(),
            last_chapter_title: chapter.title.to_string(),
            chapter_name: chapter.name.to_string(),
            release_date: chapter.release_date.to_string(),
            url: self.url_for(&chapter.href),
        }
    }
    pub fn site(&self) -> Result<Site, Error> {
        Site::new(self.site_id)
    }
    pub fn url(&self) -> String {
        self.site().unwrap().url_to_topic(&self.clone())
    }
    pub fn uri(&self) -> String {
        self.site().unwrap().uri_to_topic(&self.clone())
    }
    pub fn url_for(&self, path: &str) -> String {
        format!("{}{}", self.url(), path)
    }
}
