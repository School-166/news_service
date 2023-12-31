use super::{post::PostModel, user::UserModel};
use crate::{
    controllers::users::UserController,
    dto::PublishCommentDTO,
    prelude::{Commentable, Editable, Markable, PublishDTOBuilder, Resource},
    repositories::{
        comments::CommentsRepo,
        marks_repo::{comments::CommentsMarkRepo, MarkAbleRepo},
        posts::PostsRepo,
        users::UserRepo,
    },
    types::EditedState,
};
use async_recursion::async_recursion;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize)]
pub struct CommentModel {
    author: UserModel,
    comments: Vec<CommentModel>,
    content: String,
    dislikes: i64,
    edited: EditedState,
    likes: i64,
    published_at: NaiveDateTime,
    replys_for: Option<String>,
    post: PostModel,
    uuid: String,
}

impl CommentModel {
    #[async_recursion]
    pub async fn from_row(row: &PgRow) -> CommentModel {
        let uuid: Uuid = row.get("uuid");

        let post = PostsRepo::get_instance()
            .await
            .get_by_uuid(row.get("under_post"))
            .await
            .unwrap();
        let comments = CommentsRepo::get_instance()
            .await
            .get_by_post(&post.uuid())
            .await;

        CommentModel {
            uuid: uuid.to_string(),
            content: row.get("content"),
            published_at: row.get("published_at"),
            edited: EditedState::from_row(&row).unwrap(),
            author: UserRepo::get_instance()
                .await
                .get_by_username(row.get("author"))
                .await
                .unwrap(),
            likes: row.get("likes"),
            dislikes: row.get("dislikes"),
            comments,
            replys_for: row.get("replys_for"),
            post,
        }
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.uuid).unwrap()
    }

    pub fn contet(&self) -> String {
        self.content.clone()
    }

    pub fn under_post(&self) -> PostModel {
        self.post.clone()
    }
}

impl PublishDTOBuilder for CommentModel {
    fn build_dto(
        &self,
        content: String,
        author: crate::models::user::UserModel,
    ) -> PublishCommentDTO {
        PublishCommentDTO {
            content,
            author,
            replys_for: Some(self.clone()),
            for_post: self.under_post(),
        }
    }
}

impl Commentable for CommentModel {}

impl Editable for CommentModel {
    fn edit(&self, content: &str, user: &UserController) {
        futures::executor::block_on(async {
            CommentsRepo::get_instance()
                .await
                .edit(self.clone(), content.to_string(), user)
                .await
                .unwrap();
        })
    }
}

impl Markable for CommentModel {
    fn like(&self, user: &UserController) {
        futures::executor::block_on(async {
            CommentsMarkRepo::get_instance().like(&user, self).await
        })
    }

    fn dislike(&self, user: &UserController) {
        futures::executor::block_on(async {
            CommentsMarkRepo::get_instance().dislike(user, &self).await
        })
    }

    fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.uuid).unwrap()
    }
}

impl Resource for CommentModel {
    fn author(&self) -> UserModel {
        self.author.clone()
    }
}
