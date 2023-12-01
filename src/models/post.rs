use chrono::NaiveDateTime;

#[derive(Debug)]
struct PostModel {
    id: ,
    title_on_russian: String,
    title_on_uzbek: String,
    title_on_english: String,
    content_on_russian: String,
    content_on_uzbek: String,
    content_on_english: String,
    published_at: NaiveDateTime,
    edited: bool,
    edited_at: Option<NaiveDateTime>,
    written_by: String,
    written_on: String,
    likes: i64,
    dislikes: i64,
}


#[derive(Debug)]
struct Post {
    id: String,
    title_on_russian: String,
    title_on_uzbek: String,
    title_on_english: String,
    content_on_russian: String,
    content_on_uzbek: String,
    content_on_english: String,
    published_at: NaiveDateTime,
    edited: bool,
    edited_at: Option<NaiveDateTime>,
    written_by: String,
    written_on: String,
    likes: i64,
    dislikes: i64,
    comments: Vec<Comment>
}
    

