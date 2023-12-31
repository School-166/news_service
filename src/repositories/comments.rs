use super::posts::PostsRepo;
use crate::{
    dto::PublishCommentDTO,
    get_db_pool,
    models::{comment::CommentModel, user::UserModel},
    prelude::{Commentable, PublishDTOBuilder, ToSQL},
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

impl PublishDTOBuilder for CommentModel {
    async fn build_dto(&self, content: String, author: UserModel) -> PublishCommentDTO {
        PublishCommentDTO {
            content,
            author,
            for_post: PostsRepo::get_instance()
                .await
                .get_by_uuid(self.under_post().uuid())
                .await
                .unwrap(),
            replys_for: Some(self.clone()),
        }
    }
}

impl Commentable for CommentModel {}

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
    ) -> Result<CommentModel, sqlx::Error> {
        let edited_at = chrono::Utc::now();

        sqlx::query(
            r#"
        UPDATE comments
        SET content = $2, edited = true, edited_at = $3
        WHERE id = $1;
        "#,
        )
        .bind(comment.uuid())
        .bind(updated_content)
        .bind(edited_at)
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
