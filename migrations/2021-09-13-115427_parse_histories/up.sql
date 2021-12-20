CREATE TABLE parse_histories
(
     topic VARCHAR(50) NOT NULL,
     last_chapter_title VARCHAR(100) NOT NULL,
     release_date VARCHAR(30) NOT NULL,

     CONSTRAINT id_parse_histories UNIQUE (topic),
     CONSTRAINT idx_topic_lcp UNIQUE (topic, last_chapter_title)
);
