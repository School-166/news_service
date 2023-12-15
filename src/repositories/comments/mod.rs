use crate::{
    dto::PublishCommentDTO,
    get_db_pool,
    models::{comment::CommentModel, user::UserModel, Commentable, PublishDTOBuilder},
    prelude::{QueryInterpreter, ToSQL},
    repositories::users::{queries::GetByQueryParam, UserRepo},
    types::EditedState,
};
use async_once::AsyncOnce;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use sqlx::{postgres::PgRow, types::Uuid, FromRow, PgPool, Row};

use super::posts::{GetPostQueryParam, PostsRepo};

lazy_static! {
    static ref COMMENTS_REPO: AsyncOnce<CommentsRepo> =
        AsyncOnce::new(async { CommentsRepo(get_db_pool().await) });
}

pub struct CommentsRepo(&'static PgPool);

#[derive(Debug)]
pub enum PublishError {
    WrittenByNoone,
    WrittenUnderUnexistedPost,
    WrongLanguageCode,
}

pub struct CommentFromRow {
    uuid: Uuid,
    content: String,
    published_at: NaiveDateTime,
    edited: EditedState,
    author: String,
    likes: i64,
    dislikes: i64,
    replys_for: Option<Uuid>,
    under_post: Uuid,
}

impl CommentFromRow {
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn author(&self) -> String {
        self.author.clone()
    }

    pub fn edited(&self) -> EditedState {
        self.edited.clone()
    }

    pub fn likes(&self) -> i64 {
        self.likes
    }

    pub fn dislikes(&self) -> i64 {
        self.dislikes
    }

    pub fn published_at(&self) -> NaiveDateTime {
        self.published_at.clone()
    }

    pub fn replys_for(&self) -> Option<Uuid> {
        self.replys_for.clone()
    }

    pub fn under_post(&self) -> Uuid {
        self.under_post.clone()
    }
}

#[async_trait]
impl PublishDTOBuilder for CommentModel {
    async fn build_dto(&self, content: String, author: UserModel) -> PublishCommentDTO {
        PublishCommentDTO {
            content,
            author,
            for_post: PostsRepo::get_instance()
                .await
                .get_one(vec![GetPostQueryParam::ByUuid(self.under_post().uuid())])
                .await
                .unwrap(),
            replys_for: Some(self.clone()),
        }
    }
}

impl Commentable for CommentModel {}

impl CommentsRepo {
    pub async fn get_instance() -> &'static CommentsRepo {
        COMMENTS_REPO.get().await
    }

    pub async fn publish_comment(
        &self,
        comment: PublishCommentDTO,
    ) -> Result<CommentModel, PublishError> {
        let uuid = Uuid::new_v4();
        let published_at = chrono::Utc::now();

        if UserRepo::get_instance()
            .await
            .get_one_by(vec![GetByQueryParam::Username(
                comment.author.username().clone(),
            )])
            .await
            .is_none()
        {
            return Err(PublishError::WrittenByNoone);
        }

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
        .execute(self.0)
        .await
        .unwrap();
        Ok(self
            .get_one(vec![GetCommentQueryParam::ByUuid(uuid)])
            .await
            .unwrap())
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
        WHERE id = $1
        "#,
        )
        .bind(comment.uuid())
        .bind(updated_content)
        .bind(edited_at)
        .execute(self.0)
        .await?;

        Ok(self
            .get_one(vec![GetCommentQueryParam::ByUuid(comment.uuid())])
            .await
            .unwrap())
    }

    pub async fn get_one(&self, query: Vec<GetCommentQueryParam>) -> Option<CommentModel> {
        self.get_many(query)
            .await
            .first()
            .map_or(None, |comment| Some((*comment).clone()))
    }

    pub async fn get_many(&self, query: Vec<GetCommentQueryParam>) -> Vec<CommentModel> {
        let models = sqlx::query_as(&Self::build_sql(query))
            .fetch_all(self.0)
            .await
            .map_or(Vec::new(), |comments| comments);
        let mut comments = Vec::new();
        for model in models {
            comments.push(CommentModel::from_row(model).await)
        }
        comments
    }
}

pub enum GetCommentQueryParam {
    ByUuid(Uuid),
    ForPost(Uuid),
    WithReplies(Uuid),
    ByUser(String),
}

impl QueryInterpreter for CommentsRepo {
    type Query = GetCommentQueryParam;

    fn query() -> String {
        "select * from comments where ".to_string()
    }
}

impl ToSQL for GetCommentQueryParam {
    fn to_sql(&self) -> String {
        match self {
            GetCommentQueryParam::ByUuid(uuid) => {
                format!("uuid = '{}'", uuid)
            }
            GetCommentQueryParam::ForPost(post_uuid) => {
                format!("written_under = '{}'", post_uuid)
            }
            GetCommentQueryParam::WithReplies(comment_uuid) => {
                format!("uuid = '{}'", comment_uuid)
            }
            GetCommentQueryParam::ByUser(username) => {
                format!("written_by = '{}'", username)
            }
        }
    }
}

impl FromRow<'_, PgRow> for CommentFromRow {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let content = row.get("content");
        let published_at = row.get("published_at");
        let edited = if row.get::<bool, &str>("edited") {
            EditedState::Edited {
                edited_at: row.get("edited_at"),
            }
        } else {
            EditedState::NotEdited
        };
        let author = row.get("written_by");
        let likes = row.get("likes");
        let dislikes = row.get("dislikes");
        let replys_for = row.get("reply_to");
        let under_post = row.get("under_post");
        Ok(CommentFromRow {
            uuid: row.get("id"),
            content,
            published_at,
            edited,
            author,
            likes,
            dislikes,
            replys_for,
            under_post,
        })
    }
}
