pub mod comments;
pub mod posts;
pub mod users;

pub trait Controller {
    type Model;
    async fn model(&self) -> Self::Model;
}
