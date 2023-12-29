use self::user::UserModel;
use crate::{
    controllers::Controller, dto::PublishCommentDTO, repositories::comments::CommentsRepo,
};

pub mod comment;
pub mod post;
pub mod user;

pub trait Model {
    type Controller: Controller;

    fn controller(self) -> Self::Controller;
}

pub trait PublishDTOBuilder {
    fn build_dto(
        &self,
        content: String,
        author: UserModel,
    ) -> impl std::future::Future<Output = PublishCommentDTO> + Send;
}

pub trait Commentable: PublishDTOBuilder {
    fn comment(&self, content: String, author: UserModel) -> impl std::future::Future<Output = ()> {
        async {
            CommentsRepo::get_instance()
                .await
                .publish_comment(self.build_dto(content, author).await)
                .await
                .unwrap();
        }
    }
}
