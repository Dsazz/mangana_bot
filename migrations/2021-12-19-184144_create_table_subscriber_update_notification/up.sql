CREATE TABLE topic_notification_status
(
     topic_id INT NOT NULL,
     last_chapter_title VARCHAR(100) NOT NULL,
     updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

     PRIMARY KEY (topic_id),
     FOREIGN KEY (topic_id) REFERENCES topic(id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('topic_notification_status');
