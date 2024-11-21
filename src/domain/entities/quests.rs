use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::quests;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = quests)]
pub struct QuestEntity {
    id: i32,
    name: String,
    description: Option<String>,
    status: String,
    guild_commander_id: i32,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = quests)]
pub struct InsertQuestEntity {
    name: String,
    description: Option<String>,
    status: String,
    guild_commander_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = quests)]
pub struct UpdateQuestEntity {
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
    guild_commander_id: Option<i32>,
    updated_at: NaiveDateTime,
}
