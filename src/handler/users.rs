use crate::{
    controllers::{users::UserController, Controller},
    dto::{PublishPostDTO, PublishPostJSON, SingDTO, UserRegistrationDTO},
    repositories::{
        find_commentable,
        posts::PostsRepo,
        users::{queries::ChangeQueryParam, UserRepo},
    },
};
use actix_web::{
    get, patch, post,
    web::{Json, Path},
    HttpResponse, Responder, Scope,
};
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

pub fn user_scope() -> Scope {
    Scope::new("/users")
        .service(get_user)
        .service(publish_post)
        .service(mark_comment)
        .service(mark_post)
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
    HttpResponse::Created().json(PostsRepo::get_instance().await.publish(dto).await.unwrap())
}

#[post("/comment/{comment_uuid}/liked/{liked}")]
async fn mark_comment(path: Path<(String, bool)>, json: Json<SingDTO>) -> impl Responder {
    let (comment, liked) = path.clone();
    let sing_data = json.clone();
    let controller = match UserController::sing(sing_data.username, sing_data.password).await {
        Ok(user) => user,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    let uuid = match Uuid::from_str(&comment) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let comment = match find_commentable(uuid).await {
        Some(comment) => comment,
        None => return HttpResponse::NotFound().finish(),
    };
    if liked {
        controller.like(comment).await
    } else {
        controller.dislike(comment).await
    }
    HttpResponse::Accepted().finish()
}

#[post("/{post_uuid}/liked/{liked}")]
async fn mark_post(path: Path<(String, bool)>, json: Json<SingDTO>) -> impl Responder {
    let (post, liked) = (*path).clone();
    let sing_data = json.clone();
    let controller = match UserController::sing(sing_data.username, sing_data.password).await {
        Ok(user) => user,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    let uuid = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let post = match find_commentable(uuid).await {
        Some(post) => post,
        None => return HttpResponse::NotFound().finish(),
    };
    if liked {
        controller.like(post).await
    } else {
        controller.dislike(post).await
    }
    HttpResponse::Accepted().finish()
}

#[derive(Deserialize, Clone)]
pub struct CommentJSON {
    content: String,
}

#[post("/comment/{commentable_uuid}")]
async fn comment_comment(path: Path<String>, json: Json<(CommentJSON, SingDTO)>) -> impl Responder {
    let post = path.clone();
    let (content, sing_data) = json.clone();
    let controller = match UserController::sing(sing_data.username, sing_data.password).await {
        Ok(controller) => controller,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    let comment = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let comment = match find_commentable() {
        Some(post) => post,
        None => return HttpResponse::NotFound().finish(),
    };
    controller.comment_resource(comment, content.content).await;
    HttpResponse::Accepted().finish()
}

#[post("/comment/post/{post_uuid}")]
async fn comment_post(path: Path<String>, json: Json<(CommentJSON, SingDTO)>) -> impl Responder {
    let post = path.clone();
    let (content, sing_data) = json.clone();
    let controller = match UserController::sing(sing_data.username, sing_data.password).await {
        Ok(controller) => controller,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    let post = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let post = match PostsRepo::get_instance().await.get_by_uuid(post).await {
        Some(post) => post,
        None => return HttpResponse::NotFound().finish(),
    };
    controller.comment_resource(post, content.content).await;
    HttpResponse::Accepted().finish()
}

#[patch("/change")]
async fn change_param(params: Json<(Vec<ChangeQueryParam>, SingDTO)>) -> impl Responder {
    let (params, sing_data) = params.clone();
    let controller = match UserController::sing(sing_data.username, sing_data.password).await {
        Ok(controller) => controller,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    match controller.change_parameters(params).await {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(errors) => HttpResponse::BadRequest().json(errors),
    }
}
