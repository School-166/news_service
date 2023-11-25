use actix_web::{App, HttpServer};
use async_once::AsyncOnce;
use handler::users::user_scope;
use lazy_static::lazy_static;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod handler;
mod models;

lazy_static! {
    static ref DB_POOL: AsyncOnce<PgPool> = AsyncOnce::new(async { establish_connection().await });
    static ref PORT: u16 = dotenv::var("PORT")
        .expect("PORT must be set in .env file")
        .parse()
        .expect("PORT must be a number");
    static ref DB_ADDRES: String = dotenv::var("DB_ADDRES").expect("DB_ADDRES must be set");
}

pub async fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_scope()))
        .bind(("", PORT.to_owned()))?
        .run()
        .await
}
pub async fn establish_connection() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&DB_ADDRES)
        .await
        .expect("maybe path to database wrong")
}
