use crate::{
    prelude::OrderingDirection,
    repositories::posts::{GetPostQueryParam, OrderParam, PostsRepo},
    types::{Limit, SortDirection},
};
use actix_web::{get, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQueryParams {
    limit: Option<u8>,
    page: u32,
    tags: Vec<String>,
    sort_by: Option<OrderParam>,
    direction: Option<SortDirection>,
}

#[get("/search")]
async fn search(query: actix_web::web::Query<SearchQueryParams>) -> impl Responder {
    let tags = query.tags.clone();
    let limit = query.limit.map_or(25, |limit| limit.into());
    let limit = Limit {
        limit,
        offset: Some(query.page.clone() * limit),
    };
    let sort_direction = query
        .direction
        .clone()
        .map_or(SortDirection::Increment, |direction| direction);
    let sort_by = query
        .sort_by
        .clone()
        .map_or(OrderParam::Raiting, |params| params);
    let ordering_param = match sort_direction {
        SortDirection::Increment => OrderingDirection::Up(sort_by),
        SortDirection::Decrement => OrderingDirection::Down(sort_by),
    };
    let responce = PostsRepo::get_instance()
        .await
        .get_many(vec![GetPostQueryParam::Tags(tags)], limit, ordering_param)
        .await;

    HttpResponse::Ok().json(responce)
}
