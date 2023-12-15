use super::{post::PostModel, user::UserModel};
use crate::{
    repositories::{
        comments::{CommentFromRow, CommentsRepo, GetCommentQueryParam},
        posts::{GetPostQueryParam, PostsRepo},
        users::{queries::GetByQueryParam, UserRepo},
    },
    types::EditedState,
};
use async_recursion::async_recursion;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentModel {
    author: UserModel,
    comments: Vec<CommentModel>,
    content: String,
    dislikes: i64,
    edited: EditedState,
    likes: i64,
    published_at: NaiveDateTime,
    replys_for: Option<String>,
    under_post: PostModel,
    uuid: String,
}

impl CommentModel {
    #[async_recursion]
    pub async fn from_row(model: CommentFromRow) -> CommentModel {
        let comments = CommentsRepo::get_instance()
            .await
            .get_many(vec![GetCommentQueryParam::WithReplies(model.uuid())])
            .await;
        CommentModel {
            uuid: model.uuid().to_string(),
            content: model.content(),
            published_at: model.published_at(),
            edited: model.edited(),
            author: UserRepo::get_instance()
                .await
                .get_one_by(vec![GetByQueryParam::Username(model.author())])
                .await
                .unwrap(),
            likes: model.likes(),
            dislikes: model.dislikes(),
            comments,
            replys_for: model.replys_for().map(|uuid| uuid.to_string()),
            under_post: PostsRepo::get_instance()
                .await
                .get_one(vec![GetPostQueryParam::ByUuid(model.under_post())])
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
