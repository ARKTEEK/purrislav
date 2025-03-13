-- Your SQL goes here
CREATE TABLE birthdays
(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id             BIGINT                            NOT NULL,
    guild_id            BIGINT                            NOT NULL,
    date                DATE                              NOT NULL,
    announced_this_year BOOLEAN                           NOT NULL,
    UNIQUE (user_id, guild_id)
);