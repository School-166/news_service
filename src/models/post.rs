use super::{comment::CommentModel, user::UserModel, Commentable, PublishDTOBuilder};
use crate::{
    dto::PublishCommentDTO,
    prelude::{Markable, MarkableFromRow},
    repositories::{
        comments::{CommentsRepo, GetCommentQueryParam},
        marks_repo::{posts::PostsMarkRepo, MarkAbleRepo},
        posts::PostFromRow,
        users::{queries::GetByQueryParam, UserRepo},
    },
    types::EditedState,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(Debug, Serialize, Clone)]
pub struct PostModel {
    uuid: String,
    title: String,
    content: String,
    published_at: NaiveDateTime,
    edited: EditedState,
    author: String,
    likes: u64,
    dislikes: u64,
    comments: Vec<CommentModel>,
    tags: Vec<String>,
}

impl Markable for PostModel {
    async fn like(&self, user: UserModel) {
        PostsMarkRepo::get_instance()
            .await
            .like(user, self.clone())
            .await
    }

    async fn dislike(&self, user: UserModel) {
        PostsMarkRepo::get_instance()
            .await
            .dislike(user, self.clone())
            .await
    }

    fn uuid(&self) -> Uuid {
        PostModel::uuid(self)
    }
}

impl PublishDTOBuilder for PostModel {
    async fn build_dto(&self, content: String, author: UserModel) -> PublishCommentDTO {
        PublishCommentDTO {
            content,
            author,
            replys_for: None,
            for_post: self.clone(),
        }
    }
}

impl Commentable for PostModel {}

impl PostModel {
    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.uuid).unwrap()
    }

    pub(crate) async fn from_row(from_row_model: PostFromRow) -> Self {
        PostModel {
            uuid: from_row_model.uuid().to_string(),
            title: from_row_model.title(),
            content: from_row_model.content(),
            published_at: from_row_model.published_at(),
            edited: from_row_model.edited_state(),
            author: from_row_model.author(),
            likes: from_row_model.likes_count().await,
            dislikes: from_row_model.dislikes_count().await,
            comments: CommentsRepo::get_instance()
                .await
                .get_many(vec![GetCommentQueryParam::ForPost(from_row_model.uuid())])
                .await,
            tags: from_row_model.tags(),
        }
    }

    pub async fn author(&self) -> UserModel {
        UserRepo::get_instance()
            .await
            .get_one_by(vec![GetByQueryParam::Username(self.author.clone())])
            .await
            .unwrap()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}
