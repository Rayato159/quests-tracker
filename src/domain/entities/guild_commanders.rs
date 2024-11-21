use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::guild_commanders;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = guild_commanders)]
pub struct GuildCommanderEntity {
    id: i32,
    username: String,
    password: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = guild_commanders)]
pub struct InsertGuildCommanderEntity {
    username: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
