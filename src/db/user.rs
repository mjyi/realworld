extern crate jsonwebtoken as jwt;
use super::{Crud, Profile};

use crate::{
    auth::{Claims, Jwt},
    schema::users,
};
use chrono::{Duration, Utc};
use diesel::{deserialize::Queryable, pg::Pg, prelude::*, result::Error};
use jwt::{encode, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
    pub token: String,
}

impl Queryable<users::SqlType, Pg> for User {
    type Row = (i32, String, String, Option<String>, Option<String>, String);

    fn build(row: Self::Row) -> Self {
        User {
            id: row.0,
            username: row.1,
            email: row.2,
            bio: row.3,
            image: row.4,
            password: row.5,
            token: "".to_string(),
        }
    }
}

#[derive(Deserialize, Insertable, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UserForm {
    pub email: Option<String>,
    pub bio: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub username: Option<String>,
}

impl Crud<UserForm> for User {
    fn create(conn: &PgConnection, form: &UserForm) -> Result<Self, Error> {
        diesel::insert_into(users::table)
            .values(form)
            .get_result::<User>(conn)
    }

    fn read(conn: &PgConnection, user_id: i32) -> Result<Self, Error> {
        users::table.find(user_id).get_result::<User>(conn)
    }

    fn update(conn: &PgConnection, user_id: i32, form: &UserForm) -> Result<Self, Error> {
        diesel::update(users::table.find(user_id))
            .set(form)
            .get_result::<User>(conn)
    }

    fn delete(conn: &PgConnection, user_id: i32) -> Result<usize, Error> {
        diesel::delete(users::table.find(user_id)).execute(conn)
    }
}

impl User {
    pub fn with_email(conn: &PgConnection, email: &str) -> Result<Self, Error> {
        users::table
            .filter(users::email.eq(email))
            .get_result::<User>(conn)
    }

    pub fn with_username(conn: &PgConnection, username: &str) -> Result<Self, Error> {
        users::table
            .filter(users::username.eq(username))
            .get_result::<User>(conn)
    }

    pub fn jwt(&self, secret: &str) -> Jwt {
        let exp = Utc::now() + Duration::days(30);
        let my_claims = Claims {
            id: self.id,
            username: self.username.to_owned(),
            exp: exp.timestamp(),
        };
        encode(&Header::default(), &my_claims, secret.as_ref()).unwrap()
    }

    pub fn to_profile(&self, following: bool) -> Profile {
        Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following,
        }
    }
}
