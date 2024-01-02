use crate::{
    controllers::{users::UserController, Controller},
    dto::PublishPostDTO,
    get_db_pool,
    models::post::PostModel,
    prelude::{SortingDirection, ToSQL},
    types::Limit,
    utils::sql::SelectRequestBuilder,
};
use serde::Deserialize;
use sqlx::{postgres::PgPool, types::Uuid};

#[derive(Deserialize)]
enum SortBy {
    Popularity,
    TimeOfRelease,
    Raiting,
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

    pub async fn edit_content(&self, post: PostModel, content: &str, author: &UserController) {
        sqlx::query(
            "update posts set content = $1, edited = true, edited_at = now()  where uuid = $2 and author = $3;",
        )
        .bind(content)
        .bind(post.uuid())
            .bind(author.model().await.username())
        .execute(&self.0)
        .await
        .unwrap();
    }
    pub async fn edit_title(&self, post: PostModel, title: &str, author: &UserController) {
        sqlx::query(
            "update posts set title = $1, edited = true, edited_at = now() where uuid = $2 and author = $3;",
        )
        .bind(title)
        .bind(post.uuid())
            .bind(author.model().await.username())
        .execute(&self.0)
        .await
        .unwrap();
    }

    pub async fn get_by_uuid(&self, uuid: Uuid) -> Option<PostModel> {
        match sqlx::query("select * from posts where uuid = $1;")
            .bind(uuid)
            .fetch_one(&self.0)
            .await
        {
            Ok(row) => Some(PostModel::from_row(&row).await),
            Err(_) => None,
        }
    }

    pub async fn get_many(
        &self,
        query: Vec<GetQueryParam>,
        limit: Limit,
        order_by: SortingDirection<SortingParam>,
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
                div(count(post_mark.liked = true), count(post_mark.liked = true) + count(post_mark.liked = false) + 1) as raiting,
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
        let rows = sqlx::query(&sql)
            .fetch_all(&self.0)
            .await
            .map_or(vec![], |post| post);
        let mut models = Vec::new();
        for row in rows {
            models.push(PostModel::from_row(&row).await);
        }
        models
    }
}

pub enum GetQueryParam {
    Uuid(Uuid),
    Author(String),
    Tags(Vec<String>),
}

#[derive(Deserialize, Clone)]
pub enum SortingParam {
    Raiting,
    ReleaseTime,
}

impl ToSQL for SortingParam {
    fn to_sql(&self) -> String {
        match self {
            SortingParam::Raiting => "raiting",
            SortingParam::ReleaseTime => "published_at",
        }
        .to_string()
    }
}

impl ToSQL for GetQueryParam {
    fn to_sql(&self) -> String {
        match self {
            GetQueryParam::Uuid(uuid) => {
                format!("uuid = '{}'", uuid)
            }
            GetQueryParam::Author(username) => {
                format!("author = '{}'", username)
            }
            GetQueryParam::Tags(tags) => {
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
