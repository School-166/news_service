use async_trait::async_trait;

pub mod users;

#[async_trait]
pub trait Controller {
    type Model;
    async fn model(&self) -> Self::Model;
}
