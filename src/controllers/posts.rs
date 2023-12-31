use super::Controller;
use crate::{models::post::PostModel, repositories::posts::PostsRepo};
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
