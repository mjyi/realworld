use serde::Deserialize;
use validator::Validate;

use actix_web::{error, middleware, web, Error, HttpResponse};
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::{self, Pool};
use crate::errors::Error as CError;
use crate::errors::FieldValidErrors;
use crate::models::{NewUser, User};
use crate::schema::users;

#[derive(Deserialize)]
pub struct ReqUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1, message = "%s is not valid"))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

///  Registration
#[post("/users")]
pub async fn post_users(
    user: web::Json<ReqUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let new_user = user.into_inner().user;

    new_user.validate().map_err(FieldValidErrors)?;

    let username = new_user.username.unwrap();
    let email = new_user.email.unwrap();
    let password = new_user.password.unwrap();

    let pool = pool.clone();

    let user = web::block(move || create_user(pool, &username, &email, &password))
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

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

fn create_user(
    pool: web::Data<Pool>,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, CError> {
    let conn = &pool.get().unwrap();

    let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1))?;
    let new_user = &NewUser {
        username,
        email,
        hash,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)
        .map_err(CError::Diesel)
}
