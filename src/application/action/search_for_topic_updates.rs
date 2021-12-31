use log::info;
use crate::application::parser::contract::Parser;
use crate::domain::query::upsert_parse_history::UpsertParseHistory;
use crate::application::parser::mangapoisk::Mangapoisk;
use crate::application::parser::mangalib::Mangalib;

pub struct SearchForTopicUpdates;

impl SearchForTopicUpdates {
    pub(crate) fn new() -> SearchForTopicUpdates {
        SearchForTopicUpdates {}
    }

    pub async fn execute(&self) -> Result<(), ()> {
        info!(" |> Run SearchForTopicUpdates");
        //@todo probably I can use it(<Box<dyn ITopic + Send + Sync>) with dyn ITopic for unit tests
        let available_parsers: Vec<Box<dyn Parser + Send + Sync>> = vec![
            Box::new(Mangapoisk::new()),
            Box::new(Mangalib::new()),
        ];

        //@todo try to parallelize
        for parser in available_parsers {
            for new_history in parser.parse_all().await {
                UpsertParseHistory::new(new_history.to_raw()).execute();
            }
        }

        Ok(())
    }
}