-- Your SQL goes here
CREATE TABLE quests (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "status" VARCHAR(255) NOT NULL,
    guild_commander_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE TABLE adventurers (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE TABLE guild_commanders (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

CREATE TABLE quest_adventure_junction (
    id SERIAL PRIMARY KEY,
    quest_id INTEGER NOT NULL,
    adventurer_id INTEGER NOT NULL,
    joined_at TIMESTAMP DEFAULT now()
);

ALTER TABLE
    quests
ADD
    CONSTRAINT fk_guild_commander FOREIGN KEY (guild_commander_id) REFERENCES guild_commanders(id);

ALTER TABLE
    quest_adventure_junction
ADD
    CONSTRAINT fk_quest FOREIGN KEY (quest_id) REFERENCES quests(id),
ADD
    CONSTRAINT fk_adventurer FOREIGN KEY (adventurer_id) REFERENCES adventurers(id);