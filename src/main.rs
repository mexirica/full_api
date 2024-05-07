
use std::env;

use actix_web::{App, get, HttpServer, Responder, web};
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use utoipa::{
    openapi::security::{HttpAuthScheme,HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema
};

use utoipa_swagger_ui::SwaggerUi;

use repository::uow;
use uow::UnitOfWork;

mod auth;
mod models;
mod repository;
mod routes;
mod services;
mod docs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = connect().await;
    let uow = UnitOfWork::new(pool);

    #[derive(OpenApi)]
    #[openapi(
        paths(
        health,
        routes::auth::login
        ),
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearerAuth", SecurityScheme::Http(HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()),
            );
            components.add_security_scheme(
                "basicAuth", SecurityScheme::Http(HttpBuilder::new()
                    .scheme(HttpAuthScheme::Basic)
                    .build()),
            );
        }
    }

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(uow.clone()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone()
            ))
            .service(health)
            .configure(routes::users::configure::handler)
            .configure(routes::auth::configure::handler)
            .configure(routes::supplier::configure::handler)

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
#[utoipa::path(
responses (
(status = 200, description = "Is working!"),
(status = 500, description = "Internal server error.")
))]
#[get("/health")]
pub async fn health() -> impl Responder {
    "Is working!"
}