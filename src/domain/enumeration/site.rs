use anyhow::Error;
use strum_macros::EnumIter;
use crate::domain::model::topic::Topic;

const MANGAPOISK_URL: &str = "https://mangapoisk.ru";
const MANGAPOISK_URL_TOPIC_PREFIX: &str = "manga";
const MANGAPOISK_NAME: &str = "MANGAPOISK.RU";
const MANGAPOISK_ID: i32 = 1;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Site {
    Mangapoisk,
}

impl Site {
    pub(crate) fn new(id: i32) -> Result<Site, Error> {
        match id {
            MANGAPOISK_ID => Ok(Site::Mangapoisk),
            _ => Err(Error::msg("Unknown site")),
        }
    }

    pub fn url(self) -> &'static str {
        match self {
            Site::Mangapoisk => MANGAPOISK_URL,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Site::Mangapoisk => MANGAPOISK_NAME,
        }
    }

    pub fn id(self) -> i32 {
        match self {
            Site::Mangapoisk => MANGAPOISK_ID,
        }
    }

    pub fn topic_url_prefix(self, topic: &Topic) -> String {
        match self {
            Site::Mangapoisk => format!("{}/{}/{}", self.url(), MANGAPOISK_URL_TOPIC_PREFIX, topic.url_name),
        }
    }

    pub fn eq(self, other: Site) -> bool {
        self.id() == other.id()
    }

    pub fn not_eq(self, other: Site) -> bool {
        !self.eq(other)
    }
}
