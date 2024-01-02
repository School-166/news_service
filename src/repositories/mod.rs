use self::{comments::CommentsRepo, posts::PostsRepo};
use crate::{prelude::Resource, types::EditedState};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

pub mod comments;
pub(super) mod marks_repo;
pub mod posts;
pub mod users;

impl FromRow<'_, PgRow> for EditedState {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(if row.get::<bool, &str>("edited") {
            EditedState::Edited {
                edited_at: row.get("edited_at"),
            }
        } else {
            EditedState::NotEdited
        })
    }
}

pub async fn find_resources(uuid: Uuid) -> Option<Box<dyn Resource>> {
    if let Some(comment) = CommentsRepo::get_instance().await.get_by_uuid(&uuid).await {
        return Some(Box::new(comment));
    }
    if let Some(post) = PostsRepo::get_instance().await.get_by_uuid(uuid).await {
        return Some(Box::new(post));
    }
    None
}
