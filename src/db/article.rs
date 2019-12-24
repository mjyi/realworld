use super::*;
use crate::schema::articles;
use chrono::{DateTime, Utc};
use diesel::{deserialize::Queryable, pg::Pg, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Articles {
    pub articles: Vec<ArticleJson>,
    pub articles_count: i64,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleJson {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i32,
    pub author: Profile,
}

impl ArticleJson {
    pub fn build(article: Article, user: User, following: bool) -> Self {
        ArticleJson {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: article.tag_list,
            created_at: article.created_at,
            updated_at: article.updated_at,
            favorited:false,
            favorites_count: article.favorites_count,
            author: user.to_profile(following)
        }
    }
}

#[derive(Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: i32,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorites_count: i32,
}

#[derive(Insertable, AsChangeset, Default, Clone)]
#[table_name = "articles"]
pub struct ArticleForm {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub author: i32,
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "articles"]
#[serde(rename_all = "camelCase")]
pub struct ArticleUpdate {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleQuery {
    tab: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn list_articles(
    conn: &PgConnection,
    form: &ArticleQuery,
    user_id: Option<i64>,
    ) -> Result<Articles, Error> {
    todo!()
}

pub fn feed(conn: &PgConnection, feed: &str) -> Result<Articles, Error> {
    todo!()
}
pub fn create(pg: &PgConnection, article: &ArticleForm) -> Result<ArticleJson, Error> {
    let db_article = diesel::insert_into(articles::table)
        .values(article)
        .get_result::<Article>(pg)?;
    let user = User::read(pg, article.author)?;

    Ok(ArticleJson::build(db_article, user, false))
}

pub fn update(conn: &PgConnection, slug: &str, article: &ArticleUpdate) -> Result<ArticleJson, Error> {
    let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
        .set(article)
        .get_result::<Article>(conn)?;


}

pub fn delete(conn: &PgConnection, slug: String) -> Result<i32, Error> {
    todo!()
}

pub fn get_comments(conn: &PgConnection, slug: String) -> Result<ArticleJson, Error> {
    todo!()
}

pub fn delete_comment(
    conn: &PgConnection,
    slug: String,
    comment_id: i32,
    ) -> Result<i32, Error> {
    todo!()
}

pub fn favorite(conn: &PgConnection, slug: String) -> Result<ArticleJson, Error> {
    todo!()
}

pub fn unfavorite(conn: &PgConnection, slug: String) -> Result<ArticleJson, Error> {
    todo!()
}

pub fn tag_list(conn: &PgConnection) -> Result<Vec<String>, Error> {
    todo!()
}
