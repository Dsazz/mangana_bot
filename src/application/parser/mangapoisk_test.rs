#[cfg(test)]
mod tests {
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use regex::RegexBuilder;

    use scraper::Html;
    use crate::application::parser::contract::Parser;
    use crate::application::parser::extractor::mangapoisk::MangapoiskExtractor;
    use crate::application::parser::mangapoisk::Mangapoisk;
    use crate::application::parser::test_utils::mangapoisk::{VALID_LAST_CHAPTER_HREF, VALID_LAST_CHAPTER_HTML, VALID_LAST_CHAPTER_NAME, VALID_LAST_CHAPTER_RELEASE_DATE, VALID_LAST_CHAPTER_TITLE};
    use crate::aw;
    use crate::domain::enumeration::site::Site;
    use crate::domain::model::parse_history::ParseHistory;
    use crate::domain::model::topic::Topic;

    #[test]
    fn points_to_valid_site() {
        let parser = get_parser();
        assert_eq!(get_valid_site(),  parser.site());
    }

    #[test]
    fn points_to_valid_extractor() {
        let parser = Mangapoisk::new();
        let expected_extractor = &MangapoiskExtractor::new(Html::new_document());

        let box_extractor = parser.extractor(Html::new_document());
        let actual_extractor = box_extractor.as_any().downcast_ref::<MangapoiskExtractor>();

        assert!(actual_extractor.is_some());
        assert_eq!(expected_extractor, actual_extractor.unwrap());
    }

    #[test]
    fn validate_topic_site_with_valid_topic() {
        let parser = get_parser();
        let valid_topic = get_valid_topic();
        let result = parser.validate_topic_site(valid_topic);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_topic_site_with_topic_unknown_site() {
        let topic_with_unknown_site = Topic {
            id: 1,
            site_id: 0,
            name: "Test name".to_string(),
            url_name: "test-url".to_string()
        };
        let result = Mangapoisk::new().validate_topic_site(topic_with_unknown_site);
        assert!(result.is_err());

        let expected_error = "Unknown site".to_string();
        let actual_error = result.unwrap_err().to_string();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn validate_topic_site_with_topic_wrong_site() {
        let topic_with_wrong_site = Topic {
            id: 1,
            site_id: Site::Mangalib.id(),
            name: "Test name".to_string(),
            url_name: "test-url".to_string()
        };
        let result = Mangapoisk::new().validate_topic_site(topic_with_wrong_site);
        assert!(result.is_err());

        let expected_error = "topic site should be equal to the parser site".to_string();
        let actual_error = result.unwrap_err().to_string();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn parse_is_ok() {
        let parser = get_parser();
        let valid_topic = get_valid_topic();
        let uri = valid_topic.uri();

        let mock_server = MockServer::start();
        mock_server.mock(|when, then| {
            when.method(GET);
            then.status(200)
                .header("content-type", "text/html")
                .body(VALID_LAST_CHAPTER_HTML);
        });

        let result = aw!(parser.parse(valid_topic.clone(), mock_server.url(uri)));
        assert!(result.is_ok());

        let excepted_parse_history = ParseHistory {
            topic: valid_topic.clone(),
            last_chapter_title: VALID_LAST_CHAPTER_TITLE.to_string(),
            chapter_name: VALID_LAST_CHAPTER_NAME.to_string(),
            url: valid_topic.url_for(VALID_LAST_CHAPTER_HREF),
            release_date: VALID_LAST_CHAPTER_RELEASE_DATE.to_string()
        };
        //@todo: need to fix encoding
        assert_eq!(excepted_parse_history.topic, result.unwrap().topic);
    }

    #[test]
    fn parse_with_404_response() {
        let parser = get_parser();
        let valid_topic = get_valid_topic();
        let uri = valid_topic.uri();

        let mock_server = MockServer::start();
        mock_server.mock(|when, then| {
            when.method(GET);
            then.status(404)
                .header("content-type", "text/html");
        });

        let result = aw!(parser.parse(valid_topic, mock_server.url(uri)));
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        let re = RegexBuilder::new(r"invalid status 404")
            .multi_line(true).build().unwrap();
        assert!(re.is_match(&err));
    }

    #[test]
    fn parse_with_empty_response_text() {
        let parser = get_parser();
        let valid_topic = get_valid_topic();
        let uri = valid_topic.uri();

        let mock_server = MockServer::start();
        mock_server.mock(|when, then| {
            when.method(GET);
            then.status(200)
                .header("content-type", "text/html");
        });

        let result = aw!(parser.parse(valid_topic, mock_server.url(uri)));
        assert!(result.is_err());

        let actual_error = result.unwrap_err().to_string();
        assert_eq!("[EXTRACTOR] can't extract last chapter element", actual_error);
    }

    #[test]
    fn parse_with_bad_url() {
        let parser = get_parser();
        let valid_topic = get_valid_topic();

        let result = aw!(parser.parse(valid_topic, "bad_url.zzzzz".to_string()));
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        let re = RegexBuilder::new(r"Cannot navigate to invalid URL")
            .multi_line(true).build().unwrap();
        assert!(re.is_match(&err));
    }

    //@todo need to test
    // async fn parse_all(&self) -> Vec<ParseHistory> {
    // }
    fn get_valid_topic() -> Topic {
        Topic {
            id: 1,
            site_id: get_valid_site().id(),
            name: "Test name".to_string(),
            url_name: "test-url".to_string(),
        }
    }
    fn get_valid_site() -> Site {
        Site::Mangapoisk
    }
    fn get_parser() -> Mangapoisk {
        Mangapoisk::new()
    }
}