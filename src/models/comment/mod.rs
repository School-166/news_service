use super::Content;
use chrono::NaiveDateTime;
use sqlx::types::Uuid;
mod repository;

#[derive(Debug)]
struct CommentModel {
    id: Uuid,
    content: Content,
    published_at: NaiveDateTime,
    edited: bool,
    edited_at: Option<NaiveDateTime>,
    written_by: String,
    written_on: String,
    likes: i64,
    dislikes: i64,
    replys_for: Option<Uuid>, // References to the comment it is a reply to
}

pub struct PublishCommentsDTO {
    content: String,
    written_on: String,
    written_by: String,
    for_post: String,
}

#[derive(Debug)]
pub struct Comment {
    id: String,
    content: Content,
    published_at: NaiveDateTime,
    edited: bool,
    edited_at: Option<NaiveDateTime>,
    written_by: String,
    written_on: String,
    likes: i64,
    dislikes: i64,
    replys_for: Box<Option<Comment>>, // References to the comment it is a reply to
}
