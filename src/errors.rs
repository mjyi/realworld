use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use serde::Serialize;
use serde_json::error::Error as JsonError;
use std::collections::HashMap;
use std::io;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io `{0}`")]
    Io(#[from] io::Error),
    #[error("database `{0}`")]
    Diesel(#[from] diesel::result::Error),
    #[error("json_error `{0}`")]
    SerdeJson(#[from] JsonError),
}

#[derive(Debug, Serialize)]
pub struct Errors {
    errors: HashMap<&'static str, Vec<String>>,
}



#[derive(Error, Debug)]
#[error("...")]
pub struct FieldValidErrors(#[from] pub ValidationErrors);

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

// impl ResponseError for Error {

//     fn error_response(&self) -> web::HttpResponse {
//         match self {
//             Diesel()



//         }


//     }

// }
