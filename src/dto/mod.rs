use crate::models::{comment::CommentModel, post::PostModel, user::UserModel};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublishCommentDTO {
    pub content: String,
    pub author: UserModel,
    pub replys_for: Option<CommentModel>,
    pub for_post: PostModel,
}
