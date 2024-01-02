use crate::{
    controllers::{users::UserController, Controller},
    dto::PublishCommentDTO,
    models::user::UserModel,
    repositories::comments::CommentsRepo,
};
use uuid::Uuid;

pub trait QueryInterpreter {
    type Query: ToSQL;

    fn build_sql(queryes: Vec<Self::Query>) -> String;
}

impl ToSQL for () {
    fn to_sql(&self) -> String {
        String::new()
    }
}

pub trait PublishDTOBuilder {
    fn build_dto(&self, content: String, author: UserModel) -> PublishCommentDTO;
}

pub trait Commentable: PublishDTOBuilder {
    fn comment(&self, content: String, author: &UserController) {
        futures::executor::block_on(async {
            CommentsRepo::get_instance()
                .await
                .publish_comment(self.build_dto(content, author.model().await))
                .await
                .unwrap();
        })
    }
}

pub enum EditError {
    EditsNotAuthor,
}

pub trait Editable {
    fn edit(&self, content: &str, user: &UserController);
}

pub trait Resource
where
    Self: Markable + Commentable + Editable + Sync,
{
    fn author(&self) -> UserModel;
}

pub enum SortingDirection<T>
where
    T: ToSQL,
{
    Up(T),
    Down(T),
}

impl<T> ToSQL for SortingDirection<T>
where
    T: ToSQL,
{
    fn to_sql(&self) -> String {
        match self {
            SortingDirection::Up(order) => format!("{} desc", order.to_sql()),
            SortingDirection::Down(order) => format!("{} asc", order.to_sql()),
        }
    }
}

pub trait ToSQL {
    fn to_sql(&self) -> String;
}

pub trait Validateble {
    type Validated;
    type Target;
    type ValidationError;
    fn validate(self, target: &Self::Target)
        -> Result<Self::Validated, Vec<Self::ValidationError>>;
}

pub trait Markable {
    fn like(&self, user: &UserController);
    fn dislike(&self, user: &UserController);
    fn uuid(&self) -> Uuid;
}
