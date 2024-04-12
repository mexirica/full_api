use std::env;
use std::error::Error;
use std::future::Future;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;
use crate::models::produto::Produto;

mod models;
mod repository;
mod auth;
mod telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connect().await.expect("Erro ao se conectar no DB.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve estar setada");

    let options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    SqlitePool::connect_with(options).await
}