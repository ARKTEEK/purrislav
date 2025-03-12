-- Your SQL goes here
CREATE TABLE birthdays
(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id             BIGINT                            NOT NULL,
    date                DATE                              NOT NULL,
    announced_this_year BOOLEAN                           NOT NULL
);

-- Add a unique constraint to user_id
CREATE UNIQUE INDEX user_id_unique_index ON birthdays (user_id);
