CREATE TABLE topic
(
     id SERIAL PRIMARY KEY,
     site_id INT NOT NULL,
     name VARCHAR(50) NOT NULL,
     url_name VARCHAR(50) NOT NULL
);
CREATE INDEX topic_name_idx ON topic(name);
CREATE UNIQUE INDEX id_topic_idx ON topic(site_id, url_name);

INSERT INTO topic(site_id, name, url_name)
VALUES
    (1, 'One Punch Man', 'vanpanchmen'),
    (1, 'One Punch-Man (ONE)', 'one-punch-man-one'),
    (1, 'Boruto: Naruto Next Generations', 'boruto'),
    (1, 'One Piece', 'van-pis');
