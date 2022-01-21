// use scraper::{Html};
// use async_trait::async_trait;
// use crate::application::parser::contract::Parser;
// use crate::application::parser::extractor::contract::Extractor;
// use crate::application::parser::extractor::mangalib::MangalibExtractor;
// use crate::domain::enumeration::site::Site;
//
// #[derive(Debug)]
// pub struct Mangalib {}
//
// impl Mangalib {
//     pub(crate) fn new() -> Mangalib {
//         Mangalib{}
//     }
// }
//
// #[async_trait]
// impl Parser for Mangalib {
//     fn site(&self) -> Site { Site::Mangalib }
//     fn extractor(&self, html: Html) -> Box<dyn Extractor> {
//         Box::new(MangalibExtractor::new(html))
//     }
// }

