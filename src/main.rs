use actix_web::{App, HttpServer};
use async_once::AsyncOnce;
use handler::users::user_scope;
use lazy_static::lazy_static;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::handler::posts::posts_scope;

pub mod controllers;
pub mod dto;
mod handler;
pub mod models;
pub mod prelude;
pub mod repositories;
pub mod types;
pub mod utils;
pub mod validators;

lazy_static! {
    static ref DB_POOL: AsyncOnce<PgPool> = AsyncOnce::new(async { establish_connection().await });
    static ref PORT: u16 = dotenv::var("PORT")
        .expect("PORT must be set in .env file")
        .parse()
        .expect("PORT must be a number");
    static ref DB_ADDRES: String = dotenv::var("DB_ADDRES").expect("DB_ADDRES must be set");
}

pub async fn get_db_pool() -> PgPool {
    DB_POOL.get().await.clone()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| {
        println!("started");
        App::new().service(user_scope()).service(posts_scope())
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
pub async fn establish_connection() -> PgPool {
    PgPoolOptions::new()
        .connect(&DB_ADDRES)
        .await
        .expect("maybe path to database wrong")
}
