use diesel::sql_types::{Integer, Text, Bool};

#[derive(QueryableByName)]
pub struct CheckboxedSubscriberTopic {
    #[sql_type = "Integer"]
    pub topic_id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Bool"]
    pub checked: bool,
}