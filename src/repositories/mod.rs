use sqlx::{postgres::PgRow, FromRow, Row};

use crate::types::EditedState;

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
