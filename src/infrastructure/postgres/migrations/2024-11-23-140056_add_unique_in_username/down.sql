-- This file should undo anything in `up.sql`
ALTER TABLE
    adventurers DROP CONSTRAINT adventurers_unique_username;

ALTER TABLE
    guild_commanders DROP CONSTRAINT guild_commanders_unique_username;