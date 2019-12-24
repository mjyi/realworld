use diesel::pg::PgConnection;
use diesel::result::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

mod profile;
mod user;
mod article;
mod comment;

pub use profile::Profile;
pub use user::{User, UserForm};
pub use article::*;

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

// pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";




