#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedChapter {
    pub title: String,
    pub name: String,
    pub release_date: String,
    pub href: String,
}
impl ExtractedChapter {
    pub(crate) fn new(title: String, name: String, release_date: String, href: String) -> ExtractedChapter {
        ExtractedChapter { title, name, release_date, href }
    }
}