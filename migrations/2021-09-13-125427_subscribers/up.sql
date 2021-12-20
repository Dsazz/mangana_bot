CREATE TABLE subscribers
(
     chat_id BIGINT NOT NULL,
     CONSTRAINT id_subscribers UNIQUE (chat_id)
);