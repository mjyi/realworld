use super::*;
use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::{prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Articles {
    pub articles: Vec<Article>,
    pub articles_count: i64,
}

#[derive(Serialize, Deserialize,Debug)]
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
    pub favorites_count: i32,
    pub author: Profile,
}

impl Article {
    fn build(article: ArticleData, profile: Profile) -> Self {
        Article {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: article.tag_list,
            created_at: article.created_at,
            updated_at: article.updated_at,
            favorited: false,
            favorites_count: article.favorites_count,
            author: profile,
        }
    }

    fn favorite(mut self, f: bool) -> Self {
        self.favorited = f;
        self
    }
}

#[derive(Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
struct ArticleData {
    id: i32,
    slug: String,
    title: String,
    description: String,
    body: String,
    author: i32,
    tag_list: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    favorites_count: i32,
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

#[derive(Deserialize, Default, Clone)]
pub struct ArticleUpdate {
    pub article: ArticleUpdateData,
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "articles"]
pub struct ArticleUpdateData {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleQuery {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn list_articles(
    conn: &PgConnection,
    form: &ArticleQuery,
    user_id: Option<i32>,
) -> Result<Articles, Error> {
    let mut query = articles::table
        .inner_join(users::table)
        .left_join(
            favorites::table.on(articles::id
                .eq(favorites::article)
                .and(favorites::user.eq(user_id.unwrap_or(0)))),
        )
        .select((
            articles::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .into_boxed();

    if let Some(ref tag) = form.tag {
        query = query.filter(articles::tag_list.contains(vec![tag]))
    }
    if let Some(ref author) = form.author {
        query = query.or_filter(users::username.eq(author))
    }
    if let Some(ref favorited) = form.favorited {
        let uid = users::table
            .select(users::id)
            .filter(users::username.eq(favorited))
            .get_result::<i32>(conn)?;
        query = query.or_filter(favorites::user.eq(uid))
    }
    let articles: Vec<Article> = query
        .offset(form.offset.unwrap_or(0))
        .limit(form.limit.unwrap_or(20))
        .load::<(ArticleData, User, bool)>(conn)
        .map(|res| {
            res.into_iter()
                .map(|(article, author, favorited)| {
                    Article::build(article, author.to_profile(false)).favorite(favorited)
                })
                .collect()
        })
        .unwrap_or(vec![]);
    let articles_count = articles.len() as i64;
    Ok(Articles {
        articles,
        articles_count,
    })
}

pub fn feed(conn: &PgConnection, form: &ArticleQuery, user_id: i32) -> Result<Articles, Error> {
    let articles: Vec<Article> = articles::table
        .filter(
            articles::author.eq_any(
                follows::table
                    .select(follows::followed)
                    .filter(follows::follower.eq(user_id)),
            ),
        )
        .inner_join(users::table)
        .left_join(
            favorites::table.on(articles::id
                .eq(favorites::article)
                .and(favorites::user.eq(user_id))),
        )
        .select((
            articles::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .limit(form.limit.unwrap_or(20))
        .offset(form.offset.unwrap_or(0))
        .load::<(ArticleData, User, bool)>(conn)
        .map(|res| {
            res.into_iter()
                .map(|(article, author, favorited)| {
                    Article::build(article, author.to_profile(false)).favorite(favorited)
                })
                .collect()
        })
        .unwrap_or(vec![]);
    let articles_count = articles.len() as i64;
    Ok(Articles {
        articles,
        articles_count,
    })
}

pub fn create(pg: &PgConnection, article: &ArticleForm) -> Result<Article, Error> {
    let db_article = diesel::insert_into(articles::table)
        .values(article)
        .get_result::<ArticleData>(pg)?;
    let user = User::read(pg, article.author)?;
    let profile = user.to_profile(false);
    Ok(Article::build(db_article, profile))
}

pub fn get_article(pg: &PgConnection, slug: &str) -> Result<Article, Error> {
    articles::table
        .inner_join(users::table.on(articles::author.eq(users::id)))
        .filter(articles::slug.eq(slug))
        .select((articles::all_columns, users::all_columns))
        .first::<(ArticleData, User)>(pg)
        .map(|(a, u)| Article::build(a, u.to_profile(false)))
}

pub fn update(
    conn: &PgConnection,
    slug: &str,
    user_id: i32,
    article: &ArticleUpdateData,
) -> Result<Article, Error> {
    let article = diesel::update(
        articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(user_id))),
    )
    .set(article)
    .get_result::<ArticleData>(conn)?;

    let author = User::read(conn, article.author)?;

    Ok(Article::build(article, author.to_profile(false)))
}

pub fn delete(conn: &PgConnection, user_id: i32, slug: &str) -> Result<usize, Error> {
    diesel::delete(
        articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(user_id))),
    )
    .execute(conn)
}

pub fn favorite(conn: &PgConnection, user_id: i32, slug: &str) -> Result<Article, Error> {
    conn.transaction::<_, Error, _>(|| {
        let article_data = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(articles::favorites_count.eq(articles::favorites_count + 1))
            .get_result::<ArticleData>(conn)?;
        diesel::insert_into(favorites::table)
            .values((
                favorites::user.eq(user_id),
                favorites::article.eq(article_data.id),
            ))
            .execute(conn)?;
        let p = Profile::find_follered(conn, user_id, article_data.author)?;
        
        Ok(Article::build(article_data, p).favorite(true))
    })
}

pub fn unfavorite(conn: &PgConnection, user_id: i32, slug: &str) -> Result<Article, Error> {
    conn.transaction::<_, Error, _>(|| {
        let article_data = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(articles::favorites_count.eq(articles::favorites_count - 1))
            .get_result::<ArticleData>(conn)?;

        diesel::delete(favorites::table.find((user_id, article_data.id))).execute(conn)?;

        let p = Profile::find_follered(conn, user_id, article_data.author)?;
        Ok(Article::build(article_data, p))
    })
}

pub fn tag_list(conn: &PgConnection) -> Result<Vec<String>, Error> {
    articles::table
        .select(diesel::dsl::sql("distinct unnest(tag_list)"))
        .load::<String>(conn)
}
