use crate::{auth::Auth, db::*, errors::Errors, Pool};
use actix_web::{web, Error, HttpResponse, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileResult {
    profile: Profile,
}

impl ProfileResult {
    pub fn new(profile: Profile) -> Self {
        ProfileResult{ profile }
    }
}


#[get("/profiles/{username}")]
pub async fn get_profiles(
    auth: Option<Auth>,
    info: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let username = info.into_inner();
    let user_id = auth.map(|auth| auth.claims.id);
    let profile = web::block(move || {
        let conn = pool.get().unwrap();
        Profile::get_proflies(&conn, &username, user_id)
    })
    .await
    .map(ProfileResult::new)
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(profile))
}

#[post("/profiles/{username}/follow")]
pub async fn follow(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let followed_name = info.into_inner();
    let follower = auth.claims.id;

    let profile = web::block(move || {
        let conn = pool.get().unwrap();
        Profile::follow(&conn, &followed_name, follower)
    })
    .await
    .map(ProfileResult::new)
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(profile))
}

#[delete("profiles/{username}/follow")]
pub async fn unfollow(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let followed_name = info.into_inner();
    let follower = auth.claims.id;

    let profile = web::block(move || {
        let conn = pool.get().unwrap();
        Profile::unfollow(&conn, &followed_name, follower)
    })
    .await
    .map(ProfileResult::new)
    .map_err(Errors::from)?;
    Ok(HttpResponse::Ok().json(profile))
}
