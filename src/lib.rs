#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

use std::{env, io};

use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use actix_session::{CookieSession, Session};

mod schema;
mod routes;
mod db;
mod models;



use db::Pool;

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
            .wrap(CookieSession::signed(&[0;32]).secure(false))
            .wrap(middleware::Logger::new("%a \"%r\" :: %s :: %b bytes %T"))
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::scope("/api")
                    .service(routes::users::post_users)
                    .service(routes::users::login)
                    .service(routes::users::get_user)
                    .service(routes::users::put_user)
                )
    })
    .bind("127.0.0.1:8088")?
    .start()
    .await
}

