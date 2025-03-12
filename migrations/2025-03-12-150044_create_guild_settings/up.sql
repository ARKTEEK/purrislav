-- Your SQL goes here
CREATE TABLE guild_settings
(
    id                       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    guild_id                 BIGINT                            NOT NULL,
    announcements_channel_id BIGINT
);