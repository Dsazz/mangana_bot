table! {
    subscribers(chat_id) {
        chat_id -> BigInt,
    }
}
table! {
    app_notifications(id) {
        id -> Integer,
        notification_type -> Text,
        text -> Text,
        created_at -> Timestamp,
    }
}
table! {
    topic(id) {
        id -> Integer,
        site_id -> Integer,
        name -> Text,
        url_name -> Text,
    }
}
allow_tables_to_appear_in_same_query!(topic, parse_histories, subscriber_topics);
joinable!(topic -> parse_histories(id));
joinable!(subscriber_topics -> parse_histories(topic_id));
table! {
    parse_histories(topic_id) {
        topic_id -> Integer,
        last_chapter_title -> Text,
        chapter_name -> Text,
        url -> Text,
        release_date -> Text,
    }
}
joinable!(subscriber_topics -> topic(topic_id));
table! {
    subscriber_topics (chat_id, topic_id) {
        chat_id -> BigInt,
        topic_id -> Integer,
    }
}
table! {
    topic_notification_status(topic_id) {
        topic_id -> Integer,
        last_chapter_title -> Text,
        updated_at -> Timestamp,
    }
}
