-- This file should undo anything in `up.sql`
ALTER TABLE
    quests
ALTER COLUMN
    created_at DROP NOT NULL;

ALTER TABLE
    quests
ALTER COLUMN
    updated_at DROP NOT NULL;

ALTER TABLE
    adventurers
ALTER COLUMN
    created_at DROP NOT NULL;

ALTER TABLE
    adventurers
ALTER COLUMN
    updated_at DROP NOT NULL;

ALTER TABLE
    guild_commanders
ALTER COLUMN
    created_at DROP NOT NULL;

ALTER TABLE
    guild_commanders
ALTER COLUMN
    updated_at DROP NOT NULL;