use super::*;
use crate::schema::*;
use diesel::{dsl::exists, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn find_follered(conn: &PgConnection, follower: i32, followed: i32) -> Result<Self, Error> {
        let p: Profile = users::table
            .left_join(
                follows::table.on(follows::followed.eq(users::id).and(
                    follows::followed
                        .eq(followed)
                        .and(follows::follower.eq(follower)),
                )),
            )
            .select((
                users::all_columns,
                follows::followed.nullable().is_not_null(),
            ))
            .get_result::<(User, bool)>(conn)
            .map(|(user, following)| user.to_profile(following))?;
        Ok(p)
    }

    fn is_following(conn: &PgConnection, followed: i32, follower: i32) -> Result<bool, Error> {
        let f = diesel::select(exists(follows::table.find((followed, follower))))
            .get_result::<bool>(conn)?;
        Ok(f)
    }

    pub fn follow(conn: &PgConnection, followed_name: &str, follower: i32) -> Result<Self, Error> {
        let followed = User::with_username(conn, followed_name)?;
        diesel::insert_into(follows::table)
            .values((
                follows::follower.eq(follower),
                follows::followed.eq(followed.id),
            ))
            .execute(conn)?;

        Ok(followed.to_profile(true))
    }

    pub fn unfollow(
        conn: &PgConnection,
        followed_name: &str,
        follower: i32,
    ) -> Result<Self, Error> {
        let followed = User::with_username(conn, followed_name)?;

        diesel::delete(follows::table.find((follower, followed.id))).execute(conn)?;

        Ok(followed.to_profile(false))
    }
}
