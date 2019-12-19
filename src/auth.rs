use actix_web::{ web, dev, App, Error, HttpRequest, FromRequest};
use actix_web::http::HeaderMap;
use actix_web::http::header::AUTHORIZATION;
use actix_web::error::ErrorUnauthorized;
use futures::future::{ ok, err, Ready };
use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    pub exp: i64,
}


impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
            let headers = req.headers();
            if !headers.contains_key(AUTHORIZATION) {
                return err(ErrorUnauthorized(""))
            } 
            
            let token = headers.get(AUTHORIZATION).unwrap().to_str().unwrap_or("");
            let prefix = "Token ";
            
            if token.starts_with(prefix) {
                let token = &token[prefix.len()..];
                ok(Claims{id: 12, username: "abc".to_owned(), exp: 1233})
            } else {
                err(ErrorUnauthorized("error unauthorized"))
            }
    }
}


impl Claims {
    // pub fn decode(jwt: Jwt) -> Result<TokenData<Claims>, jwt::errors::Error> {
        
    // }

}

