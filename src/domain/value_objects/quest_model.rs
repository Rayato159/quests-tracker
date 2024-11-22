use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub adventurer_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddQuestModel {
    pub name: String,
    pub description: Option<String>,
    pub guild_commander_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditQuestModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub guild_commander_id: Option<i32>,
}
