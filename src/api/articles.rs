use actix_web::{web, Error, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use crate::{
    db::article::{self, ArticleQuery, Articles, Article, ArticleForm},
    Pool,
    auth::Auth,
    errors::Errors,
};
use validator::Validate;


#[derive(Deserialize, Debug)]
pub struct NewArticle {
    article: NewArticleData
}

#[derive(Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleData {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>
}

#[get("/articles")]
pub async fn list_articles(
    query: web::Query<ArticleQuery>,
    pool: web::Data<Pool>,
    auth: Option<Auth>,
    ) -> Result<HttpResponse, Error> {
    todo!()
}


#[get("/articles/feed")]
pub async fn feed_articles(
    // query: web::Query<>
    pool: web::Data<Pool>,
    )-> Result<HttpResponse, Error> {
    todo!()
}


#[get("/articles/{slug}")]
pub async fn get_article(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    todo!()
}

#[post("/articles")]
pub async fn create_article(
    new_article: web::Json<NewArticle>,
    auth: Auth,
    pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let article = new_article.into_inner().article;
    article.validate().map_err(Errors::from)?;

    let slug = slug::slugify(&article.title);
    let article_form = ArticleForm {
        slug,
        title: article.title,
        description: article.description,
        body: article.body,
        tag_list: article.tag_list.unwrap_or(vec![]),
        author: auth.claims.id,
    };

    let article = web::block(move || {
        let conn = pool.get().unwrap();
        article::create(&conn, &article_form)
    })
    .await
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(json!({ "article": article })))
}

#[put("articles/{slug}")]
pub async fn update_article() -> Result<HttpResponse, Error> {
    todo!()
}


#[delete("articles/{slug}")]
pub async fn delete_article() -> Result<HttpResponse, Error> {
    todo!()
}


#[post("articles/{slug}/comments")]
pub async fn add_comment() -> Result<HttpResponse, Error> {
    todo!()
}

#[get("/articles/{slug}/comments")]
pub async fn get_comments() -> Result<HttpResponse, Error> {
    todo!()
}

#[delete("/articles/{slug}/comments/{id}")]
pub async fn delete_comment() -> Result<HttpResponse, Error> {
    todo!()
}

#[post("/articles/{slug}/favorite")]
pub async fn favorite() -> Result<HttpResponse, Error> {
    todo!()
}


#[delete("/articles/{slug}/favorite")]
pub async fn unfavorite() -> Result<HttpResponse, Error> {
    todo!()
}

#[get("/tags")]
pub async fn tags() -> Result<HttpResponse, Error> {
    todo!()
}

