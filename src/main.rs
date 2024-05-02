use std::env;

use actix_web::{App, get, HttpServer, Responder, web};
use actix_web::web::Data;
use dotenv::dotenv;
use repository::uow;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;
use uow::UnitOfWork;

mod auth;
mod models;
mod repository;
mod routes;
mod telemetry;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connect().await;
    let uow = UnitOfWork::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(uow.clone()))
            .route("/health", web::get().to(|| async { "Is working!" }))
            .configure(routes::users::configure::handler)
            .configure(routes::auth::configure::handler)
            .configure(routes::fornecedor::configure::handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
async fn connect() -> SqlitePool {
    dotenv().ok();
    let base_path = env::current_dir().expect("Failed to determine the current directory");
    let database_url = base_path.join("api.db");
    let options = SqliteConnectOptions::new()
       .filename(database_url)
       .create_if_missing(true);

    SqlitePool::connect_with(options).await.expect("Failed to connect to DB")
}

