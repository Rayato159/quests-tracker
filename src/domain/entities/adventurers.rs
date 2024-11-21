use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::adventurers;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = adventurers)]
pub struct AdventurerEntity {
    id: i32,
    username: String,
    password: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = adventurers)]
pub struct InsertAdventurerEntity {
    username: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
