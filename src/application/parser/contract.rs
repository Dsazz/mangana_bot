use anyhow::Error;
use async_trait::async_trait;
use crate::domain::enumeration::site::Site;
use crate::domain::model::parse_history::ParseHistory;
use crate::domain::model::topic::Topic;

#[async_trait]
pub trait Parser {
    fn site(&self) -> Site;
    async fn parse(&self, topic: Topic) -> Result<ParseHistory, Error>;
    async fn parse_all(&self) -> Vec<ParseHistory>;
}