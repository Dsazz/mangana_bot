use anyhow::Error;
use async_trait::async_trait;
use log::{info, warn};
use scraper::Html;
use crate::application::http::client::get_request;
use crate::application::parser::extractor::contract::Extractor;
use crate::domain::enumeration::site::Site;
use crate::domain::model::parse_history::ParseHistory;
use crate::domain::model::topic::Topic;
use crate::domain::query::get_all_site_topics::GetAllSiteTopics;

#[async_trait]
pub trait Parser: Sync {
    fn site(&self) -> Site;
    fn validate_topic_site(&self, topic: Topic) -> Result<(), Error> {
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

        Ok(())
    }

    fn extractor(&self, html: Html) -> Box<dyn Extractor>;

    // @todo need to avoid this dirty hack with "test_url"
    async fn parse(&self, topic: Topic, test_url: String) -> Result<ParseHistory, Error> {
        self.validate_topic_site(topic.clone())?;

        let url = if test_url.is_empty() { topic.url() } else { test_url };

        info!("|> [PARSE] send request to: {:?}", url);
        let html_content = match get_request(url.as_str()).await {
            Ok(res) => res,
            Err(res) => bail!("[PARSE] {}", res.to_string()),
        };

        info!("  * start parsing...\n");
        let html = Html::parse_document(html_content.as_str());
        let extracted_data = self.extractor(html).extract()?;
        let parse_history = topic.to_parse_history(&extracted_data);

        info!("{}", parse_history.to_log());
        Ok(parse_history)
    }

    async fn parse_all(&self) -> Vec<ParseHistory> {
        let mut results: Vec<ParseHistory> = vec![];
        let all_topics = GetAllSiteTopics::new(self.site()).execute();
        for topic in all_topics {
            match self.parse(topic, "".to_string()).await {
                Ok(result) => results.push(result),
                Err(err) => warn!("Error parse: {:?}", err),
            }
        }

        results
    }
}