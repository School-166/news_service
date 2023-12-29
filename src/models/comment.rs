use super::{post::PostModel, user::UserModel};
use crate::{
    prelude::{Markable, MarkableFromRow},
    repositories::{
        comments::{CommentFromRow, CommentsRepo, GetCommentQueryParam},
        marks_repo::{comments::CommentsMarkRepo, MarkAbleRepo},
        posts::{GetPostQueryParam, PostsRepo},
        users::UserRepo,
    },
    types::EditedState,
};
use async_recursion::async_recursion;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::types::Uuid;
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

impl Markable for CommentModel {
    async fn like(&self, user: UserModel) {
        CommentsMarkRepo::get_instance()
            .await
            .like(user, self.clone())
            .await
    }

    async fn dislike(&self, user: UserModel) {
        CommentsMarkRepo::get_instance()
            .await
            .dislike(user, self.clone())
            .await
    }

    fn uuid(&self) -> Uuid {
        self.uuid()
    }
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
                .get_by_username(model.author())
                .await
                .unwrap(),
            likes: model.likes_count().await,
            dislikes: model.dislikes_count().await,
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
