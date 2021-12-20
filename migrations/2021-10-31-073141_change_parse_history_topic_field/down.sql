ALTER TABLE parse_histories DROP CONSTRAINT id_parse_histories;
ALTER TABLE parse_histories DROP CONSTRAINT idx_topic_lcp;
ALTER TABLE parse_histories DROP COLUMN topic_id;
ALTER TABLE parse_histories ADD COLUMN topic VARCHAR(50) NOT NULL;
ALTER TABLE parse_histories ADD CONSTRAINT id_parse_histories UNIQUE (topic);
ALTER TABLE parse_histories ADD CONSTRAINT idx_topic_lcp UNIQUE (topic, last_chapter_title);