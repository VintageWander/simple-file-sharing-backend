use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

use crate::{
    auth::utils::decode::decode_access_token,
    error::Error,
    prisma::user::{self, Data},
    Database,
};

pub struct LoggedInUser(pub Data);

#[async_trait]
impl FromRequestParts<Database> for LoggedInUser {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Database,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state)
            .await
            .expect("This should not be happening");

        let access_token = cookies
            .get("accessToken")
            .ok_or_else(|| Error::Unauthorized)?
            .value()
            .to_string();

        let user_id = decode_access_token(access_token)?;
        let user = state
            .user()
            .find_unique(user::id::equals(user_id))
            .exec()
            .await?
            .ok_or_else(|| Error::NotFound)?;
        Ok(Self(user))
    }
}
