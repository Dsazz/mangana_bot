use crate::domain::schema::subscribers;

#[derive(Queryable,Insertable)]
pub struct Subscriber {
    pub chat_id: i64,
}