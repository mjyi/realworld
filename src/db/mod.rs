use diesel::pg::PgConnection;
use diesel::result::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub mod profile;
pub mod user;
pub mod article;
pub mod comment;

pub use profile::Profile;
pub use user::{User, UserForm};
pub use article::{ ArticleForm, Article};

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

