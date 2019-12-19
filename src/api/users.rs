use serde::Deserialize;
use validator::Validate;
use bcrypt::{hash, verify, DEFAULT_COST};
use actix_web::{error, Result, middleware, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};

use crate::{
    db::{Crud , User, UserForm},
    Pool,
    errors::Errors,
    auth::Claims,
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


#[derive(Deserialize)]
pub struct LoginUser {
    user: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    email: String,
    password: String,
}


/// Authentication
#[post("/users/login")]
pub(crate) async fn login(
    user: web::Json<LoginUser>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let login_user = user.into_inner().user;
    let email = login_user.email.clone();
    let db_user = web::block(move || {
        let conn = pool.get().unwrap();
        User::with_email(&conn, &email)
    })
    .await
    .map_err(|_| Errors::with_field("email", "the email is not registered"))?;
    
    let valid = verify(&login_user.password, &db_user.password)
        .map_err(|_| Errors::with_field("password", "incorrectly"))?;
    if !valid {
        return Err(Errors::with_field("password", "incorrectly"))?;
    }

    Ok(HttpResponse::Ok().json(db_user))
        
}

#[get("/user")]
pub(crate) async fn get_user(token: Result<Claims>) -> Result<HttpResponse, Error> {
    match token {
        Ok(claims) => Ok(HttpResponse::Ok().json(claims)),
        Err(e) => Err(e),
    }  
}

#[put("/user")]
pub(crate) async fn put_user() -> &'static str {
    "put user"
}

