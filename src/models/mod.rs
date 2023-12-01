use serde::{Deserialize, Serialize};

pub mod comment;
pub mod post;
pub mod user;

pub trait Controller {
    type Model: Model;
    fn model(&self) -> Self::Model;
}

pub trait Model {
    type Controller: Controller;

    fn controller(&self) -> Self::Controller;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    russian: String,
    english: String,
    uzbek: String,
}
