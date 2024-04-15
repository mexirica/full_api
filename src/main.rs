use std::env;

use actix_web::{App, get, HttpServer, Responder, web};
use dotenv::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;

mod auth;
mod models;
mod repository;
mod routes;
mod telemetry;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connect().await.expect("Erro ao se conectar no DB.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::users::configure::handler)
            .configure(routes::auth::configure::handler)
            .configure(routes::fornecedor::configure::handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    dotenv().ok();
    let base_path = env::current_dir().expect("Failed to determine the current directory");
    let database_url = base_path.join("api.db");
    let options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}
