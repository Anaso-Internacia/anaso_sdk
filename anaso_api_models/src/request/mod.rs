use std::borrow::Cow;

use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::id::{AnasetoId, UserId};

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
    pub body_md: Cow<'a, str>,
}

/// How to sort posts
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum PostSort {
    /// Return posts sorted from newest to oldest
    New,

    /// Return posts sorted by how well they are trending
    #[default]
    Hot,

    /// Return posts sorted purely by score
    Top,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ReqListPosts {
    #[serde(default)]
    pub sort: PostSort,
    pub anaseto: Option<AnasetoId>,
    pub user: Option<UserId>,
    #[serde(with = "ts_seconds_option")]
    pub since: Option<DateTime<Utc>>,
    pub page: Option<i32>,
}
