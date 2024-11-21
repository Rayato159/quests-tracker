use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertGuildCommanderModel {
    username: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
