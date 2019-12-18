use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use derive_more::{Display, From};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

pub(crate) mod users;

// #[derive(Debug, Serialize)]
// pub struct Errors {
//     errors: HashMap<&'static str, Vec<String>>,
// }

// #[derive(Debug, From, Display)]
// pub struct FieldValidErrors(pub ValidationErrors);

// impl ResponseError for FieldValidErrors {
//     fn error_response(&self) -> web::HttpResponse {
//         let hash_map = self
//             .0
//             .errors()
//             .iter()
//             .filter_map(|(k, v)| {
//                 if let ValidationErrorsKind::Field(errors) = v {
//                     let vec = errors.into_iter().map(|f| {
//                         let code = f.code.to_string();
//                         match &f.message {
//                             Some(msg) => msg.to_string(),
//                             None => code
//                         }
//                     }).collect();
//                     Some((*k, vec))
//                 } else {
//                     None
//                 }
//             })
//             .collect::<HashMap<_, _>>();

//         let e = Errors { errors: hash_map };
//         web::HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(e)
//     }
// }

// fn validator_error_message(validation_errors : ValidationErrors) {

//     // errors.into_errors()

// }
