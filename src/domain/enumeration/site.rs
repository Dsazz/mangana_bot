use anyhow::Error;
use strum_macros::EnumIter;
use crate::domain::model::topic::Topic;

const MANGAPOISK_URL: &str = "https://mangapoisk.ru";
const MANGAPOISK_URL_TOPIC_PREFIX: &str = "manga";
const MANGAPOISK_NAME: &str = "MANGAPOISK.RU";
const MANGAPOISK_ID: i32 = 1;

const MANGALIB_URL: &str = "https://mangalib.me";
const MANGALIB_URL_TOPIC_SUFFIX: &str = "?section=chapters";
const MANGALIB_NAME: &str = "MANGALIB.ME";
const MANGALIB_ID: i32 = 2;

#[derive(Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Site {
    Mangapoisk,
    Mangalib,
}

impl Site {
    pub(crate) fn new(id: i32) -> Result<Site, Error> {
        match id {
            MANGAPOISK_ID => Ok(Site::Mangapoisk),
            MANGALIB_ID => Ok(Site::Mangalib),
            _ => bail!("Unknown site"),
        }
    }

    pub const fn url(self) -> &'static str {
        match self {
            Site::Mangapoisk => MANGAPOISK_URL,
            Site::Mangalib => MANGALIB_URL,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Site::Mangapoisk => MANGAPOISK_NAME,
            Site::Mangalib => MANGALIB_NAME,
        }
    }

    pub const fn id(self) -> i32 {
        match self {
            Site::Mangapoisk => MANGAPOISK_ID,
            Site::Mangalib => MANGALIB_ID,
        }
    }

    pub fn url_to_topic(self, topic: &Topic) -> String {
        match self {
            Site::Mangapoisk | Site::Mangalib => {
                format!("{}{}", self.url(), self.uri_to_topic(topic))
            },
        }
    }

    pub fn uri_to_topic(self, topic: &Topic) -> String {
        match self {
            Site::Mangapoisk => format!("/{}/{}", MANGAPOISK_URL_TOPIC_PREFIX, topic.url_name),
            Site::Mangalib => format!("/{}/{}", topic.url_name, MANGALIB_URL_TOPIC_SUFFIX),
        }
    }

    pub fn eq(self, other: Site) -> bool {
        self.id() == other.id()
    }
    pub fn not_eq(self, other: Site) -> bool {
        !self.eq(other)
    }
}
