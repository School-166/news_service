use crate::{
    get_db_pool,
    models::post::PostModel,
    prelude::{QueryInterpreter, ToSQL},
};
use chrono::NaiveDateTime;
use sqlx::{
    postgres::{PgPool, PgRow},
    types::Uuid,
    FromRow,
};

#[derive(Debug)]
pub struct PostFromRow {
    id: Uuid,
    title: String,
    content: String,
    published_at: NaiveDateTime,
    edited: bool,
    edited_at: Option<NaiveDateTime>,
    written_by: String,
    likes: i64,
    dislikes: i64,
}

pub struct PostsRepo(&'static PgPool);

impl PostsRepo {
    pub async fn get_instance() -> PostsRepo {
        PostsRepo(get_db_pool().await)
    }

    pub async fn publish_post(&self, post: PostModel) -> Result<PostModel, sqlx::Error> {
        let published_at = chrono::Utc::now().naive_utc();

        sqlx::query(
            r#"
            INSERT INTO posts (
                id,
                title,
                content,
                published_at,
                author
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(post.uuid())
        .bind(post.title().clone())
        .bind(post.content().clone())
        .bind(published_at)
        .bind(post.author().username().clone())
        .execute(self.0)
        .await?;

        Ok(post)
    }

    pub async fn get_one(&self, query: Vec<GetPostQueryParam>) -> Option<PostModel> {
        self.get_many(query)
            .await
            .first()
            .map(|post| (*post).clone())
    }

    pub async fn get_many(&self, query: Vec<GetPostQueryParam>) -> Vec<PostModel> {
        let from_row_posts = sqlx::query_as(&Self::build_sql(query))
            .fetch_all(self.0)
            .await
            .map_or(vec![], |post| post);
        let mut models = Vec::new();
        for from_row_model in from_row_posts {
            models.push(PostModel::from_row(from_row_model).await);
        }
        models
    }
}

pub enum GetPostQueryParam {
    ByUuid(Uuid),
    ForAuthor(String),
}

impl QueryInterpreter for PostsRepo {
    type Query = GetPostQueryParam;

    fn query() -> String {
        "select * from posts where ".to_string()
    }
}

impl ToSQL for GetPostQueryParam {
    fn to_sql(&self) -> String {
        match self {
            GetPostQueryParam::ByUuid(uuid) => {
                format!("id = '{}'", uuid)
            }
            GetPostQueryParam::ForAuthor(username) => {
                format!("author = '{}'", username)
            }
        }
    }
}

impl FromRow<'_, PgRow> for PostFromRow {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        todo!()
    }
}
