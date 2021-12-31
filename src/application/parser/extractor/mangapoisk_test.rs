#[cfg(test)]
mod tests {
    // For debugging
    // #[cfg(not(test))]
    // use log::{debug};
    // #[cfg(test)]
    // use std::{println as debug};

    use regex::Regex;
    use scraper::{Html};
    use crate::application::dto::extracted_chapter::ExtractedChapter;
    use crate::application::parser::extractor::contract::Extractor;
    use crate::application::parser::extractor::mangapoisk::{MangapoiskExtractor};
    use crate::application::parser::test_utils::mangapoisk::{VALID_LAST_CHAPTER_HREF, VALID_LAST_CHAPTER_HTML, VALID_LAST_CHAPTER_NAME, VALID_LAST_CHAPTER_RELEASE_DATE, VALID_LAST_CHAPTER_TITLE};

    #[test]
    fn extract_should_return_valid_data() {
        let extractor = get_valid_extractor();
        let expected_data: ExtractedChapter = ExtractedChapter::new(
            VALID_LAST_CHAPTER_TITLE.to_string(),
            VALID_LAST_CHAPTER_NAME.to_string(),
            VALID_LAST_CHAPTER_RELEASE_DATE.to_string(),
            VALID_LAST_CHAPTER_HREF.to_string()
        );

        let actual_result = extractor.extract();
        assert!(actual_result.is_ok());
        assert_eq!(expected_data, actual_result.unwrap());
    }

    #[test]
    fn extract_last_chapter_elem_invalid() {
        let invalid_body = get_invalid_html_body();
        let extractor = MangapoiskExtractor::new(invalid_body);
        let result = extractor.extract_last_chapter_elem();
        assert!(result.is_err());
        assert_eq!("[EXTRACTOR] can't extract last chapter element", result.unwrap_err().to_string());
    }

    //@todo: need to fix
    // #[test]
    // fn extract_last_chapter_elem_valid() {
    //     let extractor = get_valid_extractor();
    //     let result = extractor.extract_last_chapter_elem();
    //     assert!(result.is_ok());
    //
    //     let expected_html =
    //         r#"<li class="list-group-item chapter-item ">
    //                 <a href="/manga/vanpanchmen/chapter/29-199" class="">
    //                     <span class="chapter-title">Том 29 Глава 199 </span>
    //                     Предельная пушка волнового потока адского огня
    //                 </a>
    //                 <span class="chapter-date ">13 декабря 2021  </span>
    //             </li>"#;
    //     assert_eq!(expected_html, result.unwrap().html());
    // }

    #[test]
    fn extract_chapter_href_valid() {
        let chapter_fragment = get_valid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_href(&chapter_element.clone());
        assert!(result.is_ok());
        assert_eq!(VALID_LAST_CHAPTER_HREF, result.unwrap());
    }

    #[test]
    fn extract_chapter_href_invalid_link() {
        let chapter_fragment = get_invalid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_href(&chapter_element.clone());
        assert!(result.is_err());
        assert_eq!(
            format!("[EXTRACTOR] can't extract last chapter link from: {:?}", chapter_element.html()),
            result.unwrap_err().to_string(),
        );
    }

    #[test]
    fn extract_chapter_href_invalid_link_data() {
        let chapter_fragment = Html::parse_fragment(
            r#"<li class="list-group-item chapter-item ">
                    <a>
                        <span class="chapter-title">Том 29 Глава 199 </span>
                        Предельная пушка волнового потока адского огня
                    </a>
                    <span class="chapter-date ">13 декабря 2021  </span>
                </li>"#
        );
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_href(&chapter_element.clone());
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        let re = Regex::new(r"^\[EXTRACTOR] can't extract last chapter href from element").unwrap();
        assert!(re.is_match(&err));
    }

    #[test]
    fn extract_chapter_title_valid() {
        let chapter_fragment = get_valid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_title(&chapter_element.clone());
        assert!(result.is_ok());
        assert_eq!(VALID_LAST_CHAPTER_TITLE, result.unwrap());
    }


    #[test]
    fn extract_chapter_title_invalid_container() {
        let chapter_fragment = get_invalid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_title(&chapter_element.clone());
        assert!(result.is_err());
        assert_eq!(
            format!("[EXTRACTOR] can't extract chapter title container from element: {:?}", chapter_element.html()),
            result.unwrap_err().to_string(),
        );
    }

