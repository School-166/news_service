use super::{comment::CommentModel, user::UserModel};
use crate::{
    controllers::users::UserController,
    dto::PublishCommentDTO,
    prelude::{Commentable, Editable, Markable, PublishDTOBuilder, Resource},
    repositories::{
        comments::{CommentsRepo, GetCommentQueryParam},
        marks_repo::{posts::PostsMarkRepo, MarkAbleRepo},
        posts::PostsRepo,
        users::{queries::GetByQueryParam, UserRepo},
    },
    types::EditedState,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Row};
use std::str::FromStr;

#[derive(Debug, Serialize, Clone)]
pub struct PostModel {
    uuid: String,
    title: String,
    content: String,
    published_at: NaiveDateTime,
    edited: EditedState,
    author: String,
    likes: i64,
    dislikes: i64,
    comments: Vec<CommentModel>,
    tags: Vec<String>,
    raiting: f32,
}

impl PostModel {
    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.uuid).unwrap()
    }

    pub(crate) async fn from_row(row: &PgRow) -> Self {
        let uuid: Uuid = row.get("uuid");

        PostModel {
            uuid: uuid.to_string(),
            title: row.get("title"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            edited: EditedState::from_row(&row).unwrap(),
            author: row.get("author"),
            tags: row.get("tags"),
            likes: row.get("likes"),
            raiting: row.get("raiting"),
            dislikes: row.get("dislikes"),
            comments: CommentsRepo::get_instance()
                .await
                .get_many(vec![GetCommentQueryParam::Post(uuid)])
                .await,
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

impl Markable for PostModel {
    fn like(&self, user: &UserController) {
        futures::executor::block_on(async { PostsMarkRepo::get_instance().like(user, self).await })
    }

    fn dislike(&self, user: &UserController) {
        futures::executor::block_on(async {
            PostsMarkRepo::get_instance().dislike(user, self).await
        })
    }

    fn uuid(&self) -> Uuid {
        self.uuid()
    }
}

impl Editable for PostModel {
    fn edit(&self, content: &str, user: &UserController) {
        futures::executor::block_on(async {
            PostsRepo::get_instance()
                .await
                .edit_content(self.clone(), content, user)
                .await;
        })
    }
}

impl PublishDTOBuilder for PostModel {
    fn build_dto(&self, content: String, author: UserModel) -> crate::dto::PublishCommentDTO {
        PublishCommentDTO {
            content,
            author,
            replys_for: None,
            for_post: self.clone(),
        }
    }
}

impl Commentable for PostModel {}

impl Resource for PostModel {
    fn author(&self) -> UserModel {
        futures::executor::block_on(PostModel::author(self))
    }
}
