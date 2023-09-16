use askama::Template;

#[derive(Template)]
#[template(path = "password_reset.html")]
pub struct PasswordResetTemplate<'a> {
    pub name: &'a str,
    pub url: &'a str,
}