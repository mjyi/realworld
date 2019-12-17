use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use serde::Serialize;
use serde_json::error::Error as JsonError;
use validator::{ValidationErrors, ValidationErrorsKind};
use derive_more::{ From, Display};
use std::io;
use std::collections::HashMap;

#[derive(Debug, From, Display)]
pub enum Error {
    Io(io::Error),
    #[display(fmt = "database error")]
    Diesel(diesel::result::Error),
    #[display(fmt = "Parse json error")]
    SerdeJson(JsonError),
}

#[derive(Debug, Serialize)]
pub struct Errors {
    errors: HashMap<&'static str, Vec<String>>,
}


#[derive(Debug, From, Display)]
pub struct FieldValidErrors(pub ValidationErrors);

impl ResponseError for FieldValidErrors {
    fn error_response(&self) -> web::HttpResponse {
        let hash_map = self
            .0
            .errors()
            .iter()
            .filter_map(|(k, v)| {
                if let ValidationErrorsKind::Field(errors) = v {
                    let vec = errors.into_iter().map(|f| f.code.to_string()).collect();
                    Some((*k, vec))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        let e = Errors { errors: hash_map };
        web::HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e)
    }
}

