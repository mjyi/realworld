#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

use std::{env, io};

use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};

mod api;
mod db;
mod errors;
mod models;
mod schema;

async fn index() -> &'static str {
    "Hello world"
}

async fn login() -> &'static str {
    "login"
}

pub async fn run() -> io::Result<()> {
    let pool = db::establish_connection_pool().expect("cannot create db pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::new("%a \"%r\" :: %s :: %b bytes %T"))
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::scope("/api")
                    .service(api::users::post_users)
                    .service(api::users::login)
                    .service(api::users::get_user)
                    .service(api::users::put_user),
            )
    })
    .bind("127.0.0.1:8088")?
    .start()
    .await
}
