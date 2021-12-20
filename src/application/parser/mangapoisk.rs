use log::{info, warn};
use scraper::{Html, ElementRef, Selector};
use regex::Regex;
use async_trait::async_trait;
use futures::future::{join_all};
use futures::TryFutureExt;
use crate::domain::model::parse_history::{ParseHistory};
use crate::application::parser::contract::Parser;
use crate::domain::enumeration::site::Site;
use crate::domain::query::get_all_site_topics::GetAllSiteTopics;
use crate::domain::model::topic::Topic;

lazy_static! {
    static ref TEXT_CLEANER_RE: Regex = Regex::new(r"^/|\s\s+").unwrap();
    static ref CHAPTERS_LIST_SELECTOR: Selector = Selector::parse(".list-group-item").unwrap();
    static ref CHAPTER_TITLE_SELECTOR: Selector = Selector::parse(".chapter-title").unwrap();
    static ref CHAPTER_DATE_SELECTOR: Selector = Selector::parse(".chapter-date").unwrap();
    static ref CHAPTER_URL_SELECTOR: Selector = Selector::parse("a").unwrap();
}

pub struct Mangapoisk {}

#[async_trait]
impl Parser for Mangapoisk {
    fn site(&self) -> Site { Site::Mangapoisk }

    async fn parse(&self, topic: Topic) -> Result<ParseHistory, anyhow::Error> {
        // @todo need to shortcut
        let topic_site = match topic.site() {
            Ok(value) => value,
            Err(err) => {
                warn!("|> invalid or empty site for topic: {}", err);
                bail!(err)
            },
        };

        if topic_site.not_eq(self.site()) {
            warn!("|> topic site ({:?}) should be equal to the parser site {:?}", topic_site, self.site());
            bail!("topic site should be equal to the parser site")
        }
        // @todo end ----------------------------------

        info!("|> send request to: {:?}", topic.url());

        let resp = reqwest::get(topic.url()).await?;
        assert!(resp.status().is_success());
        info!(" * the request was successful.");

        info!(" * start parsing...\n");
        let body = resp.text().await?;
        let fragment = Html::parse_document(&body);
        let last_chapter = Mangapoisk::extract_last_chapter_elem(&fragment);

        let new_parse_history: ParseHistory = ParseHistory {
            topic,
            last_chapter_title: Mangapoisk::extract_chapter_title(&last_chapter),
            chapter_name: Mangapoisk::extract_chapter_name(&last_chapter),
            release_date: Mangapoisk::extract_chapter_date(&last_chapter),
            url: format!("{}{}", self.site().url(), Mangapoisk::extract_last_chapter_name(&last_chapter)),
        };

        info!("{}", new_parse_history.to_log());

        Ok(new_parse_history)
    }

    async fn parse_all(&self) -> Vec<ParseHistory> {
        let mut results = vec![];
        let all_topics = GetAllSiteTopics::new(self.site()).execute();
        for topic in all_topics {
            results.push(self.parse(topic).into_future());
        }

        join_all(results).await
            .into_iter()
            .flatten()
            .collect::<Vec<ParseHistory>>()
    }
}

impl Mangapoisk {
    pub(crate) fn new() -> Mangapoisk {
        Mangapoisk{}
    }

    fn extract_last_chapter_elem(body: &Html) -> ElementRef {
        body.select(&CHAPTERS_LIST_SELECTOR).next().unwrap()
    }

    fn extract_last_chapter_name(last_chapter: &ElementRef) -> String {
        last_chapter.select(&CHAPTER_URL_SELECTOR)
            .next().unwrap().value().attr("href").unwrap().to_string()
    }

    fn extract_chapter_title(last_chapter: &ElementRef) -> String {
        let chapter_title_container = last_chapter.select(&CHAPTER_TITLE_SELECTOR).next().expect("Fuuck");
        let chapter_title_raw = chapter_title_container.text().next().unwrap().trim();

        TEXT_CLEANER_RE.replace_all(chapter_title_raw, " ").to_string()
    }

    fn extract_chapter_name(last_chapter: &ElementRef) -> String {
        let chapter_title_container = last_chapter.select(&CHAPTER_TITLE_SELECTOR).next().unwrap();
        let chapter_name_raw = chapter_title_container.next_sibling().unwrap().value().as_text().unwrap().trim();

        TEXT_CLEANER_RE.replace_all(chapter_name_raw, " ").to_string()
    }

    fn extract_chapter_date(last_chapter: &ElementRef) -> String {
        let chapter_date_container = last_chapter.select(&CHAPTER_DATE_SELECTOR).next().unwrap();
        let chapter_date_raw = chapter_date_container.text().next().unwrap().trim();

        TEXT_CLEANER_RE.replace_all(chapter_date_raw, " ").to_string()
    }
}