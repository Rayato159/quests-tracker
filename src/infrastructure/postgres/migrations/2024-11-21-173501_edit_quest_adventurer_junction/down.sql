-- This file should undo anything in `up.sql`
DROP TABLE quest_adventurer_junction;

CREATE TABLE quest_adventure_junction (
    quest_id INTEGER NOT NULL,
    adventurer_id INTEGER NOT NULL,
    joined_at TIMESTAMP DEFAULT now(),
    PRIMARY KEY (quest_id, adventurer_id)
);