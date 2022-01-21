use std::any::Any;
use anyhow::Error;
use scraper::{ElementRef};
use crate::application::dto::extracted_chapter::ExtractedChapter;

pub trait Extractor {
    fn as_any(&self) -> &dyn Any;
    fn extract(&self) -> Result<ExtractedChapter, anyhow::Error> {
        let chapter_elem = self.extract_last_chapter_elem()?;

        let title = self.extract_chapter_title(&chapter_elem)?;
        let name = self.extract_chapter_name(&chapter_elem)?;
        let release_date = self.extract_chapter_date(&chapter_elem)?;
        let href = self.extract_chapter_href(&chapter_elem)?;
        let extracted_chapter = ExtractedChapter::new(
            title, name, release_date, href
        );

        Ok(extracted_chapter)
    }

    fn extract_last_chapter_elem(&self) -> Result<ElementRef, Error>;
    fn extract_chapter_href(&self, chapter_elem: &ElementRef) -> Result<String, Error>;
    fn extract_chapter_title(&self, chapter_elem: &ElementRef) -> Result<String, Error>;
    fn extract_chapter_name(&self, chapter_elem: &ElementRef) -> Result<String, Error>;
    fn extract_chapter_date(&self, chapter_elem: &ElementRef) -> Result<String, Error>;
}


