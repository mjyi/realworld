use super::User;
use crate::schema::*;
use diesel::{dsl::exists, prelude::*, result::Error};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl Profile {
    pub fn get_proflies(
        conn: &PgConnection,
        username: &str,
        id: Option<i32>,
    ) -> Result<Self, Error> {
        let user = users::table
            .filter(users::username.eq(username))
            .get_result::<User>(conn)?;

        let following = id
            .map(|id| Profile::is_following(conn, user.id, id).unwrap_or(false))
            .unwrap_or(false);

        Ok(Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following,
        })
    }

    fn is_following(conn: &PgConnection, followed: i32, follower: i32) -> Result<bool, Error> {
        let f = diesel::select(exists(follows::table.find((followed, follower))))
            .get_result::<bool>(conn)?;
        Ok(f)
    }

    pub fn follow(conn: &PgConnection, followed: i32, follower: i32) -> Result<Self, Error> {
        todo!();
    }

    pub fn unfollow(conn: &PgConnection, followed: i32, follower: i32) -> Result<Self, Error> {
        todo!()
    }
}