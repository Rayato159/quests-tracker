pub mod argon2_hashing;

use anyhow::Result;
use mockall::automock;

#[automock]
pub trait Hashing {
    fn hash(&self, password: String) -> Result<String>;
    fn verify(&self, password: String, hashed_password: String) -> Result<bool>;
}
