use super::{users::UserController, Controller};
use crate::{
    dto::PublishCommentDTO,
    models::comment::CommentModel,
    prelude::{Commentable, Editable, Markable, PublishDTOBuilder, Resource},
    repositories::{
        comments::CommentsRepo,
        marks_repo::{comments::CommentsMarkRepo, MarkAbleRepo},
    },
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
            .get_by_uuid(self.uuid.clone())
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

impl PublishDTOBuilder for CommentController {
    async fn build_dto(
        &self,
        content: String,
        author: crate::models::user::UserModel,
    ) -> PublishCommentDTO {
        PublishCommentDTO {
            content,
            author,
            replys_for: Some(self.model().await),
            for_post: self.model().await.under_post(),
        }
    }
}

impl Commentable for CommentController {}

impl Editable for CommentController {
    async fn edit(&self, content: &str) {
        let _ = CommentsRepo::get_instance()
            .await
            .edit_comment(self.model().await, content.to_string())
            .await;
    }
}

impl Markable for CommentController {
    async fn like(&self, user: &UserController) {
        CommentsMarkRepo::get_instance()
            .await
            .like(user.model().await, self.model().await)
            .await
    }

    async fn dislike(&self, user: &UserController) {
        CommentsMarkRepo::get_instance()
            .await
            .dislike(user.model().await, self.model().await)
            .await
    }

    fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}

impl Resource for CommentController {}
