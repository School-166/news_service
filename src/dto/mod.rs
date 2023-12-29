use crate::models::{
    comment::CommentModel,
    post::PostModel,
    user::{UserModel, UserType},
};
use chrono::NaiveDate;
use serde::Deserialize;

pub struct PublishCommentDTO {
    pub content: String,
    pub author: UserModel,
    pub replys_for: Option<CommentModel>,
    pub for_post: PostModel,
}

#[derive(Deserialize)]
pub struct PublishPostJSON {
    pub title: String,
    pub content: String,
}

pub struct PublishPostDTO {
    pub content: String,
    pub author: UserModel,
    pub title: String,
}

#[derive(Clone, Deserialize)]
pub struct UserRegistrationDTO {
    pub username: String,
    pub last_name: String,
    pub first_name: String,
    pub birth_date: NaiveDate,
    pub user_specs: UserType,
    pub about: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Clone, Deserialize)]
pub struct SingDTO {
    pub username: String,
    pub password: String,
}
