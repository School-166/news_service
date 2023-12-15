use serde::Deserialize;
use uuid::Uuid;

pub mod controller;

#[derive(Debug, Deserialize, Clone)]
pub struct Post {}

impl Post {
    pub fn uuid(&self) -> Uuid {
        todo!()
    }
}
