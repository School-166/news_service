use crate::models::user::UserModel;
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

pub enum OrderingDirection<T>
where
    T: ToSQL,
{
    Up(T),
    Down(T),
}

impl<T> ToSQL for OrderingDirection<T>
where
    T: ToSQL,
{
    fn to_sql(&self) -> String {
        match self {
            OrderingDirection::Up(order) => format!("{} desc", order.to_sql()),
            OrderingDirection::Down(order) => format!("{} asc", order.to_sql()),
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
    fn like(&self, user: UserModel) -> impl std::future::Future<Output = ()> + Send;
    fn dislike(&self, user: UserModel) -> impl std::future::Future<Output = ()> + Send;
    fn uuid(&self) -> Uuid;
}

pub trait MarkableFromRow {
    fn likes_count(&self) -> impl std::future::Future<Output = u64> + Send;
    fn dislikes_count(&self) -> impl std::future::Future<Output = u64> + Send;
    fn uuid(&self) -> Uuid;
}
