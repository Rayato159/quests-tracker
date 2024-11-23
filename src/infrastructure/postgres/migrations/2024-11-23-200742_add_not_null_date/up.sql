-- Your SQL goes here
-- Set default values for existing NULL rows
UPDATE
    quests
SET
    created_at = now()
WHERE
    created_at IS NULL;

UPDATE
    quests
SET
    updated_at = now()
WHERE
    updated_at IS NULL;

UPDATE
    adventurers
SET
    created_at = now()
WHERE
    created_at IS NULL;

UPDATE
    adventurers
SET
    updated_at = now()
WHERE
    updated_at IS NULL;

UPDATE
    guild_commanders
SET
    created_at = now()
WHERE
    created_at IS NULL;

UPDATE
    guild_commanders
SET
    updated_at = now()
WHERE
    updated_at IS NULL;

-- Add NOT NULL constraints
ALTER TABLE
    quests
ALTER COLUMN
    created_at
SET
    NOT NULL;

ALTER TABLE
    quests
ALTER COLUMN
    updated_at
SET
    NOT NULL;

ALTER TABLE
    adventurers
ALTER COLUMN
    created_at
SET
    NOT NULL;

ALTER TABLE
    adventurers
ALTER COLUMN
    updated_at
SET
    NOT NULL;

ALTER TABLE
    guild_commanders
ALTER COLUMN
    created_at
SET
    NOT NULL;

ALTER TABLE
    guild_commanders
ALTER COLUMN
    updated_at
SET
    NOT NULL;