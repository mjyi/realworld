use crate::{auth::Auth, db::*, errors::Errors, Pool};
use actix_web::{web, Error, HttpResponse, Result};
use serde::Deserialize;

#[get("/profiles/{username}")]
pub(crate) async fn get_profiles(
    auth: Option<Auth>,
    info: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let username = info.into_inner();

    let user_id = auth.map(|auth| auth.claims.id);

    let profile = web::block(move || {
        let conn = pool.get().unwrap();
        let user = User::with_username(&conn, &username)?;
        Profile::get_proflies(&conn, &username, user_id)
    })
    .await
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(profile))
}

#[post("/profiles/{username}/follow")]
pub(crate) async fn follow(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    todo!()
}

#[delete("profiles/{username}/follow")]
pub(crate) async fn unfollow(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    todo!()
}
