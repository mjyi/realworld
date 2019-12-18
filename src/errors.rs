use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use derive_more::{Display, From};
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::error::Error as JsonError;
use std::collections::HashMap;
use std::fmt;
use std::io;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug, From, Display)]
pub enum CliError {
    Io(io::Error),
    #[display(fmt = "database error")]
    Diesel(DieselError),
    #[display(fmt = "Parse json error")]
    SerdeJson(JsonError),
    R2d2Error(r2d2::Error),
    EnvError(std::env::VarError),
}

impl std::error::Error for CliError {

}


#[derive(Debug, Serialize)]
pub struct Errors {
    #[serde(skip_serializing)]
    status_code: StatusCode,
    errors: HashMap<&'static str, Vec<String>>,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<ValidationErrors> for Errors {
    fn from(valied_errors: ValidationErrors) -> Self {
        let hash_map = valied_errors
            .errors()
            .iter()
            .filter_map(|(k, v)| {
                if let ValidationErrorsKind::Field(errors) = v {
                    let vec = errors
                        .into_iter()
                        .map(|f| {
                            let code = f.code.to_string();
                            match &f.message {
                                Some(msg) => msg.to_string(),
                                None => code,
                            }
                        })
                        .collect();
                    Some((*k, vec))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        Self {
            status_code: StatusCode::UNPROCESSABLE_ENTITY,
            errors: hash_map,
        }
    }
}

impl ResponseError for Errors {
    fn error_response(&self) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code).json(self)
    }
}
