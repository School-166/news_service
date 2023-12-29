use crate::{models::user::UserModel, prelude::Markable};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub mod comments;
pub mod posts;

pub trait MarkableRepoMethods {
    type Markable: Clone + Send + Markable + Sync;

    fn pool(&self) -> &PgPool;
    fn table() -> String;
    fn markable_column() -> String;

    async fn cancel_mark_method(
        &self,
        user: UserModel,
        markable: Self::Markable,
    ) -> Result<(), ()> {
        let sql = format!(
            "delete {} where username = $1 and {} = $2;",
            Self::table(),
            Self::markable_column()
        );
        if let Err(_) = sqlx::query(&sql)
            .bind(user.username())
            .bind(markable.uuid())
            .execute(self.pool())
            .await
        {
            return Err(());
        }
        Ok(())
    }

    async fn mark(&self, user: UserModel, markable: Self::Markable, liked: bool) {
        if self
            .is_marked_by(user.clone(), markable.clone(), None)
            .await
        {
            self.cancel_mark_method(user.clone(), markable.clone())
                .await
                .unwrap();
        }
        let sql = format!(
            "insert into {} (uuid, username, {}, mark) values($1, $2, $3, $4);",
            Self::table(),
            Self::markable_column()
        );
        let uuid = Uuid::new_v4();
        sqlx::query(&sql)
            .bind(uuid)
            .bind(user.username())
            .bind(markable.uuid())
            .bind(liked)
            .execute(self.pool())
            .await
            .unwrap();
    }

    async fn is_marked_by(
        &self,
        user: UserModel,
        markable: Self::Markable,
        mark: Option<bool>,
    ) -> bool {
        let sql = format!(
            "select count(*) from {} where {} = $1 and username = $2{};",
            Self::table(),
            Self::markable_column(),
            match mark {
                Some(liked) => format!(" and liked = {}", liked),
                None => String::new(),
            }
        );
        sqlx::query(&sql)
            .bind(markable.uuid())
            .bind(user.username())
            .fetch_one(self.pool())
            .await
            .unwrap()
            .get::<i32, &str>("count(*)")
            != 0
    }
}

pub trait MarkAbleRepo: MarkableRepoMethods + Sync {
    async fn get_instance() -> Self;

    async fn like(&self, user: UserModel, markable: Self::Markable) {
        self.mark(user, markable, true).await
    }

    async fn dislike(&self, user: UserModel, markable: Self::Markable) {
        self.mark(user, markable, false).await
    }

    async fn is_liked_by(&self, user: UserModel, markable: Self::Markable) -> bool {
        self.is_marked_by(user, markable, Some(true)).await
    }

    async fn is_disliked_by(&self, user: UserModel, markable: Self::Markable) -> bool {
        self.is_marked_by(user, markable, Some(false)).await
    }

    async fn cancel_mark(&self, user: UserModel, markable: Self::Markable) -> Result<(), ()> {
        self.cancel_mark_method(user, markable).await
    }
}
