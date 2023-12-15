use super::{comment::CommentModel, user::UserModel};
use crate::repositories::posts::PostFromRow;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostModel {
    id: String,
    title: String,
    content: String,
    published_at: NaiveDateTime,
    edited: bool,
    edited_at: Option<NaiveDateTime>,
    written_by: String,
    written_on: String,
    likes: i64,
    dislikes: i64,
    comments: Vec<CommentModel>,
}

impl PostModel {
    pub fn uuid(&self) -> Uuid {
        Uuid::from_str(&self.id).unwrap()
    }

    pub(crate) async fn from_row(from_row_model: PostFromRow) -> Self {
        todo!()
    }

    pub fn author(&self) -> UserModel {
        todo!()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
}
