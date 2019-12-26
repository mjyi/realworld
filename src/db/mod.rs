use diesel::pg::PgConnection;
use diesel::result::Error;

pub mod article;
pub mod comment;
pub mod profile;
pub mod user;

pub use article::{Article, ArticleForm};
pub use profile::Profile;
pub use user::{User, UserForm};

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
