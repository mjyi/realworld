use crate::schema::users;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::result::Error;


#[derive(Queryable, Serialize)]
pub struct User {
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
    pub hash: &'a str,
}


pub fn create_user(
    conn: &PgConnection,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, Error> {
    
    let hash = hash(password, DEFAULT_COST).expect("Couldn't hash passowrd");
    let new_user = &NewUser { 
        username, 
        email, 
        hash: &hash 
    };
    
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
}

