use log::info;
use crate::domain::query::find_parse_history::FindParseHistory;
use crate::domain::query::upsert_parse_history::UpsertParseHistory;
use crate::application::parser::mangapoisk::Mangapoisk;
use crate::application::parser::contract::Parser;


pub struct FindNewHistoryUpdates;

impl FindNewHistoryUpdates {
    pub(crate) fn new() -> FindNewHistoryUpdates {
        FindNewHistoryUpdates {}
    }

    pub async fn execute(&self) -> Result<(), ()> {
        info!(" |> Run FindNewHistoryUpdates");
        let parser = Mangapoisk::new();//@todo change to contract
        for new_history in parser.parse_all().await {
            //@todo try to parallelize
            match FindParseHistory::new(&new_history).execute() {
                None => {
                    info!("   * history data have been updated: {:?}", new_history.topic.name);
                    UpsertParseHistory::new(new_history.to_raw()).execute();
                }
                Some(_) => {
                    UpsertParseHistory::new(new_history.to_raw()).execute();
                    continue;
                }
            }
        }

        Ok(())
    }
}