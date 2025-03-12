-- Your SQL goes here
CREATE TABLE guild_settings
(
    id                       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    guild_id                 BIGINT                            NOT NULL,
    announcements_channel_id BIGINT
);

CREATE UNIQUE INDEX guild_id_unique_index ON guild_settings (guild_id);
