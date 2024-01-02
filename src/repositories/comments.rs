use crate::{
    controllers::{users::UserController, Controller},
    dto::PublishCommentDTO,
    get_db_pool,
    models::comment::CommentModel,
    prelude::ToSQL,
    utils::sql::SelectRequestBuilder,
};
use sqlx::{types::Uuid, PgPool};

#[derive(Clone)]
pub struct CommentsRepo(PgPool);

#[derive(Debug)]
pub enum PublishError {
    WrittenByNoone,
    WrittenUnderUnexistedPost,
    WrongLanguageCode,
}

impl CommentsRepo {
    pub async fn get_instance() -> CommentsRepo {
        CommentsRepo(get_db_pool().await)
    }

    pub async fn publish_comment(
        &self,
        comment: PublishCommentDTO,
    ) -> Result<CommentModel, PublishError> {
        let uuid = Uuid::new_v4();
        let published_at = chrono::Utc::now();

        sqlx::query(
            r#"
        INSERT INTO comments (
            uuid,
            written_under,
            content,
            published_at,
            written_by,
            likes,
            dislikes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, 0)
        "#,
        )
        .bind(uuid)
        .bind(comment.for_post.uuid())
        .bind(comment.content)
        .bind(published_at)
        .bind(comment.author.username())
        .execute(&self.0)
        .await
        .unwrap();
        Ok(self.get_by_uuid(uuid).await.unwrap())
    }

    pub async fn edit_comment(
        &self,
        comment: CommentModel,
        updated_content: String,
        author: &UserController,
    ) -> Result<CommentModel, sqlx::Error> {
        let edited_at = chrono::Utc::now();

        sqlx::query(
            r#"
        UPDATE comments
        SET content = $1, edited = true, edited_at = $2
        WHERE id = $3 and author = $4;
        "#,
        )
        .bind(updated_content)
        .bind(edited_at)
        .bind(comment.uuid())
        .bind(author.model().await.username())
        .execute(&self.0)
        .await?;

        Ok(self.get_by_uuid(comment.uuid()).await.unwrap())
    }

    pub async fn get_many(&self, query: Vec<GetCommentQueryParam>) -> Vec<CommentModel> {
        let sql = SelectRequestBuilder::<(), _>::new(
            "select 
                            comments.uuid,
                            comments.written_under,
                            comments.content,
                            comments.published_at,
                            comments.edited,
                            comments.edited_at,
                            comments.author,
                            comments.replys_for,
                                        
                count(comment_mark.liked = true) as likes,                  
                count(comment_mark.liked = false) as dislikes
             from comments join comment_mark on comments.uuid = comment_marks.comment"
                .to_string(),
            query,
        )
        .group_by(
            "comments.uuid,
             comments.written_under,
             comments.content,
             comments.published_at,
             comments.edited,
             comments.edited_at,
             comments.author,
             comments.replys_for
                                        
            "
            .to_string(),
        )
        .build();

        let rows = sqlx::query(&sql)
            .fetch_all(&self.0)
            .await
            .map_or(Vec::new(), |comments| comments);
        let mut comments = Vec::new();
        for model in rows {
            comments.push(CommentModel::from_row(model).await)
        }
        comments
    }

    pub async fn get_by_uuid(&self, uuid: Uuid) -> Option<CommentModel> {
        self.get_many(vec![GetCommentQueryParam::Uuid(uuid)])
            .await
            .first()
            .map_or(None, |comment| Some((*comment).clone()))
    }
}

pub enum GetCommentQueryParam {
    Uuid(Uuid),
    Post(Uuid),
    Replies(Uuid),
    User(String),
}

impl ToSQL for GetCommentQueryParam {
    fn to_sql(&self) -> String {
        match self {
            GetCommentQueryParam::Uuid(uuid) => {
                format!("uuid = '{}'", uuid)
            }
            GetCommentQueryParam::Post(post_uuid) => {
                format!("written_under = '{}'", post_uuid)
            }
            GetCommentQueryParam::Replies(comment_uuid) => {
                format!("replys_for = '{}'", comment_uuid)
            }
            GetCommentQueryParam::User(username) => {
                format!("author = '{}'", username)
            }
        }
    }
}
