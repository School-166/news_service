use super::*;
use crate::{get_db_pool, models::comment::CommentModel};
use sqlx::PgPool;

pub struct CommentsMarkRepo(PgPool);

impl MarkableRepoMethods for CommentsMarkRepo {
    type Markable = CommentModel;
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
    fn get_instance() -> Self {
        CommentsMarkRepo(futures::executor::block_on(get_db_pool()))
    }
}
