use crate::schema::users;
use serde::{ Serialize, Deserialize };
use diesel::prelude::*;
use diesel::result::Error;
use super::Crud;

#[derive(Queryable, Serialize)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
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
        diesel::insert_into(users::table).values(form).get_result::<User>(conn)
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


