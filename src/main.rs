use actix_web::{App, HttpServer, web};
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use crate::models::produto::Produto;
use crate::repository::find_all;

mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "postgres://@localhost:5432/api";
    let pool = PgPoolOptions::new().connect_lazy(url).expect("Erro ao se conectar ao Banco de Dados");

    find_all::<Produto>(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
