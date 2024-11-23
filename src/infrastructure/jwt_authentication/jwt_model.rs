use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Roles,
    pub exp: usize,
    pub iat: usize,
}

// Example of usage:
// &Claims {
//     sub: sub.to_string(),
//     role: role.to_string(),
//     exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
//     iat: Utc::now().timestamp() as usize,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    Adventurer,
    GuildCommander,
}
