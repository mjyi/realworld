use serde::{Serialize, Deserialize};
use super::*;
use chrono::{DateTime, Utc};
use crate::schema::*;
use diesel::{deserialize::Queryable, pg::Pg, prelude::*, result::Error};

#[derive(Serialize, Deserialize)]
pub struct Articles {
    pub articles: Vec<Article>,
    pub articles_count: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: Profile,
}




