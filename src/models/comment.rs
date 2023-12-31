use super::{post::PostModel, user::UserModel};
use crate::{
    repositories::{
        comments::{CommentsRepo, GetCommentQueryParam},
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
    dislikes: u64,
    edited: EditedState,
    likes: u64,
    published_at: NaiveDateTime,
    replys_for: Option<String>,
    under_post: PostModel,
    uuid: String,
}

impl CommentModel {
    #[async_recursion]
    pub async fn from_row(row: PgRow) -> CommentModel {
        let uuid = row.get("uuid");
        let comments = CommentsRepo::get_instance()
            .await
            .get_many(vec![GetCommentQueryParam::Replies(uuid.clone())])
            .await;
        CommentModel {
            uuid,
            content: row.get("content"),
            published_at: row.get("published_at"),
            edited: EditedState::from_row(&row),
            author: UserRepo::get_instance()
                .await
                .get_by_username(row.get("author"))
                .await
                .unwrap(),
            likes: row.get("likes"),
            dislikes: row.get("dislikes"),
            comments,
            replys_for: row
                .get("replys_for")
                .replys_for()
                .map(|uuid| uuid.to_string()),
            under_post: PostsRepo::get_instance()
                .await
                .get_by_uuid(row.get("under_post"))
                .await
                .unwrap(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.uuid).unwrap()
    }

    pub fn contet(&self) -> String {
        self.content.clone()
    }

    pub fn under_post(&self) -> PostModel {
        self.under_post.clone()
    }
}
