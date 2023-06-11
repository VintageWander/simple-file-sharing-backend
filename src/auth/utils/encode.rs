use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::{
    config::{ACCESS_TOKEN_SECRET, REFRESH_TOKEN_SECRET},
    error::Error,
    user::model::select::UserSelect,
};

use super::{Claims, TokenType};

fn encode_token(user: &UserSelect, token_type: TokenType) -> Result<String, Error> {
    let (token_secret, duration) = match token_type {
        TokenType::Access => (ACCESS_TOKEN_SECRET, Duration::hours(3)),
        TokenType::Refresh => (REFRESH_TOKEN_SECRET, Duration::hours(12)),
    };

    let exp = Utc::now().checked_add_signed(duration).unwrap().timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        name: user.username.clone(),
        exp,
    };

    let header = Header::new(Algorithm::HS512);

    let key = EncodingKey::from_secret(token_secret.as_bytes());

    let encode = encode(&header, &claims, &key)?;

    Ok(encode)
}

pub fn encode_access_token(user: &UserSelect) -> Result<String, Error> {
    encode_token(user, TokenType::Access)
}

pub fn encode_refresh_token(user: &UserSelect) -> Result<String, Error> {
    encode_token(user, TokenType::Refresh)
}
