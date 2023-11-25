use actix_web::{App, HttpServer};
use handler::users::user_scope;
use lazy_static::lazy_static;
use models::user::UserRepo;
use sqlx::PgPool;

mod handler;
mod models;

lazy_static! {
    static ref DB_POOL: PgPool = establish_connection();
}

pub fn get_db_pool()->PgPool{
    DB_POOL.clone()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_scope()))
        .run()
        .await
}
pub fn establish_connection() -> PgPool {
    todo!()
}
