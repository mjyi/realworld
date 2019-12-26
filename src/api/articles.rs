use crate::{
    auth::Auth,
    db::{
        article::{self, Article, ArticleForm, ArticleQuery, ArticleUpdate},
        comment::{self, Comment},
    },
    errors::Errors,
    Pool,
};
use actix_web::{http::StatusCode, web, Error, HttpResponse, Result};
use serde::{ Serialize, Deserialize };
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct NewArticle {
    article: NewArticleData,
}

#[derive(Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleData {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct NewComment {
    comment: NewCommentData,
}

#[derive(Deserialize)]
struct NewCommentData {
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleResult {
    article: Article
}

impl ArticleResult {
    pub fn new(article: Article) -> Self {
        ArticleResult { article }
    }
}


#[derive(Serialize,Deserialize)]
pub struct CommentResult {
    comment: Comment
}

impl CommentResult {
    pub fn new(comment: Comment) -> Self {
        CommentResult { comment }
    }
}

#[get("/articles")]
pub async fn list_articles(
    query: web::Query<ArticleQuery>,
    pool: web::Data<Pool>,
    auth: Option<Auth>,
) -> Result<HttpResponse, Error> {
    let user_id = auth.map(|a| a.claims.id);
    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::list_articles(&conn, &query, user_id)
    })
    .await
    .map_err(|e| Errors::from(e).set_code(StatusCode::OK))?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/articles/feed")]
pub async fn feed_articles(
    query: web::Query<ArticleQuery>,
    pool: web::Data<Pool>,
    auth: Auth,
) -> Result<HttpResponse, Error> {

    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::feed(&conn, &query, auth.claims.id)
    })
    .await
    .map_err(|e| Errors::from(e).code(StatusCode::OK))?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/articles/{slug}")]
pub async fn get_article(
    info: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let slug = info.into_inner();
    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::get_article(&conn, &slug)
    })
    .await
    .map(ArticleResult::new)
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/articles")]
pub async fn create_article(
    new_article: web::Json<NewArticle>,
    auth: Auth,
    pool: web::Data<Pool>,
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
    .map(ArticleResult::new)
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(article))
}

#[put("articles/{slug}")]
pub async fn update_article(
    auth: Auth,
    info: web::Path<String>,
    article: web::Json<ArticleUpdate>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let mut article = article.into_inner().article;
    let slug = info.into_inner();
    if let Some(ref title) = article.title {
        article.slug = Some(slug::slugify(title));
    }
    let user_id = auth.claims.id;

    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::update(&conn, &slug, user_id, &article)
    })
    .await
    .map(ArticleResult::new)
    .map_err(Errors::from)?;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("articles/{slug}")]
pub async fn delete_article(
    auth: Auth,
    info: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let slug = info.into_inner();
    let user_id = auth.claims.id;

    web::block(move || {
        let conn = pool.get().unwrap();
        article::delete(&conn, user_id, &slug)
    })
    .await
    .map_err(Errors::from)?;

    Ok(HttpResponse::new(StatusCode::OK))
}

#[post("articles/{slug}/comments")]
pub async fn add_comment(
    info: web::Path<String>,
    auth: Auth,
    comment: web::Json<NewComment>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_id = auth.claims.id;
    let slug = info.into_inner();
    let body = comment.comment.body.clone();

    let result = web::block(move || {
        let conn = pool.get().unwrap();
        comment::add_comment(&conn, user_id, &slug, &body)
    })
    .await
    .map(CommentResult::new)
    .map_err(Errors::from)?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/articles/{slug}/comments")]
pub async fn get_comments(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_id = auth.claims.id;
    let slug = info.into_inner();
    let result = web::block(move || {
        let conn = pool.get().unwrap();
        comment::get_comments(&conn, user_id, &slug)
    })
    .await
    .map_err(Errors::from)?;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/articles/{slug}/comments/{id}")]
pub async fn delete_comment(
    info: web::Path<(String, i32)>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let (slug, comment_id) = (info.0.clone(), info.1);
    let user_id = auth.claims.id;
    web::block(move || {
        let conn = pool.get().unwrap();
        comment::delete_comment(&conn, user_id, &slug, comment_id)
    })
    .await
    .map_err(Errors::from)?;

    Ok(HttpResponse::new(StatusCode::OK))
}

#[post("/articles/{slug}/favorite")]
pub async fn favorite(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let slug = info.into_inner();
    let user_id = auth.claims.id;
    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::favorite(&conn, user_id, &slug)
    })
    .await
    .map(ArticleResult::new)
    .map_err(Errors::from)?;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/articles/{slug}/favorite")]
pub async fn unfavorite(
    info: web::Path<String>,
    auth: Auth,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let slug = info.into_inner();
    let user_id = auth.claims.id;
    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::unfavorite(&conn, user_id, &slug)
    })
    .await
    .map(ArticleResult::new)
    .map_err(Errors::from)?;
    Ok(HttpResponse::Ok().json(result))
}


#[derive(Serialize, Deserialize)]
pub struct TagsResult {
    tags: Vec<String>
}

#[get("/tags")]
pub async fn tags(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let conn = pool.get().unwrap();
        article::tag_list(&conn)
    })
    .await
    .map(|tags| TagsResult{tags})
    .map_err(Errors::from)?;

    Ok(HttpResponse::Ok().json(result))
}