    #[test]
    fn extract_chapter_title_invalid_element_data() {
        let chapter_fragment = Html::parse_fragment(
            r#"<li class="list-group-item chapter-item ">
                    <a>
                        <span class="chapter-title"></span>
                        Предельная пушка волнового потока адского огня
                    </a>
                    <span class="chapter-date ">13 декабря 2021  </span>
                </li>"#
        );
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_title(&chapter_element.clone());
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        let re = Regex::new(r"^\[EXTRACTOR] can't extract chapter title container from element").unwrap();
        assert!(re.is_match(&err));
    }

    #[test]
    fn extract_chapter_name_valid() {
        let chapter_fragment = get_valid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_name(&chapter_element.clone());
        assert!(result.is_ok());
        assert_eq!(VALID_LAST_CHAPTER_NAME, result.unwrap());
    }

    #[test]
    fn extract_chapter_name_empty_valid() {
        let chapter_fragment = Html::parse_fragment(
            r#"<li class="list-group-item chapter-item ">
                    <a>
                        <span class="chapter-title">Test</span>
                    </a>
                    <span class="chapter-date ">13 декабря 2021  </span>
                </li>"#
        );
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_name(&chapter_element.clone());
        assert!(result.is_ok());

        let empty_name = "";
        assert_eq!(empty_name, result.unwrap());
    }

    #[test]
    fn extract_chapter_name_invalid_container() {
        let chapter_fragment = get_invalid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_name(&chapter_element.clone());
        assert!(result.is_err());
        assert_eq!(
            format!("[EXTRACTOR] can't extract chapter title container from element: {:?}", chapter_element.html()),
            result.unwrap_err().to_string(),
        );
    }

    #[test]
    fn extract_chapter_name_with_empty_name_element() {
        let chapter_fragment = Html::parse_fragment(
            r#"<li class="list-group-item chapter-item ">
                    <a><span class="chapter-title"></span></a>
                    <span class="chapter-date ">13 декабря 2021  </span>
                </li>"#
        );
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_name(&chapter_element.clone());
        assert!(result.is_ok());

        let output = result.unwrap();
        let expected_output = "".to_string();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn extract_chapter_name_with_invalid_name_element() {
        let chapter_fragment = Html::parse_fragment(
            r#"<li class="list-group-item chapter-item ">
                    <a><span class="chapter-title"></span><br/></a>
                    <span class="chapter-date ">13 декабря 2021  </span>
                </li>"#
        );
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_name(&chapter_element.clone());
        assert!(result.is_ok());

        let output = result.unwrap();
        let expected_output = "".to_string();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn extract_chapter_date_valid() {
        let chapter_fragment = get_valid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_date(&chapter_element.clone());
        assert!(result.is_ok());
        assert_eq!(VALID_LAST_CHAPTER_RELEASE_DATE, result.unwrap());
    }


    #[test]
    fn extract_chapter_date_invalid_container() {
        let chapter_fragment = get_invalid_html_body();
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_date(&chapter_element.clone());
        assert!(result.is_err());
        assert_eq!(
            format!("[EXTRACTOR] can't extract chapter date container from element: {:?}", chapter_element.html()),
            result.unwrap_err().to_string(),
        );
    }

    #[test]
    fn extract_chapter_date_invalid_element_data() {
        let chapter_fragment = Html::parse_fragment(
            r#"<li class="list-group-item chapter-item ">
                    <a>
                        <span class="chapter-title"></span>
                        Предельная пушка волнового потока адского огня
                    </a>
                    <span>13 декабря 2021  </span>
                </li>"#
        );
        let chapter_element = chapter_fragment.root_element();

        let result = get_valid_extractor().extract_chapter_date(&chapter_element.clone());
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        let re = Regex::new(r"^\[EXTRACTOR] can't extract chapter date container from element").unwrap();
        assert!(re.is_match(&err));
    }

    fn get_valid_html_body() -> Html {
        Html::parse_document(VALID_LAST_CHAPTER_HTML)
    }

    fn get_invalid_html_body() -> Html {
        Html::parse_document("<div>Not found</div>")
    }

    fn get_valid_extractor() -> MangapoiskExtractor {
        MangapoiskExtractor::new(get_valid_html_body())
    }
}