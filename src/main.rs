use actix_web::{App, HttpServer};
use handler::users::user_scope;

mod handler;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_scope()))
        .run()
        .await
}
