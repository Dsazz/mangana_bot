use scraper::{Html};
use async_trait::async_trait;
use crate::application::parser::contract::Parser;
use crate::application::parser::extractor::contract::Extractor;
use crate::application::parser::extractor::mangapoisk::MangapoiskExtractor;
use crate::domain::enumeration::site::Site;

#[derive(Debug)]
pub struct Mangapoisk {}

impl Mangapoisk {
    pub(crate) fn new() -> Mangapoisk {
        Mangapoisk{}
    }
}

#[async_trait]
impl Parser for Mangapoisk {
    fn site(&self) -> Site { Site::Mangapoisk }
    fn extractor(&self, html: Html) -> Box<dyn Extractor> {
        Box::new(MangapoiskExtractor::new(html))
    }
}

