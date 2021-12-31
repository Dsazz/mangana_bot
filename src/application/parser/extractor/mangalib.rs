use std::any::Any;
use anyhow::Error;
use scraper::{Html, ElementRef, Selector};
use crate::application::parser::extractor::contract::Extractor;

lazy_static! {
    static ref LAST_CHAPTER_CONTAINER_SELECTOR: Selector = Selector::parse(".media-chapter").unwrap();
    static ref CHAPTER_DATE_SELECTOR: Selector = Selector::parse(".media-chapter__date").unwrap();
    static ref CHAPTER_TITLE_LINK_SELECTOR: Selector = Selector::parse(".media-chapter__name > a").unwrap();
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq)]
pub struct MangalibExtractor {
    body: Html,
}

impl MangalibExtractor {
    pub(crate) fn new(body: Html) -> MangalibExtractor {
        MangalibExtractor{ body }
    }
}
impl Extractor for MangalibExtractor {
    fn as_any(&self) -> &dyn Any { self }
    fn extract_last_chapter_elem(&self) -> Result<ElementRef, Error> {
        match self.body.select(&LAST_CHAPTER_CONTAINER_SELECTOR).next() {
            None => bail!("[EXTRACTOR] can't extract last chapter element"),
            Some(elem) => Ok(elem),
        }
    }

    fn extract_chapter_href(&self, chapter_elem: &ElementRef) -> Result<String, Error> {
        let link = match chapter_elem.select(&CHAPTER_TITLE_LINK_SELECTOR).next() {
            None => bail!("[EXTRACTOR] can't extract last chapter link from: {:?}", chapter_elem.html()),
            Some(elem) => elem,
        };
        match link.value().attr("href") {
            None => bail!("[EXTRACTOR] can't extract last chapter href from element: {:?}", link.html()),
            Some(href) => Ok(href.to_string()),
        }
    }

    fn extract_chapter_title(&self, chapter_elem: &ElementRef) -> Result<String, Error> {
        let chapter_title_container = match chapter_elem.select(&CHAPTER_TITLE_LINK_SELECTOR).next() {
            None => bail!("[EXTRACTOR] can't extract chapter title container from element: {:?}", chapter_elem.html()),
            Some(elem) => elem,
        };
        let chapter_title_raw = match chapter_title_container.text().next() {
            None => bail!("[EXTRACTOR] can't extract chapter title from element: {:?}", chapter_elem.html()),
            Some(text) => text.trim(),
        };

        let full_title: Vec<&str> = chapter_title_raw.split('-').collect();
        let chapter_title_index = 0;
        match full_title.get(chapter_title_index) {
            None => Ok("".to_string()),
            Some(text) => Ok(text.trim().to_string()),
        }
    }

    fn extract_chapter_name(&self, chapter_elem: &ElementRef) -> Result<String, Error> {
        //@todo probably should be extracted into a separate method
        let chapter_title_container = match chapter_elem.select(&CHAPTER_TITLE_LINK_SELECTOR).next() {
            None => bail!("[EXTRACTOR] can't extract chapter title container from element: {:?}", chapter_elem.html()),
            Some(elem) => elem,
        };
        let chapter_title_raw = match chapter_title_container.text().next() {
            None => bail!("[EXTRACTOR] can't extract chapter title from element: {:?}", chapter_elem.html()),
            Some(text) => text.trim(),
        };

        let full_title: Vec<&str> = chapter_title_raw.split('-').collect();
        let chapter_name_index = 1;
        match full_title.get(chapter_name_index) {
            None => Ok("".to_string()),
            Some(text) => Ok(text.trim().to_string()),
        }
    }

    fn extract_chapter_date(&self, chapter_elem: &ElementRef) -> Result<String, Error> {
        let chapter_date_container = match chapter_elem.select(&CHAPTER_DATE_SELECTOR).next() {
            None => bail!("[EXTRACTOR] can't extract chapter date container from element: {:?}", chapter_elem.html()),
            Some(elem) => elem,
        };
        match chapter_date_container.text().next() {
            None => bail!("[EXTRACTOR] can't extract chapter date from container"),
            Some(text) => Ok(text.trim().to_string()),
        }
    }
}