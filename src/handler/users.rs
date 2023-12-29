use crate::{
    controllers::{users::UserController, Controller},
    dto::{PublishPostDTO, PublishPostJSON, SingDTO, UserRegistrationDTO},
    models::Model,
    repositories::{
        comments::{CommentsRepo, GetCommentQueryParam},
        posts::{GetPostQueryParam, PostsRepo},
        users::UserRepo,
    },
};
use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

pub fn user_scope() -> Scope {
    Scope::new("/users")
        .service(get_user)
        .service(publish_post)
        .service(dislike_comment)
        .service(dislike_post)
        .service(like_comment)
        .service(like_post)
        .service(comment_comment)
        .service(comment_post)
        .service(register)
}

#[get("/")]
async fn get_user(sing_dto: Json<SingDTO>) -> impl Responder {
    match UserController::sing(sing_dto.username.clone(), sing_dto.password.clone()).await {
        Ok(user) => HttpResponse::Ok().json(user.model().await),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[post("/")]
async fn register(publish_dto: Json<UserRegistrationDTO>) -> impl Responder {
    match UserRepo::get_instance()
        .await
        .register_user(publish_dto.clone())
        .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[post("/{username}/post")]
async fn publish_post(
    username: Path<String>,
    publish_dto: Json<PublishPostJSON>,
) -> impl Responder {
    let author = UserRepo::get_instance()
        .await
        .get_by_username((*username).clone())
        .await;
    let dto = match author {
        Some(author) => PublishPostDTO {
            content: (*publish_dto).content.clone(),
            title: (*publish_dto).title.clone(),
            author,
        },
        None => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::Created().json(
        PostsRepo::get_instance()
            .await
            .publish_post(dto)
            .await
            .unwrap(),
    )
}

#[derive(Serialize)]
enum MarkErrors {
    UserNotFound,
    MarkableNotFound,
}

#[post("/{username}/like/comment/{comment_uuid}")]
async fn like_comment(path: Path<(String, String)>) -> impl Responder {
    let (username, comment) = (*path).clone();
    let user = match UserRepo::get_instance()
        .await
        .get_by_username(username)
        .await
    {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(MarkErrors::UserNotFound),
    };
    let comment = match Uuid::from_str(&comment) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let comment = match CommentsRepo::get_instance()
        .await
        .get_one(vec![GetCommentQueryParam::ByUuid(comment)])
        .await
    {
        Some(comment) => comment,
        None => return HttpResponse::NotFound().json(MarkErrors::MarkableNotFound),
    };
    user.controller().like(comment).await;
    HttpResponse::Accepted().finish()
}

#[post("/{username}/dislike/comment/{comment_uuid}")]
async fn dislike_comment(path: Path<(String, String)>) -> impl Responder {
    let (username, comment) = (*path).clone();
    let user = match UserRepo::get_instance()
        .await
        .get_by_username(username)
        .await
    {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(MarkErrors::UserNotFound),
    };
    let comment = match Uuid::from_str(&comment) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let comment = match CommentsRepo::get_instance()
        .await
        .get_one(vec![GetCommentQueryParam::ByUuid(comment)])
        .await
    {
        Some(comment) => comment,
        None => return HttpResponse::NotFound().json(MarkErrors::MarkableNotFound),
    };
    user.controller().dislike(comment).await;
    HttpResponse::Accepted().finish()
}

#[post("/{username}/like/post/{post_uuid}")]
async fn like_post(path: Path<(String, String)>) -> impl Responder {
    let (username, post) = (*path).clone();
    let user = match UserRepo::get_instance()
        .await
        .get_by_username(username)
        .await
    {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(MarkErrors::UserNotFound),
    };
    let post = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let post = match PostsRepo::get_instance()
        .await
        .get_one(vec![GetPostQueryParam::ByUuid(post)])
        .await
    {
        Some(post) => post,
        None => return HttpResponse::NotFound().json(MarkErrors::MarkableNotFound),
    };
    user.controller().like(post).await;
    HttpResponse::Accepted().finish()
}

#[post("/{username}/dislike/post/{post_uuid}")]
async fn dislike_post(path: Path<(String, String)>) -> impl Responder {
    let (username, post) = (*path).clone();
    let user = match UserRepo::get_instance()
        .await
        .get_by_username(username)
        .await
    {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(MarkErrors::UserNotFound),
    };
    let post = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let post = match PostsRepo::get_instance()
        .await
        .get_one(vec![GetPostQueryParam::ByUuid(post)])
        .await
    {
        Some(post) => post,
        None => return HttpResponse::NotFound().json(MarkErrors::MarkableNotFound),
    };
    user.controller().dislike(post).await;
    HttpResponse::Accepted().finish()
}

#[derive(Deserialize)]
pub struct CommentJSON {
    content: String,
}

#[post("/{username}/comment/comment/{comment_uuid}")]
async fn comment_comment(path: Path<(String, String)>, json: Json<CommentJSON>) -> impl Responder {
    let (username, comment) = (*path).clone();
    let content = json.content.clone();
    let user = match UserRepo::get_instance()
        .await
        .get_by_username(username)
        .await
    {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(MarkErrors::UserNotFound),
    };
    let comment = match Uuid::from_str(&comment) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let comment = match CommentsRepo::get_instance()
        .await
        .get_one(vec![GetCommentQueryParam::ByUuid(comment)])
        .await
    {
        Some(comment) => comment,
        None => return HttpResponse::NotFound().json(MarkErrors::MarkableNotFound),
    };
    user.controller().comment(comment, content).await;
    HttpResponse::Accepted().finish()
}

#[post("/{username}/comment/post/{post_uuid}")]
async fn comment_post(path: Path<(String, String)>, json: Json<CommentJSON>) -> impl Responder {
    let (username, post) = (*path).clone();
    let content = json.content.clone();
    let user = match UserRepo::get_instance()
        .await
        .get_by_username(username)
        .await
    {
        Some(user) => user,
        None => return HttpResponse::NotFound().json(MarkErrors::UserNotFound),
    };
    let post = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let post = match PostsRepo::get_instance()
        .await
        .get_one(vec![GetPostQueryParam::ByUuid(post)])
        .await
    {
        Some(post) => post,
        None => return HttpResponse::NotFound().json(MarkErrors::MarkableNotFound),
    };
    user.controller().comment(post, content).await;
    HttpResponse::Accepted().finish()
}
