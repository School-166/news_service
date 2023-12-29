use super::*;
use crate::{get_db_pool, models::comment::CommentModel, repositories::comments::CommentFromRow};
use sqlx::PgPool;

pub struct CommentsMarkRepo(PgPool);

impl MarkableRepoMethods for CommentsMarkRepo {
    type Markable = CommentModel;
    type FromRowModel = CommentFromRow;
    fn pool(&self) -> &PgPool {
        &self.0
    }

    fn table() -> String {
        "commets_mark".to_string()
    }

    fn markable_column() -> String {
        "comment".to_string()
    }
}

impl MarkAbleRepo for CommentsMarkRepo {
    async fn get_instance() -> Self {
        CommentsMarkRepo(get_db_pool().await)
    }
}
