use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;
use dotenv::dotenv;
use std::env;
use crate::errors::CliError;

pub mod user;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub trait Crud<T> {
    fn create(conn: &PgConnection, form: &T) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(conn: &PgConnection, id: i32) -> Result<Self, Error>
    where
        Self: Sized;

    fn update(conn: &PgConnection, id: i32, form: &T) -> Result<Self, Error>
    where
        Self: Sized;

    fn delete(conn: &PgConnection, id: i32) -> Result<usize, Error>
    where
        Self: Sized;
}

