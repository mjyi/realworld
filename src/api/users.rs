use serde::Deserialize;
use validator::Validate;
use bcrypt::{hash, DEFAULT_COST};
use actix_web::{error, middleware, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};

use crate::{
    db::{Crud , User, UserForm},
    Pool,
    errors::Errors,
};


#[derive(Deserialize)]
pub struct ReqUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1, message = "username is not valid"))]
    username: Option<String>,
    #[validate(email(message = "Email is not valid"))]
    email: Option<String>,
    #[validate(length(min = 8, message = "password is to0 short"))]
    password: Option<String>,
}


#[derive(Deserialize)]
struct LoginUser {
    user: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    email: Option<String>,
    password: Option<String>,
}

///  Registration
#[post("/users")]
pub async fn post_users(
    user: web::Json<ReqUser>,
    pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
    let new_user = user.into_inner().user;

    new_user.validate().map_err(Errors::from)?;

    let username = new_user.username.unwrap();
    let email = new_user.email.unwrap();
    let password = new_user.password.unwrap();

    let hashed = hash(&password, DEFAULT_COST).unwrap();

    let user_form = UserForm {
        username: Some(username),
        password: Some(hashed),
        email: Some(email),
        bio: None,
        image: None,
    };

    let user = web::block(move || {
        let conn = pool.get().unwrap();
        User::create(&conn, &user_form)
    })
    .await
    .map_err(Errors::from)?;
    
    Ok(HttpResponse::Ok().json(user))
}


/// Authentication
#[post("/users/login")]
pub(crate) async fn login() -> &'static str {
    "login"
}

#[get("/user")]
pub(crate) async fn get_user() -> &'static str {
    "get user"
}

#[put("/user")]
pub(crate) async fn put_user() -> &'static str {
    "put user"
}

