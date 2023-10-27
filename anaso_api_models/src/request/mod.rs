use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_email::Email;

#[derive(Deserialize, Serialize)]
pub struct ReqRegisterUser<'a> {
    pub username: Cow<'a, str>,
    pub email: Email,
    pub password: Cow<'a, str>,
}

#[derive(Deserialize, Serialize)]
pub struct ReqLogin<'a> {
    pub user_or_email: Cow<'a, str>,
    pub password: Cow<'a, str>,
}
