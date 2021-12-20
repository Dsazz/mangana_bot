ALTER TABLE parse_histories DROP CONSTRAINT id_parse_histories;
ALTER TABLE parse_histories DROP CONSTRAINT idx_topic_lcp;
ALTER TABLE parse_histories DROP COLUMN topic;
ALTER TABLE parse_histories ADD COLUMN topic_id INT NOT NULL;
ALTER TABLE parse_histories ADD CONSTRAINT id_parse_histories FOREIGN KEY (topic_id) REFERENCES topic(id) ON DELETE CASCADE;
ALTER TABLE parse_histories ADD CONSTRAINT idx_topic_lcp UNIQUE (topic_id, last_chapter_title);