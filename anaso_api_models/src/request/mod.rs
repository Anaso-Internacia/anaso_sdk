use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::id::AnasetoId;

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

#[derive(Deserialize, Serialize)]
pub struct ReqNewAnaseto<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

#[derive(Deserialize, Serialize)]
pub struct ReqNewPost<'a> {
    pub anaseto_id: AnasetoId,
    pub title: Cow<'a, str>,
    pub content_md: Cow<'a, str>,
}
