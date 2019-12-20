extern crate jsonwebtoken as jwt;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jwt::{decode, Validation};
use serde::{Deserialize, Serialize};

pub type Jwt = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    pub exp: i64,
}

#[derive(Debug)]
pub struct Auth {
    pub jwt: Jwt,
    pub claims: Claims,
}

impl FromRequest for Auth {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let headers = req.headers();
        if !headers.contains_key(AUTHORIZATION) {
            return err(ErrorUnauthorized(""));
        }

        let token = headers.get(AUTHORIZATION).unwrap().to_str().unwrap_or("");
        let prefix = "Token ";

        if token.starts_with(prefix) {
            let jwt = &token[prefix.len()..];

            if let Ok(claims) = Claims::decode(jwt.to_owned(), "secret") {
                let auth = Auth {
                    jwt: jwt.to_owned(),
                    claims,
                };
                return ok(auth);
            }
        }

        err(ErrorUnauthorized("error unauthorized"))
    }
}

impl Claims {
    pub fn decode(jwt: Jwt, secret: &str) -> Result<Self, jwt::errors::Error> {
        let data = decode::<Claims>(&jwt, secret.as_ref(), &Validation::default())?;
        Ok(data.claims)
    }
}
