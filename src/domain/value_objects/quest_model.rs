use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entities::quests::{AddQuestEntity, EditQuestEntity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub adventurer_count: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddQuestModel {
    pub name: String,
    pub description: Option<String>,
    pub guild_commander_id: i32,
}

impl AddQuestModel {
    pub fn to_entity(&self) -> AddQuestEntity {
        AddQuestEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            guild_commander_id: self.guild_commander_id,
            status: crate::domain::value_objects::quest_statuses::QuestStatuses::Open.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditQuestModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub guild_commander_id: Option<i32>,
}

impl EditQuestModel {
    pub fn to_entity(&self) -> EditQuestEntity {
        EditQuestEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            guild_commander_id: self.guild_commander_id,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
