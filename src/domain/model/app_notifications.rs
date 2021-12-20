use crate::domain::schema::app_notifications;

#[derive(Debug, Clone, Queryable)]
pub struct AppNotification {
    pub id: i32,
    pub notification_type: String,
    pub text: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "app_notifications"]
pub struct NewAppNotification {
    pub text: String,
    pub notification_type: String
}
