use std::env;

use actix_web::{App, HttpServer, Responder, web};
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use utoipa::OpenApi;


use repository::uow;
use uow::UnitOfWork;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod models;
mod repository;
mod routes;
mod telemetry;
mod services;
mod docs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = connect().await;
    let uow = UnitOfWork::new(pool);
    let openapi = docs::OpenApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(uow.clone()))
            .route("/health", web::get().to(|| async { "Is working!" }))
            .configure(routes::users::configure::handler)
            .configure(routes::auth::configure::handler)
            .configure(routes::supplier::configure::handler)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()))
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

