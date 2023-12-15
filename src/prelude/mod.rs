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
    type ValidationError;
    fn validate(self, target: &UserModel) -> Result<Self::Validated, Vec<Self::ValidationError>>;
}
