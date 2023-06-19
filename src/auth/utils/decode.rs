use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{
    config::{access_token_secret, refresh_token_secret},
    error::Error,
};

use super::{Claims, TokenType};

fn decode_token(token: String, token_type: TokenType) -> Result<String, Error> {
    let token_secret = match token_type {
        TokenType::Access => access_token_secret(),
        TokenType::Refresh => refresh_token_secret(),
    };

    let key = DecodingKey::from_secret(token_secret.as_bytes());

    let validation = Validation::new(Algorithm::HS512);

    let decoded = decode::<Claims>(&token, &key, &validation)?;

    Ok(decoded.claims.sub)
}

pub fn decode_access_token(token: String) -> Result<String, Error> {
    decode_token(token, TokenType::Access)
}

pub fn decode_refresh_token(token: String) -> Result<String, Error> {
    decode_token(token, TokenType::Refresh)
}
