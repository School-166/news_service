use actix_web::{get, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQueryParams {
    limit: Option<u8>,
    page: u32,
    tags: Vec<String>,
    sort_by: Option<SortBy>,
    direction: Option<SortDirection>,
}

#[derive(Deserialize)]
enum SortDirection {
    Increment,
    Decrement,
}

#[derive(Deserialize)]
enum SortBy {
    Popularity,
    TimeOfRelease,
    Raiting,
}

#[get("/search")]
async fn search(query: actix_web::web::Query<SearchQueryParams>) -> impl Responder {
    todo!()
}
