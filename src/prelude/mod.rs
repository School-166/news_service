use uuid::Uuid;

use crate::models::user::UserModel;

pub trait QueryInterpreter {
    type Query: ToSQL;

    fn build_sql(queryes: Vec<Self::Query>) -> String {
        format!(
            "{} where {} {};",
            Self::query(),
            match queryes.first() {
                Some(query) => query.to_sql(),
                None => return Self::query(),
            },
            queryes
                .iter()
                .map(|query| format!(" and {}", query.to_sql()))
                .collect::<String>()
        )
    }

    fn query() -> String;
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
    async fn like(&self, user: UserModel);
    async fn dislike(&self, user: UserModel);
    fn uuid(&self) -> Uuid;
}

pub trait MarkableFromRow {
    async fn likes_count(&self) -> u64;
    async fn dislikes_count(&self) -> u64;
    fn uuid(&self) -> Uuid;
}
