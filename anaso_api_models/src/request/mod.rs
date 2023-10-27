use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_email::Email;

#[derive(Deserialize, Serialize)]
pub struct ReqRegisterUser<'a> {
    username: Cow<'a, str>,
    email: Email,
    password: Cow<'a, str>,
}

#[derive(Deserialize, Serialize)]
pub struct ReqLogin<'a> {
    user_or_email: Cow<'a, str>,
    password: Cow<'a, str>,
}
