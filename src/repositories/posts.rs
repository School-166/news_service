use crate::{
    dto::PublishPostDTO,
    get_db_pool,
    models::post::PostModel,
    prelude::{OrderingDirection, ToSQL},
    types::{EditedState, Limit},
    utils::sql::SelectRequestBuilder,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::{
    postgres::{PgPool, PgRow},
    types::Uuid,
    FromRow, Row,
};

#[derive(Deserialize)]
enum SortBy {
    Popularity,
    TimeOfRelease,
    Raiting,
}

#[derive(Debug, Clone)]
pub struct PostFromRow {
    uuid: Uuid,
    title: String,
    content: String,
    published_at: NaiveDateTime,
    edited: EditedState,
    author: String,
    likes_count: i64,
    dislikes_count: i64,
    tags: Vec<String>,
    raiting: f32,
}

impl PostFromRow {
    pub fn likes_count(&self) -> u64 {
        self.likes_count as u64
    }
    pub fn dislikes_count(&self) -> u64 {
        self.dislikes_count as u64
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }
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
    pub fn raiting(&self) -> f32 {
        self.raiting
    }
}

pub struct PostsRepo(PgPool);

impl PostsRepo {
    pub async fn get_instance() -> PostsRepo {
        PostsRepo(get_db_pool().await)
    }

    pub async fn publish(&self, post: PublishPostDTO) -> Result<PostModel, sqlx::Error> {
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

        Ok(self.get_by_uuid(uuid).await.unwrap())
    }

    pub async fn get_by_uuid(&self, uuid: Uuid) -> Option<PostModel> {
        match sqlx::query_as("select * from posts where uuid = $1;")
            .bind(uuid)
            .fetch_one(&self.0)
            .await
        {
            Ok(post) => Some(PostModel::from_row(post).await),
            Err(_) => None,
        }
    }

    pub async fn get_many(
        &self,
        query: Vec<GetPostQueryParam>,
        limit: Limit,
        order_by: OrderingDirection<OrderParam>,
    ) -> Vec<PostModel> {
        let sql = SelectRequestBuilder::new(
            "select posts.uuid,
                posts.title,
                posts.content,
                posts.published_at,
                posts.author, 
                posts.edited ,
                posts.edited_at,
                posts.tags,
                div(count(post_mark.liked = true), count(post_mark.liked = true) + count(post_mark.liked = false)) as raiting,
                count(post_mark.liked = true) as likes,                  
                count(post_mark.liked = false) as dislikes
                from posts join post_mark on posts.uuid = post_mark.post"
                .to_string(),
            query,
        )
        .limit(limit)
        .order_by(order_by)
        .group_by(
            "
                posts.uuid,
                posts.content,
                posts.title,
                posts.published_at,
                posts.author,
                posts.edited,
                posts.edited_at,
                posts.tags
            
            "
            .to_string(),
        ).build();
        let from_row_posts = sqlx::query_as(&sql)
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
    Uuid(Uuid),
    Author(String),
    Tags(Vec<String>),
}

#[derive(Deserialize)]
pub enum OrderParam {
    Raiting,
    ReleaseTime,
}

impl ToSQL for OrderParam {
    fn to_sql(&self) -> String {
        match self {
            OrderParam::Raiting => "raiting",
            OrderParam::ReleaseTime => "published_at",
        }
        .to_string()
    }
}

impl ToSQL for GetPostQueryParam {
    fn to_sql(&self) -> String {
        match self {
            GetPostQueryParam::Uuid(uuid) => {
                format!("uuid = '{}'", uuid)
            }
            GetPostQueryParam::Author(username) => {
                format!("author = '{}'", username)
            }
            GetPostQueryParam::Tags(tags) => {
                let tag_string;
                tag_string = {
                    if tags.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "{}{}",
                            tags[0],
                            tags[1..tags.len()]
                                .iter()
                                .map(|tag| format!(", {}", tag))
                                .collect::<String>()
                        )
                    }
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
            likes_count: row.get("likes"),
            raiting: row.get("raiting"),
            dislikes_count: row.get("dislikes"),
        })
    }
}
