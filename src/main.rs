use actix_web::{App, HttpServer};
use handler::users::user_scope;
use models::user::UserRepo;
use sqlx::PgPool;

mod handler;
mod models;

static (USER_REPO, POSTS_REPO) =  

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_scope()))
        .run()
        .await
}
pub fn establish_connection() -> (UserRepo, _) {
    todo!()
}
