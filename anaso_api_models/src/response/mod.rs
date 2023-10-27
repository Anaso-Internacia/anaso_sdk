use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::id::*;

#[derive(Deserialize, Serialize)]
pub struct ApiInfo<'a> {
    pub version: Cow<'a, str>,
    pub deprecation_warning: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData<'a> {
    pub id: UserId,
    pub username: Cow<'a, str>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelfUserData<'a> {
    pub id: UserId,
    pub username: Cow<'a, str>,
    pub email: Email,
}
