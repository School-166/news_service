// use actix_web::{get, Responder};
use crate::{repositories::posts::OrderParam, types::SortDirection};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQueryParams {
    limit: Option<u8>,
    page: u32,
    tags: Vec<String>,
    sort_by: Option<OrderParam>,
    direction: Option<SortDirection>,
}

// #[get("/search")]
// async fn search(query: actix_web::web::Query<SearchQueryParams>) -> impl Responder {
//     todo!()
// }
