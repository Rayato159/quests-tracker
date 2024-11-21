-- Your SQL goes here
DROP TABLE quest_adventure_junction;

CREATE TABLE quest_adventurer_junction (
    quest_id INTEGER NOT NULL,
    adventurer_id INTEGER NOT NULL,
    PRIMARY KEY (quest_id, adventurer_id)
);