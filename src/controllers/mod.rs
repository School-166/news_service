pub mod comments;
pub mod posts;
pub mod users;

pub trait Controller {
    type Model;
    fn model(&self) -> impl std::future::Future<Output = Self::Model> + Send;
}
