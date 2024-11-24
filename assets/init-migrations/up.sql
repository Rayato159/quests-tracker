-- Your SQL goes here
CREATE TABLE quests (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "status" VARCHAR(255) NOT NULL,
    guild_commander_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

CREATE TABLE adventurers (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE guild_commanders (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE quest_adventurer_junction (
    quest_id INTEGER NOT NULL,
    adventurer_id INTEGER NOT NULL,
    PRIMARY KEY (quest_id, adventurer_id)
);

ALTER TABLE
    quests
ADD
    CONSTRAINT fk_guild_commander FOREIGN KEY (guild_commander_id) REFERENCES guild_commanders(id);

ALTER TABLE
    quest_adventurer_junction
ADD
    CONSTRAINT fk_quest FOREIGN KEY (quest_id) REFERENCES quests(id),
ADD
    CONSTRAINT fk_adventurer FOREIGN KEY (adventurer_id) REFERENCES adventurers(id);