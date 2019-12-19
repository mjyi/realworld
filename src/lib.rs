#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;

use std::{error::Error, fmt, io};

use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub mod api;
pub mod db;
pub mod errors;
pub mod schema;
pub mod models;
pub mod auth;

use errors::CliError;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppConfig {
    pool: Pool,
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppConfig")
    }
}

fn db_pool(database_url: &str) -> Result<Pool, CliError> {
    let manager = ConnectionManager::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}

pub async fn run(database_url: &str) -> Result<(), errors::CliError> {
    
    let pool = db_pool(database_url)?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::new("%a \"%r\" :: %s :: %b bytes %T"))
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
    .await?;

    Ok(())
}
