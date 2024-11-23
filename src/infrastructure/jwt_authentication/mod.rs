pub mod authentication_model;
pub mod jwt_model;

use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use jwt_model::Claims;

pub fn generate_token(secret: String, claims: &Claims) -> Result<String> {
    // HSA256
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(secret: String, token: String) -> Result<Claims> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}
