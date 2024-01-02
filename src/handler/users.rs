use crate::{
    controllers::{users::UserController, Controller},
    dto::{PublishPostDTO, PublishPostJSON, SingDTO, UserRegistrationDTO},
    repositories::{
        find_resources,
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
        .service(mark)
        .service(row)
        .service(register)
}

#[get("/")]
async fn get_user(sing_dto: Json<SingDTO>) -> impl Responder {
    match UserController::sing(&sing_dto).await {
        Ok(user) => HttpResponse::Ok().json(user.model().await),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[post("/")]
async fn register(publish_dto: Json<UserRegistrationDTO>) -> impl Responder {
    match UserRepo::get_instance()
        .await
        .register(publish_dto.clone())
        .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[post("/post")]
async fn publish_post(publish_dto: Json<(PublishPostJSON, SingDTO)>) -> impl Responder {
    let (publish_dto, sing_data) = publish_dto.clone();
    let author = UserController::sing(&sing_data).await;
    let dto = match author {
        Ok(author) => PublishPostDTO {
            content: publish_dto.content.clone(),
            title: publish_dto.title.clone(),
            author: author.model().await,
        },
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::Created().json(PostsRepo::get_instance().await.publish(dto).await.unwrap())
}

#[post("/{resource_uuid}/liked/{liked}")]
async fn mark(path: Path<(String, bool)>, json: Json<SingDTO>) -> impl Responder {
    let (post, liked) = (*path).clone();
    let sing_data = json.clone();
    let controller = match UserController::sing(&sing_data).await {
        Ok(user) => user,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    let uuid = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let post = match find_resources(uuid).await {
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
async fn row(path: Path<String>, json: Json<(CommentJSON, SingDTO)>) -> impl Responder {
    let post = path.clone();
    let (content, sing_data) = json.clone();
    let controller = match UserController::sing(&sing_data).await {
        Ok(controller) => controller,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    let resource_uuid = match Uuid::from_str(&post) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let comment = match find_resources(resource_uuid).await {
        Some(post) => post,
        None => return HttpResponse::NotFound().finish(),
    };
    controller.comment(comment, content.content).await;
    HttpResponse::Accepted().finish()
}

#[patch("/change")]
async fn change_param(params: Json<(Vec<ChangeQueryParam>, SingDTO)>) -> impl Responder {
    let (params, sing_data) = params.clone();
    let controller = match UserController::sing(&sing_data).await {
        Ok(controller) => controller,
        Err(err) => return HttpResponse::BadRequest().json(err),
    };
    match controller.change_parameters(params).await {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(errors) => HttpResponse::BadRequest().json(errors),
    }
}
