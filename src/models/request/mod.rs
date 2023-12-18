use std::borrow::Cow;

use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::id::{AnasetoId, UserId};

/// Create user
#[derive(Deserialize, Serialize)]
pub struct ReqRegisterUser<'a> {
    pub username: Cow<'a, str>,
    pub email: Email,
    pub password: Cow<'a, str>,
}

/// Login
#[derive(Deserialize, Serialize)]
pub struct ReqLogin<'a> {
    pub user_or_email: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

/// Create Anaseto
#[derive(Deserialize, Serialize)]
pub struct ReqNewAnaseto<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

/// List Anasetoj
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ReqListAnasetoj<'a> {
    /// Get an Anaseto by exact name
    pub name: Option<Cow<'a, str>>,

    /// Search for Anasetoj with names that contain the search string
    pub search: Option<Cow<'a, str>>,

    /// Search for Anasetoj with names that start with the search string
    ///
    /// This can be useful for auto-fill search boxes
    pub starts_with: Option<Cow<'a, str>>,
}

/// Create Post
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

/// List posts
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ReqListPosts {
    #[serde(default)]
    pub sort: PostSort,
    pub anaseto: Option<AnasetoId>,
    pub user: Option<UserId>,
    #[serde(default)]
    pub prefer_md: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub since: Option<DateTime<Utc>>,
    pub page: Option<i64>,
}
