use crate::{
    controllers::{
        users::{SingError, UserController},
        Controller,
    },
    dto::{PublishPostDTO, PublishPostJSON, SingDTO, UserRegistrationDTO},
    repositories::{
        find_resources,
        posts::PostsRepo,
        users::{queries::ChangeQueryParam, UserRepo},
    },
    utils::logger::Logger,
};
use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
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

fn log_sing_error(logger: &dyn Logger, username: &str, err: &SingError) {
    logger.error(&format!(
        "can't sing to {}. {}",
        username,
        serde_json::to_string(&err).unwrap()
    ));
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

fn log_uuid_generating_error(logger: &dyn Logger, uuid_string: &str) {
    logger.error(&format!("can't build uuid from \"{}\"", uuid_string))
}

fn log_resource_getting_error(logger: &dyn Logger, uuid: &Uuid) {
    logger.error(&format!(
        "can't find resource by uuid = {}",
        uuid.to_string()
    ))
}

#[post("/{resource_uuid}/liked/{liked}")]
async fn mark(
    path: Path<(String, bool)>,
    json: Json<SingDTO>,
    logger: Data<dyn Logger>,
) -> impl Responder {
    let (resource_uuid, liked) = (*path).clone();
    let sing_data = json.clone();
    let user_controller = match UserController::sing(&sing_data).await {
        Ok(controller) => controller,
        Err(err) => {
            log_sing_error(logger.as_ref(), &sing_data.username, &err);
            return HttpResponse::BadRequest().json(err);
        }
    };
    let resource_uuid = match Uuid::from_str(&resource_uuid) {
        Ok(uuid) => uuid,
        Err(_) => {
            log_uuid_generating_error(logger.as_ref(), &resource_uuid);
            return HttpResponse::BadRequest().finish();
        }
    };
    let resource = match find_resources(resource_uuid).await {
        Some(resource) => resource,
        None => {
            log_resource_getting_error(logger.as_ref(), &resource_uuid);
            return HttpResponse::NotFound().finish();
        }
    };
    if liked {
        user_controller.like(resource.as_ref()).await
    } else {
        user_controller.dislike(resource.as_ref()).await
    }
    HttpResponse::Accepted().finish()
}

#[derive(Deserialize, Clone)]
pub struct CommentJSON {
    content: String,
}

#[post("/comment/{resource_uuid}")]
async fn row(
    path: Path<String>,
    json: Json<(CommentJSON, SingDTO)>,
    logger: Data<dyn Logger>,
) -> impl Responder {
    let resource_uuid = path.clone();
    let (comment, sing_data) = json.clone();
    logger.info(&format!(
        "started comment query by {}",
        sing_data.username.clone()
    ));
    let controller = match UserController::sing(&sing_data).await {
        Ok(controller) => controller,
        Err(err) => {
            log_sing_error(logger.as_ref(), &sing_data.username, &err);
            return HttpResponse::BadRequest().json(err);
        }
    };
    let resource_uuid = match Uuid::from_str(&resource_uuid) {
        Ok(uuid) => uuid,
        Err(_) => {
            log_uuid_generating_error(logger.as_ref(), &resource_uuid);
            return HttpResponse::BadRequest().finish();
        }
    };
    let resource = match find_resources(resource_uuid).await {
        Some(post) => post,
        None => {
            log_resource_getting_error(logger.as_ref(), &resource_uuid);
            return HttpResponse::NotFound().finish();
        }
    };
    controller.comment(resource.as_ref(), comment.content).await;
    logger.info(&format!(
        "resource: {}, commented succesfully",
        resource_uuid
    ));
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
