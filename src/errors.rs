use std::{collections::HashMap, fmt, io};
use serde::Serialize;
use serde_json::error::Error as JsonError;
use actix_web::{
    web, 
    http::StatusCode,
    error::BlockingError,
    ResponseError
};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use validator::{ValidationErrors, ValidationErrorsKind};
use derive_more::{Display, From};


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

impl std::error::Error for CliError {}

#[derive(Debug, Serialize)]
pub struct Errors {
    #[serde(skip_serializing)]
    status_code: StatusCode,
    errors: HashMap<&'static str, Vec<String>>,
}

impl Errors {
    pub fn new() -> Self {
        Errors {
            status_code: StatusCode::OK,
            errors: HashMap::new(),
        }
    }

    pub fn set_code(&mut self, code: StatusCode) {
        self.status_code = code
    }

    pub fn insert_error(&mut self, field: &'static str, error: &str) {
        self.errors.insert(field, vec![error.to_string()]).unwrap();
    }
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

impl From<DieselError> for Errors {
    fn from(err: DieselError) -> Self {
        let mut errors = Errors::new();
        if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("users_username_key") => errors.insert_error("username", "duplicated"),
                Some("users_email_key") => errors.insert_error("email", "duplicated"),
                _ => (),
            }
        }

        errors
    }
}


impl From<BlockingError<DieselError>> for Errors {
    fn from(err: BlockingError<DieselError>) -> Self {
        match err {
            BlockingError::Error(e) =>Errors::from(e),
            _ => Errors::new(),
        }
    }
}

impl ResponseError for Errors {
    fn error_response(&self) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code).json(self)
    }
}
