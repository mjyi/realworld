#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::net::IpAddr;
use std::{env, fmt};

pub mod api;
pub mod auth;
pub mod db;
pub mod errors;
pub mod models;
pub mod schema;

use errors::CliError;

pub struct Settings {
    pub database_url: String,
    pub jwt_secret: String,
    pub hostname: String,
    pub bind: IpAddr,
    pub port: u16,
}

impl Settings {
    pub fn get() -> Self {
        dotenv().ok();
        Settings {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            hostname: env::var("HOSTNAME").unwrap_or_else(|_| "www".to_string()),
            bind: env::var("BIND")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .parse()
                .unwrap(),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8088".to_string())
                .parse()
                .unwrap(),
        }
    }
}

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

pub async fn run(settings: Settings) -> Result<(), errors::CliError> {
    let pool = db_pool(&settings.database_url)?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Cors::new().max_age(3600).finish())
            .wrap(middleware::Logger::new("%a \"%r\" :: %s :: %b bytes %T"))
            .service(
                web::scope("/api")
                    .service(api::users::post_users)
                    .service(api::users::login)
                    .service(api::users::get_user)
                    .service(api::users::put_user)
                    .service(api::profile::get_profiles)
                    .service(api::profile::follow)
                    .service(api::profile::unfollow)
                    .service(api::articles::list_articles)
                    .service(api::articles::feed_articles)
                    .service(api::articles::get_article)
                    .service(api::articles::create_article)
                    .service(api::articles::update_article)
                    .service(api::articles::delete_article)
                    .service(api::articles::add_comment)
                    .service(api::articles::get_comments)
                    .service(api::articles::delete_comment)
                    .service(api::articles::favorite)
                    .service(api::articles::unfavorite)
                    .service(api::articles::tags),
            )
    })
    .bind((settings.bind, settings.port))?
    .start()
    .await?;

    Ok(())
}
