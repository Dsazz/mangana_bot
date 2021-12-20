use crate::domain::schema::parse_histories;
use teloxide::{utils::{markdown::{escape, italic, link}}};
use crate::domain::model::topic::Topic;
use diesel::sql_types::{Integer, Text};


#[derive(Debug, Clone, Queryable, QueryableByName, Insertable, AsChangeset, Associations)]
#[table_name = "parse_histories"]
#[belongs_to(Topic, foreign_key = "topic_id")]
pub struct Raw {
    #[sql_type = "Integer"]
    pub topic_id: i32,
    #[sql_type = "Text"]
    pub last_chapter_title: String,
    #[sql_type = "Text"]
    pub chapter_name: String,
    #[sql_type = "Text"]
    pub url: String,
    #[sql_type = "Text"]
    pub release_date: String,
}

impl Raw {
    pub fn to_normal(&self, topic: Topic) -> ParseHistory {
        ParseHistory {
            topic,
            last_chapter_title: self.last_chapter_title.to_string(),
            chapter_name: self.chapter_name.to_string(),
            url: self.url.to_string(),
            release_date: self.release_date.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseHistory {
    pub topic: Topic,
    pub last_chapter_title: String,
    pub chapter_name: String,
    pub url: String,
    pub release_date: String,
}
impl ParseHistory {

    pub fn to_raw(&self) -> Raw {
        Raw {
            topic_id: self.topic.id,
            last_chapter_title: self.last_chapter_title.to_string(),
            chapter_name: self.chapter_name.to_string(),
            url: self.url.to_string(),
            release_date: self.release_date.to_string()
        }
    }

    /*
     * @todo probably need to move to some Formatter structure
     *
     * output example:
     *
     *  👁️‍️ One Punch Man
     *    📖 [Том 29 Глава 192](http://manga.com/link) | 'Chapter name'
     *    ⏱ 29 Августа 2021
     */
    pub fn formatted(&self) -> String {
        let mut result = "".to_owned();

        result.push_str(&format!("\u{1f441}\u{fe0f}\u{200d}\u{fe0f} {}\n", escape(&self.topic.name)));
        result.push_str(&format!("    \u{1f4d6} {}", link(
            &self.url, &italic(&self.last_chapter_title),
        )));
        if !self.chapter_name.trim().is_empty() {
            result.push_str(&format!(" \\| '{}'", italic(&escape(&self.chapter_name))));
        }
        result.push_str(&format!("\n    \u{23f1} {}", italic(&escape(&self.release_date))));

        result
    }

    pub fn to_log(&self) -> String {
        let mut data = "|> Parse History: \n".to_owned();
        data.push_str(&format!("  topic_id   | {}\n", &self.topic.id));
        data.push_str(&format!("  topic_name | {}\n", &self.topic.name));
        data.push_str(&format!("  title      | {}\n", &self.last_chapter_title));
        data.push_str(&format!("  name       | {}\n", &self.chapter_name));
        data.push_str(&format!("  release    | {}\n", &self.release_date));
        data.push_str(&format!("  url        | {}\n", &self.url));
        data.push_str("----------------------------------------------\n");

        data
    }
}