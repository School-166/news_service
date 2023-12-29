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
    async fn get_instance() -> Self {
        PostsMarkRepo(get_db_pool().await)
    }
}
