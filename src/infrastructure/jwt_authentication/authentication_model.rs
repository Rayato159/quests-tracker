use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}
