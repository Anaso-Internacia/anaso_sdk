use std::borrow::Cow;

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::id::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiInfo<'a> {
    pub version: Cow<'a, str>,
    pub deprecation_warning: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RegistrationError {
    UsernameTaken,
    EmailTaken,
    PasswordError,
    InsecurePassword,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PostData<'a> {
    pub id: PostId,
    pub anaseto_id: AnasetoId,
    pub user_id: UserId,
    pub title: Cow<'a, str>,
    pub body_md: Option<Cow<'a, str>>,
    pub body_html: Option<Cow<'a, str>>,
    pub score: i64,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}
