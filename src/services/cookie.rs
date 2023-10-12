use actix_web::cookie::Cookie as ActixCookie;
use actix_web::cookie::time::Duration;

pub trait Cookie<'a> {
    const NAME: &'static str;
    const EXPIRE: i64;
    fn set(value: &'a str) -> ActixCookie<'a> {
        Self::build(value, Self::EXPIRE)
    }

    fn remove() -> ActixCookie<'a> {
        let mut cookie = Self::build("", -1);
        cookie.make_removal();

        cookie
    }

    fn build(value: &'a str, expire: i64) -> ActixCookie<'a> {
        ActixCookie::build(Self::NAME, value)
            .path("/")
            .max_age(Duration::new(expire, 0))
            .http_only(true)
            .secure(!cfg!(debug_assertions))
            .finish()
    }
}

pub struct AccessTokenCookie;

impl Cookie<'_> for AccessTokenCookie {
    const NAME: &'static str = "__Secured-ATT";
    const EXPIRE: i64 = 60 * 60;
}