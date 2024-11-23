-- Your SQL goes here
ALTER TABLE
    adventurers
ADD
    CONSTRAINT adventurers_unique_username UNIQUE (username);

ALTER TABLE
    guild_commanders
ADD
    CONSTRAINT guild_commanders_unique_username UNIQUE (username);