CREATE TABLE app_notifications
(
     id SERIAL PRIMARY KEY,

     notification_type CHAR(30) NOT NULL,
     text TEXT NOT NULL,
     created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
