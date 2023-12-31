use crate::{
    dto::SortDirectionDTO,
    prelude::SortingDirection,
    repositories::posts::{GetQueryParam, PostsRepo, SortingParam},
    types::Limit,
};
use actix_web::{get, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchQueryParams {
    limit: Option<u8>,
    page: u32,
    tags: Vec<String>,
    sort_by: Option<SortingParam>,
    direction: Option<SortDirectionDTO>,
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
        .map_or(SortDirectionDTO::Increment, |direction| direction);
    let sort_by = query
        .sort_by
        .clone()
        .map_or(SortingParam::Raiting, |params| params);
    let ordering_param = match sort_direction {
        SortDirectionDTO::Increment => SortingDirection::Up(sort_by),
        SortDirectionDTO::Decrement => SortingDirection::Down(sort_by),
    };
    let responce = PostsRepo::get_instance()
        .await
        .get_many(vec![GetQueryParam::Tags(tags)], limit, ordering_param)
        .await;

    HttpResponse::Ok().json(responce)
}
