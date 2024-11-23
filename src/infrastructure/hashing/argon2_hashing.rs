use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use super::Hashing;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Argon2Hashing;

impl Argon2Hashing {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Argon2Hashing {
    fn default() -> Self {
        Self::new()
    }
}

impl Hashing for Argon2Hashing {
    fn hash(&self, password: String) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let bytes_password = password.as_bytes();

        let argon2 = Argon2::default();

        let result = argon2
            .hash_password(bytes_password, &salt)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(result.to_string())
    }

    fn verify(&self, password: String, hashed_password: String) -> Result<bool> {
        let parsed_hash =
            PasswordHash::new(&hashed_password).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let bytes_password = password.as_bytes();

        Ok(Argon2::default()
            .verify_password(bytes_password, &parsed_hash)
            .is_ok())
    }
}
