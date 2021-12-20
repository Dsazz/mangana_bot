CREATE TABLE subscriber_topics
(
     chat_id BIGINT NOT NULL,
     topic_id INT NOT NULL,
     PRIMARY KEY (chat_id, topic_id),
     FOREIGN KEY (chat_id) REFERENCES subscribers(chat_id) ON DELETE CASCADE,
     FOREIGN KEY (topic_id) REFERENCES topic(id) ON DELETE CASCADE
);
