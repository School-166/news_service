use super::Controller;
use crate::{
    models::{post::PostModel, Model},
    repositories::posts::PostsRepo,
};
use uuid::Uuid;

pub struct PostController {
    uuid: Uuid,
}

impl Controller for PostController {
    type Model = PostModel;

    async fn model(&self) -> Self::Model {
        PostsRepo::get_instance()
            .await
            .get_by_uuid(self.uuid.clone())
            .await
            .unwrap()
    }
}

impl Model for PostModel {
    type Controller = PostController;

    fn controller(self) -> Self::Controller {
        PostController { uuid: self.uuid() }
    }
}