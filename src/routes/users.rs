use serde::Deserialize;
use validator::Validate;

use crate::db::Pool;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}


#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}


///  Registration
#[post("/users")]
pub(crate) async fn post_users() -> &'static str {
    
    "users .." 
}

/// Authentication
#[post("/users/login")]
pub(crate) async fn login() -> &'static str {
    "login"
}


#[get("/user")]
pub(crate) async fn get_user() -> &'static str { "get user" }

#[put("/user")]
pub(crate) async fn put_user() -> &'static str { "put user" }

