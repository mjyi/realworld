use actix_web::{web, Error, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;


// pub async articles(query: web::Query<>) {}
