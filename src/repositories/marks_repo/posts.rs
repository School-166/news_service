use super::*;
use crate::{get_db_pool, models::post::PostModel};
use sqlx::PgPool;

pub struct PostsMarkRepo(PgPool);

impl MarkableRepoMethods for PostsMarkRepo {
    type Markable = PostModel;
    fn pool(&self) -> &PgPool {
        &self.0
    }

    fn table() -> String {
        "posts_mark".to_string()
    }

    fn markable_column() -> String {
        "post".to_string()
    }
}

impl MarkAbleRepo for PostsMarkRepo {
    fn get_instance() -> Self {
        PostsMarkRepo(futures::executor::block_on(get_db_pool()))
    }
}
