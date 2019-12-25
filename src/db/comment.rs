use crate::{
    db::{Crud, Profile, User},
    schema::*,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::exists, pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comments {
    comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Profile,
}

#[derive(Queryable, Clone)]
pub struct CommentData {
    pub id: i32,
    pub body: String,
    pub article: i32,
    pub author: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn build(comment: CommentData, user: Profile) -> Self {
        Comment {
            id: comment.id,
            body: comment.body,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            author: user,
        }
    }
}

pub fn add_comment(
    conn: &PgConnection,
    user: i32,
    slug: &str,
    body: &str,
) -> Result<Comment, Error> {
    let article_id = articles::table
        .filter(articles::slug.eq(slug))
        .select(articles::id)
        .get_result::<i32>(conn)?;

    let comment_data = diesel::insert_into(comments::table)
        .values((
            comments::body.eq(body),
            comments::author.eq(user),
            comments::article.eq(article_id),
        ))
        .get_result::<CommentData>(conn)?;

    let user = User::read(conn, user)?.to_profile(false);
    Ok(Comment::build(comment_data, user))
}

pub fn get_comments(conn: &PgConnection, user_id: i32, slug: &str) -> Result<Comments, Error> {
    let comments = comments::table
        .inner_join(
            articles::table.on(comments::article
                .eq(articles::id)
                .and(articles::slug.eq(slug))),
        )
        .left_join(users::table.on(articles::author.eq(users::id)))
        .left_join(
            follows::table.on(users::id
                .eq(follows::followed)
                .and(follows::follower.eq(user_id))),
        )
        .select((
            comments::all_columns,
            users::all_columns,
            follows::follower.nullable().is_not_null(),
        ))
        .load::<(CommentData, User, bool)>(conn)?
        .into_iter()
        .map(|(c, u, f)| Comment::build(c, u.to_profile(f)))
        .collect();
    Ok(Comments { comments })
}

pub fn delete_comment(
    conn: &PgConnection,
    user_id: i32,
    slug: String,
    comment_id: i32,
) -> Result<usize, Error> {
    let auth = diesel::select(exists(
        articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(user_id))),
    ))
    .get_result::<bool>(conn)?;

    diesel::delete(comments::table)
        .filter(comments::id.eq(comment_id))
        .execute(conn)
}
