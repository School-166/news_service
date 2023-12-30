use super::Controller;
use crate::{
    models::comment::CommentModel,
    repositories::comments::{CommentsRepo, GetCommentQueryParam},
};
use uuid::Uuid;

pub struct CommentController {
    uuid: Uuid,
}

impl Controller for CommentController {
    type Model = CommentModel;

    async fn model(&self) -> Self::Model {
        CommentsRepo::get_instance()
            .await
            .get_one(vec![GetCommentQueryParam::Uuid(self.uuid.clone())])
            .await
            .unwrap()
    }
}

impl CommentController {
    pub async fn edit_comment(&self, content: String) {
        let _ = CommentsRepo::get_instance()
            .await
            .edit_comment(self.model().await, content)
            .await;
    }
}