use crate::{
    dto::PublishPostDTO,
    get_db_pool,
    models::post::PostModel,
    prelude::{MarkableFromRow, QueryInterpreter, ToSQL},
    types::EditedState,
};
use chrono::NaiveDateTime;
use sqlx::{
    postgres::{PgPool, PgRow},
    types::Uuid,
    FromRow, Row,
};

use super::marks_repo::{posts::PostsMarkRepo, MarkAbleRepo};

#[derive(Debug, Clone)]
pub struct PostFromRow {
    uuid: Uuid,
    title: String,
    content: String,
    published_at: NaiveDateTime,
    edited: EditedState,
    author: String,
    tags: Vec<String>,
}

impl MarkableFromRow for PostFromRow {
    async fn likes_count(&self) -> u64 {
        PostsMarkRepo::get_instance()
            .await
            .likes_count(self.clone())
            .await
    }
    async fn dislikes_count(&self) -> u64 {
        PostsMarkRepo::get_instance()
            .await
            .dislikes_count(self.clone())
            .await
    }

    fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}

impl PostFromRow {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn published_at(&self) -> NaiveDateTime {
        self.published_at.clone()
    }

    pub fn edited_state(&self) -> EditedState {
        self.edited.clone()
    }

    pub fn author(&self) -> String {
        self.author.clone()
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

pub struct PostsRepo(PgPool);

impl PostsRepo {
    pub async fn get_instance() -> PostsRepo {
        PostsRepo(get_db_pool().await)
    }

    pub async fn publish_post(&self, post: PublishPostDTO) -> Result<PostModel, sqlx::Error> {
        let published_at = chrono::Utc::now().naive_utc();
        let uuid = Uuid::new_v4();
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
        .bind(uuid)
        .bind(post.title.clone())
        .bind(post.content.clone())
        .bind(published_at)
        .bind(post.author.username().clone())
        .execute(&self.0)
        .await?;

        Ok(self
            .get_one(vec![GetPostQueryParam::ByUuid(uuid)])
            .await
            .unwrap())
    }

    pub async fn get_one(&self, query: Vec<GetPostQueryParam>) -> Option<PostModel> {
        self.get_many(query)
            .await
            .first()
            .map(|post| (*post).clone())
    }

    pub async fn get_many(&self, query: Vec<GetPostQueryParam>) -> Vec<PostModel> {
        let from_row_posts = sqlx::query_as(&Self::build_sql(query))
            .fetch_all(&self.0)
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
    ByTags(Vec<String>),
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
            GetPostQueryParam::ByTags(tags) => {
                let mut tag_string;
                let first = tags.first();
                tag_string = match first {
                    Some(first) => {
                        tags.remove(0);
                        if tags.is_empty() {
                            format!("{}", first)
                        } else {
                            format!(
                                "{}, {}",
                                first,
                                tags.iter()
                                    .map(|tag| format!(", {}", tag))
                                    .collect::<String>()
                            )
                        }
                    }
                    None => String::new(),
                };
                format!("tags @> \"{}{}{}\"", "{", tag_string, "}")
            }
        }
    }
}

impl FromRow<'_, PgRow> for PostFromRow {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(PostFromRow {
            uuid: row.get("uuid"),
            title: row.get("title"),
            content: row.get("content"),
            published_at: row.get("published_at"),
            edited: EditedState::from_row(row)?,
            author: row.get("author"),
            tags: row.get("tags"),
        })
    }
}
