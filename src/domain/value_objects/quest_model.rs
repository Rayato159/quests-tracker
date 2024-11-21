use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestModel {
    id: i32,
    name: String,
    description: Option<String>,
    status: String,
    guild_commander_id: i32,
    adventurer_count: i32,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertQuestModel {
    name: String,
    description: Option<String>,
    status: String,
    guild_commander_id: i32,
    adventurer_count: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateQuestModel {
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
    guild_commander_id: Option<i32>,
    adventurer_count: Option<i32>,
    updated_at: NaiveDateTime,
}
