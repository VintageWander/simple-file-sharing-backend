use cookie::{time::Duration, Cookie, SameSite};

pub fn make_access_cookie<'a>(access_token: String) -> Cookie<'a> {
    Cookie::build(("accessToken", access_token))
        .path("/")
        .max_age(Duration::hours(3))
        .http_only(true)
        .same_site(SameSite::None)
        .build()
}

pub fn make_refresh_cookie<'a>(refresh_token: String) -> Cookie<'a> {
    Cookie::build(("refreshToken", refresh_token))
        .path("/")
        .max_age(Duration::hours(12))
        .http_only(true)
        .same_site(SameSite::None)
        .build()
}
